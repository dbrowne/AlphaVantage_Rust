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



use  std::collections::HashMap;
use  std::error::Error;
use chrono::{DateTime, Local};
use crate::alpha_lib::macros::FuncType;
use crate::alpha_lib::alpha_data_types::AlphaSymbol;
use crate::create_url;
use crate::db_funcs::{create_symbol, establish_connection};

use  crate::security_types::sec_types::{SecurityType};



use crate::alpha_lib::alpha_funcs::normalize_alpha_region;


/// # process_symbols Function
///
/// This function makes HTTP requests to the Alpha Vantage API to retrieve financial data.
/// It reads symbols from an array of strings and makes requests at a rate of 75 requests/minute.
/// This function checks for duplicate symbols and writes unique records into a database.
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
/// ```
/// let symbols = vec![vec!["AAPL", "MSFT", "GOOG"], vec!["TSLA", "AMZN"]];
/// let result = process_symbols(symbols);
///
/// match result {
///     Ok(_) => println!("Operation completed successfully."),
///     Err(e) => println!("An error occurred: {}", e),
/// }
/// ```
///
/// Note: It is assumed that the `ALPHA_VANTAGE_API_KEY` environment variable has been set with a valid API key.
async fn process_symbols(sec_vec: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var("ALPHA_VANTAGE_API_KEY")
        .map_err(|e| format!("Couldn't read ALPHA_VANTAGE_API_KEY: {}", e))?;

    let mut type_map: HashMap<SecurityType, i32> = HashMap::new();
    let mut symbol_map: HashMap<String, i32> = HashMap::new();
    let mut err_ct = 0;

    const SYMBOL: &str = "symbol";
    const MAX_ERRORS: i32 = 50;
    const RATE: u32 = 75; // Requests per minute

    let conn = &mut establish_connection()?;
    let client = reqwest::Client::new();
    let mut dur_time: DateTime<Local>;
    let mut resp_time: DateTime<Local>;
    let min_time = std::time::Duration::from_millis(350);

    for sym_vec in sec_vec {
        for symbol in sym_vec {
            let url =create_url!(FuncType::SymSearch,symbol,api_key);

            let resp = client.get(&url).send().await;
            resp_time = Local::now();
            if let Ok(resp) = resp {
                let text = match resp.text().await {
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
                    let mut record: AlphaSymbol = result.expect("process_symbols: can't read record");
                    if symbol_map.insert(record.symbol.clone(), 1).is_some() {
                        println!("Duplicate symbol: {}", record.symbol);
                        continue;
                    }

                    let (sec_type, sec_type_string) = SecurityType::get_detailed_sec_type(record.s_type.as_str(), record.name.as_str());
                    record.s_type = sec_type_string;
                    record.region = normalize_alpha_region(record.region.as_str());

                    let sid = SecurityType::encode(sec_type, *type_map.entry(sec_type).or_insert(0) as u32);
                    create_symbol(conn, sid, record).expect("Can't write to DB fatal error");
                    dur_time = Local::now();

                    // Changed to std::time::Duration
                    if (dur_time - resp_time).to_std()? < min_time {
                        tokio::time::sleep(min_time).await;  // asynchronous sleep
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