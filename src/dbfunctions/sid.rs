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

pub fn get_sid(conn: &mut PgConnection, ticker: String) -> Result<i64, diesel::result::Error> {
  use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

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
  use diesel::{dsl::max, ExpressionMethods, QueryDsl, RunQueryDsl};

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
