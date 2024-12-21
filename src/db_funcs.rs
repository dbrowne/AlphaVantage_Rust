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
  AlphaSymbol, GTopStat,
};
use crate::{
  db_models::{
    NewProcState, NewProcType
    , NewTopStat,
  },
  schema::procstates::dsl::procstates
  ,
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
pub fn parse_time(
  time_str: &str,
  error_message: &str,
  a_sym: &AlphaSymbol,
) -> Result<NaiveTime, Error> {
  NaiveTime::parse_from_str(time_str, "%H:%M").map_err(|e| {
    let msg = format!("{}: {}. Symbol: {}", error_message, e, a_sym.symbol);
    Error::TimeParse(msg)
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
