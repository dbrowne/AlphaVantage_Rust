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


use std::collections::{HashMap};
use std::error::Error;
use diesel::PgConnection;
use crate::alpha_lib::alpha_io_funcs::{get_api_key, get_news_root};
use crate::alpha_lib::news_type::{RawFeed, NewsRoot};
use crate::dbfunctions::topic_refs::{get_topics, insert_topic};
use crate::create_url;
use crate::dbfunctions::author::{get_authors, insert_author};
use crate::schema::feeds::sid;
use crate::schema::tickersentiments::ticker;


#[derive(Debug, Default)]
struct Params {
    topics: HashMap<String, i32>,
    authors: HashMap<String, i32>,
}

pub fn load_news(conn: &mut PgConnection, s_id: & i64, tkr: &String) -> Result<(), Box<dyn Error>> {
    let api_key = get_api_key()?;


    let url = create_url!(FuncType:NewsQuery,tkr,api_key);
    let root = get_news_root(&url)?;
    process_news(conn, s_id, tkr,root)?;
    Ok(())
}

fn process_news(conn: &mut PgConnection,s_id:&i64, tkr:&String, root: NewsRoot) -> Result<(), Box<dyn Error>> {
    let item_count = root.items.parse::<i32>()?;
    let sentiment_def = root.sentiment_score_definition;
    let relevance_def = root.relevance_score_definition;


    process_feed(conn, s_id, tkr, root.feed)?;
    Ok(())
}

fn process_feed(conn: &mut PgConnection, s_id:&i64, tkr:&String, feed: Vec<RawFeed>) -> Result<(), Box<dyn Error>> {
    let mut params = Params::default();

    let topics = get_topics(conn)?;
    let authors = get_authors(conn)?;


    for tp in topics {
        params.topics.insert(tp.name, tp.id);
    }

    for auth in authors {
        params.authors.insert(auth.author_name, auth.id);
    }


    for article in feed {
        process_article(conn, &s_id, &tkr, article, &mut params)?;
    }

    Ok(())
}


fn process_article(conn: &mut PgConnection, s_id:&i64, tkr:&String, article: RawFeed, params: &mut Params) -> Result<(), Box<dyn Error>> {
    for auth in article.authors {
        if params.authors.contains_key(&auth) {
            continue;
        }
        if auth.eq("") {
            continue;
        }
        println!("Inserting new author {}", auth);
        let auth = insert_author(conn, auth)?;
        params.authors.insert(auth.author_name, auth.id);
    }


    for topic in article.topics {
        if params.topics.contains_key(&topic.topic) {
            continue;
        }
        println!("Inserting new topic {}", topic.topic);
        let tp = insert_topic(conn, topic.topic)?;
        params.topics.insert(tp.name, tp.id);
    };

    println!("{:?} {:?}", article.title, article.url);


    Ok(())
}
