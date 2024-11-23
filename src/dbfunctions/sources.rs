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

use crate::{
  db_models::{NewSource, Source},
  dbfunctions::common::*,
  schema::sources::dsl::sources,
};

#[derive(Error, Debug)]
pub enum Error {
  #[error(transparent)]
  DB(#[from] diesel::result::Error),
  #[error("Unexpected error: {0}")]
  UnEx(String),
}
pub fn get_sources(conn: &mut PgConnection) -> Result<Vec<Source>, Error> {
  sources.load::<Source>(conn).map_err(Error::from)
}

pub fn get_source_by_name(conn: &mut PgConnection, auth_name: String) -> Result<Source, Error> {
  use crate::schema::sources::dsl::source_name;

  sources
    .filter(source_name.eq(auth_name))
    .first::<Source>(conn)
    .map_err(Error::from)
}

pub fn get_source_by_id(conn: &mut PgConnection, src_id: i32) -> Result<Source, Error> {
  use crate::schema::sources::dsl::id;

  sources
    .filter(id.eq(src_id))
    .first::<Source>(conn)
    .map_err(Error::from)
}

pub fn insert_source(
  conn: &mut PgConnection,
  src_name: String,
  domain_name: String,
) -> Result<Source, Error> {
  let nsrc = NewSource {
    source_name: &src_name,
    domain: &domain_name,
  };
  diesel::insert_into(sources)
    .values(&nsrc)
    .get_result(conn)
    .map_err(Error::from)
}
