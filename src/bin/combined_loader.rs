/*
 *
 *
 *
 *
 * MIT License
 * Copyright (c) 2024. Dwight J. Browne
 * dwight[-dot-]browne[-at-]dwightjbrowne[-dot-]com
 *
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#[cfg(not(tarpaulin_include))]
use std::{
  env::var,
  fs::File,
  io::{BufWriter, Write},
  process,
};

use alpha_vantage_rust::{
  alpha_lib::{
    alpha_io::news_loader::{load_news, Params},
    alpha_io_funcs::{load_intraday, load_summary},
    misc_functions::get_exe_name,
  },
  db_funcs::{
    get_proc_id_or_insert, get_sids_and_names_with_overview, log_proc_end, log_proc_start,
  },
  dbfunctions::{
    author::get_authors, base::establish_connection_or_exit, sources::get_sources,
    topic_refs::get_topics,
  },
};
use dotenvy::dotenv;
use indicatif::ProgressBar;
fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();
  let conn = &mut establish_connection_or_exit();

  let id_val = get_proc_id_or_insert(conn, &get_exe_name()).unwrap();

  let pid = log_proc_start(conn, id_val).unwrap();
  let results: Vec<(i64, String)> = get_sids_and_names_with_overview(conn).unwrap_or_else(|err| {
    println!("Cannot load results from database {}", err);
    _ = log_proc_end(conn, pid, 3).unwrap();
    process::exit(1);
  });

  let count_of_sids = results.len();

  let mut params = Params::default();
  let topics = get_topics(conn)?;
  let authors = get_authors(conn)?;
  let sources = get_sources(conn)?;

  for (sid, name) in results.iter() {
    params.names_to_sid.insert(name.clone(), *sid);
  }

  params.topics = topics.iter().map(|t| (t.name.clone(), t.id)).collect();
  params.authors = authors
    .iter()
    .map(|a| (a.author_name.clone(), a.id))
    .collect();
  params.sources = sources
    .iter()
    .map(|s| (s.source_name.clone(), s.id))
    .collect();
  let mut symbol_log: BufWriter<File> = BufWriter::new(File::create("/tmp/symbol_log.txt")?);

  let progress = ProgressBar::new(count_of_sids as u64);
  progress.set_style(
    indicatif::ProgressStyle::default_bar()
      .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
      .expect("Error setting progress bar style")
      .progress_chars("##-"),
  );

  for (s_id, symb) in results {
    let _news_status = load_news(conn, &s_id, &symb, &mut params, &mut symbol_log);
    if let Err(err) = load_intraday(conn, &symb, s_id) {
      //todo: improve logging
      // println!("Error getting intraday prices {} for sid {}", err, sid);
      continue;
    }
    if let Err(err) = load_summary(conn, &symb, s_id) {
      println!("Error loading open close prices {} for sid {}", err, symb);
    }

    progress.inc(1);
  }
  progress.finish_with_message("News loading complete");
  symbol_log.flush()?;
  progress.finish();
  _ = log_proc_end(conn, pid, 2).unwrap();
  Result::Ok(())
}
