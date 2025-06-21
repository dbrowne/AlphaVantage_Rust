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
use diesel::PgConnection;

use crate::{
  alpha_lib::core::alpha_data_types::AlphaSymbol, dbfunctions::common::Error,
  security_types::SymbolFlag, util,
};

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
  use chrono::Local;
  use diesel::RunQueryDsl;

  use crate::{
    db_models::{NewSymbol, Symbol},
    schema::symbols,
  };
  let now = Local::now().naive_local();

  // Use the helper function to parse market open and close times
  let market_open = util::parse::parse_time(&a_sym.marketOpen, "market open time", &a_sym)?;
  let market_close = util::parse::parse_time(&a_sym.marketClose, "market close time", &a_sym)?;

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
pub fn set_symbol_booleans(
  conn: &mut PgConnection,
  s_id: i64,
  flag: SymbolFlag,
  value: bool,
) -> Result<(), Error> {
  use chrono::{DateTime, Local};
  use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

  use crate::{
    db_models::Symbol,
    schema::symbols::dsl::{intraday, m_time, overview, sid, summary, symbols},
  };
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
  use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};

  use crate::schema::symbols::dsl::{region, sec_type, sid, symbol, symbols};

  symbols
    .filter(region.eq(reg).and(sec_type.eq(s_typ)))
    .select((symbol, sid))
    .load::<(String, i64)>(conn)
}
