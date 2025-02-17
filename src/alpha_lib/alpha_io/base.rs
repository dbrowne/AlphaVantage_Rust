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

extern crate chrono_tz;

use std::{collections::HashMap, env::VarError, thread, time};

use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime};
use diesel::PgConnection;
use serde_json::Value;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum Error {
  #[error("Failed to retreive API key")]
  ApiKey(#[from] VarError),
  #[error(transparent)]
  DbConn(#[from] diesel::result::Error),
  #[error(transparent)]
  Serde(#[from] serde_json::Error),
  #[error(transparent)]
  Reqwest(#[from] reqwest::Error),
  #[error(transparent)]
  Csv(#[from] csv::Error),
  #[error("Error count exceeded {0}")]
  ErrorCount(usize),
  #[error(transparent)]
  ParseDate(#[from] chrono::ParseError),
  #[error(transparent)]
  DBFuncs(#[from] crate::dbfunctions::common::Error),
  #[error(transparent)]
  AlphaDataTypes(#[from] crate::alpha_lib::core::alpha_data_types::Error),
  #[error("Error getting {0} from JSON data")]
  MissingHeader(String),
  #[error("Max exceeded : {0}")]
  MaxExceeded(String),
  #[error("Unexpected error: {0}")]
  UnEx(String),
}

use crate::{
  alpha_lib::core::{
    alpha_data_types::{
      AlphaSymbol, Convert, FullOverview, RawDailyPrice, RawIntraDayPrice, Root, TopType,
    },
    alpha_funcs::{normalize_alpha_region, top_constants},
    news_type::NewsRoot,
  },
  create_url,
  db_models::IntraDayPrice,
  dbfunctions::{
    base::establish_connection_or_exit,
    overview::create_overview,
    price::{create_intra_day, get_intr_day_max_date, get_summary_max_date, insert_open_close},
    sid::{get_next_sid, get_sid},
    symbols::{create_symbol, get_symbols_and_sids_for},
    tops::insert_top_stat,
  },
  security_types::sec_types::SecurityType,
};

const SYMBOL: &str = "symbol";
const MAX_ERRORS: i32 = 50;

pub fn get_api_key() -> Result<String, VarError> {
  std::env::var("ALPHA_VANTAGE_API_KEY")
}

/// # process_symbols Function
///
/// This function makes HTTP requests to the Alpha Vantage API to retrieve the basic symbol data
///for the symbol table.
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
/// use alpha_vantage_rust::alpha_lib::alpha_io_funcs::process_symbols;
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
pub fn process_symbols(sec_vec: Vec<Vec<String>>, load_missed: bool) -> Result<(), Error> {
  let api_key = get_api_key()?;

  let mut type_map: HashMap<SecurityType, i32> = HashMap::new();
  let mut symbol_map: HashMap<String, i32> = HashMap::new();
  let mut err_ct = 0;

  let conn = &mut establish_connection_or_exit();
  let mut dur_time: DateTime<Local>;
  let mut resp_time: DateTime<Local>;
  let min_time = Duration::milliseconds(350); //We cant make MIN_TIME a constant because it is not a primitive type

  if load_missed {
    let symbs = get_symbols_and_sids_for(conn, "USA".to_string(), "Eqty".to_string())?;
    for (symb, _) in symbs {
      symbol_map.insert(symb, 1);
    }
  }

  for sym_vec in sec_vec {
    for symb in sym_vec {
      let url = create_url!(FuncType::SymSearch, symb, api_key);
      let resp = reqwest::blocking::get(&url); //todo: change to async & refactor
      resp_time = Local::now();
      if let Ok(resp) = resp {
        let text = match resp.text() {
          Ok(text) => text,
          Err(e) => {
            err_ct += 1;
            if err_ct > MAX_ERRORS {
              println!("Too many errors: {}", err_ct);
              return Err(e.into());
            }
            continue;
          }
        };

        if err_ct > MAX_ERRORS {
          let errmsg = format!("Too many errors: {}", err_ct);
          return Err(Error::MaxExceeded(errmsg));
        }

        if !text.contains(SYMBOL) {
          continue;
        }

        let mut rdr = csv::Reader::from_reader(text.as_bytes());
        for result in rdr.deserialize() {
          let mut record: AlphaSymbol = result.expect("process_symbols: can't read record");
          if symbol_map.insert(record.symbol.clone(), 1).is_some() {
            // todo: Improve logging
            // println!("Duplicate symbol: {}", record.symbol);
            continue;
          }

          let (sec_type, sec_type_string) =
            SecurityType::get_detailed_sec_type(record.s_type.as_str(), record.name.as_str());
          record.s_type = sec_type_string.clone();
          record.region = normalize_alpha_region(record.region.as_str());

          if load_missed && record.region.ne("USA") {
            continue;
          }
          if !type_map.contains_key(&sec_type) {
            type_map.insert(sec_type, 1);
          } else {
            type_map.entry(sec_type).and_modify(|e| *e += 1);
          }
          let s_id: i64;
          if load_missed {
            s_id = get_next_sid(conn, sec_type_string)?
          } else {
            s_id = SecurityType::encode(sec_type, type_map.get(&sec_type).unwrap().clone() as u32);
          }
          create_symbol(conn, s_id, record)?;

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
/// This function contacts the external API specified by the `ALPHA_VANTAGE_API_KEY` environment
/// variable to get a detailed overview of a financial entity identified by its `sid` and `symbol`.
/// After obtaining the overview, the function processes the response to create a `FullOverview`
/// struct and subsequently stores it in the database.
///
/// # Parameters
///
/// * `sid`: An `i64` identifier representing the financial entity.
/// * `symb`: A `String` representing the symbol of the financial entity.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>`: Returns an `Ok(())` if the operation is successful. Returns an
///   `Err` wrapped in a `Box` if any error occurs.
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
/// * It might return an error if there's a problem establishing a database connection, making the
///   external API request, or processing the response.
pub fn get_overview(connection: &mut PgConnection, s_id: i64, symb: String) -> Result<(), Error> {
  const SYMBOL: &str = "Symbol";
  let api_key = get_api_key()?;
  let url = create_url!(FuncType::Overview, symb, api_key);
  let response = reqwest::blocking::get(&url)?;
  println!("Response is: {:?}", response);

  let text = response.text()?;

  if !text.contains(SYMBOL) {
    println!("Missing overview  for Symbol {}: {:?}", symb, text);
    thread::sleep(time::Duration::from_secs(1));
    return Ok(());
  }
  let json = serde_json::from_str::<Value>(&text)?;
  let ov = FullOverview::new(s_id, json);
  if let Some(ov) = ov {
    println!("Overview: {:?}", ov);
    create_overview(connection, ov)?;
  } else {
    println!("Failed to create overview for symbol {}: {:?}", symb, text);
    return Err(Error::UnEx(format!(
      "Failed to create overview for symbol {}",
      symb
    )));
  }

  Ok(())
}

fn get_api_data(url: &str) -> Result<String, Error> {
  let response = reqwest::blocking::get(url)?;
  let text = response.text()?;
  Ok(text)
}

fn get_top_data(url: &str) -> Result<Root, Error> {
  let response = reqwest::blocking::get(url)?;
  let text = response.json::<Root>()?;
  Ok(text)
}

pub fn get_news_root(url: &str) -> Result<NewsRoot, Error> {
  let response = reqwest::blocking::get(url)?;
  let text = response.json::<NewsRoot>()?;
  Ok(text)
}

fn parse_intraday_from_csv(text: &str) -> Result<Vec<RawIntraDayPrice>, Error> {
  let mut recs = csv::Reader::from_reader(text.as_bytes());
  recs
    .deserialize()
    .collect::<Result<Vec<RawIntraDayPrice>, _>>()
    .map_err(|e| e.into())
}
/// Persists a list of intraday price ticks into the database.
///
/// This function retrieves the latest intraday price timestamp for a given security (`s_id`).
/// - If there is no existing data (`NoData` error), all ticks are inserted.
/// - If data exists, only ticks with timestamps newer than the latest recorded timestamp are inserted.
///
/// # Arguments
/// - `connection` - A mutable reference to the PostgreSQL connection.
/// - `s_id` - The security ID associated with the price data.
/// - `symb` - The symbol of the security.
/// - `ticks` - A vector of `RawIntraDayPrice` values representing tick data to be persisted.
///
/// # Returns
/// - `Ok(())` if the operation succeeds.
/// - `Err(Error::DBFuncs(e))` if a database-related error occurs.
/// - `Err(Error::ParseDate(e))` if parsing the timestamp fails.
///
/// # Behavior
/// - Calls `get_intr_day_max_date` to fetch the latest recorded timestamp.
/// - If no prior data exists, all ticks are inserted.
/// - Otherwise, only ticks with a timestamp greater than the latest recorded date are inserted.
/// - Calls `create_intra_day` to persist valid ticks into the database.
///
/// # Errors
/// - If there is an error retrieving the latest timestamp, it propagates the error.
/// - If there is an error inserting data into the database, it is ignored (consider logging it instead).
///
/// # Example
/// ```ignore
/// let mut connection = establish_connection();
/// let s_id = 123;
/// let symb = "AAPL";
/// let ticks = vec![
///     RawIntraDayPrice {
///         timestamp: "2024-02-17 15:30:00".to_string(),
///         open: 150.0,
///         high: 155.0,
///         low: 149.0,
///         close: 152.0,
///         volume: 10000,
///     },
/// ];
///
/// persist_ticks(&mut connection, s_id, symb, ticks).unwrap();
/// `
fn persist_ticks(
  connection: &mut PgConnection,
  s_id: i64,
  symb: &str,
  ticks: Vec<RawIntraDayPrice>,
) -> Result<(), Error> {
  let last_date = match get_intr_day_max_date(connection, s_id) {
    Ok(date) => Some(date),
    Err(crate::dbfunctions::common::Error::NoData(_)) =>None,
    Err(e) => return Err(Error::DBFuncs(e)),
  };


  for tick in ticks {
    let tmp_tick = IntraDayPrice {
      eventid: 0,
      tstamp: NaiveDateTime::parse_from_str(&tick.timestamp, "%Y-%m-%d %H:%M:%S")?,
      sid: s_id,
      symbol: symb.to_string(),
      open: tick.open,
      high: tick.high,
      low: tick.low,
      close: tick.close,
      volume: tick.volume,
    };
    if last_date.is_none() || tmp_tick.tstamp > last_date.unwrap(){
      let _ = create_intra_day(connection, tmp_tick);
    }
  }
  Ok(())
}

pub fn load_intraday(
  conn: &mut PgConnection,
  symb: &String,
  s_id: i64,
  sectype: SecurityType,
) -> Result<(), Error> {
  const HEADER: &str = "timestamp,open,high,low,close,volume";
  let api_key = get_api_key()?;
  let url = match sectype {
    SecurityType::Crypto => create_url!(FuncType::CryptoIntraDay, symb, api_key),
    SecurityType::Equity => create_url!(FuncType::TsIntra, symb, api_key),
    _ => panic!("Unknown security type"),
  };

  let text = get_api_data(&url)?;
  if !text.contains(HEADER) {
    // todo: Improve logging here
    // eprintln!("Error: for {}: {:?}", symb, text);
    // alpha vantage will return an error for a symol missing a price
    return Ok(());
  };

  let ticks = parse_intraday_from_csv(&text)?;
  persist_ticks(conn, s_id, &symb, ticks)?;

  Ok(())
}

fn gen_new_summary_price(json_inp: (&String, &Value), sym: String) -> Option<RawDailyPrice> {
  //todo   Refactor
  let dt = NaiveDate::parse_from_str(json_inp.0, "%Y-%m-%d")
    .map_err(|e| {
      println!("Error parsing date: {:?}", e);
      e
    })
    .ok()?;

  let open = json_inp.1["1. open"]
    .as_str()
    .unwrap()
    .parse::<f32>()
    .ok()?;
  let high = json_inp.1["2. high"]
    .as_str()
    .unwrap()
    .parse::<f32>()
    .ok()?;
  let low = json_inp.1["3. low"].as_str().unwrap().parse::<f32>().ok()?;
  let close = json_inp.1["4. close"]
    .as_str()
    .unwrap()
    .parse::<f32>()
    .ok()?;
  let volume = json_inp.1["5. volume"]
    .as_str()
    .unwrap()
    .parse::<i32>()
    .ok()?;

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
/// Loads and updates the daily stock price summaries for a given symbol.
///
/// This function retrieves the daily stock prices from an external API and updates
/// the database with the new information that has not yet been recorded up to the
/// latest date known in the database for the given stock symbol and source ID.
///
/// # Parameters
/// - `conn`: A mutable reference to a PostgreSQL connection to perform database operations.
/// - `symb`: The stock symbol for which the daily summary is being loaded.
/// - `s_id`: A unique identifier for the source of the stock data.
///
/// # Returns
/// - `Ok(())` if the daily summaries were successfully loaded and updated in the database.
/// - `Err(e)` where `e` is a boxed error if any operation within the function fails, including API
///   data retrieval, data parsing, or database operations.
///
/// # Errors
/// - Returns an error if the API key retrieval fails.
/// - Returns an error if there is a failure in fetching or parsing the API data.
/// - Database related errors are propagated if any insert operation fails.
///
/// # Example
/// ```ignore
/// use your_crate::db_operations::{PgConnection, load_summary};
///
/// let mut conn = PgConnection::establish("connection_string").unwrap();
/// let symbol = "AAPL".to_string();
/// let source_id = 1;
///
/// match load_summary(&mut conn, symbol, source_id) {
///     Ok(_) => println!("Summary loaded successfully."),
///     Err(e) => eprintln!("Failed to load summary: {}", e),
/// }
/// ```
///
/// # Remarks
/// - The function checks if the returned data from the API contains a specific header ("Meta
///   Data"). If this header is not present, it assumes that there was an error with the provided
///   symbol (such as missing price data) and will not perform any database updates for this symbol.
/// - It logs the latest date for which data is available in the database and only inserts new
///   records for dates that are after this last known date.
pub fn load_summary(conn: &mut PgConnection, symb: &str, s_id: i64) -> Result<(), Error> {
  let api_key = get_api_key()?;
  let url = create_url!(FuncType::TsDaily, symb, api_key);
  let text = get_api_data(&url)?;
  const HEADER: &str = "Meta Data";
  if !text.contains(HEADER) {
    //todo: improve logging here
    // eprintln!("Error: for {}: {:?}", symb.clone(), text);
    // alpha vantage will return an error for a symol missing a price
    return Ok(());
  };

  let daily_prices = get_open_close(&text, &symb.to_string())?;
  let last_date = get_summary_max_date(conn, s_id)?;
  //todo: improve logging here

  // println!("last date for sid{} is {:?}", s_id, last_date);
  for oc in daily_prices {
    //todo: improve logging here

    // print!("{:?}", oc.date);
    if oc.date > last_date {
      insert_open_close(conn, &symb.to_string(), s_id, oc)?;
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
/// * `symb` - A reference to a string representing the financial symbol for which we are retrieving
///   the opening and closing prices.
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
fn get_open_close(inp: &str, symb: &String) -> Result<Vec<RawDailyPrice>, Error> {
  const HEADER: &str = "Time Series (Daily)";
  let mut daily_prices: Vec<RawDailyPrice> = Vec::new();
  let json_data: Value = serde_json::from_str(inp)?;

  let json_prices = json_data[HEADER]
    .as_object()
    .ok_or_else(|| Error::MissingHeader(HEADER.to_string()))?;

  for (date, data) in json_prices.iter() {
    if let Some(open_close) = gen_new_summary_price((date, data), symb.clone()) {
      daily_prices.push(open_close);
    }
  }

  Ok(daily_prices)
}

pub fn process_digital_symbols(sed_vec: Vec<String>) -> Result<(), Error> {
  let sec_type = "Crypto";
  let region = "USA";
  let currency = "USD";
  let timezone = "UTC-04";
  let marketopen = "00:00";
  let marketclose = "23:59";

  let mut symbol_map: HashMap<String, i64> = HashMap::new();
  let conn = &mut establish_connection_or_exit();

  // assuming there are no digital currencies
  let mut base_sid = 1;
  for sym_string in sed_vec {
    let sym_vec: Vec<&str> = sym_string.split(',').collect();
    let symbol = sym_vec[0].to_string();
    let name = sym_vec[1].to_string();

    let s_id = SecurityType::encode(SecurityType::Crypto, base_sid);
    let record = AlphaSymbol::new(
      symbol,
      name,
      sec_type.to_string(),
      region.to_string(),
      marketopen.to_string(),
      marketclose.to_string(),
      timezone.to_string(),
      currency.to_string(),
      1.0,
    );
    if symbol_map.insert(record.symbol.clone(), s_id).is_some() {
      eprintln!("Duplilcate symbol: {}", record.symbol);
    } else {
      println!("Inserting symbol: {}:{}", record.symbol, s_id);
      create_symbol(conn, s_id, record).expect("Can't insert symbol");
    }

    base_sid += 1;
  }

  println!("Total sybols: {}", symbol_map.len());

  Ok(())
}

/// Loads the top stock performers from an API and updates the database with the current
/// information.
///
/// This function fetches data for top gainers, top losers, and most actively traded stocks from
/// The AlphaVantage API and updates the database with these details, organizing the data by
/// category and tagging each entry with the timestamp of the last update.
///
/// # Parameters
/// - `conn`: A mutable reference to a PostgreSQL connection to perform database operations.
///
/// # Returns
/// - `Ok(())` if the data is successfully fetched and processed into the database.
/// - `Err(e)` where `e` is a boxed error if there is a failure in any part of the process,
///   including retrieving or processing the API data, or any database operations.
///
/// # Errors
/// - The function propagates errors from the underlying API call, data parsing, or database
///   operations.
/// - Errors from obtaining the API key or constructing the URL are also propagated.
///
/// # Example
/// ```ignore
/// use alpha_vantage_rust::db_operations::{PgConnection, load_tops};
///
/// let mut conn = PgConnection::establish("connection_string").unwrap();
///
/// match load_tops(&mut conn) {
///     Ok(_) => println!("Top stocks data updated successfully."),
///     Err(e) => eprintln!("Failed to update top stocks data: {}", e),
/// }
/// ```
///
/// # Remarks
/// - The function first retrieves an API key and constructs a URL for the API request.
/// - The data for top gainers, top losers, and most actively traded stocks is retrieved as a `Root`
///   object.
/// - Each category of data (top gainers, losers, and active trades) is processed separately.
/// - All entries are tagged with a timestamp indicating the last update from the API, ensuring that
///   the database contains the most current information available.
///
/// This function is designed to be run at regular intervals to keep the database up to date with
/// the latest market movements.
pub fn load_tops(conn: &mut PgConnection) -> Result<(), Error> {
  let api_key = get_api_key()?;
  let url = create_url!(FuncType::TopQuery, " ", api_key);

  let root: Root = get_top_data(&url)?;

  let last_update = get_time_stamp(root.last_updated)?;

  process_data_for_type(conn, &root.top_gainers, TopType::TopGainer, last_update)?;
  process_data_for_type(conn, &root.top_losers, TopType::TopLoser, last_update)?;
  process_data_for_type(
    conn,
    &root.most_actively_traded,
    TopType::TopActive,
    last_update,
  )?;

  Ok(())
}

/// Processes a set of data items for a specific type, converting them to top statistics
/// and inserting them into a database.
///
/// This function iterates through a collection of data items, converts each item into a
/// top statistic using the provided conversion trait, determines the corresponding ID
/// for the ticker, and then inserts the top statistic into the database.
///
/// # Parameters
///
/// * `conn`: A mutable reference to a PostgreSQL connection.
/// * `data`: A slice of data items that implement the `Convert` trait.
/// * `top_type`: The type of top data being processed, represented by the `TopType` enum.
/// * `last_update`: A timestamp indicating the last update time for the data.
///
/// # Returns
///
/// * `Ok(())` if the processing succeeds.
/// * `Err(Box<dyn Error>)` if any step in the process encounters an error.
///
/// # Note
///
/// If there's an error while fetching the ID for a specific ticker, the function will
/// skip that ticker and continue with the next item in the data collection.
fn process_data_for_type(
  conn: &mut PgConnection,
  data: &[impl Convert],
  top_type: TopType,
  last_update: NaiveDateTime,
) -> Result<(), Error> {
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

/// Parses a string to extract a date and time, ignoring the timezone.
///
/// This function takes a string input that contains a date, time, and timezone,
/// parses it to extract the datetime component, and returns a `NaiveDateTime` object.
/// The timezone part of the string is ignored during parsing.
///
/// # Parameters
/// - `inp`: A string in the format "YYYY-MM-DD HH:MM:SS TZ" where TZ is the timezone.
///
/// # Returns
/// - `Ok(NaiveDateTime)` if the datetime string is successfully parsed.
/// - `Err(e)` where `e` is a boxed error if parsing fails.
///
/// # Errors
/// - The function will return an error if the input string does not contain exactly two parts
///   separated by a space, or if the datetime part of the string is not in the expected format.
///
/// # Example
/// ```ignore
/// use alpha_vantage_rust::time_functions::get_time_stamp;
/// use chrono::NaiveDateTime;
///
/// let input = "2023-04-01 15:30:00 UTC";
/// match get_time_stamp(input.to_string()) {
///     Ok(datetime) => println!("Parsed datetime: {}", datetime),
///     Err(e) => eprintln!("Error parsing datetime: {}", e),
/// }
/// ```
///
/// # Remarks
/// - This function assumes that the input string will always follow the format with a space
///   separating the datetime and timezone components. It uses `rsplitn` with a limit of 2 to split
///   the string from the end, ensuring that the last part (timezone) is separated first.
fn get_time_stamp(inp: String) -> Result<NaiveDateTime, Error> {
  let mut parts = inp.rsplitn(2, ' ');
  let _tz = parts.next().unwrap();
  let tm = parts.next().unwrap();
  let naive_dt = NaiveDateTime::parse_from_str(tm, "%Y-%m-%d %H:%M:%S")?;
  Ok(naive_dt)
}

#[cfg(test)]
mod test {
  use crate::alpha_lib::alpha_io::base::get_time_stamp;

  #[test]
  fn t_001() {
    let inp = "2023-10-03 16:15:59 US/Eastern";

    assert!(get_time_stamp(inp.to_string()).is_ok());
  }
}
