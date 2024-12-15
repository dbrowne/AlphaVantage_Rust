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

use bincode::Options;
use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime};
use crc32fast::Hasher;
use serde::Serialize;

use crate::{
  alpha_lib::core::news_type::RawFeed,
  db_models::{NewNewsOverview, NewsOverview},
  dbfunctions::common::*,
  schema::newsoverviews::dsl::newsoverviews,
};
#[derive(Error, Debug)]
pub enum Error {
  #[error(transparent)]
  DB(#[from] diesel::result::Error),
  #[error("Unexpected error: {0}")]
  UnEx(String),
}
pub fn insert_news_root(
  conn: &mut PgConnection,
  s_id: i64,
  item_count: i32,
  news: Vec<RawFeed>,
) -> Result<NewsOverview, Error> {
  let local: DateTime<Local> = Local::now();
  let date = NaiveDate::from_ymd_opt(local.year(), local.month(), local.day())
    .unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());
  let tim = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
  let creattion_date = NaiveDateTime::new(date, tim);

  let rt = NewNewsOverview {
    items: &item_count,
    sid: s_id.clone(),
    hashid: &get_hash_id(news),
    creation: &creattion_date,
  };

  diesel::insert_into(newsoverviews)
    .values(&rt)
    .get_result(conn)
    .map_err(Error::from)
}

fn get_hash_id(news: Vec<RawFeed>) -> String {
  let bytes = convert_to_bytes(news);
  calculate_checksum(&bytes)
}
fn convert_to_bytes<T>(vec: Vec<T>) -> Vec<u8>
where
  T: Serialize,
{
  let mut bytes = Vec::new();
  for item in vec {
    // Correct usage with the default options
    let serialized = bincode::options().serialize(&item).unwrap();
    bytes.extend(serialized);
  }
  bytes
}

fn calculate_checksum(bytes: &[u8]) -> String {
  let mut hasher = Hasher::new();
  hasher.update(bytes);
  hasher.finalize().to_string()
}
