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
use crate::db_models::{IntraDayPrice, NewIntraDayPrice, NewOverview, NewOverviewext, NewSymbol,Symbol};
use crate::security_types::sec_types::SymbolFlag;
use chrono::{DateTime, Local, NaiveTime};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::{env, error::Error, process};


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
///```ignore
/// use db_funcs::establish_connection;
///
/// fn main() {
///     match establish_connection() {
///         Ok(conn) => println!("Successfully connected to the database."),
///         Err(e) => eprintln!("Database connection failed: {}", e),
///     }
/// }
/// ```
///
pub fn establish_connection_or_exit() -> PgConnection {
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(db) => db,
        Err(_) => {
            println!("No Database url set");
            process::exit(1);
        }
    };

    let conn = PgConnection::establish(&database_url).unwrap_or_else(|_| {
        println!("Can't establish db connection");
        process::exit(1);
    }
    );


    conn
}

/// Parses a time string into a `NaiveTime` struct.
///
/// This function expects `time_str` to be in the format "HH:MM", where HH represents the hour in 24-hour format
/// and MM represents minutes. For example, "13:45" represents 1:45 PM. If the string is not in this format,
/// the function will return an error.
///
/// # Arguments
///
/// * `time_str`: A string slice that holds the time to be parsed.
/// * `error_message`: A string slice used as the error message in case the `time_str` parsing fails.
/// * `a_sym`: A reference to an `AlphaSymbol` struct, which is included in the error message if parsing fails.
///
/// # Returns
///
/// * `Ok(NaiveTime)`: If the `time_str` is successfully parsed into `NaiveTime`.
/// * `Err(Box<dyn Error>)`: If the `time_str` cannot be parsed into `NaiveTime`.
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
///
fn parse_time(
    time_str: &str,
    error_message: &str,
    a_sym: &AlphaSymbol,
) -> Result<NaiveTime, Box<dyn Error>> {
    match NaiveTime::parse_from_str(time_str, "%H:%M") {
        Ok(time) => Ok(time),
        Err(e) => {
            eprintln!("Error parsing {}: {:?}, {:?}", error_message, a_sym, e);
            Err(Box::new(e))
        }
    }
}

/// Creates a new symbol entry in the database.
///
/// This function takes a database connection, a symbol id (`sid`), and an `AlphaSymbol` struct as arguments.
/// It then creates a new symbol with these details and saves it to the 'symbols' table in the database.
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
/// * `Err(Box<dyn Error>)`: If there was an error inserting the symbol into the database.
///
/// # Errors
///
/// This function will return an error if there's a problem inserting the new symbol into the database.
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
pub fn create_symbol(
    conn: &mut PgConnection,
    s_id: i64,
    a_sym: AlphaSymbol,
) -> Result<(), Box<dyn Error>> {
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

    match diesel::insert_into(symbols::table)
        .values(&new_symbol)
        .get_result::<Symbol>(conn)
    {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!(
                "Error saving new symbols {:?} for sid: {}{:?} ",
                e, s_id, a_sym
            );
            Err(Box::new(e))
        }
    }
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
/// * `full_ov`: A `FullOverview` struct containing comprehensive overview data of the financial entity.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>`: Returns an `Ok(())` if the operation is successful. Returns an `Err` wrapped in a `Box` if any error occurs.
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
/// * This function will panic if there's an error while saving data into the `overviews` or `overviewexts` tables.
/// * This function will also panic if there's an error setting the symbol flag for the provided `sid` in `FullOverview`.
///
/// # ToDo
///
/// * Refactor the database insertion code to enhance maintainability.
/// * Consider returning a custom error type or using more descriptive error handling.

pub fn create_overview(
    conn: &mut PgConnection,
    full_ov: FullOverview,
) -> Result<(), Box<dyn Error>> {
    use crate::schema::overviewexts;
    use crate::schema::overviews;

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
    if let  Err(err) = diesel::insert_into(overviews::table)
        .values(&new_overview)
        .execute(conn) {
        eprintln!("Error {:?}",err);

    }


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
    if let  Err(err) = diesel::insert_into(overviewexts::table)
        .values(&new_overviewext)
        .execute(conn){
        eprintln!("{:?}",err);
        eprintln!("cannot insert overviewext");
    }



    if  let  Err(err) = set_symbol_booleans(conn, full_ov.sid.clone(), SymbolFlag::Overview, true) {
        eprintln!("{:?}",err);
        eprintln!("Cannot set symbol flag for overview: fro sid {:?}",full_ov.sid);
    };
    Ok(())
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
/// - `SymbolFlag::All`: the function will update all three fields (`overview`, `intraday`, and `summary`).
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
) -> Result<(), Box<dyn Error>> {
    use crate::schema::symbols::dsl::{intraday, m_time, sid, overview, summary, symbols};
    let localt: DateTime<Local> = Local::now();
    let now = localt.naive_local();
    match flag {
        SymbolFlag::Overview => {
            diesel::update(symbols.find(sid))
                .set((overview.eq(value), m_time.eq(now)))
                .get_result::<Symbol>(conn)
                .map_err(|e: diesel::result::Error| {
                    eprintln!("Cannot update overview for sid {}: {:?}", s_id, e);
                    Box::<dyn Error>::from(e)
                })?;
        }
        SymbolFlag::Intraday => {
            diesel::update(symbols.find(sid))
                .set((intraday.eq(value), m_time.eq(now)))
                .get_result::<Symbol>(conn)
                .map_err(|e: diesel::result::Error| {
                    eprintln!("Cannot update intraday for sid {}: {:?}", s_id, e);
                    Box::<dyn Error>::from(e)
                })?;
        }
        SymbolFlag::Summary => {
            diesel::update(symbols.find(sid))
                .set((summary.eq(value), m_time.eq(now)))
                .get_result::<Symbol>(conn)
                .map_err(|e: diesel::result::Error| {
                    eprintln!("Cannot update summary for sid {}: {:?}", s_id, e);
                    Box::<dyn Error>::from(e)
                })?;
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
                .get_result::<Symbol>(conn)
                .map_err(|e: diesel::result::Error| {
                    eprintln!("Cannot update all flags for sid {}: {:?}", s_id, e);
                    Box::<dyn Error>::from(e)
                })?;
        }
    }

    Ok(())
}

/// Fetch the IDs and names of symbols for a specific region and type from the database.
///
/// This function retrieves the IDs (`sid`) and names (`symbol`) of all symbols in the database that match
/// the specified region (`reg`) and security type (`s_typ`).
///
/// The retrieved data is returned as a vector of tuples, where each tuple represents a symbol and contains
/// the symbol's ID and name.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the Postgres connection.
/// * `reg` - The region for which to retrieve symbols. This should match the `region` field in the symbols table in the database.
/// * `s_typ` - The type of security for which to retrieve symbols. This should match the `sec_type` field in the symbols table in the database.
///
/// # Returns
///
/// This function returns a `Result` type. If the operation is successful, it returns `Ok(Vec<(i64, String)>)`,
/// where `Vec<(i64, String)>` is a vector of tuples containing the ID and name of each matching symbol.
///
/// If the operation fails, it returns `Err(diesel::result::Error)`, where `diesel::result::Error` is the error
/// generated by the Diesel ORM or the PostgreSQL database.
///
/// # Errors
///
/// This function will return an error if the database operation fails. For example, if there's a problem with the
/// database connection, or if the `region` or `sec_type` fields do not exist in the symbols table in the database.
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


pub fn get_sids_and_names_with_overview(
    conn: &mut PgConnection) -> Result<Vec<(i64, String)>, diesel::result::Error> {
    use crate::schema::symbols::dsl::{sid, symbol, symbols, overview};
    symbols
        .filter(overview.eq(true))
        .select((sid, symbol))
        .load::<(i64, String)>(conn)
}

pub fn create_intra_day(conn: &mut PgConnection, tick: IntraDayPrice) -> Result<(), Box<dyn Error>> {
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

    let _ = match set_symbol_booleans(conn, new_mkt_price.sid.clone(), SymbolFlag::Intraday, true) {
        Ok(_) => (),
        Err(err) => {
            println!("{:?}", err);
            println!("cannot set overfiew flag for sid: {}", new_mkt_price.sid.clone());
            return  Err(err);
        }
    };
    Ok(())
}
