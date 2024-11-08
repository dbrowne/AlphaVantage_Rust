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
use crate::{
  db_models::{Feed, NewFeed},
  dbfunctions::common::*,
  schema::feeds::dsl::feeds,
};

pub fn ins_n_ret_feed(
  conn: &mut PgConnection,
  s_id: &i64,
  inp_newsoverviewid: i32,
  inp_articleid: String,
  inp_sourceid: i32,
  inp_osentiment: f64,
  inp_sentlabel: String,
) -> Result<Feed, Box<dyn Error>> {
  // let local :DateTime<Local>= Local::now();
  // let date= NaiveDate::from_ymd_opt(local.year(),local.month(),local.day()).
  // unwrap_or(NaiveDate::from_ymd_opt(1900,1,1).unwrap()); let tim = NaiveTime::from_hms_opt(0,0,
  // 0).unwrap(); let creattion_date= NaiveDateTime::new(date,tim);

  let rt = NewFeed {
    sid: &s_id.clone(),
    newsoverviewid: &inp_newsoverviewid,
    articleid: &inp_articleid,
    sourceid: &inp_sourceid,
    osentiment: &inp_osentiment,
    sentlabel: &inp_sentlabel,
  };

  let root = diesel::insert_into(feeds).values(&rt).get_result(conn);

  match root {
    Ok(r) => Ok(r),
    Err(e) => Err(Box::new(e)),
  }
}
