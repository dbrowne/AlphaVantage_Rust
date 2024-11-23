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

extern crate ring;
use chrono::NaiveDateTime;
use ring::digest::{digest, SHA256};

use crate::{
  db_models::{Article, NewArticle},
  dbfunctions::common::*,
  schema::articles::dsl::articles,
};

#[derive(Error, Debug)]
pub enum Error {
  #[error(transparent)]
  DB(#[from] diesel::result::Error),
  #[error("Unexpected error: {0}")]
  UnEx(String),
  #[error(transparent)]
  TM(#[from] chrono::format::ParseError),
}

pub fn get_article_hashes(conn: &mut PgConnection) -> Result<Vec<String>, Error> {
  use crate::schema::articles::dsl::hashid;
  articles
    .select(hashid)
    .load::<String>(conn)
    .map_err(Error::from)
}

pub fn get_article_by_hash(conn: &mut PgConnection, hash_id: String) -> Result<Article, Error> {
  use crate::schema::articles::dsl::*;

  articles
    .filter(hashid.eq(hash_id.clone()))
    .first::<Article>(conn)
    .map_err(Error::from)
}

pub fn insert_article(
  conn: &mut PgConnection,
  s_ourceid: i32,
  c_ategory: String,
  t_itle: String,
  u_rl: String,
  s_ummary: String,
  b_anner: String,
  a_uthor: i32,
  t_published: String,
) -> Result<Article, Error> {
  let time_format = "%Y%m%dT%H%M%S";

  let parsed_date =
    NaiveDateTime::parse_from_str(&t_published, time_format).map_err(Error::from)?;
  let string_to_hash = format!("{}{}{}", t_itle.clone(), u_rl.clone(), s_ummary.clone());
  let hash = format!("{:?}", digest(&SHA256, string_to_hash.as_bytes()));

  let n_a = NewArticle {
    hashid: &hash,
    sourceid: &s_ourceid,
    category: &c_ategory,
    title: &t_itle.clone(),
    url: &u_rl,
    summary: &s_ummary,
    banner: &b_anner,
    author: &a_uthor,
    ct: &parsed_date,
  };

  diesel::insert_into(articles)
    .values(&n_a)
    .get_result(conn)
    .map_err(Error::from)
}
