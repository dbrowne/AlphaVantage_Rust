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


extern crate chrono;

use std::collections::{HashMap};
use std::error::Error;
use diesel::PgConnection;
use crate::alpha_lib::alpha_io_funcs::{get_api_key, get_news_root};
use crate::alpha_lib::news_type::{RawFeed, NewsRoot};
use crate::dbfunctions::topic_refs::{get_topics, insert_topic};
use crate::create_url;
use crate::dbfunctions::author::{get_author_by_id, get_authors, insert_author};
use crate::dbfunctions::sources::{get_sources, insert_source};
use chrono::prelude::*;
use crate::dbfunctions::articles::insert_article;
use crate::dbfunctions::news_root::insert_news_root;
use crate::schema::authors::author_name;


#[derive(Debug, Default)]
pub struct Params {
    topics: HashMap<String, i32>,
    authors: HashMap<String, i32>,
    sources: HashMap<String, i32>,
}

pub fn load_news(conn: &mut PgConnection, s_id: &i64, tkr: &String, params: &mut Params) -> Result<(), Box<dyn Error>> {
    let api_key = get_api_key()?;
    let url = create_url!(FuncType:NewsQuery,tkr,api_key);
    let root = get_news_root(&url)?;
    process_news(conn, s_id, tkr, root, params)?;
    Ok(())
}

fn process_news(conn: &mut PgConnection, s_id: &i64, tkr: &String, root: NewsRoot, params: &mut Params) -> Result<(), Box<dyn Error>> {
    let item_count = root.items.parse::<i32>()?;
    let sentiment_def = root.sentiment_score_definition;
    let relevance_def = root.relevance_score_definition;
    let overview = insert_news_root(conn, *s_id, item_count, sentiment_def, relevance_def)?;

    process_feed(conn, s_id, tkr, root.feed, overview.id, params)?;
    Ok(())
}

fn process_feed(conn: &mut PgConnection, s_id: &i64, tkr: &String, feed: Vec<RawFeed>, overview_id: i32, params: &mut Params) -> Result<(), Box<dyn Error>> {
    for article in feed {
        process_article(conn, &s_id, &tkr, article, params)?;
    }

    Ok(())
}


fn process_article(conn: &mut PgConnection, s_id: &i64, tkr: &String, article: RawFeed, params: &mut Params) -> Result<(), Box<dyn Error>> {
    let mut author_id: i32 = -1;
    let mut topic_id: i32 = -1;
    let mut source_id: i32 = -1;

    let sources = params.sources.clone();

    for (src,_)  in sources {
        if params.sources.contains_key(&article.source) {
            let s_id: Result<i32, Box<dyn Error>> = match params.sources.get(&src) {
                Some(&source_id) => Ok(source_id),
                None => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "No source id"))),
            };
            continue;
        } else {
            let src = insert_source(conn, article.source.clone(), article.source_domain.clone())?;
            params.sources.insert(src.source_name, src.id.clone());
            source_id = src.id;
        }
    }


    let art = insert_article(conn, source_id,
                             article.category_within_source,
                             article.title, article.url, article.summary,
                             article.banner_image,
                             author_id, article.time_published)?;

    let authors = params.authors.clone();


    for auth in &article.authors {
        if params.authors.contains_key(auth) {
            author_id = *params.authors.get(auth).unwrap_or(&-1);
        } else {
            println!("Inserting new author {}", article.authors[0].clone());
            let auth = insert_author(conn, article.authors[0].clone())?;
            params.authors.insert(auth.author_name, auth.id.clone());
            author_id = auth.id;
        }
    }


    for topic in article.topics {
        if params.topics.contains_key(&topic.topic) {
            topic_id = *params.topics.get(&topic.topic).unwrap_or(&-1);
            continue;
        } else {
            println!("Inserting new topic {}", topic.topic);
            let tp = insert_topic(conn, topic.topic)?;
            params.topics.insert(tp.name, tp.id.clone());
        }
    };


    Ok(())
}
