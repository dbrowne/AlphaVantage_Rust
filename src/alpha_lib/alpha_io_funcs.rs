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

use crate::alpha_lib::alpha_data_types::{AlphaSymbol, FullOverview};
use crate::alpha_lib::alpha_funcs::normalize_alpha_region;
use crate::create_url;
use crate::db_funcs::{create_overview, create_symbol, establish_connection};
use crate::security_types::sec_types::SecurityType;
use chrono::{DateTime, Duration, Local};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::{thread, time};

const SYMBOL: &str = "symbol";
const MAX_ERRORS: i32 = 50;

/// # process_symbols Function
///
/// This function makes HTTP requests to the Alpha Vantage API to retrieve financial data.
/// It reads symbols from an array of strings and makes requests at a maximum rate of 75 reqs/min.
/// This function checks for duplicate symbols and writes unique records into the sec database.
///
/// # Arguments
///
/// * `sec_vec` - A 2D vector containing symbol strings.
///
/// # Returns
///
/// * `Ok(())` - The function completed successfully.
/// * `Err(Box<dyn Error>)` - An error occurred during the operation.
///
/// # Errors
///
/// This function will return an error if there are any issues reading the environment variables,
/// making the HTTP request, parsing the response, or interacting with the database.
///
/// # Example
///
/// ```ignore
/// use AlphaVantage_Rust::alpha_lib::alpha_io_funcs::process_symbols;
/// let symbols = vec![vec!["AAPL".to_string(),  "GOOG".to_string()], vec!["TSLA".to_string()]];
/// let result = process_symbols(symbols);
///
/// match result {
///     Ok(_) => println!("Operation completed successfully."),
///     Err(e) => println!("An error occurred: {}", e),
/// }
/// ```
///
/// Note: It is assumed that the `ALPHA_VANTAGE_API_KEY` environment variable has been set with a valid API key.

pub fn process_symbols(sec_vec: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var("ALPHA_VANTAGE_API_KEY")
        .map_err(|e| format!("Couldn't read ALPHA_VANTAGE_API_KEY: {}", e))?;

    let mut type_map: HashMap<SecurityType, i32> = HashMap::new();
    let mut symbol_map: HashMap<String, i32> = HashMap::new();
    let mut err_ct = 0;

    let conn = &mut establish_connection()?;
    let mut dur_time: DateTime<Local>;
    let mut resp_time: DateTime<Local>;
    let min_time = Duration::milliseconds(350); //We cant make MIN_TIME a constant because it is not a primitive type

    for sym_vec in sec_vec {
        for symbol in sym_vec {
            let url = create_url!(FuncType:SymSearch,symbol,api_key);
            let resp = reqwest::blocking::get(&url); //todo: change to async & refactor
            resp_time = Local::now();
            if let Ok(resp) = resp {
                let text = match resp.text() {
                    Ok(text) => text,
                    Err(e) => {
                        println!("Couldn't read text: {}", e);
                        err_ct += 1;
                        if err_ct > MAX_ERRORS {
                            println!("Too many errors: {}", err_ct);
                            return Err(e.into());
                        }
                        continue;
                    }
                };

                if err_ct > MAX_ERRORS {
                    return Err(format!("Too many errors: {}", err_ct).into());
                }

                if !text.contains(SYMBOL) {
                    println!("Bad response for symbol: {}", symbol);
                    println!("text error: for {:?}", text);
                    continue;
                }

                let mut rdr = csv::Reader::from_reader(text.as_bytes());
                for result in rdr.deserialize() {
                    let mut record: AlphaSymbol =
                        result.expect("process_symbols: can't read record");
                    if symbol_map.insert(record.symbol.clone(), 1).is_some() {
                        println!("Duplicate symbol: {}", record.symbol);
                        continue;
                    }

                    let (sec_type, sec_type_string) = SecurityType::get_detailed_sec_type(
                        record.s_type.as_str(),
                        record.name.as_str(),
                    );
                    record.s_type = sec_type_string;
                    record.region = normalize_alpha_region(record.region.as_str());
                    if !type_map.contains_key(&sec_type) {
                        type_map.insert(sec_type, 1);
                    } else {
                        type_map.entry(sec_type).and_modify(|e| *e += 1);
                    }
                    let sid = SecurityType::encode(
                        sec_type,
                        type_map.get(&sec_type).unwrap().clone() as u32,
                    );

                    create_symbol(conn, sid, record).expect("Can't write to DB fatal error");
                    dur_time = Local::now();

                    if dur_time - resp_time < min_time {
                        // Current rate limit is 75 per minute
                        std::thread::sleep(std::time::Duration::from_secs(1));
                        println!("stats:{}, {:?}", Local::now(), type_map);
                    }
                }
            } else {
                println!("Error: {:?}", resp);
            }
        }
    }

    Ok(())
}

/// Fetches and processes the overview of a financial entity using an external API.
///
/// This function contacts the external API specified by the `ALPHA_VANTAGE_API_KEY` environment variable to get a detailed overview of a financial entity identified by its `sid` and `symbol`.
/// After obtaining the overview, the function processes the response to create a `FullOverview` struct and subsequently stores it in the database.
///
/// # Parameters
///
/// * `sid`: An `i64` identifier representing the financial entity.
/// * `symbol`: A `String` representing the symbol of the financial entity.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>`: Returns an `Ok(())` if the operation is successful. Returns an `Err` wrapped in a `Box` if any error occurs.
///
/// # Environment Variables
///
/// * `ALPHA_VANTAGE_API_KEY`: This environment variable should be set with the API key that will be used to access the external API.
///
/// # Examples
///
/// ```ignore
/// let sid = 12345;
/// let symbol = "AAPL".to_string();
///
/// match get_overview(sid, symbol) {
///     Ok(_) => println!("Overview fetched and processed successfully."),
///     Err(e) => println!("Error fetching or processing overview: {:?}", e),
/// }
/// ```
///
/// # Panics
///
/// * This function will panic if the `ALPHA_VANTAGE_API_KEY` environment variable is not set.
///
/// # Errors
///
/// * It might return an error if there's a problem establishing a database connection, making the external API request, or processing the response.
pub fn get_overview(sid: i64, symbol: String) -> Result<(), Box<dyn Error>> {
    const SYMBOL: &str = "Symbol";
    let connection = &mut establish_connection()?;
    let api_key =
        std::env::var("ALPHA_VANTAGE_API_KEY").expect("ALPHA_VANTAGE_API_KEY must be set");
    let url = create_url!(FuncType:Overview,symbol,api_key);
    let response = reqwest::blocking::get(&url);

    if let Ok(response) = response {
        println!("Response {:?}", response);
        let text = match response.text() {
            Ok(text) => text,
            Err(err) => {
                println!("Error getting text from response: {:?}", err);
                return Err("Error getting text from response".into());
            }
        };

        if !text.contains(SYMBOL) {
            println!("Error: for {}: {:?}", symbol, text);
            thread::sleep(time::Duration::from_secs(1));
        } else {
            let json = match serde_json::from_str::<Value>(&text) {
                Ok(json) => json,
                Err(err) => {
                    println!("Error parsing json: {:?}", err);
                    return Err("Error parsing json".into());
                }
            };
            let ov = FullOverview::new(sid, json);
            if let Some(ov) = ov {
                println!("Overview: {:?}", ov);
                create_overview(connection, ov)?;
            } else {
                println!("Error: for {}: {:?}", symbol, text);
            }
        }
    } else {
        println!("Error getting response: {:?}", response);
    }

    Ok(())
}
