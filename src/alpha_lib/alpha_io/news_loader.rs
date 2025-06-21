/*
 * MIT License
 * Copyright (c) 2024. Dwight J. Browne
 * dwight[-dot-]browne[-at-]dwightjbrowne[-dot-]com
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

//! News data loading and processing module
//!
//! This module provides functionality for fetching, processing, and storing
//! news articles and related metadata from Alpha Vantage API.

use std::{collections::HashMap, fs::File, io::BufWriter};

use diesel::PgConnection;
use thiserror::Error;

use crate::{
  alpha_lib::{
    alpha_io::{
      base::{get_api_key, get_news_data},
      news_root::insert_news_root,
    },
    core::news_type::{NewsRoot, RawFeed, TickerSentiment, Topic},
    misc_functions::log_missed_symbol,
  },
  create_url,
  dbfunctions::{
    articles::insert_article, author::insert_author, author_map::insert_author_map,
    feed::insert_feed, sources::insert_source, ticker_sentiments::ins_ticker_sentiment,
    topic_maps::ins_topic_map, topic_refs::insert_topic,
  },
};

/// Custom error types for news processing
#[derive(Error, Debug)]
pub enum NewsError {
  #[error("API key retrieval failed")]
  ApiKey(#[from] std::env::VarError),
  #[error("Database operation failed")]
  Database(#[from] diesel::result::Error),
  #[error("HTTP request failed")]
  HttpRequest(#[from] reqwest::Error),
  #[error("JSON parsing failed")]
  JsonParsing(#[from] serde_json::Error),
  #[error("Number parsing failed")]
  NumberParsing(#[from] std::num::ParseIntError),
  #[error("Float parsing failed")]
  FloatParsing(#[from] std::num::ParseFloatError),
  #[error("IO operation failed")]
  Io(#[from] std::io::Error),
  #[error("Base error")]
  Base(#[from] crate::alpha_lib::alpha_io::base::BaseError),
  #[error("Source operation failed")]
  SourceOperation(#[from] crate::dbfunctions::sources::Error),
  #[error("Author operation failed")]
  AuthorOperation(#[from] crate::dbfunctions::author::Error),
  #[error("Topic operation failed")]
  TopicOperation(#[from] crate::dbfunctions::topic_refs::Error),
  #[error("News root operation failed")]
  NewsRootOperation(#[from] crate::alpha_lib::alpha_io::news_root::Error),
  #[error("Ticker sentiment operation failed")]
  TickerSentimentOperation(#[from] crate::dbfunctions::ticker_sentiments::Error),
  #[error("Topic map operation failed")]
  TopicMapOperation(#[from] crate::dbfunctions::topic_maps::Error),
  #[error("Author map operation failed")]
  AuthorMapOperation(#[from] crate::dbfunctions::author_map::Error),
  #[error("Article operation failed")]
  ArticleOperation(#[from] crate::dbfunctions::articles::Error),
  #[error("Feed operation failed")]
  FeedOperation(#[from] crate::dbfunctions::feed::Error),
  #[error("No source ID available")]
  NoSourceId,
  #[error("Article insertion failed")]
  ArticleInsertion,
  #[error("Feed insertion failed")]
  FeedInsertion,
  #[error("Invalid topic ID for topic: {0}")]
  InvalidTopicId(String),
}

/// Configuration and caching for news processing
#[derive(Debug, Default)]
pub struct NewsProcessingContext {
  pub topics: HashMap<String, i32>,
  pub authors: HashMap<String, i32>,
  pub sources: HashMap<String, i32>,
  pub symbol_to_sid: HashMap<String, i64>,
}

impl NewsProcessingContext {
  /// Create a new processing context
  pub fn new() -> Self {
    Self::default()
  }

  /// Create context with pre-populated symbol mappings
  pub fn with_symbols(symbol_mappings: HashMap<String, i64>) -> Self {
    Self {
      symbol_to_sid: symbol_mappings,
      ..Default::default()
    }
  }

  /// Get or insert a source, returning its ID
  fn get_or_insert_source(
    &mut self,
    conn: &mut PgConnection,
    source_name: String,
    source_domain: String,
  ) -> Result<i32, NewsError> {
    if let Some(&source_id) = self.sources.get(&source_name) {
      return Ok(source_id);
    }

    let source = insert_source(conn, source_name.clone(), source_domain)?;
    self.sources.insert(source_name, source.id);
    Ok(source.id)
  }

  /// Get or insert an author, returning its ID
  fn get_or_insert_author(
    &mut self,
    conn: &mut PgConnection,
    author_name: String,
  ) -> Result<i32, NewsError> {
    if let Some(&author_id) = self.authors.get(&author_name) {
      return Ok(author_id);
    }

    let author = insert_author(conn, author_name.clone())?;
    self.authors.insert(author_name, author.id);
    Ok(author.id)
  }

  /// Get or insert a topic, returning its ID
  fn get_or_insert_topic(
    &mut self,
    conn: &mut PgConnection,
    topic_name: String,
  ) -> Result<i32, NewsError> {
    if let Some(&topic_id) = self.topics.get(&topic_name) {
      return Ok(topic_id);
    }

    println!("Inserting new topic: {}", topic_name);
    let topic = insert_topic(conn, topic_name.clone())?;
    self.topics.insert(topic_name, topic.id);
    Ok(topic.id)
  }

  /// Get symbol ID from cache
  fn get_symbol_id(&self, symbol: &str) -> Option<i64> {
    self.symbol_to_sid.get(symbol).copied()
  }
}

/// News processor for handling news data operations
pub struct NewsProcessor {
  context: NewsProcessingContext,
}

impl NewsProcessor {
  /// Create a new news processor
  pub fn new() -> Self {
    Self {
      context: NewsProcessingContext::new(),
    }
  }

  /// Create processor with existing context
  pub fn with_context(context: NewsProcessingContext) -> Self {
    Self { context }
  }

  /// Load news data for a specific symbol
  pub fn load_news(
    &mut self,
    conn: &mut PgConnection,
    symbol_id: i64,
    ticker: &str,
    symbol_log: &mut BufWriter<File>,
  ) -> Result<(), NewsError> {
    let api_key = get_api_key()?;
    let url = create_url!(FuncType::NewsQuery, ticker, api_key);
    let news_root = get_news_data(&url)?;

    self.process_news_root(conn, symbol_id, ticker, news_root, symbol_log)
  }

  /// Process news root data
  fn process_news_root(
    &mut self,
    conn: &mut PgConnection,
    symbol_id: i64,
    ticker: &str,
    root: NewsRoot,
    symbol_log: &mut BufWriter<File>,
  ) -> Result<(), NewsError> {
    let item_count = root.items.parse::<i32>()?;

    if item_count < 1 {
      return Ok(()); // No news items to process
    }

    let news_overview = insert_news_root(conn, symbol_id, item_count, root.feed.clone())?;
    self.process_articles(
      conn,
      symbol_id,
      ticker,
      root.feed,
      news_overview.id,
      symbol_log,
    )?;

    Ok(())
  }

  /// Process all articles in the feed
  fn process_articles(
    &mut self,
    conn: &mut PgConnection,
    symbol_id: i64,
    ticker: &str,
    articles: Vec<RawFeed>,
    overview_id: i32,
    symbol_log: &mut BufWriter<File>,
  ) -> Result<(), NewsError> {
    for article in articles {
      if let Err(e) =
        self.process_single_article(conn, symbol_id, ticker, article, overview_id, symbol_log)
      {
        eprintln!("Error processing article for {}: {}", ticker, e);
        // Continue processing other articles even if one fails
      }
    }

    Ok(())
  }

  /// Process a single article
  fn process_single_article(
    &mut self,
    conn: &mut PgConnection,
    symbol_id: i64,
    _ticker: &str,
    article: RawFeed,
    overview_id: i32,
    symbol_log: &mut BufWriter<File>,
  ) -> Result<(), NewsError> {
    // Get or create source
    let source_id = self.context.get_or_insert_source(
      conn,
      article.source.clone(),
      article.source_domain.clone(),
    )?;

    // Get or create author (use first author or default)
    let author_name = article
      .authors
      .first()
      .cloned()
      .unwrap_or_else(|| "_None_".to_string());
    let author_id = self.context.get_or_insert_author(conn, author_name)?;

    // Process topics
    self.process_article_topics(conn, &article.topics)?;

    // Insert article
    let article_record = insert_article(
      conn,
      source_id,
      article.category_within_source,
      article.title.clone(),
      article.url,
      article.summary,
      article.banner_image,
      author_id,
      article.time_published,
    )?;

    // Insert feed
    let feed_record = insert_feed(
      conn,
      &symbol_id,
      overview_id,
      article_record.hashid,
      source_id,
      article.overall_sentiment_score,
      article.overall_sentiment_label,
    )?;

    // Process related data
    self.process_sentiments(conn, article.ticker_sentiment, feed_record.id, symbol_log)?;
    self.process_topic_mappings(conn, symbol_id, article.topics, feed_record.id)?;
    self.process_author_mapping(conn, feed_record.id, author_id)?;

    Ok(())
  }

  /// Process topics for an article
  fn process_article_topics(
    &mut self,
    conn: &mut PgConnection,
    topics: &[Topic],
  ) -> Result<(), NewsError> {
    for topic in topics {
      self
        .context
        .get_or_insert_topic(conn, topic.topic.clone())?;
    }
    Ok(())
  }

  /// Process sentiment data
  fn process_sentiments(
    &mut self,
    conn: &mut PgConnection,
    sentiments: Vec<TickerSentiment>,
    feed_id: i32,
    symbol_log: &mut BufWriter<File>,
  ) -> Result<(), NewsError> {
    for sentiment in sentiments {
      if let Err(e) = self.process_single_sentiment(conn, sentiment, feed_id, symbol_log) {
        eprintln!("Error processing sentiment: {}", e);
        // Continue with other sentiments
      }
    }
    Ok(())
  }

  /// Process a single sentiment record
  fn process_single_sentiment(
    &mut self,
    conn: &mut PgConnection,
    sentiment: TickerSentiment,
    feed_id: i32,
    symbol_log: &mut BufWriter<File>,
  ) -> Result<(), NewsError> {
    let sentiment_score = sentiment.ticker_sentiment_score.parse::<f64>()?;
    let relevance_score = sentiment.relevance_score.parse::<f64>()?;

    let symbol_id = match self.context.get_symbol_id(&sentiment.ticker) {
      Some(id) => id,
      None => {
        log_missed_symbol(symbol_log, &sentiment.ticker)?;
        return Ok(()); // Skip this sentiment
      }
    };

    ins_ticker_sentiment(
      conn,
      &symbol_id,
      feed_id,
      relevance_score,
      sentiment_score,
      sentiment.ticker_sentiment_label,
    )?;

    Ok(())
  }

  /// Process topic mappings
  fn process_topic_mappings(
    &mut self,
    conn: &mut PgConnection,
    symbol_id: i64,
    topics: Vec<Topic>,
    feed_id: i32,
  ) -> Result<(), NewsError> {
    for topic in topics {
      if let Err(e) = self.process_single_topic_mapping(conn, symbol_id, topic, feed_id) {
        eprintln!("Error processing topic mapping: {}", e);
        // Continue with other topics
      }
    }
    Ok(())
  }

  /// Process a single topic mapping
  fn process_single_topic_mapping(
    &mut self,
    conn: &mut PgConnection,
    symbol_id: i64,
    topic: Topic,
    feed_id: i32,
  ) -> Result<(), NewsError> {
    let topic_id = self
      .context
      .topics
      .get(&topic.topic)
      .copied()
      .ok_or_else(|| NewsError::InvalidTopicId(topic.topic.clone()))?;

    if topic_id < 1 {
      return Err(NewsError::InvalidTopicId(topic.topic));
    }

    let relevance_score = topic.relevance_score.parse::<f64>()?;

    ins_topic_map(conn, symbol_id, feed_id, topic_id, relevance_score)?;

    Ok(())
  }

  /// Process author mapping
  fn process_author_mapping(
    &mut self,
    conn: &mut PgConnection,
    feed_id: i32,
    author_id: i32,
  ) -> Result<(), NewsError> {
    insert_author_map(conn, feed_id, author_id)?;
    Ok(())
  }

  /// Get current processing context (for inspection/debugging)
  pub fn context(&self) -> &NewsProcessingContext {
    &self.context
  }

  /// Get mutable context (for external modifications)
  pub fn context_mut(&mut self) -> &mut NewsProcessingContext {
    &mut self.context
  }
}

/// Batch news processing utilities
pub mod batch_processing {
  use super::*;

  /// Configuration for batch processing
  #[derive(Debug, Clone)]
  pub struct BatchConfig {
    pub max_errors_per_symbol: usize,
    pub continue_on_error: bool,
    pub log_progress: bool,
  }

  impl Default for BatchConfig {
    fn default() -> Self {
      Self {
        max_errors_per_symbol: 5,
        continue_on_error: true,
        log_progress: true,
      }
    }
  }

  /// Process news for multiple symbols
  pub fn process_symbols_batch(
    symbols: Vec<(i64, String)>, // (symbol_id, ticker)
    context: NewsProcessingContext,
    config: BatchConfig,
    symbol_log: &mut BufWriter<File>,
  ) -> Result<NewsProcessingContext, NewsError> {
    let mut processor = NewsProcessor::with_context(context);
    let mut conn = crate::dbfunctions::base::establish_connection_or_exit();

    for (symbol_id, ticker) in symbols {
      if config.log_progress {
        println!("Processing news for symbol: {} (ID: {})", ticker, symbol_id);
      }

      match processor.load_news(&mut conn, symbol_id, &ticker, symbol_log) {
        Ok(_) => {
          if config.log_progress {
            println!("Successfully processed news for: {}", ticker);
          }
        }
        Err(e) => {
          eprintln!("Error processing news for {}: {}", ticker, e);
          if !config.continue_on_error {
            return Err(e);
          }
        }
      }
    }

    Ok(processor.context)
  }
}

// Legacy function wrappers for backward compatibility
pub fn load_news(
  conn: &mut PgConnection,
  s_id: &i64,
  tkr: &String,
  params: &mut Params,
  symbol_log: &mut BufWriter<File>,
) -> Result<(), Box<dyn std::error::Error>> {
  let context = NewsProcessingContext::from(params.clone());
  let mut processor = NewsProcessor::with_context(context);

  match processor.load_news(conn, *s_id, tkr, symbol_log) {
    Ok(_) => {
      // Update the legacy params structure
      *params = Params::from(processor.context);
      Ok(())
    }
    Err(e) => Err(Box::new(e)),
  }
}

pub fn process_news(
  conn: &mut PgConnection,
  s_id: &i64,
  tkr: &String,
  root: NewsRoot,
  params: &mut Params,
  symbol_log: &mut BufWriter<File>,
) -> Result<(), Box<dyn std::error::Error>> {
  let context = NewsProcessingContext::from(params.clone());
  let mut processor = NewsProcessor::with_context(context);

  match processor.process_news_root(conn, *s_id, tkr, root, symbol_log) {
    Ok(_) => {
      // Update the legacy params structure
      *params = Params::from(processor.context);
      Ok(())
    }
    Err(e) => Err(Box::new(e)),
  }
}

// Legacy type alias and compatibility layer
#[derive(Debug, Default, Clone)]
pub struct Params {
  pub topics: HashMap<String, i32>,
  pub authors: HashMap<String, i32>,
  pub sources: HashMap<String, i32>,
  pub names_to_sid: HashMap<String, i64>, // Legacy field name
}

impl From<NewsProcessingContext> for Params {
  fn from(context: NewsProcessingContext) -> Self {
    Self {
      topics: context.topics,
      authors: context.authors,
      sources: context.sources,
      names_to_sid: context.symbol_to_sid,
    }
  }
}

impl From<Params> for NewsProcessingContext {
  fn from(params: Params) -> Self {
    Self {
      topics: params.topics,
      authors: params.authors,
      sources: params.sources,
      symbol_to_sid: params.names_to_sid,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_news_processing_context_creation() {
    let context = NewsProcessingContext::new();
    assert!(context.topics.is_empty());
    assert!(context.authors.is_empty());
    assert!(context.sources.is_empty());
    assert!(context.symbol_to_sid.is_empty());
  }

  #[test]
  fn test_context_with_symbols() {
    let mut symbols = HashMap::new();
    symbols.insert("AAPL".to_string(), 123);
    symbols.insert("GOOGL".to_string(), 456);

    let context = NewsProcessingContext::with_symbols(symbols);
    assert_eq!(context.symbol_to_sid.len(), 2);
    assert_eq!(context.get_symbol_id("AAPL"), Some(123));
    assert_eq!(context.get_symbol_id("TSLA"), None);
  }

  #[test]
  fn test_batch_config_default() {
    let config = batch_processing::BatchConfig::default();
    assert_eq!(config.max_errors_per_symbol, 5);
    assert!(config.continue_on_error);
    assert!(config.log_progress);
  }

  #[test]
  fn test_news_processor_creation() {
    let processor = NewsProcessor::new();
    assert!(processor.context.topics.is_empty());
  }
}
