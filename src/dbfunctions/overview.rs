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

use crate::{alpha_lib::core::alpha_data_types::FullOverview, dbfunctions::common::Error};

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
  use chrono::{DateTime, Local};
  use diesel::RunQueryDsl;

  use crate::{
    db_models::{NewOverview, NewOverviewext},
    dbfunctions::symbols,
    schema::{overviewexts, overviews},
    security_types::SymbolFlag,
  };

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

  symbols::set_symbol_booleans(conn, full_ov.sid.clone(), SymbolFlag::Overview, true)?;

  Ok(())
}

pub fn get_full_overview(conn: &mut PgConnection, sym: &str) -> Result<FullOverview, Error> {
  use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

  use crate::{
    db_models::{Overview, Overviewext},
    schema::{overviewexts, overviews},
  };
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
