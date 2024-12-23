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

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
use diesel::{dsl::max, pg::PgConnection, prelude::*};
pub use thiserror::Error;

// NOTE!!! THIS WILL BE BROKEN INTO SEPARATE FILES INTO dbfunctions
use crate::alpha_lib::core::alpha_data_types::{
  AlphaSymbol, FullOverview, GTopStat, RawDailyPrice,
};
use crate::{
  db_models::{
    IntraDayPrice, NewIntraDayPrice, NewOverview, NewOverviewext, NewProcState, NewProcType,
    NewSummaryPrice, NewSymbol, NewTopStat, Overview, Overviewext, Symbol,
  },
  schema::procstates::dsl::procstates,
  security_types::sec_types::SymbolFlag,
};
#[derive(Error, Debug)]
pub enum Error {
  #[error(transparent)]
  ParseError(#[from] chrono::ParseError),
  #[error("Failed to parse time:{0}")]
  TimeParse(String),
  #[error(transparent)]
  Diesel(#[from] diesel::result::Error),
  #[error("No intraday prices found for sid: {0}")]
  NoData(i64),
  #[error("Unique constraint violation")]
  UniqueViolation,
}

/// Parses a time string into a `NaiveTime` struct.
///
/// This function expects `time_str` to be in the format "HH:MM", where HH represents the hour in
/// 24-hour format and MM represents minutes. For example, "13:45" represents 1:45 PM. If the string
/// is not in this format, the function will return an error.
///
/// # Arguments
///
/// * `time_str`: A string slice that holds the time to be parsed.
/// * `error_message`: A string slice used as the error message in case the `time_str` parsing
///   fails.
/// * `a_sym`: A reference to an `AlphaSymbol` struct, which is included in the error message if
///   parsing fails.
///
/// # Returns
///
/// * `Ok(NaiveTime)`: If the `time_str` is successfully parsed into `NaiveTime`.
/// * `Error)`: If the `time_str` cannot be parsed into `NaiveTime`.
///
/// # Errors
///
/// This function will return an error if `time_str` cannot be parsed into a `NaiveTime` struct.
///
/// # Example
///
///
/// let time_str = "13:45";
/// let a_sym = AlphaSymbol::new();
/// match parse_time(time_str, "Failed to parse time string", &a_sym) {
///     Ok(time) => println!("Parsed time: {:?}", time),
///     Err(e) => println!("Error: {}", e),
/// }
fn parse_time(
  time_str: &str,
  error_message: &str,
  a_sym: &AlphaSymbol,
) -> Result<NaiveTime, Error> {
  NaiveTime::parse_from_str(time_str, "%H:%M").map_err(|e| {
    let msg = format!("{}: {}. Symbol: {}", error_message, e, a_sym.symbol);
    Error::TimeParse(msg)
  })
}

/// Creates a new symbol entry in the database.
///
/// This function takes a database connection, a symbol id (`sid`), and an `AlphaSymbol` struct as
/// arguments. It then creates a new symbol with these details and saves it to the 'symbols' table
/// in the database.
///
/// # Arguments
///
/// * `conn`: A mutable reference to a `PgConnection`. Represents the current database connection.
/// * `sid`: A 64-bit integer. Represents the symbol id for the new symbol entry.
/// * `a_sym`: An `AlphaSymbol` struct. Contains the details of the new symbol.
///
/// # Returns
///
/// * `Ok(())`: If the symbol is successfully inserted into the database.
/// * `Error)`: If there was an error inserting the symbol into the database.
///
/// # Errors
///
/// This function will return an error if there's a problem inserting the new symbol into the
/// database.
///
/// # Example
///
/// ```ignore
/// let conn = &mut establish_connection().unwrap();
/// let sid = 12345;
/// let a_sym = AlphaSymbol::new(/* parameters */);
/// match create_symbol(conn, sid, a_sym) {
///     Ok(_) => println!("Successfully inserted new symbol"),
///     Err(e) => println!("Error inserting new symbol: {}", e),
/// }
/// ```
pub fn create_symbol(conn: &mut PgConnection, s_id: i64, a_sym: AlphaSymbol) -> Result<(), Error> {
  use crate::schema::symbols;
  let now = Local::now().naive_local();

  // Use the helper function to parse market open and close times
  let market_open = parse_time(&a_sym.marketOpen, "market open time", &a_sym)?;
  let market_close = parse_time(&a_sym.marketClose, "market close time", &a_sym)?;

  let new_symbol: NewSymbol = NewSymbol {
    sid: &s_id,
    symbol: &a_sym.symbol,
    name: &a_sym.name,
    sec_type: &a_sym.s_type,
    region: &a_sym.region,
    marketopen: &market_open,
    marketclose: &market_close,
    timezone: &a_sym.timezone,
    currency: &a_sym.currency,
    overview: &false,
    intraday: &false,
    summary: &false,
    c_time: &now,
    m_time: &now,
  };

  diesel::insert_into(symbols::table)
    .values(&new_symbol)
    .get_result::<Symbol>(conn)
    .map(|_| ())
    .map_err(Error::from)
}

/// Inserts a full overview of a financial entity into the database.
///
/// This function takes in a `PgConnection` reference and a `FullOverview` struct.
/// It then inserts the overview data into two database tables: `overviews` and `overviewexts`.
/// Additionally, it sets a symbol flag for the given financial entity.
///
/// # Parameters
///
/// * `conn`: A mutable reference to a `PgConnection` which represents the database connection.
/// * `full_ov`: A `FullOverview` struct containing comprehensive overview data of the financial
///   entity.
///
/// # Returns
///
/// * `Result<(), Error>`: Returns an `Ok(())` if the operation is successful. Returns an `Err`
///   wrapped in a `Box` if any error occurs.
///
/// # Examples
///
/// ```ignore
/// let conn = establish_connection();
/// let overview = FullOverview { /* ...populate data fields... */ };
///
/// match create_overview(&mut conn, overview) {
///     Ok(_) => println!("Overview created successfully."),
///     Err(e) => println!("Error creating overview: {:?}", e),
/// }
/// ```
///
/// # Panics
///
/// * This function will panic if there's an error while saving data into the `overviews` or
///   `overviewexts` tables.
/// * This function will also panic if there's an error setting the symbol flag for the provided
///   `sid` in `FullOverview`.
///
/// # ToDo
///
/// * Refactor the database insertion code to enhance maintainability.
/// * Consider returning a custom error type or using more descriptive error handling.

pub fn create_overview(conn: &mut PgConnection, full_ov: FullOverview) -> Result<(), Error> {
  use crate::schema::{overviewexts, overviews};

  let localt: DateTime<Local> = Local::now();
  let now = localt.naive_local(); // NaiveDateTime::now();

  let new_overview: NewOverview = NewOverview {
    sid: &full_ov.sid,
    symbol: &full_ov.symbol,
    name: &full_ov.name,
    description: &full_ov.description,
    cik: &full_ov.cik,
    exch: &full_ov.exch,
    curr: &full_ov.curr,
    country: &full_ov.country,
    sector: &full_ov.sector,
    industry: &full_ov.industry,
    address: &full_ov.address,
    fiscalyearend: &full_ov.fiscalyearend,
    latestquarter: &full_ov.latestquarter,
    marketcapitalization: &full_ov.marketcapitalization,
    ebitda: &full_ov.ebitda,
    peratio: &full_ov.peratio,
    pegratio: &full_ov.pegratio,
    bookvalue: &full_ov.bookvalue,
    dividendpershare: &full_ov.dividendpershare,
    dividendyield: &full_ov.dividendyield,
    eps: &full_ov.eps,
    c_time: &now,
    mod_time: &now,
  };
  //todo: refactor this
  _ = diesel::insert_into(overviews::table)
    .values(&new_overview)
    .execute(conn)
    .map_err(Error::from);

  let new_overviewext: NewOverviewext = NewOverviewext {
    sid: &full_ov.sid.clone(),
    revenuepersharettm: &full_ov.revenuepersharettm,
    profitmargin: &full_ov.profitmargin,
    operatingmarginttm: &full_ov.operatingmarginttm,
    returnonassetsttm: &full_ov.returnonassetsttm,
    returnonequityttm: &full_ov.returnonequityttm,
    revenuettm: &full_ov.revenuettm,
    grossprofitttm: &full_ov.grossprofitttm,
    dilutedepsttm: &full_ov.dilutedepsttm,
    quarterlyearningsgrowthyoy: &full_ov.quarterlyearningsgrowthyoy,
    quarterlyrevenuegrowthyoy: &full_ov.quarterlyrevenuegrowthyoy,
    analysttargetprice: &full_ov.analysttargetprice,
    trailingpe: &full_ov.trailingpe,
    forwardpe: &full_ov.forwardpe,
    pricetosalesratiottm: &full_ov.pricetosalesratiottm,
    pricetobookratio: &full_ov.pricetobookratio,
    evtorevenue: &full_ov.evtorevenue,
    evtoebitda: &full_ov.evtoebitda,
    beta: &full_ov.beta,
    annweekhigh: &full_ov.annweekhigh,
    annweeklow: &full_ov.annweeklow,
    fiftydaymovingaverage: &full_ov.fiftydaymovingaverage,
    twohdaymovingaverage: &full_ov.twohdaymovingaverage,
    sharesoutstanding: &full_ov.sharesoutstanding,
    dividenddate: &full_ov.dividenddate,
    exdividenddate: &full_ov.exdividenddate,
    c_time: &now,
    mod_time: &now,
  };

  //todo: refactor this
  _ = diesel::insert_into(overviewexts::table)
    .values(&new_overviewext)
    .execute(conn)
    .map(|_| ())
    .map_err(Error::from);

  set_symbol_booleans(conn, full_ov.sid.clone(), SymbolFlag::Overview, true)?;

  Ok(())
}

pub fn get_full_overview(conn: &mut PgConnection, sym: &str) -> Result<FullOverview, Error> {
  use crate::schema::{overviewexts, overviews};
  let overview = overviews::table
    .filter(overviews::symbol.eq(sym))
    .first::<Overview>(conn)
    .map_err(Error::from)?;

  let overviewext = overviewexts::table
    .filter(overviewexts::sid.eq(overview.sid))
    .first::<Overviewext>(conn)
    .map_err(Error::from)?;

  Ok(FullOverview {
    sid: overview.sid,
    symbol: overview.symbol,
    name: overview.name,
    description: overview.description,
    cik: overview.cik,
    exch: overview.exch,
    curr: overview.curr,
    country: overview.country,
    sector: overview.sector,
    industry: overview.industry,
    address: overview.address,
    fiscalyearend: overview.fiscalyearend,
    latestquarter: overview.latestquarter,
    marketcapitalization: overview.marketcapitalization,
    ebitda: overview.ebitda,
    peratio: overview.peratio,
    pegratio: overview.pegratio,
    bookvalue: overview.bookvalue,
    dividendpershare: overview.dividendpershare,
    dividendyield: overview.dividendyield,
    eps: overview.eps,
    revenuepersharettm: overviewext.revenuepersharettm,
    profitmargin: overviewext.profitmargin,
    operatingmarginttm: overviewext.operatingmarginttm,
    returnonassetsttm: overviewext.returnonassetsttm,
    returnonequityttm: overviewext.returnonequityttm,
    revenuettm: overviewext.revenuettm,
    grossprofitttm: overviewext.grossprofitttm,
    dilutedepsttm: overviewext.dilutedepsttm,
    quarterlyearningsgrowthyoy: overviewext.quarterlyearningsgrowthyoy,
    quarterlyrevenuegrowthyoy: overviewext.quarterlyrevenuegrowthyoy,
    analysttargetprice: overviewext.analysttargetprice,
    trailingpe: overviewext.trailingpe,
    forwardpe: overviewext.forwardpe,
    pricetosalesratiottm: overviewext.pricetosalesratiottm,
    pricetobookratio: overviewext.pricetobookratio,
    evtorevenue: overviewext.evtorevenue,
    evtoebitda: overviewext.evtoebitda,
    beta: overviewext.beta,
    annweekhigh: overviewext.annweekhigh,
    annweeklow: overviewext.annweeklow,
    fiftydaymovingaverage: overviewext.fiftydaymovingaverage,
    twohdaymovingaverage: overviewext.twohdaymovingaverage,
    sharesoutstanding: overviewext.sharesoutstanding,
    dividenddate: overviewext.dividenddate,
    exdividenddate: overviewext.exdividenddate,
  })
}
/// Update boolean values (flags) of a specified symbol in the database.
///
/// This function receives a database connection, a symbol ID (`sid`), a flag
/// type (`SymbolFlag`), and a boolean value (`value`) as input.
/// The `SymbolFlag` can be one of: `Overview`, `Intraday`, `Summary`, or `All`.
///
/// For a given symbol ID (`sid`), the function will set the specified flag(s)
/// to the provided boolean value (`value`) and update the modification time
/// (`m_time`) to the current local time.
///
/// Depending on the value of `SymbolFlag`, the function will update different
/// fields of the symbol entry in the database:
///
/// - `SymbolFlag::Overview`: the function will update the `overview` field.
/// - `SymbolFlag::Intraday`: the function will update the `intraday` field.
/// - `SymbolFlag::Summary`: the function will update the `summary` field.
/// - `SymbolFlag::All`: the function will update all three fields (`overview`, `intraday`, and
///   `summary`).
///
/// The function will return `Ok(())` if the database update operation is
/// successful, otherwise, it will return an error.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the Postgres connection.
/// * `sid` - A symbol ID as i64.
/// * `flag` - A flag that indicates which field(s) to update.
/// * `value` - A boolean value to set for the specified field(s).
///
/// # Returns
///
/// This function will return a standard `Result`. If the operation is successful,
/// it will return `Ok(())`. If the operation fails, it will return `Err(e)`,
/// where `e` is the error generated by the Diesel ORM or the PostgreSQL database.
///
/// # Errors
///
/// This function will return an error if the database operation fails, for example,
/// if there's a problem with the connection, or if the symbol with the specified `sid`
/// does not exist in the database.
fn set_symbol_booleans(
  conn: &mut PgConnection,
  s_id: i64,
  flag: SymbolFlag,
  value: bool,
) -> Result<(), Error> {
  use crate::schema::symbols::dsl::{intraday, m_time, overview, sid, summary, symbols};
  let localt: DateTime<Local> = Local::now();
  let now = localt.naive_local();
  match flag {
    SymbolFlag::Overview => {
      diesel::update(symbols)
        .filter(sid.eq(s_id))
        .set((overview.eq(value), m_time.eq(now)))
        .get_result::<Symbol>(conn)?;
    }
    SymbolFlag::Intraday => {
      diesel::update(symbols)
        .filter(sid.eq(s_id))
        .set((intraday.eq(value), m_time.eq(now)))
        .get_result::<Symbol>(conn)?;
    }
    SymbolFlag::Summary => {
      diesel::update(symbols)
        .filter(sid.eq(s_id))
        .set((summary.eq(value), m_time.eq(now)))
        .get_result::<Symbol>(conn)?;
    }
    SymbolFlag::All => {
      // todo: need to test this
      diesel::update(symbols.find(sid))
        .set((
          overview.eq(value),
          intraday.eq(value),
          summary.eq(value),
          m_time.eq(now),
        ))
        .get_result::<Symbol>(conn)?;
    }
  }

  Ok(())
}

/// Fetch the IDs and names of symbols for a specific region and type from the database.
///
/// This function retrieves the IDs (`sid`) and names (`symbol`) of all symbols in the database that
/// match the specified region (`reg`) and security type (`s_typ`).
///
/// The retrieved data is returned as a vector of tuples, where each tuple represents a symbol and
/// contains the symbol's ID and name.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the Postgres connection.
/// * `reg` - The region for which to retrieve symbols. This should match the `region` field in the
///   symbols table in the database.
/// * `s_typ` - The type of security for which to retrieve symbols. This should match the `sec_type`
///   field in the symbols table in the database.
///
/// # Returns
///
/// This function returns a `Result` type. If the operation is successful, it returns `Ok(Vec<(i64,
/// String)>)`, where `Vec<(i64, String)>` is a vector of tuples containing the ID and name of each
/// matching symbol.
///
/// If the operation fails, it returns `Err(diesel::result::Error)`, where `diesel::result::Error`
/// is the error generated by the Diesel ORM or the PostgreSQL database.
///
/// # Errors
///
/// This function will return an error if the database operation fails. For example, if there's a
/// problem with the database connection, or if the `region` or `sec_type` fields do not exist in
/// the symbols table in the database.
pub fn get_sids_and_names_for(
  conn: &mut PgConnection,
  reg: String,
  s_typ: String,
) -> Result<Vec<(i64, String)>, diesel::result::Error> {
  use crate::schema::symbols::dsl::{region, sec_type, sid, symbol, symbols};

  symbols
    .filter(region.eq(reg).and(sec_type.eq(s_typ)))
    .select((sid, symbol))
    .load::<(i64, String)>(conn)
}

/// Retrieves a list of symbols and their corresponding sids from the database for a specified
/// region, security type, and after a specified date.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the PostgreSQL connection.
/// * `region` - A string specifying the region to filter by.
/// * `sec_typ` - A string specifying the security type to filter by.
/// * `after_date` - A string representing the date in the format "yyyy-mm-dd". Only symbols with a
///   `c_time` greater than this date will be retrieved.
///
/// # Returns
///
/// A `Result` containing a vector of tuples, where each tuple consists of a symbol (String) and its
/// corresponding sid (i64), or a `diesel::result::Error` if an error occurs.
///
/// # Errors
///
/// Returns a `diesel::result::Error` if there is an issue with the database query or if the date
/// string cannot be parsed.
///
/// # Example
///
/// ```ignore
/// let connection = establish_connection();
/// let region = "US".to_string();
/// let sec_typ = "Equity".to_string();
/// let after_date = "2023-01-01".to_string();
/// let result = get_sids_and_names_after(&mut connection, region, sec_typ, after_date);
/// match result {
///     Ok(data) => println!("Retrieved data: {:?}", data),
///     Err(e) => println!("Error occurred: {:?}", e),
/// }
/// ```
pub fn get_sids_and_names_after(
  conn: &mut PgConnection,
  region: String,
  sec_typ: String,
  after_date: String,
) -> Result<Vec<(String, i64)>, diesel::result::Error> {
  use crate::schema::symbols::dsl::{c_time, region as db_region, sec_type, sid, symbol, symbols};

  // Parse the after_date string into a NaiveDate
  let after_date = NaiveDate::parse_from_str(&after_date, "%Y-%m-%d")
    .map_err(|_| diesel::result::Error::NotFound)?; // Handle parsing error

  // Convert NaiveDate to NaiveDateTime to compare with the c_time timestamp
  let after_date_time = after_date.and_hms_opt(0, 0, 0).unwrap(); // Set the time to the start of the day

  symbols
    .filter(
      db_region
        .eq(region)
        .and(sec_type.eq(sec_typ))
        .and(c_time.gt(after_date_time)),
    )
    .select((symbol, sid))
    .load::<(String, i64)>(conn)
}

/// Retrieves a list of symbols and their corresponding sids from the database for a specified
/// region and security type.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the PostgreSQL connection.
/// * `reg` - A string specifying the region to filter by.
/// * `s_typ` - A string specifying the security type to filter by.
///
/// # Returns
///
/// A `Result` containing a vector of tuples, where each tuple consists of a symbol (String) and its
/// corresponding sid (i64), or a `diesel::result::Error` if an error occurs.
///
/// # Errors
///
/// Returns a `diesel::result::Error` if there is an issue with the database query.
///
/// # Example
///
/// ```ignore
/// let connection = establish_connection();
/// let reg = "US".to_string();
/// let s_typ = "Equity".to_string();
/// let result = get_symbols_and_sids_for(&mut connection, reg, s_typ);
/// match result {
///     Ok(data) => println!("Retrieved data: {:?}", data),
///     Err(e) => println!("Error occurred: {:?}", e),
/// }
/// ```
pub fn get_symbols_and_sids_for(
  conn: &mut PgConnection,
  reg: String,
  s_typ: String,
) -> Result<Vec<(String, i64)>, diesel::result::Error> {
  use crate::schema::symbols::dsl::{region, sec_type, sid, symbol, symbols};

  symbols
    .filter(region.eq(reg).and(sec_type.eq(s_typ)))
    .select((symbol, sid))
    .load::<(String, i64)>(conn)
}

/// Retrieves a list of sids and their corresponding symbols from the database where the overview
/// flag is set to true.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the PostgreSQL connection.
///
/// # Returns
///
/// A `Result` containing a vector of tuples, where each tuple consists of a sid (i64) and its
/// corresponding symbol (String), or a `diesel::result::Error` if an error occurs.
///
/// # Errors
///
/// Returns a `diesel::result::Error` if there is an issue with the database query.
///
/// # Example
///
/// ```ignore
/// let connection = establish_connection();
/// let result = get_sids_and_names_with_overview(&mut connection);
/// match result {
///     Ok(data) => println!("Retrieved data: {:?}", data),
///     Err(e) => println!("Error occurred: {:?}", e),
/// }
/// ```
pub fn get_sids_and_names_with_overview(
  conn: &mut PgConnection,
) -> Result<Vec<(i64, String)>, diesel::result::Error> {
  use crate::schema::symbols::dsl::{overview, sid, symbol, symbols};
  symbols
    .filter(overview.eq(true))
    .select((sid, symbol))
    .load::<(i64, String)>(conn)
}

pub fn create_intra_day(conn: &mut PgConnection, tick: IntraDayPrice) -> Result<(), Error> {
  use crate::schema::intradayprices;

  let new_mkt_price = NewIntraDayPrice {
    sid: &tick.sid,
    tstamp: &tick.tstamp,
    symbol: &tick.symbol,
    open: &tick.open,
    high: &tick.high,
    low: &tick.low,
    close: &tick.close,
    volume: &tick.volume,
  };
  diesel::insert_into(intradayprices::table)
    .values(&new_mkt_price)
    .execute(conn)?;

  set_symbol_booleans(conn, new_mkt_price.sid.clone(), SymbolFlag::Intraday, true)
    .map(|_| ())
    .map_err(Error::from)
}

pub fn insert_open_close(
  conn: &mut PgConnection,
  symb: &String,
  s_id: i64,
  open_close: RawDailyPrice,
) -> Result<(), Error> {
  use crate::schema::summaryprices;

  let np: NewSummaryPrice = NewSummaryPrice {
    date: &open_close.date,
    sid: &s_id,
    symbol: &symb,
    open: &open_close.open,
    high: &open_close.high,
    low: &open_close.low,
    close: &open_close.close,
    volume: &open_close.volume,
  };
  diesel::insert_into(summaryprices::table)
    .values(&np)
    .execute(conn)?;
  set_symbol_booleans(conn, s_id.clone(), SymbolFlag::Summary, true)
    .map(|_| ())
    .map_err(Error::from)
}

///
///
/// todo: get rid of the cut and paste
pub fn get_intr_day_max_date(conn: &mut PgConnection, s_id: i64) -> Result<NaiveDateTime, Error> {
  use crate::schema::intradayprices::dsl::{intradayprices, sid, tstamp};

  intradayprices
    .filter(sid.eq(s_id))
    .select(tstamp)
    .order(tstamp.desc())
    .first::<NaiveDateTime>(conn)
    .map_err(|err| match err {
      diesel::result::Error::NotFound => Error::NoData(s_id),
      other => Error::Diesel(other),
    })
}

pub fn get_summary_max_date(conn: &mut PgConnection, s_id: i64) -> Result<NaiveDate, Error> {
  use crate::schema::summaryprices::dsl::{date, sid, summaryprices};

  summaryprices
    .filter(sid.eq(s_id))
    .select(date)
    .order(date.desc())
    .first::<NaiveDate>(conn)
    .map_err(|err| match err {
      diesel::result::Error::NotFound => Error::NoData(s_id),
      other => Error::Diesel(other),
    })
}

pub fn get_sid(conn: &mut PgConnection, ticker: String) -> Result<i64, diesel::result::Error> {
  use crate::schema::symbols::dsl::{sid, symbol, symbols};
  let res = symbols
    .filter(symbol.eq(ticker.clone()))
    .select(sid)
    .load::<i64>(conn);
  let _tt = match res {
    Ok(res) => {
      if res.len() > 0 {
        return Ok(res[0]);
      } else {
        //todo:: fix error logging
        // eprintln!("Cannot find sid for ticker {}", ticker);
        return Err(diesel::result::Error::NotFound);
      }
    }
    Err(err) => {
      //todo:  fix error logging
      // eprintln!("Cannot find sid for ticker {}", ticker);
      return Err(err);
    }
  };
}

pub fn get_next_sid(conn: &mut PgConnection, s_type: String) -> Result<i64, diesel::result::Error> {
  use crate::schema::symbols::dsl::{sec_type, sid, symbols};

  let result = symbols
    .filter(sec_type.eq(s_type.clone()))
    .select(max(sid))
    .first::<Option<i64>>(conn);

  match result {
    Ok(Some(max_sid)) => Ok(max_sid + 1),
    Ok(None) => {
      eprintln!("No entries found for sec_type, assigning default start sid of 1");
      Ok(1) // Assuming SID starts at 1 if no existing records are found
    }
    Err(err) => {
      eprintln!("Cannot find next sid for sec_type {:?}", s_type);
      Err(err)
    }
  }
}

pub fn insert_top_stat(
  conn: &mut PgConnection,
  s_id: i64,
  ts: GTopStat,
  evt_type: &str,
  upd_time: NaiveDateTime,
) -> Result<(), Error> {
  use crate::schema::topstats;
  let ns = NewTopStat {
    date: &upd_time,
    event_type: evt_type,
    sid: &s_id,
    symbol: &ts.ticker,
    price: &ts.price,
    change_val: &ts.change_amount,
    change_pct: &ts.change_percentage,
    volume: &ts.volume,
  };

  // todo: refactor this  crap error handling
  match diesel::insert_into(topstats::table)
    .values(&ns)
    .execute(conn)
  {
    Ok(row_count) => {
      println!("Row count: {}", row_count);
      Ok(())
    }
    Err(_) => {
      // Handle unique violation specifically
      println!(
        "Unique constraint violation for sid: {}, event_type: {}",
        s_id, evt_type
      );
      Err(Error::UniqueViolation)
    }
  }
}

pub fn get_proc_id(conn: &mut PgConnection, proc_name: &str) -> Result<i32, diesel::result::Error> {
  use crate::schema::proctypes;

  let result = proctypes::table
    .filter(proctypes::name.eq(proc_name))
    .select(proctypes::id)
    .first::<i32>(conn);
  result
}

pub fn get_proc_name(conn: &mut PgConnection, p_id: i32) -> Result<String, diesel::result::Error> {
  use crate::schema::proctypes;

  let result = proctypes::table
    .filter(proctypes::id.eq(p_id))
    .select(proctypes::name)
    .first::<String>(conn);
  result
}

pub fn get_proc_id_or_insert(
  conn: &mut PgConnection,
  proc_name: &str,
) -> Result<i32, diesel::result::Error> {
  use crate::schema::proctypes;

  let result = proctypes::table
    .filter(proctypes::name.eq(proc_name))
    .select(proctypes::id)
    .first::<i32>(conn);
  match result {
    Ok(res) => Ok(res),
    Err(_) => {
      let new_proc = NewProcType {
        name: &proc_name.to_string(),
      };
      let res = diesel::insert_into(proctypes::table)
        .values(&new_proc)
        .returning(proctypes::id)
        .get_result::<i32>(conn);
      res
    }
  }
}

pub fn log_proc_start(conn: &mut PgConnection, pid: i32) -> Result<i32, diesel::result::Error> {
  use crate::schema::procstates;
  let localt: DateTime<Local> = Local::now();
  let now = localt.naive_local();

  let st = NewProcState {
    proc_id: &pid,
    start_time: &now,
    end_state: &1,
    end_time: &now,
  };

  let res = diesel::insert_into(procstates::table)
    .values(&st)
    .returning(procstates::spid)
    .get_result::<i32>(conn);

  res
}

pub fn log_proc_end(
  conn: &mut PgConnection,
  pid: i32,
  e_state: i32,
) -> Result<usize, diesel::result::Error> {
  use crate::schema::procstates::dsl::{end_state, end_time, spid};

  let localt: NaiveDateTime = Local::now().naive_local();
  diesel::update(procstates.filter(spid.eq(pid)))
    .set((end_time.eq(localt), end_state.eq(&e_state)))
    .execute(conn)
}
