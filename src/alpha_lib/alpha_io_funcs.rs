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

use std::{collections::HashMap, env::VarError, error::Error, thread, time};

use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime};
use diesel::PgConnection;
use serde_json::Value;

use crate::{
  alpha_lib::{
    alpha_data_types::{
      AlphaSymbol, Convert, FullOverview, RawDailyPrice, RawIntraDayPrice, Root, TopType,
    },
    alpha_funcs::{normalize_alpha_region, top_constants},
    news_type::NewsRoot,
  },
  create_url,
  db_funcs::{
    create_intra_day, create_overview, create_symbol, get_intr_day_max_date, get_next_sid, get_sid,
    get_summary_max_date, get_symbols_and_sids_for, insert_open_close, insert_top_stat,
  },
  db_models::IntraDayPrice,
  dbfunctions::base::establish_connection_or_exit,
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
pub fn process_symbols(sec_vec: Vec<Vec<String>>, load_missed: bool) -> Result<(), Box<dyn Error>> {
  let api_key = get_api_key()?;

  let mut type_map: HashMap<SecurityType, i32> = HashMap::new();
  let mut symbol_map: HashMap<String, i32> = HashMap::new();
  let mut err_ct = 0;

  let conn = &mut establish_connection_or_exit();
  let mut dur_time: DateTime<Local>;
  let mut resp_time: DateTime<Local>;
  let min_time = Duration::milliseconds(350); //We cant make MIN_TIME a constant because it is not a primitive type

  if load_missed {
    let symbs = get_symbols_and_sids_for(conn, "USA".to_string(), "Eqty".to_string())
      .expect("Can't get symbols");
    for (symb, _sid) in symbs {
      symbol_map.insert(symb, 1);
    }
  }

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
          //todo: Improve logging here
          // println!("Bad response for symbol: {:?}", symb);
          // println!("text error: for {:?}", text);
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
            s_id = get_next_sid(conn, sec_type_string).expect("Can't get next sid");
          } else {
            s_id = SecurityType::encode(sec_type, type_map.get(&sec_type).unwrap().clone() as u32);
          }
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
pub fn get_overview(
  connection: &mut PgConnection,
  s_id: i64,
  symb: String,
) -> Result<(), Box<dyn Error>> {
  const SYMBOL: &str = "Symbol";
  let api_key = get_api_key()?;
  let url = create_url!(FuncType:Overview,symb,api_key);
  let response = reqwest::blocking::get(&url);

  if let Ok(response) = response {
    println!("Response is: {:?}", response);
    let text = match response.text() {
      Ok(text) => text,
      Err(err) => {
        println!("Error getting text from response: {:?}", err);
        return Err("Error getting text from response".into());
      }
    };

    if !text.contains(SYMBOL) {
      println!("Missing overview  for Symbol {}: {:?}", symb, text);
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
        //todo: improve logging here
        // println!("Error: for {}: {:?}", symb, text);
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

pub fn get_news_root(url: &str) -> Result<NewsRoot, Box<dyn Error>> {
  let response = reqwest::blocking::get(url)?;
  let text = response.json::<NewsRoot>()?;
  Ok(text)
}

fn parse_intraday_from_csv(text: &str) -> Result<Vec<RawIntraDayPrice>, Box<dyn Error>> {
  let mut recs = csv::Reader::from_reader(text.as_bytes());
  recs
    .deserialize()
    .collect::<Result<Vec<RawIntraDayPrice>, _>>()
    .map_err(|e| e.into())
}

fn persist_ticks(
  connection: &mut PgConnection,
  s_id: i64,
  symb: &String,
  ticks: Vec<RawIntraDayPrice>,
) -> Result<(), Box<dyn Error>> {
  let last_date = get_intr_day_max_date(connection, s_id);

  let mut _skipped = 0;

  let mut _processed = 0;

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
    if tmp_tick.tstamp > last_date {
      let _ = create_intra_day(connection, tmp_tick);
      _processed += 1;
    } else {
      _skipped += 1;
    }
  }
  Ok(())
}

pub fn load_intraday(
  conn: &mut PgConnection,
  symb: &String,
  s_id: i64,
) -> Result<(), Box<dyn Error>> {
  const HEADER: &str = "timestamp,open,high,low,close,volume";
  let api_key = get_api_key()?;
  let url = create_url!(FuncType:TsIntra,symb,api_key);
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
pub fn load_summary(conn: &mut PgConnection, symb: &str, s_id: i64) -> Result<(), Box<dyn Error>> {
  let api_key = get_api_key()?;
  let url = create_url!(FuncType:TsDaily,symb,api_key);
  let text = get_api_data(&url)?;
  const HEADER: &str = "Meta Data";
  if !text.contains(HEADER) {
    //todo: improve logging here
    // eprintln!("Error: for {}: {:?}", symb.clone(), text);
    // alpha vantage will return an error for a symol missing a price
    return Ok(());
  };

  let daily_prices = get_open_close(&text, &symb.to_string())?;
  let last_date = get_summary_max_date(conn, s_id);
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
fn get_open_close(inp: &str, symb: &String) -> Result<Vec<RawDailyPrice>, Box<dyn Error>> {
  const HEADER: &str = "Time Series (Daily)";
  let mut daily_prices: Vec<RawDailyPrice> = Vec::new();
  let json_data: Value = serde_json::from_str(inp)?;

  let json_prices = json_data[HEADER].as_object().ok_or_else(|| {
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
pub fn load_tops(conn: &mut PgConnection) -> Result<(), Box<dyn Error>> {
  let api_key = get_api_key()?;
  let url = create_url!(FuncType:TopQuery," ",api_key);

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
fn get_time_stamp(inp: String) -> Result<NaiveDateTime, Box<dyn Error>> {
  let mut parts = inp.rsplitn(2, ' ');
  let _tz = parts.next().unwrap();
  let tm = parts.next().unwrap();
  let naive_dt = NaiveDateTime::parse_from_str(tm, "%Y-%m-%d %H:%M:%S")?;
  Ok(naive_dt)
}

#[cfg(test)]
mod test {
  use crate::alpha_lib::alpha_io_funcs::get_time_stamp;

  #[test]
  fn t_001() {
    let inp = "2023-10-03 16:15:59 US/Eastern";

    assert!(get_time_stamp(inp.to_string()).is_ok());
  }
}
