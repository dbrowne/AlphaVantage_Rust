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

use std::error::Error;

use diesel::{PgConnection, RunQueryDsl};

use crate::{
  db_models::{NewTopicMap, TopicMap},
  schema::topicmaps::dsl::topicmaps,
};

pub fn ins_topic_map(
  conn: &mut PgConnection,
  inp_sid: i64,
  inp_feedid: i32,
  inp_topicid: i32,
  inp_relscore: f64,
) -> Result<TopicMap, Box<dyn Error>> {
  let rt = NewTopicMap {
    sid: &inp_sid,
    feedid: &inp_feedid,
    topicid: &inp_topicid,
    relscore: &inp_relscore,
  };

  let root = diesel::insert_into(topicmaps).values(&rt).get_result(conn);
  match root {
    Ok(root) => Ok(root),
    Err(err) => {
      eprintln!("Error inserting Topic Map {}", err);
      Err(Box::new(err))
    }
  }
}
