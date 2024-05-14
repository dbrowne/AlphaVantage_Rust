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

use dotenvy::dotenv;
use serde_json;
use std::path::PathBuf;
use std::{fs, process};
use std::fs::File;
use std::io::{BufWriter, Write};
use AlphaVantage_Rust::alpha_lib::alpha_io::news_loader::process_news;
use AlphaVantage_Rust::alpha_lib::alpha_io::news_loader::Params;
use AlphaVantage_Rust::alpha_lib::news_type::NewsRoot;
use AlphaVantage_Rust::db_funcs::get_sids_and_names_with_overview;
use AlphaVantage_Rust::dbfunctions::author::get_authors;
use AlphaVantage_Rust::dbfunctions::base::establish_connection_or_exit;
use AlphaVantage_Rust::dbfunctions::sources::get_sources;
use AlphaVantage_Rust::dbfunctions::topic_refs::get_topics;


fn main(){
    loader();
}
fn loader() {
    const GLW_NEWS: &str = "/mnt/source1/djbGR/portfolio/AlphaVantage_Rust/src/bin/GLW_query.json";

    dotenv().ok();

    let current_dir = std::env::current_dir().unwrap();
    let mut tests_data_dir = PathBuf::from(&current_dir);
    tests_data_dir.push("tests");
    let mut file_path = PathBuf::from(&tests_data_dir);
    file_path.push(GLW_NEWS);
    let rawdta = fs::read_to_string(file_path).expect("Cannot read data file GLW_query.json");
    let dt: NewsRoot = serde_json::from_str(&rawdta).expect("Cannot parse JSON data");

    let conn = &mut establish_connection_or_exit();

    let results: Vec<(i64, String)> =
        get_sids_and_names_with_overview(conn).unwrap_or_else(|err| {
            println!("Cannot load results from database {}", err);
            process::exit(1);
        });

    let mut params = Params::default();
    let mut topics = get_topics(conn).unwrap();
    let mut authors = get_authors(conn).unwrap();
    let mut sources = get_sources(conn).unwrap();

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

    let sid = 5344;
    let mut symbol_log: BufWriter<File> = BufWriter::new(File::create("/tmp/symbol_log.txt").unwrap());
    _ = (process_news(conn, &sid, &"GLW".to_string(), dt, &mut params, &mut  symbol_log)).unwrap_or_else(|err| {
        println!("Cannot process news {}", err);
    });
    symbol_log.flush().unwrap();
}
