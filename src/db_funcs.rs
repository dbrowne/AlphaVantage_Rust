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


use diesel::pg::PgConnection;
use std::{env, error::Error};
use dotenvy::dotenv;
use diesel::prelude::*;
use chrono::{ Local, NaiveTime};
use crate::db_models::{Symbol, NewSymbol};
use crate::alpha_lib::alpha_data_types::AlphaSymbol;

/// Establishes a connection to a Postgres database using Diesel.
///
/// This function will read the `DATABASE_URL` environment variable,
/// which should contain the database connection string.
/// It is expected that this environment variable is already set before
/// calling this function.
///
/// If the `DATABASE_URL` environment variable is not set, the function will
/// return an Err value with the message "DATABASE_URL must be set".
///
/// If a connection to the database cannot be established,
/// the function will return an Err value with a message indicating
/// the failure to connect to the database URL provided.
///
/// # Returns
///
/// This function will return a Result:
/// - On successful connection to the database, it will return `Ok(PgConnection)`.
/// - On failure, it will return `Err`, with a dynamic Error (`Box<dyn Error>`) indicating the reason for failure.
///
/// # Example
///
/// ```rust,no_run
/// use db_funcs::establish_connection;
///
/// fn main() {
///     match establish_connection() {
///         Ok(conn) => println!("Successfully connected to the database."),
///         Err(e) => eprintln!("Database connection failed: {}", e),
///     }
/// }
/// ```
pub fn establish_connection() -> Result<PgConnection, Box<dyn Error>> {
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(db) => db,
        Err(_) => return Err("DATABASE_URL must be set".into()),
    };

    let conn = PgConnection::establish(&database_url)
        .map_err(|_|-> Box<dyn Error> { format!("Error connecting to {}", database_url).into() })?;

    Ok(conn)
}




fn parse_time(time_str: &str, error_message: &str, a_sym: &AlphaSymbol) -> Result<NaiveTime, Box<dyn Error>> {
    match NaiveTime::parse_from_str(time_str, "%H:%M") {
        Ok(time) => Ok(time),
        Err(e) => {
            eprintln!("Error parsing {}: {:?}, {:?}", error_message, a_sym, e);
            Err(Box::new(e))
        }
    }
}

pub fn create_symbol(conn: &mut PgConnection, sid: i64, a_sym: AlphaSymbol) -> Result<(), Box<dyn Error>> {
    use crate::schema::symbols;
    let now = Local::now().naive_local();

    // Use the helper function to parse market open and close times
    let market_open = parse_time(&a_sym.marketOpen, "market open time", &a_sym)?;
    let market_close = parse_time(&a_sym.marketClose, "market close time", &a_sym)?;

    let new_symbol: NewSymbol = NewSymbol {
        sid: &sid,
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

    match diesel::insert_into(symbols::table)
        .values(&new_symbol)
        .get_result::<Symbol>(conn)
    {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error saving new symbolts {:?} for sid: {}{:?} ", e, sid, a_sym);
            Err(Box::new(e))
        }
    }
}