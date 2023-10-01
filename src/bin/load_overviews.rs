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

use chrono::{prelude::*, Duration};
use dotenvy::dotenv;
use std::process;
use AlphaVantage_Rust::alpha_lib::alpha_io_funcs::get_overview;
use AlphaVantage_Rust::db_funcs::{establish_connection_or_exit, get_sids_and_names_for};
extern crate lazy_static;
use lazy_static::lazy_static;

//We can't make MIN_TIME a constant because it is not a primitive type
lazy_static! {
    static ref MIN_TIME: Duration = Duration::milliseconds(350);
}
const COUNTRY: &str = "USA";
const TYPE: &str = "Eqty";

fn main() {
    dotenv().ok();
    let mut resp_time: DateTime<Local>;
    let mut dur_time: DateTime<Local>;
    let conn = &mut establish_connection_or_exit();

    let res = get_sids_and_names_for(conn, COUNTRY.to_string(), TYPE.to_string());
    let results = match res {
        Ok(results) => results,
        Err(err) => {
            println!("Error running reader: {}", err);
            process::exit(1);
        }
    };

    for (sid, symbol) in results {
        println!("{}: {}", sid, symbol);
        dur_time = Local::now();
        if let Err(err) = get_overview(sid, symbol) {
            println!("Error running reader: {}", err);
            process::exit(1);
        }

        resp_time = Local::now();
        if resp_time - dur_time < *MIN_TIME {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
