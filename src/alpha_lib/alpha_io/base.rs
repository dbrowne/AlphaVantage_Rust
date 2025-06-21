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

//! Alpha Vantage API integration and data processing module
//!
//! This module provides functionality for fetching, processing, and storing
//! financial data from the Alpha Vantage API.

use std::{collections::HashMap, env::VarError, thread, time::Duration};

use chrono::{Local, NaiveDate, NaiveDateTime};
use diesel::PgConnection;
use serde_json::Value;
use thiserror::Error;

use crate::{
  alpha_lib::core::{
    alpha_data_types::{
      AlphaSymbol, Convert, FullOverview, RawDailyPrice, RawIntraDayPrice, Root, TopType,
    },
    alpha_funcs::{normalize_alpha_region, top_constants},
    news_type::NewsRoot,
  },
  create_url,
  db_models::IntraDayPrice,
  dbfunctions::{
    base::establish_connection_or_exit,
    overview::create_overview,
    price::{create_intra_day, get_intr_day_max_date, get_summary_max_date, insert_open_close},
    sid::{get_next_sid, get_sid},
    symbols::{create_symbol, get_symbols_and_sids_for},
    tops::insert_top_stat,
  },
  security_types::SecurityType,
};

/// Custom error types for the base module
#[derive(Error, Debug)]
pub enum BaseError {
  #[error("Failed to retrieve API key")]
  ApiKey(#[from] VarError),
  #[error("Database connection error")]
  DbConnection(#[from] diesel::result::Error),
  #[error("JSON serialization/deserialization error")]
  Serialization(#[from] serde_json::Error),
  #[error("HTTP request error")]
  HttpRequest(#[from] reqwest::Error),
  #[error("CSV parsing error")]
  CsvParsing(#[from] csv::Error),
  #[error("Date parsing error")]
  DateParsing(#[from] chrono::ParseError),
  #[error("Database function error")]
  DatabaseFunction(#[from] crate::dbfunctions::common::Error),
  #[error("Alpha data types error")]
  AlphaDataTypes(#[from] crate::alpha_lib::core::alpha_data_types::Error),
  #[error("Missing header '{0}' in API response")]
  MissingHeader(String),
  #[error("Error count exceeded: {0}")]
  ErrorCountExceeded(usize),
  #[error("Maximum limit exceeded: {0}")]
  MaximumExceeded(String),
  #[error("Unexpected error: {0}")]
  Unexpected(String),
}

/// Configuration constants
pub mod config {
  use chrono::Duration;

  pub const SYMBOL_FIELD: &str = "symbol";
  pub const MAX_ERRORS: i32 = 50;
  pub const API_RATE_LIMIT_MS: i64 = 350;
  pub const OVERVIEW_SYMBOL_FIELD: &str = "Symbol";
  pub const INTRADAY_HEADER: &str = "timestamp,open,high,low,close,volume";
  pub const DAILY_METADATA_HEADER: &str = "Meta Data";
  pub const DAILY_TIMESERIES_HEADER: &str = "Time Series (Daily)";

  /// Minimum time between API requests (75 requests per minute)
  pub fn min_request_interval() -> Duration {
    Duration::milliseconds(API_RATE_LIMIT_MS)
  }
}

/// API client for Alpha Vantage
pub struct AlphaVantageClient {
  api_key: String,
  client: reqwest::blocking::Client,
}

impl AlphaVantageClient {
  /// Create a new AlphaVantage client
  pub fn new() -> Result<Self, BaseError> {
    Ok(Self {
      api_key: get_api_key()?,
      client: reqwest::blocking::Client::new(),
    })
  }

  /// Get data from API endpoint
  fn get_text(&self, url: &str) -> Result<String, BaseError> {
    let response = self.client.get(url).send()?;
    Ok(response.text()?)
  }

  /// Get JSON data from API endpoint
  fn get_json<T>(&self, url: &str) -> Result<T, BaseError>
  where
    T: for<'de> serde::Deserialize<'de>,
  {
    let response = self.client.get(url).send()?;
    Ok(response.json::<T>()?)
  }

  /// Get the API key
  pub fn api_key(&self) -> &str {
    &self.api_key
  }
}

/// Symbol processing functionality
pub mod symbol_processing {
  use super::*;
  use crate::security_types::SecurityType;

  /// Configuration for symbol processing
  #[derive(Debug, Clone)]
  pub struct ProcessingConfig {
    pub load_missed: bool,
    pub region_filter: Option<String>,
    pub max_errors: i32,
  }

  impl Default for ProcessingConfig {
    fn default() -> Self {
      Self {
        load_missed: false,
        region_filter: Some("USA".to_string()),
        max_errors: config::MAX_ERRORS,
      }
    }
  }

  /// Symbol processor for handling batch symbol operations
  pub struct SymbolProcessor {
    client: AlphaVantageClient,
    config: ProcessingConfig,
    type_counts: HashMap<SecurityType, i32>,
    symbol_cache: HashMap<String, i32>,
    error_count: i32,
  }

  impl SymbolProcessor {
    pub fn new(config: ProcessingConfig) -> Result<Self, BaseError> {
      let mut processor = Self {
        client: AlphaVantageClient::new()?,
        config,
        type_counts: HashMap::new(),
        symbol_cache: HashMap::new(),
        error_count: 0,
      };

      if processor.config.load_missed {
        processor.load_existing_symbols()?;
      }

      Ok(processor)
    }

    fn load_existing_symbols(&mut self) -> Result<(), BaseError> {
      let conn = &mut establish_connection_or_exit();
      let symbols = get_symbols_and_sids_for(conn, "USA".to_string(), "Equity".to_string())?;

      for (symbol, _) in symbols {
        self.symbol_cache.insert(symbol, 1);
      }

      Ok(())
    }

    /// Process a batch of symbols
    pub fn process_symbols(&mut self, symbol_batches: Vec<Vec<String>>) -> Result<(), BaseError> {
      let conn = &mut establish_connection_or_exit();
      let min_interval = config::min_request_interval();

      for batch in symbol_batches {
        for symbol in batch {
          if self.error_count > self.config.max_errors {
            return Err(BaseError::ErrorCountExceeded(self.error_count as usize));
          }

          let start_time = Local::now();

          match self.process_single_symbol(conn, &symbol) {
            Ok(_) => {}
            Err(e) => {
              self.error_count += 1;
              eprintln!("Error processing symbol {}: {}", symbol, e);

              if self.error_count > self.config.max_errors {
                return Err(BaseError::ErrorCountExceeded(self.error_count as usize));
              }
              continue;
            }
          }

          // Rate limiting
          let elapsed = Local::now() - start_time;
          if elapsed < min_interval {
            thread::sleep(Duration::from_secs(1));
            println!("Stats: {}, {:?}", Local::now(), self.type_counts);
          }
        }
      }

      Ok(())
    }

    fn process_single_symbol(
      &mut self,
      conn: &mut PgConnection,
      symbol: &str,
    ) -> Result<(), BaseError> {
      let url = create_url!(FuncType::SymSearch, symbol, self.client.api_key());
      let text = self.client.get_text(&url)?;

      if !text.contains(config::SYMBOL_FIELD) {
        return Ok(()); // Skip if no symbol data
      }

      let mut reader = csv::Reader::from_reader(text.as_bytes());

      for result in reader.deserialize() {
        let mut record: AlphaSymbol = result?;

        // Skip duplicates
        if self.symbol_cache.insert(record.symbol.clone(), 1).is_some() {
          continue;
        }

        // Classify security type
        let (sec_type, sec_type_string) =
          SecurityType::classify_detailed(&record.s_type, &record.name);

        record.s_type = sec_type_string.clone();
        record.region = normalize_alpha_region(&record.region);

        // Apply region filter
        if self.config.load_missed && record.region != "USA" {
          continue;
        }

        // Update type counts
        self.update_type_count(sec_type);

        // Generate security ID
        let s_id = if self.config.load_missed {
          get_next_sid(conn, sec_type_string)?
        } else {
          SecurityType::encode(sec_type, *self.type_counts.get(&sec_type).unwrap() as u32)
        };

        // Create symbol record
        create_symbol(conn, s_id, record)?;
      }

      Ok(())
    }

    fn update_type_count(&mut self, sec_type: SecurityType) {
      *self.type_counts.entry(sec_type).or_insert(0) += 1;
    }

    /// Get current type statistics
    pub fn type_statistics(&self) -> &HashMap<SecurityType, i32> {
      &self.type_counts
    }
  }
}

/// Data fetching and processing functionality
pub mod data_processing {
  use super::*;

  /// Overview data processor
  pub struct OverviewProcessor {
    client: AlphaVantageClient,
  }

  impl OverviewProcessor {
    pub fn new() -> Result<Self, BaseError> {
      Ok(Self {
        client: AlphaVantageClient::new()?,
      })
    }

    /// Fetch and process overview data for a symbol
    pub fn process_overview(
      &self,
      connection: &mut PgConnection,
      s_id: i64,
      symbol: String,
    ) -> Result<(), BaseError> {
      let url = create_url!(FuncType::Overview, symbol, self.client.api_key());
      let text = self.client.get_text(&url)?;

      if !text.contains(config::OVERVIEW_SYMBOL_FIELD) {
        println!("Missing overview for symbol {}: {}", symbol, text);
        thread::sleep(Duration::from_secs(1));
        return Ok(());
      }

      let json: Value = serde_json::from_str(&text)?;

      if let Some(overview) = FullOverview::new(s_id, json) {
        println!("Overview: {:?}", overview);
        create_overview(connection, overview)?;
      } else {
        return Err(BaseError::Unexpected(format!(
          "Failed to create overview for symbol {}",
          symbol
        )));
      }

      Ok(())
    }
  }

  /// Intraday price processor
  pub struct IntradayProcessor {
    client: AlphaVantageClient,
  }

  impl IntradayProcessor {
    pub fn new() -> Result<Self, BaseError> {
      Ok(Self {
        client: AlphaVantageClient::new()?,
      })
    }

    /// Load intraday price data
    pub fn load_intraday(
      &self,
      conn: &mut PgConnection,
      symbol: &str,
      s_id: i64,
      sec_type: SecurityType,
    ) -> Result<(), BaseError> {
      let url = match sec_type {
        SecurityType::Crypto => {
          create_url!(FuncType::CryptoIntraDay, symbol, self.client.api_key())
        }
        SecurityType::Equity => create_url!(FuncType::TsIntra, symbol, self.client.api_key()),
        _ => {
          return Err(BaseError::Unexpected(
            "Unsupported security type for intraday data".to_string(),
          ))
        }
      };

      let text = self.client.get_text(&url)?;

      if !text.contains(config::INTRADAY_HEADER) {
        return Ok(()); // Alpha Vantage returns error for missing price data
      }

      let ticks = self.parse_intraday_csv(&text)?;
      self.persist_ticks(conn, s_id, symbol, ticks)?;

      Ok(())
    }

    fn parse_intraday_csv(&self, text: &str) -> Result<Vec<RawIntraDayPrice>, BaseError> {
      let mut reader = csv::Reader::from_reader(text.as_bytes());
      reader
        .deserialize()
        .collect::<Result<Vec<RawIntraDayPrice>, _>>()
        .map_err(BaseError::from)
    }

    fn persist_ticks(
      &self,
      connection: &mut PgConnection,
      s_id: i64,
      symbol: &str,
      ticks: Vec<RawIntraDayPrice>,
    ) -> Result<(), BaseError> {
      let last_date = get_intr_day_max_date(connection, s_id)?;
      let mut processed = 0;
      let mut skipped = 0;

      for tick in ticks {
        let parsed_time = NaiveDateTime::parse_from_str(&tick.timestamp, "%Y-%m-%d %H:%M:%S")?;

        let intraday_price = IntraDayPrice {
          eventid: 0,
          tstamp: parsed_time,
          sid: s_id,
          symbol: symbol.to_string(),
          open: tick.open,
          high: tick.high,
          low: tick.low,
          close: tick.close,
          volume: tick.volume,
        };

        if intraday_price.tstamp > last_date {
          create_intra_day(connection, intraday_price)?;
          processed += 1;
        } else {
          skipped += 1;
        }
      }

      println!("Processed: {}, Skipped: {}", processed, skipped);
      Ok(())
    }
  }

  /// Daily summary processor
  pub struct SummaryProcessor {
    client: AlphaVantageClient,
  }

  impl SummaryProcessor {
    pub fn new() -> Result<Self, BaseError> {
      Ok(Self {
        client: AlphaVantageClient::new()?,
      })
    }

    /// Load daily summary data
    pub fn load_summary(
      &self,
      conn: &mut PgConnection,
      symbol: &str,
      s_id: i64,
    ) -> Result<(), BaseError> {
      let url = create_url!(FuncType::TsDaily, symbol, self.client.api_key());
      let text = self.client.get_text(&url)?;

      if !text.contains(config::DAILY_METADATA_HEADER) {
        return Ok(()); // Alpha Vantage returns error for missing price data
      }

      let daily_prices = self.parse_daily_prices(&text, symbol)?;
      let last_date = get_summary_max_date(conn, s_id)?;

      for price in daily_prices {
        if price.date > last_date {
          insert_open_close(conn, &symbol.to_string(), s_id, price)?;
        }
      }

      Ok(())
    }

    fn parse_daily_prices(
      &self,
      input: &str,
      symbol: &str,
    ) -> Result<Vec<RawDailyPrice>, BaseError> {
      let json_data: Value = serde_json::from_str(input)?;

      let price_data = json_data[config::DAILY_TIMESERIES_HEADER]
        .as_object()
        .ok_or_else(|| BaseError::MissingHeader(config::DAILY_TIMESERIES_HEADER.to_string()))?;

      let mut daily_prices = Vec::new();

      for (date_str, data) in price_data {
        if let Some(price) = self.create_daily_price(date_str, data, symbol) {
          daily_prices.push(price);
        }
      }

      Ok(daily_prices)
    }

    fn create_daily_price(
      &self,
      date_str: &str,
      data: &Value,
      symbol: &str,
    ) -> Option<RawDailyPrice> {
      let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()?;
      let open = data["1. open"].as_str()?.parse().ok()?;
      let high = data["2. high"].as_str()?.parse().ok()?;
      let low = data["3. low"].as_str()?.parse().ok()?;
      let close = data["4. close"].as_str()?.parse().ok()?;
      let volume = data["5. volume"].as_str()?.parse().ok()?;

      Some(RawDailyPrice {
        date,
        symbol: symbol.to_string(),
        open,
        high,
        low,
        close,
        volume,
      })
    }
  }

  /// Top performers processor
  pub struct TopProcessor {
    client: AlphaVantageClient,
  }

  impl TopProcessor {
    pub fn new() -> Result<Self, BaseError> {
      Ok(Self {
        client: AlphaVantageClient::new()?,
      })
    }

    /// Load top performers data
    pub fn load_tops(&self, conn: &mut PgConnection) -> Result<(), BaseError> {
      let url = create_url!(FuncType::TopQuery, " ", self.client.api_key());
      let root: Root = self.client.get_json(&url)?;
      let last_update = parse_timestamp(&root.last_updated)?;

      self.process_top_data(conn, &root.top_gainers, TopType::TopGainer, last_update)?;
      self.process_top_data(conn, &root.top_losers, TopType::TopLoser, last_update)?;
      self.process_top_data(
        conn,
        &root.most_actively_traded,
        TopType::TopActive,
        last_update,
      )?;

      Ok(())
    }

    fn process_top_data(
      &self,
      conn: &mut PgConnection,
      data: &[impl Convert],
      top_type: TopType,
      last_update: NaiveDateTime,
    ) -> Result<(), BaseError> {
      for item in data {
        let top_stat = item.make_top_stat()?;

        let s_id = match get_sid(conn, top_stat.ticker.clone()) {
          Ok(value) => value,
          Err(_) => continue, // Skip if symbol not found
        };

        let type_constant = top_constants(&top_type);
        insert_top_stat(conn, s_id, top_stat.clone(), &type_constant, last_update)?;
      }

      Ok(())
    }
  }
}

/// Cryptocurrency symbol processing
pub mod crypto_processing {
  use super::*;

  /// Default cryptocurrency configuration
  #[derive(Debug, Clone)]
  pub struct CryptoConfig {
    pub security_type: String,
    pub region: String,
    pub currency: String,
    pub timezone: String,
    pub market_open: String,
    pub market_close: String,
  }

  impl Default for CryptoConfig {
    fn default() -> Self {
      Self {
        security_type: "Crypto".to_string(),
        region: "USA".to_string(),
        currency: "USD".to_string(),
        timezone: "UTC-04".to_string(),
        market_open: "00:00".to_string(),
        market_close: "23:59".to_string(),
      }
    }
  }

  /// Process cryptocurrency symbols
  pub fn process_digital_symbols(symbol_data: Vec<String>) -> Result<(), BaseError> {
    let config = CryptoConfig::default();
    let mut symbol_map = HashMap::new();
    let conn = &mut establish_connection_or_exit();
    let mut base_sid = 1u32;

    for symbol_string in symbol_data {
      let parts: Vec<&str> = symbol_string.split(',').collect();
      if parts.len() < 2 {
        continue;
      }

      let symbol = parts[0].to_string();
      let name = parts[1].to_string();
      let s_id = SecurityType::encode(SecurityType::Crypto, base_sid);

      let record = AlphaSymbol::new(
        symbol.clone(),
        name,
        config.security_type.clone(),
        config.region.clone(),
        config.market_open.clone(),
        config.market_close.clone(),
        config.timezone.clone(),
        config.currency.clone(),
        1.0,
      );

      if symbol_map.insert(symbol.clone(), s_id).is_some() {
        eprintln!("Duplicate symbol: {}", symbol);
      } else {
        println!("Inserting symbol: {}:{}", symbol, s_id);
        create_symbol(conn, s_id, record)?;
      }

      base_sid += 1;
    }

    println!("Total symbols processed: {}", symbol_map.len());
    Ok(())
  }
}

/// News processing functionality
pub fn get_news_data(url: &str) -> Result<NewsRoot, BaseError> {
  let client = AlphaVantageClient::new()?;
  Ok(client.get_json(url)?)
}

/// Utility functions
pub mod utils {
  use super::*;

  /// Parse timestamp from Alpha Vantage format
  pub fn parse_timestamp(input: &str) -> Result<NaiveDateTime, BaseError> {
    let mut parts = input.rsplitn(2, ' ');
    let _timezone = parts.next().unwrap();
    let datetime_str = parts.next().unwrap();

    Ok(NaiveDateTime::parse_from_str(
      datetime_str,
      "%Y-%m-%d %H:%M:%S",
    )?)
  }
}

/// Get API key from environment
pub fn get_api_key() -> Result<String, VarError> {
  std::env::var("ALPHA_VANTAGE_API_KEY")
}

// Re-export commonly used items
pub use crypto_processing::*;
pub use data_processing::*;
pub use symbol_processing::*;
pub use utils::*;

// Legacy function wrappers for backward compatibility
pub fn process_symbols(sec_vec: Vec<Vec<String>>, load_missed: bool) -> Result<(), BaseError> {
  let config = ProcessingConfig {
    load_missed,
    ..Default::default()
  };

  let mut processor = SymbolProcessor::new(config)?;
  processor.process_symbols(sec_vec)
}

pub fn get_overview(
  connection: &mut PgConnection,
  s_id: i64,
  symbol: String,
) -> Result<(), BaseError> {
  let processor = OverviewProcessor::new()?;
  processor.process_overview(connection, s_id, symbol)
}

pub fn load_intraday(
  conn: &mut PgConnection,
  symbol: &String,
  s_id: i64,
  sec_type: SecurityType,
) -> Result<(), BaseError> {
  let processor = IntradayProcessor::new()?;
  processor.load_intraday(conn, symbol, s_id, sec_type)
}

pub fn load_summary(conn: &mut PgConnection, symbol: &str, s_id: i64) -> Result<(), BaseError> {
  let processor = SummaryProcessor::new()?;
  processor.load_summary(conn, symbol, s_id)
}

pub fn load_tops(conn: &mut PgConnection) -> Result<(), BaseError> {
  let processor = TopProcessor::new()?;
  processor.load_tops(conn)
}

// Legacy alias for the error type
pub type Error = BaseError;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_timestamp() {
    let input = "2023-10-03 16:15:59 US/Eastern";
    assert!(parse_timestamp(input).is_ok());
  }

  #[test]
  fn test_crypto_config_default() {
    let config = CryptoConfig::default();
    assert_eq!(config.security_type, "Crypto");
    assert_eq!(config.region, "USA");
  }

  #[test]
  fn test_processing_config_default() {
    let config = ProcessingConfig::default();
    assert!(!config.load_missed);
    assert_eq!(config.region_filter, Some("USA".to_string()));
    assert_eq!(config.max_errors, config::MAX_ERRORS);
  }

  #[test]
  fn test_alpha_vantage_client_creation() {
    // This test will fail if API key is not set, which is expected
    match AlphaVantageClient::new() {
      Ok(_) => println!("Client created successfully"),
      Err(BaseError::ApiKey(_)) => println!("API key not set - expected for testing"),
      Err(e) => panic!("Unexpected error: {}", e),
    }
  }
}
