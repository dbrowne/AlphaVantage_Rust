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
use chrono::{NaiveDate, NaiveDateTime};
use diesel::PgConnection;

use crate::{
  alpha_lib::core::alpha_data_types::RawDailyPrice, db_models::IntraDayPrice,
  dbfunctions::common::Error,
};

pub fn create_intra_day(conn: &mut PgConnection, tick: IntraDayPrice) -> Result<(), Error> {
  use diesel::RunQueryDsl;

  use crate::{
    db_models::NewIntraDayPrice, dbfunctions::symbols, schema::intradayprices,
    security_types::SymbolFlag,
  };

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

  symbols::set_symbol_booleans(conn, new_mkt_price.sid.clone(), SymbolFlag::Intraday, true)
    .map(|_| ())
    .map_err(Error::from)
}

pub fn insert_open_close(
  conn: &mut PgConnection,
  symb: &String,
  s_id: i64,
  open_close: RawDailyPrice,
) -> Result<(), Error> {
  use diesel::RunQueryDsl;

  use crate::{
    db_models::NewSummaryPrice, dbfunctions::symbols, schema::summaryprices,
    security_types::SymbolFlag,
  };

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
  symbols::set_symbol_booleans(conn, s_id.clone(), SymbolFlag::Summary, true)
    .map(|_| ())
    .map_err(Error::from)
}

///
///
/// todo: get rid of the cut and paste
pub fn get_intr_day_max_date(conn: &mut PgConnection, s_id: i64) -> Result<NaiveDateTime, Error> {
  use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

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
  use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

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
