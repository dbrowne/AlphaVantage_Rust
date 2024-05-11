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

use crate::alpha_lib::alpha_io_funcs::{get_api_key, get_news_root};
use crate::alpha_lib::news_type::{NewsRoot, RawFeed};
use crate::create_url;
use crate::dbfunctions::articles::insert_article;
use crate::dbfunctions::author::insert_author;
use crate::dbfunctions::news_root::insert_news_root;
use crate::dbfunctions::sources::insert_source;
use crate::dbfunctions::topic_refs::insert_topic;

use crate::dbfunctions::feed::ins_n_ret_feed;
use diesel::PgConnection;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Default)]
pub struct Params {
    pub topics: HashMap<String, i32>,
    pub authors: HashMap<String, i32>,
    pub sources: HashMap<String, i32>,
    pub names_to_sid: HashMap<String, i64>,
}

pub fn load_news(
    conn: &mut PgConnection,
    s_id: &i64,
    tkr: &String,
    params: &mut Params,
) -> Result<(), Box<dyn Error>> {
    let api_key = get_api_key()?;
    let url = create_url!(FuncType:NewsQuery,tkr,api_key);
    let root = get_news_root(&url)?;
    process_news(conn, s_id, tkr, root, params)?;
    Ok(())
}

pub fn process_news(
    conn: &mut PgConnection,
    s_id: &i64,
    tkr: &String,
    root: NewsRoot,
    params: &mut Params,
) -> Result<(), Box<dyn Error>> {
    let item_count = root.items.parse::<i32>()?;
    let sentiment_def = root.sentiment_score_definition;
    let relevance_def = root.relevance_score_definition;
    if item_count < 1 {
        println!("No news items for {}", tkr);
        return Ok(());
    }

    if let Ok(overview) = insert_news_root(conn, *s_id, item_count, root.feed.clone()) {
        process_feed(conn, s_id, tkr, root.feed, overview.id, params)?;
    } else {
        println!("Cannot insert news root for {}", tkr);
    }
    Ok(())
}

fn process_feed(
    conn: &mut PgConnection,
    s_id: &i64,
    tkr: &String,
    feed: Vec<RawFeed>,
    overview_id: i32,
    params: &mut Params,
) -> Result<(), Box<dyn Error>> {
    for article in feed {
        process_article(conn, &s_id, &tkr, article, overview_id, params)?;
    }

    Ok(())
}

fn process_article(
    conn: &mut PgConnection,
    s_id: &i64,
    tkr: &String,
    article: RawFeed,
    overview_id: i32,
    params: &mut Params,
) -> Result<(), Box<dyn Error>> {
    let mut author_id: i32 = -1;
    let mut topic_id: i32 = -1;
    let mut source_id: i32 = -1;

    let sources = params.sources.clone();
    // bad logic here need to fix
    if sources.is_empty() {
        let src = insert_source(conn, article.source.clone(), article.source_domain.clone())?;
        params.sources.insert(src.source_name, src.id.clone());
        source_id = src.id;
    } else {
        if params.sources.contains_key(&article.source.to_string()) {
            source_id = params
                .sources
                .get(&article.source.to_string())
                .unwrap()
                .clone();
        } else {
            let src = insert_source(conn, article.source.clone(), article.source_domain.clone())?;
            params.sources.insert(src.source_name, src.id.clone());
            source_id = src.id;
        }
    }

    if source_id == -1 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No source id",
        )));
    }
    let mut first_author_name = "_None_".to_string();
    let auths = params.authors.clone();
    if auths.is_empty() {
        let auth = insert_author(conn, article.authors[0].clone())?;
        params
            .authors
            .insert(auth.author_name.clone(), auth.id.clone());
        author_id = auth.id;
    } else {
        if article.authors.len() > 0 {
            first_author_name = article.authors[0].to_string();
        }

        if let Some(&tauthor_id) = params.authors.get(&first_author_name) {
            author_id = tauthor_id;
        } else {
            // Author doesn't exist in the map, insert it
            let auth = insert_author(conn, first_author_name)?;
            params
                .authors
                .insert(auth.author_name.clone(), auth.id.clone());
            author_id = auth.id;
        }
    }


    if let Ok(art) = insert_article(
        conn,
        source_id,
        article.category_within_source,
        article.title,
        article.url,
        article.summary,
        article.banner_image,
        author_id,
        article.time_published,
    ) {
        if let Ok(ret) = ins_n_ret_feed(
            conn,
            &s_id.clone(),
            overview_id ,
            art.hashid,
            source_id,
            article.overall_sentiment_score,
            article.overall_sentiment_label,
        ) {
            println!("Inserted article {} for sid {}", art.title, s_id);
        } else {
            println!("Cannot insert feed {} for sid {}", art.title, s_id);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Cannot insert article",
            )));
        }
    } else {
        println!("Cannot insert  for sid {}",  s_id);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Cannot insert article",
        )));
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
    }

    Ok(())
}
