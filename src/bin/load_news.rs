/*
 *
 *
 *
 *
 * MIT License
 * Copyright (c) 2023. Dwight J. Browne
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
use std::process;
use AlphaVantage_Rust::alpha_lib::alpha_io::news_loader::load_news;
use AlphaVantage_Rust::db_funcs::get_sids_and_names_with_overview;
use AlphaVantage_Rust::dbfunctions::base::establish_connection_or_exit;
fn main() {
    dotenv().ok();
    let conn = &mut establish_connection_or_exit();

    let results: Vec<(i64, String)> = get_sids_and_names_with_overview(conn)
        .unwrap_or_else(|err| {
            println!("Cannot load results from database {}", err);
            process::exit(1);
        }
        );


    for (s_id, symb) in results{
        let news_status = load_news(conn, &s_id,&symb);
        match news_status {
            Ok(_news) => println!("News loaded for {}: {}",s_id, symb),
            Err(err) => {
                eprintln!("Error loading news {} for {}", err,symb);
            }
        }
    }


}