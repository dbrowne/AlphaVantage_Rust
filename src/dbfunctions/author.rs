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
  db_models::{Author, NewAuthor},
  dbfunctions::common::*,
  schema::authors::dsl::authors,
};

pub fn get_authors(conn: &mut PgConnection) -> Result<Vec<Author>, Box<dyn Error>> {
  let auths = authors.load::<Author>(conn);
  match auths {
    Ok(auths) => Ok(auths),
    Err(err) => {
      eprintln!("Error loading authors {}", err);
      Err(Box::new(err))
    }
  }
}

pub fn get_author_by_name(
  conn: &mut PgConnection,
  auth_name: String,
) -> Result<Author, Box<dyn Error>> {
  use crate::schema::authors::dsl::author_name;

  let author = authors
    .filter(author_name.eq(auth_name))
    .first::<Author>(conn);
  match author {
    Ok(author) => Ok(author),
    Err(err) => {
      eprintln!("Error loading author {}", err);
      Err(Box::new(err))
    }
  }
}

pub fn get_author_by_id(conn: &mut PgConnection, author_id: i32) -> Result<Author, Box<dyn Error>> {
  use crate::schema::authors::dsl::id;

  let author = authors.filter(id.eq(author_id)).first::<Author>(conn);
  match author {
    Ok(author) => Ok(author),
    Err(err) => {
      eprintln!("Error loading author {}", err);
      Err(Box::new(err))
    }
  }
}

pub fn insert_author(conn: &mut PgConnection, author: String) -> Result<Author, Box<dyn Error>> {
  let auth = NewAuthor {
    author_name: &author,
  };
  let author = diesel::insert_into(authors).values(&auth).get_result(conn);
  match author {
    Ok(author) => Ok(author),
    Err(err) => {
      eprintln!("Error inserting author {}", err);
      Err(Box::new(err))
    }
  }
}
