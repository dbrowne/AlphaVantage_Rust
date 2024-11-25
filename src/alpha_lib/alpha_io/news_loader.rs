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

extern crate chrono;

use std::{collections::HashMap, error::Error, fs::File, io::BufWriter};

use diesel::PgConnection;

use crate::{
  alpha_lib::{
    alpha_io_funcs::{get_api_key, get_news_root},
    misc_functions::log_missed_symbol,
    news_type::{NewsRoot, RawFeed, TickerSentiment, Topic},
  },
  create_url,
  dbfunctions::{
    articles::insert_article, author::insert_author, author_map::insert_author_map,
    feed::insert_feed, news_root::insert_news_root, sources::insert_source,
    ticker_sentiments::ins_ticker_sentiment, topic_maps::ins_topic_map, topic_refs::insert_topic,
  },
};

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
  symbol_log: &mut BufWriter<File>,
) -> Result<(), Box<dyn Error>> {
  let api_key = get_api_key()?;
  let url = create_url!(FuncType::NewsQuery, tkr, api_key);
  let root = get_news_root(&url)?;

  process_news(conn, s_id, tkr, root, params, symbol_log)
}

pub fn process_news(
  conn: &mut PgConnection,
  s_id: &i64,
  tkr: &String,
  root: NewsRoot,
  params: &mut Params,
  symbol_log: &mut BufWriter<File>,
) -> Result<(), Box<dyn Error>> {
  let item_count = root.items.parse::<i32>()?;
  if item_count < 1 {
    // todo: "Improve logging here!"
    // println!("No news items for {}", tkr);
    return Ok(());
  }

  let res = insert_news_root(conn, *s_id, item_count, root.feed.clone());
  if let Ok(overview) = res {
    process_feed(conn, s_id, tkr, root.feed, overview.id, params, symbol_log)?;
  } else if let Err(_e) = res {
    //todo: "Improve logging here!"
    // println!("Cannot insert news root for {}: err {} ", tkr, e);
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
  symbol_log: &mut BufWriter<File>,
) -> Result<(), Box<dyn Error>> {
  for article in feed {
    process_article(conn, &s_id, &tkr, article, overview_id, params, symbol_log)?;
  }

  Ok(())
}

fn process_article(
  conn: &mut PgConnection,
  s_id: &i64,
  _tkr: &String,
  article: RawFeed,
  overview_id: i32,
  params: &mut Params,
  symbol_log: &mut BufWriter<File>,
) -> Result<(), Box<dyn Error>> {
  let author_id: i32;
  // let mut topic_id: i32;
  let source_id: i32;

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

  if source_id == 0 {
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

  for topic in article.topics.clone() {
    if params.topics.contains_key(&topic.topic) {
      // topic_id = *params.topics.get(&topic.topic).unwrap_or(&-1);
      continue;
    } else {
      println!("Inserting new topic {}", topic.topic);
      let tp = insert_topic(conn, topic.topic)?;
      params.topics.insert(tp.name, tp.id.clone());
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
    if let Ok(feed) = insert_feed(
      conn,
      &s_id.clone(),
      overview_id,
      art.hashid,
      source_id,
      article.overall_sentiment_score,
      article.overall_sentiment_label,
    ) {
      if let Ok(_ts) = load_sentiments(conn, article.ticker_sentiment, params, feed.id, symbol_log)
      {
        // todo: Improve logging
        // println!("Inserted sentiments for {}", art.title);
      } else {
        // todo: Improve logging
        // println!("Cannot insert sentiments for {}", art.title);
      }
      if let Ok(_tp) = load_topic_map(conn, s_id, article.topics, feed.id, params) {
        // todo: Improve logging
        // println!("Inserted topics for {}", art.title);
      } else {
        // todo: Improve logging

        // println!("Cannot insert topics for {}", art.title);
      }
      if let Ok(_am) = insert_author_map(conn, feed.id, author_id) {
        // todo: Improve logging
        // println!("Inserted author map for {}", art.title);
      } else {
        // todo: Improve logging
        // println!("Cannot insert author map for {}", art.title);
      }
    } else {
      // todo: Improve logging
      // println!("Cannot insert feed {} for sid {}", art.title, s_id);
      return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Cannot insert article",
      )));
    }
  } else {
    // todo: Improve logging
    // println!("Cannot insert  for sid {}", s_id);
    return Err(Box::new(std::io::Error::new(
      std::io::ErrorKind::Other,
      "Cannot insert article",
    )));
  }

  Ok(())
}

fn load_sentiments(
  conn: &mut PgConnection,
  sentiments: Vec<TickerSentiment>,
  params: &mut Params,
  inp_feed_id: i32,
  symbol_log: &mut BufWriter<File>,
) -> Result<(), Box<dyn Error>> {
  for sent in sentiments {
    let sent_label = sent.ticker_sentiment_label;
    let sent_score = sent.ticker_sentiment_score.parse::<f64>()?;
    let sent_rel = sent.relevance_score.parse::<f64>()?;
    let sent_tkr = sent.ticker.clone();
    let sid = params.names_to_sid.get(&sent_tkr).unwrap_or(&-1);
    if *sid == -1 {
      //todo:: Improve logging

      // println!("Cannot find sid for {}", sent_tkr);
      log_missed_symbol(symbol_log, &sent_tkr)?;
      continue;
    }
    _ = ins_ticker_sentiment(conn, sid, inp_feed_id, sent_rel, sent_score, sent_label)
  }
  Ok(())
}

fn load_topic_map(
  conn: &mut PgConnection,
  inp_sid: &i64,
  topics: Vec<Topic>,
  inp_feed_id: i32,
  params: &mut Params,
) -> Result<(), Box<dyn Error>> {
  for topic in topics {
    let topic_id = *params.topics.get(&topic.topic).unwrap_or(&-1);
    if topic_id < 1 {
      println!("Cannot find topic id for {}", topic.topic);
      continue;
    }
    let _ = ins_topic_map(
      conn,
      *inp_sid,
      inp_feed_id,
      topic_id,
      topic.relevance_score.parse::<f64>()?,
    )?;
  }
  Ok(())
}
