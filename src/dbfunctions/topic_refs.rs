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

use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::{error::Error, process};
use crate::db_models::{NewTopicRef, TopicRef};
use crate::schema::topicrefs::dsl::topicrefs;


pub  fn get_topics(conn: &mut PgConnection) ->Result<Vec<TopicRef>, Box<dyn Error>> {

    let topics = topicrefs.load::<TopicRef>(conn);
    match topics {
        Ok(topics) => Ok(topics),
        Err(err) => {
            eprintln!("Error loading topics {}", err);
            process::exit(1);
        }
    }
}

pub fn get_topic_by_id(conn: &mut PgConnection, topic_id: i32) -> Result<TopicRef, Box<dyn Error>> {
    use crate::schema::topicrefs::dsl::{id };

    let topic = topicrefs
        .filter(id.eq(topic_id))
        .first::<TopicRef>(conn);
    match topic {
        Ok(topic) => Ok(topic),
        Err(err) => {
            eprintln!("Error loading topic {}", err);
            process::exit(1);
        }
    }
}

pub  fn get_id_topic_by_name(conn: &mut PgConnection, topic_name: String) ->Result<TopicRef, Box<dyn Error>> {
    use crate::schema::topicrefs::dsl::{name };

    let topic = topicrefs
        .filter(name.eq(topic_name))
        .first::<TopicRef>(conn);
    match topic {
        Ok(topic) => Ok(topic),
        Err(err) => {
            eprintln!("Error loading topic {}", err);
            process::exit(1);
        }
    }
}

pub fn insert_topic(conn: &mut PgConnection, topicName:String) ->Result<(), Box<dyn Error>>{

    let new_topic = NewTopicRef{
        name: &topicName,
    };
    let row_cnt = diesel::insert_into(topicrefs)
        .values(&new_topic)
        .execute(conn)?;

    if row_cnt == 1 {
        return Ok(());
    }

    eprintln!("Error row_count should be 1 have {}", row_cnt);

    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "db insert problem")))
}



