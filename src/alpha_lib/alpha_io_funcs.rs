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

extern crate chrono_tz;
use crate::alpha_lib::alpha_data_types::{AlphaSymbol, Convert, FullOverview, RawDailyPrice, RawIntraDayPrice, Root, TopType};
use crate::alpha_lib::alpha_funcs::{normalize_alpha_region, top_constants};
use crate::create_url;
use crate::db_funcs::{create_intra_day, create_overview, create_symbol, get_max_date, get_sid, insert_open_close, insert_top_stat};
use crate::dbfunctions::base::establish_connection_or_exit;
use crate::security_types::sec_types::SecurityType;
use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::{thread, time};
use std::env::VarError;

use diesel::PgConnection;
use crate::db_models::IntraDayPrice;

const SYMBOL: &str = "symbol";
const MAX_ERRORS: i32 = 50;


fn get_api_key() -> Result<String, VarError> {
    std::env::var("ALPHA_VANTAGE_API_KEY")
}


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
///
/// TODO:  Refactor as this is a bit of a mess
///
pub fn process_symbols(sec_vec: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let api_key = get_api_key()?;

    let mut type_map: HashMap<SecurityType, i32> = HashMap::new();
    let mut symbol_map: HashMap<String, i32> = HashMap::new();
    let mut err_ct = 0;

    let conn = &mut establish_connection_or_exit();
    let mut dur_time: DateTime<Local>;
    let mut resp_time: DateTime<Local>;
    let min_time = Duration::milliseconds(350); //We cant make MIN_TIME a constant because it is not a primitive type

    for sym_vec in sec_vec {
        for symb in sym_vec {
            let url = create_url!(FuncType:SymSearch,symb,api_key);
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
                    println!("Bad response for symbol: {:?}", symb);
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
                    let s_id: i64 = SecurityType::encode(
                        sec_type,
                        type_map.get(&sec_type).unwrap().clone() as u32,
                    );

                    create_symbol(conn, s_id, record).expect("Can't write to DB fatal error");
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
/// * `symb`: A `String` representing the symbol of the financial entity.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>`: Returns an `Ok(())` if the operation is successful. Returns an `Err` wrapped in a `Box` if any error occurs.
///
///
/// # Examples
///
/// ```ignore
/// let sid = 12345;
/// let symb = "AAPL".to_string();
///
/// match get_overview(sid, symb) {
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
pub fn get_overview(connection: &mut PgConnection, s_id: i64, symb: String) -> Result<(), Box<dyn Error>> {
    const SYMBOL: &str = "Symbol";
    let api_key = get_api_key()?;
    let url = create_url!(FuncType:Overview,symb,api_key);
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
            println!("Error: for {}: {:?}", symb, text);
            thread::sleep(time::Duration::from_secs(1));
        } else {
            let json = match serde_json::from_str::<Value>(&text) {
                Ok(json) => json,
                Err(err) => {
                    println!("Error parsing json: {:?}", err);
                    return Err("Error parsing json".into());
                }
            };
            let ov = FullOverview::new(s_id, json);
            if let Some(ov) = ov {
                println!("Overview: {:?}", ov);
                create_overview(connection, ov)?;
            } else {
                println!("Error: for {}: {:?}", symb, text);
            }
        }
    } else {
        println!("Error getting response: {:?}", response);
    }

    Ok(())
}


fn get_api_data(url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;
    let text = response.text()?;
    Ok(text)
}

fn get_top_data(url: &str) -> Result<Root, Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;
    let text = response.json::<Root>()?;
    Ok(text)
}

fn parse_intraday_from_csv(text: &str) -> Result<Vec<RawIntraDayPrice>, Box<dyn Error>> {
    let mut recs = csv::Reader::from_reader(text.as_bytes());
    recs.deserialize()
        .collect::<Result<Vec<RawIntraDayPrice>, _>>()
        .map_err(|e| e.into())
}

fn persist_ticks(connection: &mut PgConnection, s_id: i64, symb: String, ticks: Vec<RawIntraDayPrice>) -> Result<(), Box<dyn Error>> {
    for tick in ticks {
        let tmp_tick = IntraDayPrice {
            eventid: 0,
            tstamp: NaiveDateTime::parse_from_str(&tick.timestamp, "%Y-%m-%d %H:%M:%S")?,
            sid: s_id,
            symbol: symb.clone(),
            open: tick.open,
            high: tick.high,
            low: tick.low,
            close: tick.close,
            volume: tick.volume,
        };
        _ = create_intra_day(connection, tmp_tick);
    }
    Ok(())
}

pub fn load_intraday(conn: &mut PgConnection, symb: String, s_id: i64) -> Result<(), Box<dyn Error>> {
    const HEADER: &str = "timestamp,open,high,low,close,volume";
    let api_key = get_api_key()?;
    let url = create_url!(FuncType:TsIntra,symb,api_key);
    let text = get_api_data(&url)?;
    if !text.contains(HEADER) {
        eprintln!("Error: for {}: {:?}", symb, text);
        // alpha vantage will return an error for a symol missing a price
        return Ok(());
    };

    let ticks = parse_intraday_from_csv(&text)?;
    persist_ticks(conn, s_id, symb, ticks)?;

    Ok(())
}


fn gen_new_summary_price(json_inp: (&String, &Value), sym: String) -> Option<RawDailyPrice> {
    let dt = NaiveDate::parse_from_str(json_inp.0, "%Y-%m-%d").map_err(|e| {
        println!("Error parsing date: {:?}", e);
        e
    }).ok()?;

    let open = json_inp.1["1. open"].as_str().unwrap().parse::<f32>().ok()?;
    let high = json_inp.1["2. high"].as_str().unwrap().parse::<f32>().ok()?;
    let low = json_inp.1["3. low"].as_str().unwrap().parse::<f32>().ok()?;
    let close = json_inp.1["4. close"].as_str().unwrap().parse::<f32>().ok()?;
    let volume = json_inp.1["5. volume"].as_str().unwrap().parse::<i32>().ok()?;

    Some(RawDailyPrice {
        date: dt,
        symbol: sym,
        open,
        high,
        low,
        close,
        volume,
    })
}

pub fn load_summary(conn: &mut PgConnection, symb: String, s_id: i64) -> Result<(), Box<dyn Error>> {
    let api_key = get_api_key()?;
    let url = create_url!(FuncType:TsDaily,symb.clone(),api_key);
    let text = get_api_data(&url)?;
    const HEADER: &str = "Meta Data";
    if !text.contains(HEADER) {
        eprintln!("Error: for {}: {:?}", symb.clone(), text);
        // alpha vantage will return an error for a symol missing a price
        return Ok(());
    };

    let daily_prices = get_open_close(&text, &symb)?;
    let last_date = get_max_date(conn, s_id);
    println!("last date for sid{} is {:?}", s_id, last_date);
    for oc in daily_prices {
        print!("{:?}", oc.date);
        if oc.date > last_date {
            insert_open_close(conn, symb.clone(), s_id, oc)?;
        }
    }

    Ok(())
}


/// Retrieves the opening and closing prices for a given financial symbol from a JSON string.
///
/// This function parses the input JSON string, expecting to find daily prices under
/// the "Time Series (Daily)" key. For each day, it will attempt to generate a `RawDailyPrice`
/// using the provided `gen_new_summary_price` function and the given financial symbol.
///
/// # Arguments
/// * `inp` - A string slice representing the JSON input containing daily prices.
/// * `symb` - A reference to a string representing the financial symbol for which we are
///            retrieving the opening and closing prices.
///
/// # Returns
/// * On success, a `Result` containing a vector of `RawDailyPrice` items.
/// * On error, a `Result` containing an error of type `Box<dyn Error>`.
///
/// # Errors
/// This function will return an error if:
/// * The input string cannot be parsed into a valid JSON structure.
/// * The parsed JSON does not contain the expected "Time Series (Daily)" key or it's not an object.
/// * Any error that might arise from the `gen_new_summary_price` function.
///
/// # Examples
///
/// ```ignore
/// // Assuming a valid JSON string `json_str` and a symbol `symb`:
/// let results = get_open_close(&json_str, &symb);
/// match results {
///     Ok(daily_prices) => {
///         for price in daily_prices {
///             println!("{:?}", price);
///         }
///     },
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
///
fn get_open_close(inp: &str, symb: &String) -> Result<Vec<RawDailyPrice>, Box<dyn Error>> {
    const HEADER: &str = "Time Series (Daily)";
    let mut daily_prices: Vec<RawDailyPrice> = Vec::new();
    let json_data: Value = serde_json::from_str(inp)?;

    let json_prices = json_data[HEADER]
        .as_object()
        .ok_or_else(|| {
            let err_msg = format!("Error getting {} from json data", HEADER);
            eprintln!("{}", err_msg);
            err_msg
        })?;

    for (date, data) in json_prices.iter() {
        if let Some(open_close) = gen_new_summary_price((date, data), symb.clone()) {
            daily_prices.push(open_close);
        }
    }

    Ok(daily_prices)
}



pub fn load_tops(conn: &mut PgConnection) -> Result<(), Box<dyn Error>> {
    let api_key = get_api_key()?;
    let url = create_url!(FuncType:TopQuery," ",api_key);

    let root: Root = get_top_data(&url)?;

    let last_update = get_time_stamp(root.last_updated)?;

    process_data_for_type(conn, &root.top_gainers, TopType::TopGainer, last_update)?;
    process_data_for_type(conn, &root.top_losers, TopType::TopLoser, last_update)?;
    process_data_for_type(conn, &root.most_actively_traded, TopType::TopActive, last_update)?;

    Ok(())
}

fn process_data_for_type(
    conn: &mut PgConnection,
    data: &[impl Convert],
    top_type: TopType,
    last_update: NaiveDateTime
) -> Result<(), Box<dyn Error>> {
    for item in data {
        let tt = item.make_top_stat()?;
        let s_id = match get_sid(conn, tt.ticker.clone()) {
            Ok(value) => value,
            Err(_) => continue,
        };
        let typ = top_constants(&top_type);
        insert_top_stat(conn, s_id, tt.clone(), &typ, last_update)?;
    }
    Ok(())
}


fn get_time_stamp(inp :String)->Result<NaiveDateTime, Box<dyn Error>>{

    let mut parts = inp.rsplitn(2, ' ');
    let _tz = parts.next().unwrap();
    let tm= parts.next().unwrap();
    println!("parts {:?}",tm);
    let naive_dt = NaiveDateTime::parse_from_str(tm, "%Y-%m-%d %H:%M:%S")?;
    Ok(naive_dt )
}


#[cfg(test)]
mod test {

    use crate::alpha_lib::alpha_io_funcs::get_time_stamp;

    #[test]
    fn t_001(){
        let inp ="2023-10-03 16:15:59 US/Eastern";

        assert!(get_time_stamp(inp.to_string()).is_ok());

    }

}