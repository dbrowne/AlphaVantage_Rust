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
use AlphaVantage_Rust::alpha_lib::alpha_io_funcs::load_summary;
use AlphaVantage_Rust::alpha_lib::misc_functions::get_exe_name;
use AlphaVantage_Rust::db_funcs::{
    get_proc_id_or_insert, get_sids_and_names_with_overview, log_proc_end, log_proc_start,
};
use AlphaVantage_Rust::dbfunctions::base::establish_connection_or_exit;

fn main() {
    let conn = &mut establish_connection_or_exit();
    dotenv().ok();
    let id_val = get_proc_id_or_insert(conn, &get_exe_name()).unwrap();
    let pid = log_proc_start(conn, id_val).unwrap();
    let results: Vec<(i64, String)> =
        get_sids_and_names_with_overview(conn).unwrap_or_else(|err| {
            eprintln!("Cannot load results from database {}", err);
            _ = log_proc_end(conn, pid, 3).unwrap();

            process::exit(1);
        });

    for (sid, symbol) in results {
        println!("{}:{}", sid, symbol);
        if let Err(err) = load_summary(conn, symbol, sid) {
            println!("Error loading open close prices {} for sid {}", err, sid);
            _ = log_proc_end(conn, pid, 3).unwrap();

            process::exit(1);
        }
    }
    _ = log_proc_end(conn, pid, 2).unwrap();
}
