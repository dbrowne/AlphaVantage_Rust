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

#![allow(unexpected_cfgs)]
#[cfg(not(tarpaulin_include))]
use std::process;

use alpha_vantage_rust::{
  alpha_lib::{alpha_io::base::load_intraday, misc_functions::get_exe_name},
  db_funcs::{
    get_proc_id_or_insert, log_proc_end, log_proc_start,
  },
  dbfunctions::base::establish_connection_or_exit,
  security_types::sec_types::SecurityType,
};
use dotenvy::dotenv;
use indicatif::ProgressBar;
use alpha_vantage_rust::dbfunctions::combined::get_sids_and_names_with_overview;

fn main() {
  let conn = &mut establish_connection_or_exit();

  dotenv().ok();

  let id_val = get_proc_id_or_insert(conn, &get_exe_name()).unwrap();
  let pid = log_proc_start(conn, id_val).unwrap();
  let results: Vec<(i64, String)> = get_sids_and_names_with_overview(conn).unwrap_or_else(|err| {
    println!("Cannot load results from database {}", err);
    _ = log_proc_end(conn, pid, 3).unwrap();
    process::exit(1);
  });

  let progress_size = results.len() as u64;
  let bar = ProgressBar::new(progress_size);
  bar.set_style(
    indicatif::ProgressStyle::default_bar()
      .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
      .expect("Error setting progress bar style")
      .progress_chars("##-"),
  );

  for (sid, symbol) in results {
    bar.inc(1);

    if let Err(_err) = load_intraday(conn, &symbol, sid, SecurityType::Equity) {
      //todo: improve logging
      // println!("Error getting intraday prices {} for sid {}", err, sid);
      continue;
    }
  }
  bar.finish();
  _ = log_proc_end(conn, pid, 2).unwrap();
}
