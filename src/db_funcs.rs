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
use std::{env, error::Error, process};
use dotenvy::dotenv;
use diesel::prelude::*;
use chrono::{DateTime, Local, NaiveTime};
use crate::db_models::{Symbol, NewSymbol, NewOverview, Overview, NewOverviewext, Overviewext};
use crate::alpha_lib::alpha_data_types::{AlphaSymbol, FullOverview};
use crate::security_types::sec_types::SymbolFlag;

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
/// ```
/// let time_str = "13:45";
/// let a_sym = AlphaSymbol::new();
/// match parse_time(time_str, "Failed to parse time string", &a_sym) {
///     Ok(time) => println!("Parsed time: {:?}", time),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
fn parse_time(time_str: &str, error_message: &str, a_sym: &AlphaSymbol) -> Result<NaiveTime, Box<dyn Error>> {
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


pub fn create_overview(conn: &mut PgConnection, full_ov: FullOverview) -> Result<(), Box<dyn Error>> {
    use crate::schema::overviews;
    use crate::schema::overviewexts;

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
    let _ = diesel::insert_into(overviews::table)
        .values(&new_overview)
        .get_result::<Overview>(conn)
        .expect("Error saving new overview");

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
    let _ = diesel::insert_into(overviewexts::table)
        .values(&new_overviewext)
        .get_result::<Overviewext>(conn)
        .expect("Error saving new overviewext");

    let _ = match set_symbol_booleans(conn, full_ov.sid.clone(), SymbolFlag::Overview, true) {
        Ok(_) => (),
        Err(err) => {
            println!("{:?}", err);
            println!("cannot set symbolt flag for sid: {}", full_ov.sid.clone());
            process::exit(1);
        }
    };
    Ok(())
}

fn set_symbol_booleans(conn: &mut PgConnection, sid: i64, flag: SymbolFlag, value: bool) -> Result<(), Box<dyn Error>> {
    use crate::schema::symbols::dsl::{symbols, overview, intraday, summary, m_time};
    let localt: DateTime<Local> = Local::now();
    let now = localt.naive_local();
    let rslt: QueryResult<Symbol>;
    match flag {
        SymbolFlag::Overview => {
            rslt = diesel::update(symbols.find(sid))
                .set((overview.eq(value), m_time.eq(now)))
                .get_result::<Symbol>(conn);
        }
        SymbolFlag::Intraday => {
            rslt = diesel::update(symbols.find(sid))
                .set((intraday.eq(value), m_time.eq(now)))
                .get_result::<Symbol>(conn);
        }
        SymbolFlag::Summary => {
            rslt = diesel::update(symbols.find(sid))
                .set((summary.eq(value), m_time.eq(now)))
                .get_result::<Symbol>(conn);
        }
        SymbolFlag::All => {  // todo: need to test this
            rslt = diesel::update(symbols.find(sid))
                .set((overview.eq(value), intraday.eq(value), summary.eq(value), m_time.eq(now)))
                .get_result::<Symbol>(conn);
        }
    }

    match rslt {
        Ok(_) => return Ok(()),
        Err(e) => {
            println!("Cannot update ");
            return Err(e.into());
        }
    };
}