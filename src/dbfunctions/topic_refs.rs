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
  db_models::{NewTopicRef, TopicRef},
  dbfunctions::common::*,
  schema::topicrefs::dsl::topicrefs,
};

#[derive(Error, Debug)]
pub enum Error {
  #[error(transparent)]
  DB(#[from] diesel::result::Error),
  #[error("Unexpected error: {0}")]
  UnEx(String),
}
pub fn get_topics(conn: &mut PgConnection) -> Result<Vec<TopicRef>, Error> {
  topicrefs.load::<TopicRef>(conn).map_err(Error::from)
}

pub fn get_topic_by_id(conn: &mut PgConnection, topic_id: i32) -> Result<TopicRef, Error> {
  use crate::schema::topicrefs::dsl::id;

  topicrefs
    .filter(id.eq(topic_id))
    .first::<TopicRef>(conn)
    .map_err(Error::from)
}

pub fn get_id_topic_by_name(
  conn: &mut PgConnection,
  topic_name: String,
) -> Result<TopicRef, Error> {
  use crate::schema::topicrefs::dsl::name;

  topicrefs
    .filter(name.eq(topic_name))
    .first::<TopicRef>(conn)
    .map_err(Error::from)
}

pub fn insert_topic(conn: &mut PgConnection, topic_name: String) -> Result<TopicRef, Error> {
  let new_topic = NewTopicRef { name: &topic_name };
  diesel::insert_into(topicrefs)
    .values(&new_topic)
    .get_result::<TopicRef>(conn)
    .map_err(Error::from)
}
