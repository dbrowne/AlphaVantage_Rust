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

pub fn get_proc_id(conn: &mut PgConnection, proc_name: &str) -> Result<i32, diesel::result::Error> {
  use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

  use crate::schema::proctypes;

  let result = proctypes::table
    .filter(proctypes::name.eq(proc_name))
    .select(proctypes::id)
    .first::<i32>(conn);
  result
}

pub fn get_proc_name(conn: &mut PgConnection, p_id: i32) -> Result<String, diesel::result::Error> {
  use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

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
  use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

  use crate::{db_models::NewProcType, schema::proctypes};

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
  use chrono::{DateTime, Local};
  use diesel::RunQueryDsl;

  use crate::{db_models::NewProcState, schema::procstates};
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
  use chrono::{Local, NaiveDateTime};
  use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

  use crate::schema::procstates::{
    dsl::{end_state, end_time, spid},
    table as procstates,
  };

  let localt: NaiveDateTime = Local::now().naive_local();
  diesel::update(procstates.filter(spid.eq(pid)))
    .set((end_time.eq(localt), end_state.eq(&e_state)))
    .execute(conn)
}
