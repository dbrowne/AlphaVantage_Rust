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
  db_models::{AuthorMap, NewAuthorMap},
  dbfunctions::common::*,
  schema::authormaps::dsl::authormaps,
};

#[derive(Error, Debug)]
pub enum Error {
  #[error(transparent)]
  DB(#[from] diesel::result::Error),
  #[error("Unexpected error: {0}")]
  UnEx(String),
}

pub fn insert_author_map(
  conn: &mut PgConnection,
  feed_id: i32,
  author_id: i32,
) -> Result<AuthorMap, Error> {
  let new_author_map = NewAuthorMap {
    feedid: &feed_id,
    authorid: &author_id,
  };

  diesel::insert_into(authormaps)
    .values(&new_author_map)
    .get_result::<AuthorMap>(conn)
    .map_err(Error::from)
}
