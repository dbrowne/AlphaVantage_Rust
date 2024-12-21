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
use chrono::NaiveDateTime;
use diesel::PgConnection;

use crate::{alpha_lib::core::alpha_data_types::GTopStat, dbfunctions::common::Error};

pub fn insert_top_stat(
  conn: &mut PgConnection,
  s_id: i64,
  ts: GTopStat,
  evt_type: &str,
  upd_time: NaiveDateTime,
) -> Result<(), Error> {
  use diesel::RunQueryDsl;

  use crate::{db_models::NewTopStat, schema::topstats};
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
