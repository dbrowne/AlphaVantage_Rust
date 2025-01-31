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
  use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};

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
  use chrono::NaiveDate;
  use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};

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
  use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

  use crate::schema::symbols::dsl::{overview, sid, symbol, symbols};
  symbols
    .filter(overview.eq(true))
    .select((sid, symbol))
    .load::<(i64, String)>(conn)
}
