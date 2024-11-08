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
  db_models::{NewSource, Source},
  dbfunctions::common::*,
  schema::sources::dsl::sources,
};

pub fn get_sources(conn: &mut PgConnection) -> Result<Vec<Source>, Box<dyn Error>> {
  let srcs = sources.load::<Source>(conn);
  match srcs {
    Ok(srcs) => Ok(srcs),
    Err(err) => {
      eprintln!("Error loading sources {}", err);
      Err(Box::new(err))
    }
  }
}

pub fn get_source_by_name(
  conn: &mut PgConnection,
  auth_name: String,
) -> Result<Source, Box<dyn Error>> {
  use crate::schema::sources::dsl::source_name;

  let source = sources
    .filter(source_name.eq(auth_name))
    .first::<Source>(conn);
  match source {
    Ok(source) => Ok(source),
    Err(err) => {
      eprintln!("Error loading source {}", err);
      Err(Box::new(err))
    }
  }
}

pub fn get_source_by_id(conn: &mut PgConnection, src_id: i32) -> Result<Source, Box<dyn Error>> {
  use crate::schema::sources::dsl::id;

  let source = sources.filter(id.eq(src_id)).first::<Source>(conn);
  match source {
    Ok(source) => Ok(source),
    Err(err) => {
      eprintln!("Error loading source {}", err);
      Err(Box::new(err))
    }
  }
}

pub fn insert_source(
  conn: &mut PgConnection,
  src_name: String,
  domain_name: String,
) -> Result<Source, Box<dyn Error>> {
  let nsrc = NewSource {
    source_name: &src_name,
    domain: &domain_name,
  };
  let src = diesel::insert_into(sources).values(&nsrc).get_result(conn);
  match src {
    Ok(src) => Ok(src),
    Err(err) => {
      eprintln!("Error inserting source {}", err);
      Err(Box::new(err))
    }
  }
}
