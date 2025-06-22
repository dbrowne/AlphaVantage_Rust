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

//! Alpha Vantage data types and conversion utilities
//!
//! This module provides strongly-typed representations of Alpha Vantage API responses
//! and utilities for converting between different data formats.

use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;

/// Custom error types for Alpha Vantage data operations
#[derive(Error, Debug)]
pub enum AlphaDataError {
  #[error("Float parsing failed")]
  ParseFloat(#[from] std::num::ParseFloatError),
  #[error("Integer parsing failed")]
  ParseInt(#[from] std::num::ParseIntError),
  #[error("Date parsing failed")]
  ParseDate(#[from] chrono::ParseError),
  #[error("Missing required field: {0}")]
  MissingField(String),
  #[error("Invalid field value for '{field}': {value}")]
  InvalidFieldValue { field: String, value: String },
  #[error("JSON field access error for field: {0}")]
  JsonFieldAccess(String),
}

/// Configuration constants for data parsing
pub mod config {
  use chrono::NaiveDate;

  pub const ERROR_STRING: &str = "__Error__";
  pub const DEFAULT_FLOAT: f32 = -9.99;
  pub const DEFAULT_DOUBLE: f64 = -9.99;
  pub const DEFAULT_INT: i64 = -999;

  /// Default fallback date for invalid dates
  pub fn default_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(1900, 1, 1).expect("Default date should always be valid")
  }
}

/// Symbol information from Alpha Vantage symbol search
#[derive(Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AlphaSymbol {
  pub symbol: String,
  pub name: String,
  #[serde(alias = "type")]
  pub s_type: String,
  pub region: String,
  pub marketOpen: String,
  pub marketClose: String,
  pub timezone: String,
  pub currency: String,
  pub matchScore: f32,
}

impl AlphaSymbol {
  /// Create a new AlphaSymbol instance
  pub fn new(
    symbol: String,
    name: String,
    s_type: String,
    region: String,
    market_open: String,
    market_close: String,
    timezone: String,
    currency: String,
    match_score: f32,
  ) -> Self {
    Self {
      symbol,
      name,
      s_type,
      region,
      marketOpen: market_open,
      marketClose: market_close,
      timezone,
      currency,
      matchScore: match_score,
    }
  }

  /// Check if this symbol represents a high-quality match
  pub fn is_high_quality_match(&self) -> bool {
    self.matchScore >= 0.8
  }

  /// Get formatted market hours string
  pub fn market_hours(&self) -> String {
    format!(
      "{} - {} ({})",
      self.marketOpen, self.marketClose, self.timezone
    )
  }
}

/// JSON field extraction utilities with proper error handling
pub mod json_extractors {
  use super::*;

  /// Extract string field with proper error handling
  pub fn extract_string(json: &Value, field: &str) -> Result<String, AlphaDataError> {
    json[field]
      .as_str()
      .map(|s| s.to_string())
      .ok_or_else(|| AlphaDataError::JsonFieldAccess(field.to_string()))
  }

  /// Extract string field with fallback
  pub fn extract_string_with_fallback(json: &Value, field: &str, fallback: &str) -> String {
    json[field].as_str().unwrap_or(fallback).to_string()
  }

  /// Extract f32 field with proper error handling
  pub fn extract_f32(json: &Value, field: &str) -> Result<f32, AlphaDataError> {
    let value_str = json[field]
      .as_str()
      .ok_or_else(|| AlphaDataError::JsonFieldAccess(field.to_string()))?;

    if value_str.is_empty() {
      return Ok(config::DEFAULT_FLOAT);
    }

    value_str
      .parse::<f32>()
      .map_err(|_| AlphaDataError::InvalidFieldValue {
        field: field.to_string(),
        value: value_str.to_string(),
      })
  }

  /// Extract f32 field with fallback
  pub fn extract_f32_with_fallback(json: &Value, field: &str) -> f32 {
    extract_f32(json, field).unwrap_or(config::DEFAULT_FLOAT)
  }

  /// Extract i64 field with proper error handling
  pub fn extract_i64(json: &Value, field: &str) -> Result<i64, AlphaDataError> {
    let value_str = json[field]
      .as_str()
      .ok_or_else(|| AlphaDataError::JsonFieldAccess(field.to_string()))?;

    if value_str.is_empty() {
      return Ok(config::DEFAULT_INT);
    }

    value_str
      .parse::<i64>()
      .map_err(|_| AlphaDataError::InvalidFieldValue {
        field: field.to_string(),
        value: value_str.to_string(),
      })
  }

  /// Extract i64 field with fallback
  pub fn extract_i64_with_fallback(json: &Value, field: &str) -> i64 {
    extract_i64(json, field).unwrap_or(config::DEFAULT_INT)
  }

  /// Extract f64 field with proper error handling
  pub fn extract_f64(json: &Value, field: &str) -> Result<f64, AlphaDataError> {
    let value_str = json[field]
      .as_str()
      .ok_or_else(|| AlphaDataError::JsonFieldAccess(field.to_string()))?;

    if value_str.is_empty() {
      return Ok(config::DEFAULT_DOUBLE);
    }

    value_str
      .parse::<f64>()
      .map_err(|_| AlphaDataError::InvalidFieldValue {
        field: field.to_string(),
        value: value_str.to_string(),
      })
  }

  /// Extract f64 field with fallback
  pub fn extract_f64_with_fallback(json: &Value, field: &str) -> f64 {
    extract_f64(json, field).unwrap_or(config::DEFAULT_DOUBLE)
  }

  /// Extract date field with proper error handling
  pub fn extract_date(json: &Value, field: &str) -> Result<NaiveDate, AlphaDataError> {
    let date_str = json[field]
      .as_str()
      .ok_or_else(|| AlphaDataError::JsonFieldAccess(field.to_string()))?;

    if date_str.is_empty() {
      return Ok(config::default_date());
    }

    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
      .or_else(|_| NaiveDate::parse_from_str(date_str, "%Y-%m-%d"))
      .map_err(AlphaDataError::from)
  }

  /// Extract date field with fallback
  pub fn extract_date_with_fallback(json: &Value, field: &str) -> NaiveDate {
    extract_date(json, field).unwrap_or_else(|_| config::default_date())
  }
}

/// Complete company overview data from Alpha Vantage
#[derive(Debug, Clone)]
pub struct FullOverview {
  pub sid: i64,
  pub symbol: String,
  pub name: String,
  pub description: String,
  pub cik: String,
  pub exch: String,
  pub curr: String,
  pub country: String,
  pub sector: String,
  pub industry: String,
  pub address: String,
  pub fiscalyearend: String,
  pub latestquarter: NaiveDate,
  pub marketcapitalization: i64,
  pub ebitda: i64,
  pub peratio: f32,
  pub pegratio: f32,
  pub bookvalue: f64,
  pub dividendpershare: f32,
  pub dividendyield: f32,
  pub eps: f32,
  pub revenuepersharettm: f32,
  pub profitmargin: f32,
  pub operatingmarginttm: f32,
  pub returnonassetsttm: f32,
  pub returnonequityttm: f32,
  pub revenuettm: i64,
  pub grossprofitttm: i64,
  pub dilutedepsttm: f32,
  pub quarterlyearningsgrowthyoy: f32,
  pub quarterlyrevenuegrowthyoy: f32,
  pub analysttargetprice: f32,
  pub trailingpe: f32,
  pub forwardpe: f32,
  pub pricetosalesratiottm: f32,
  pub pricetobookratio: f32,
  pub evtorevenue: f32,
  pub evtoebitda: f32,
  pub beta: f64,
  pub annweekhigh: f64,
  pub annweeklow: f64,
  pub fiftydaymovingaverage: f64,
  pub twohdaymovingaverage: f64,
  pub sharesoutstanding: f64,
  pub dividenddate: NaiveDate,
  pub exdividenddate: NaiveDate,
}

impl FullOverview {
  /// Create FullOverview from JSON data with proper error handling
  pub fn from_json(sid: i64, json: Value) -> Result<Self, AlphaDataError> {
    use json_extractors::*;

    Ok(Self {
      sid,
      symbol: extract_string_with_fallback(&json, "Symbol", config::ERROR_STRING),
      name: extract_string_with_fallback(&json, "Name", config::ERROR_STRING),
      description: extract_string_with_fallback(&json, "Description", config::ERROR_STRING),
      cik: extract_string_with_fallback(&json, "CIK", config::ERROR_STRING),
      exch: extract_string_with_fallback(&json, "Exchange", config::ERROR_STRING),
      curr: extract_string_with_fallback(&json, "Currency", config::ERROR_STRING),
      country: extract_string_with_fallback(&json, "Country", config::ERROR_STRING),
      sector: extract_string_with_fallback(&json, "Sector", config::ERROR_STRING),
      industry: extract_string_with_fallback(&json, "Industry", config::ERROR_STRING),
      address: extract_string_with_fallback(&json, "Address", config::ERROR_STRING),
      fiscalyearend: extract_string_with_fallback(&json, "FiscalYearEnd", config::ERROR_STRING),
      latestquarter: extract_date_with_fallback(&json, "LatestQuarter"),
      marketcapitalization: extract_i64_with_fallback(&json, "MarketCapitalization"),
      ebitda: extract_i64_with_fallback(&json, "EBITDA"),
      peratio: extract_f32_with_fallback(&json, "PERatio"),
      pegratio: extract_f32_with_fallback(&json, "PEGRatio"),
      bookvalue: extract_f64_with_fallback(&json, "BookValue"),
      dividendpershare: extract_f32_with_fallback(&json, "DividendPerShare"),
      dividendyield: extract_f32_with_fallback(&json, "DividendYield"),
      eps: extract_f32_with_fallback(&json, "EPS"),
      revenuepersharettm: extract_f32_with_fallback(&json, "RevenuePerShareTTM"),
      profitmargin: extract_f32_with_fallback(&json, "ProfitMargin"),
      operatingmarginttm: extract_f32_with_fallback(&json, "OperatingMarginTTM"),
      returnonassetsttm: extract_f32_with_fallback(&json, "ReturnOnAssetsTTM"),
      returnonequityttm: extract_f32_with_fallback(&json, "ReturnOnEquityTTM"),
      revenuettm: extract_i64_with_fallback(&json, "RevenueTTM"),
      grossprofitttm: extract_i64_with_fallback(&json, "GrossProfitTTM"),
      dilutedepsttm: extract_f32_with_fallback(&json, "DilutedEPSTTM"),
      quarterlyearningsgrowthyoy: extract_f32_with_fallback(&json, "QuarterlyEarningsGrowthYOY"),
      quarterlyrevenuegrowthyoy: extract_f32_with_fallback(&json, "QuarterlyRevenueGrowthYOY"),
      analysttargetprice: extract_f32_with_fallback(&json, "AnalystTargetPrice"),
      trailingpe: extract_f32_with_fallback(&json, "TrailingPE"),
      forwardpe: extract_f32_with_fallback(&json, "ForwardPE"),
      pricetosalesratiottm: extract_f32_with_fallback(&json, "PriceToSalesRatioTTM"),
      pricetobookratio: extract_f32_with_fallback(&json, "PriceToBookRatio"),
      evtorevenue: extract_f32_with_fallback(&json, "EVToRevenue"),
      evtoebitda: extract_f32_with_fallback(&json, "EVToEBITDA"),
      beta: extract_f64_with_fallback(&json, "Beta"),
      annweekhigh: extract_f64_with_fallback(&json, "52WeekHigh"),
      annweeklow: extract_f64_with_fallback(&json, "52WeekLow"),
      fiftydaymovingaverage: extract_f64_with_fallback(&json, "50DayMovingAverage"),
      twohdaymovingaverage: extract_f64_with_fallback(&json, "200DayMovingAverage"),
      sharesoutstanding: extract_f64_with_fallback(&json, "SharesOutstanding"),
      dividenddate: extract_date_with_fallback(&json, "DividendDate"),
      exdividenddate: extract_date_with_fallback(&json, "ExDividendDate"),
    })
  }

  /// Legacy constructor for backward compatibility
  pub fn new(sid: i64, json: Value) -> Option<Self> {
    Self::from_json(sid, json).ok()
  }

  /// Check if this overview contains valid data
  pub fn is_valid(&self) -> bool {
    self.symbol != config::ERROR_STRING
      && self.name != config::ERROR_STRING
      && !self.symbol.is_empty()
  }

  /// Get market cap in billions
  pub fn market_cap_billions(&self) -> f64 {
    self.marketcapitalization as f64 / 1_000_000_000.0
  }

  /// Check if company pays dividends
  pub fn pays_dividends(&self) -> bool {
    self.dividendpershare > 0.0 && self.dividendyield > 0.0
  }
}

/// Intraday price data point
#[derive(Debug, Clone, Default, Deserialize)]
pub struct RawIntraDayPrice {
  pub timestamp: String,
  pub open: f32,
  pub high: f32,
  pub low: f32,
  pub close: f32,
  pub volume: i32,
}

impl RawIntraDayPrice {
  /// Parse timestamp to datetime
  pub fn parse_timestamp(&self) -> Result<chrono::NaiveDateTime, chrono::ParseError> {
    chrono::NaiveDateTime::parse_from_str(&self.timestamp, "%Y-%m-%d %H:%M:%S")
  }

  /// Calculate price change from open to close
  pub fn price_change(&self) -> f32 {
    self.close - self.open
  }

  /// Calculate percentage change from open to close
  pub fn percentage_change(&self) -> f32 {
    if self.open == 0.0 {
      0.0
    } else {
      ((self.close - self.open) / self.open) * 100.0
    }
  }

  /// Check if this is a bullish candle (close > open)
  pub fn is_bullish(&self) -> bool {
    self.close > self.open
  }
}

/// Daily price data point
#[derive(Debug, Clone, Default)]
pub struct RawDailyPrice {
  pub date: NaiveDate,
  pub symbol: String,
  pub open: f32,
  pub high: f32,
  pub low: f32,
  pub close: f32,
  pub volume: i32,
}

impl RawDailyPrice {
  /// Create new daily price record
  pub fn new(
    date: NaiveDate,
    symbol: String,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    volume: i32,
  ) -> Self {
    Self {
      date,
      symbol,
      open,
      high,
      low,
      close,
      volume,
    }
  }

  /// Calculate daily price change
  pub fn price_change(&self) -> f32 {
    self.close - self.open
  }

  /// Calculate daily percentage change
  pub fn percentage_change(&self) -> f32 {
    if self.open == 0.0 {
      0.0
    } else {
      ((self.close - self.open) / self.open) * 100.0
    }
  }

  /// Get trading range (high - low)
  pub fn trading_range(&self) -> f32 {
    self.high - self.low
  }

  /// Check if this is a bullish day
  pub fn is_bullish(&self) -> bool {
    self.close > self.open
  }
}

/// Types of top performers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopType {
  TopGainer,
  TopLoser,
  TopActive,
}

impl TopType {
  pub fn as_str(&self) -> &'static str {
    match self {
      Self::TopGainer => "top_gainer",
      Self::TopLoser => "top_loser",
      Self::TopActive => "top_active",
    }
  }
}

/// Root response for top gainers/losers/active stocks
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
  pub metadata: String,
  #[serde(rename = "last_updated")]
  pub last_updated: String,
  #[serde(rename = "top_gainers")]
  pub top_gainers: Vec<TopGainer>,
  #[serde(rename = "top_losers")]
  pub top_losers: Vec<TopLoser>,
  #[serde(rename = "most_actively_traded")]
  pub most_actively_traded: Vec<MostActivelyTraded>,
}

/// Individual top performer structs
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopGainer {
  pub ticker: String,
  pub price: String,
  #[serde(rename = "change_amount")]
  pub change_amount: String,
  #[serde(rename = "change_percentage")]
  pub change_percentage: String,
  pub volume: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopLoser {
  pub ticker: String,
  pub price: String,
  #[serde(rename = "change_amount")]
  pub change_amount: String,
  #[serde(rename = "change_percentage")]
  pub change_percentage: String,
  pub volume: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MostActivelyTraded {
  pub ticker: String,
  pub price: String,
  #[serde(rename = "change_amount")]
  pub change_amount: String,
  #[serde(rename = "change_percentage")]
  pub change_percentage: String,
  pub volume: String,
}

/// Generic top statistic with parsed numeric values
#[derive(Debug, Clone)]
pub struct TopStatistic {
  pub ticker: String,
  pub price: f32,
  pub change_amount: f32,
  pub change_percentage: f32,
  pub volume: i32,
}

impl TopStatistic {
  /// Calculate market cap if price per share is given
  pub fn estimated_market_cap(&self, shares_outstanding: f64) -> f64 {
    self.price as f64 * shares_outstanding
  }

  /// Check if this is a significant price movement (>5%)
  pub fn is_significant_movement(&self) -> bool {
    self.change_percentage.abs() > 5.0
  }
}

/// Trait for converting top performer types to generic statistics
pub trait TopPerformerConvert {
  fn to_top_statistic(&self) -> Result<TopStatistic, AlphaDataError>;
}

/// Utility functions for parsing string values from API
mod parsing_utils {
  use super::AlphaDataError;

  /// Parse percentage string (removes % sign if present)
  pub fn parse_percentage(value: &str) -> Result<f32, AlphaDataError> {
    let cleaned = value.trim_end_matches('%');
    cleaned.parse::<f32>().map_err(AlphaDataError::from)
  }

  /// Parse numeric string with error context
  pub fn parse_f32(value: &str, field_name: &str) -> Result<f32, AlphaDataError> {
    value
      .parse::<f32>()
      .map_err(|_| AlphaDataError::InvalidFieldValue {
        field: field_name.to_string(),
        value: value.to_string(),
      })
  }

  /// Parse integer string with error context
  pub fn parse_i32(value: &str, field_name: &str) -> Result<i32, AlphaDataError> {
    value
      .parse::<i32>()
      .map_err(|_| AlphaDataError::InvalidFieldValue {
        field: field_name.to_string(),
        value: value.to_string(),
      })
  }
}

/// Macro for implementing TopPerformerConvert trait
macro_rules! impl_top_performer_convert {
  ($type:ty) => {
    impl TopPerformerConvert for $type {
      fn to_top_statistic(&self) -> Result<TopStatistic, AlphaDataError> {
        use crate::alpha_lib::core::alpha_data_types::parsing_utils::*;

        Ok(TopStatistic {
          ticker: self.ticker.clone(),
          price: parse_f32(&self.price, "price")?,
          change_amount: parse_f32(&self.change_amount, "change_amount")?,
          change_percentage: parse_percentage(&self.change_percentage)?,
          volume: parse_i32(&self.volume, "volume")?,
        })
      }
    }
  };
}

// Apply the macro to all top performer types
impl_top_performer_convert!(TopGainer);
impl_top_performer_convert!(TopLoser);
impl_top_performer_convert!(MostActivelyTraded);

// Legacy trait and type aliases for backward compatibility
pub trait Convert {
  fn make_top_stat(&self) -> Result<GTopStat, Error>;
}

pub type GTopStat = TopStatistic;
pub type Error = AlphaDataError;

/// Legacy conversion implementation
impl<T> Convert for T
where
  T: TopPerformerConvert,
{
  fn make_top_stat(&self) -> Result<GTopStat, Error> {
    self.to_top_statistic()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::Datelike;
  use serde_json::json;

  #[test]
  fn test_alpha_symbol_creation() {
    let symbol = AlphaSymbol::new(
      "AAPL".to_string(),
      "Apple Inc".to_string(),
      "Equity".to_string(),
      "USA".to_string(),
      "09:30".to_string(),
      "16:00".to_string(),
      "UTC-05".to_string(),
      "USD".to_string(),
      0.95,
    );

    assert_eq!(symbol.symbol, "AAPL");
    assert!(symbol.is_high_quality_match());
    assert!(symbol.market_hours().contains("09:30"));
  }

  #[test]
  fn test_json_extractors() {
    let json = json!({
        "test_string": "hello",
        "test_float": "123.45",
        "test_int": "42",
        "test_date": "2023-01-01",
        "empty_field": "",
        "invalid_float": "not_a_number"
    });

    // Test successful extractions
    assert_eq!(
      json_extractors::extract_string(&json, "test_string").unwrap(),
      "hello"
    );
    assert_eq!(
      json_extractors::extract_f32(&json, "test_float").unwrap(),
      123.45
    );
    assert_eq!(json_extractors::extract_i64(&json, "test_int").unwrap(), 42);

    // Test fallbacks
    assert_eq!(
      json_extractors::extract_f32_with_fallback(&json, "empty_field"),
      config::DEFAULT_FLOAT
    );
    assert_eq!(
      json_extractors::extract_f32_with_fallback(&json, "invalid_float"),
      config::DEFAULT_FLOAT
    );

    // Test error cases
    assert!(json_extractors::extract_string(&json, "nonexistent").is_err());
    assert!(json_extractors::extract_f32(&json, "invalid_float").is_err());
  }

  #[test]
  fn test_full_overview_from_json() {
    let json = json!({
        "Symbol": "AAPL",
        "Name": "Apple Inc",
        "MarketCapitalization": "2000000000000",
        "DividendPerShare": "0.88",
        "DividendYield": "0.005",
        "LatestQuarter": "2023-12-31",
        "PERatio": "25.5"
    });

    let overview = FullOverview::from_json(123, json).unwrap();
    assert_eq!(overview.symbol, "AAPL");
    assert_eq!(overview.sid, 123);
    assert!(overview.is_valid());
    assert!(overview.pays_dividends());
    assert!(overview.market_cap_billions() > 1000.0);
  }

  #[test]
  fn test_raw_intraday_price_methods() {
    let price = RawIntraDayPrice {
      timestamp: "2023-01-01 10:30:00".to_string(),
      open: 100.0,
      high: 105.0,
      low: 99.0,
      close: 103.0,
      volume: 1000,
    };

    assert_eq!(price.price_change(), 3.0);
    assert_eq!(price.percentage_change(), 3.0);
    assert!(price.is_bullish());
    assert!(price.parse_timestamp().is_ok());
  }

  #[test]
  fn test_raw_daily_price_methods() {
    let price = RawDailyPrice::new(
      chrono::NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
      "AAPL".to_string(),
      150.0,
      155.0,
      148.0,
      152.0,
      50000,
    );

    assert_eq!(price.price_change(), 2.0);
    assert!(price.percentage_change() > 1.0);
    assert_eq!(price.trading_range(), 7.0);
    assert!(price.is_bullish());
  }

  #[test]
  fn test_top_performer_conversion() {
    let gainer = TopGainer {
      ticker: "TEST".to_string(),
      price: "100.50".to_string(),
      change_amount: "5.25".to_string(),
      change_percentage: "5.5%".to_string(),
      volume: "1000000".to_string(),
    };

    let stat = gainer.to_top_statistic().unwrap();
    assert_eq!(stat.ticker, "TEST");
    assert_eq!(stat.price, 100.50);
    assert_eq!(stat.change_amount, 5.25);
    assert_eq!(stat.change_percentage, 5.5);
    assert_eq!(stat.volume, 1000000);
    assert!(stat.is_significant_movement());
  }

  #[test]
  fn test_top_performer_conversion_errors() {
    let invalid_gainer = TopGainer {
      ticker: "TEST".to_string(),
      price: "invalid_price".to_string(),
      change_amount: "5.25".to_string(),
      change_percentage: "5.5%".to_string(),
      volume: "1000000".to_string(),
    };

    assert!(invalid_gainer.to_top_statistic().is_err());
  }

  #[test]
  fn test_top_statistic_methods() {
    let stat = TopStatistic {
      ticker: "AAPL".to_string(),
      price: 150.0,
      change_amount: 8.0,
      change_percentage: 5.5,
      volume: 1000000,
    };

    assert!(stat.is_significant_movement());
    assert_eq!(
      stat.estimated_market_cap(1_000_000_000.0),
      150_000_000_000.0
    );
  }

  #[test]
  fn test_top_type_as_str() {
    assert_eq!(TopType::TopGainer.as_str(), "top_gainer");
    assert_eq!(TopType::TopLoser.as_str(), "top_loser");
    assert_eq!(TopType::TopActive.as_str(), "top_active");
  }

  #[test]
  fn test_parsing_utils() {
    assert_eq!(parsing_utils::parse_percentage("5.5%").unwrap(), 5.5);
    assert_eq!(parsing_utils::parse_percentage("5.5").unwrap(), 5.5);
    assert_eq!(parsing_utils::parse_f32("123.45", "test").unwrap(), 123.45);
    assert_eq!(parsing_utils::parse_i32("42", "test").unwrap(), 42);

    // Test error cases
    assert!(parsing_utils::parse_f32("invalid", "test").is_err());
    assert!(parsing_utils::parse_i32("invalid", "test").is_err());
  }

  #[test]
  fn test_legacy_convert_trait() {
    let gainer = TopGainer {
      ticker: "LEGACY".to_string(),
      price: "50.0".to_string(),
      change_amount: "2.5".to_string(),
      change_percentage: "5.0%".to_string(),
      volume: "500000".to_string(),
    };

    // Test legacy trait still works
    let legacy_stat = gainer.make_top_stat().unwrap();
    assert_eq!(legacy_stat.ticker, "LEGACY");
    assert_eq!(legacy_stat.price, 50.0);
  }

  #[test]
  fn test_error_types() {
    // Test that all error types can be created
    let _missing_field_err = AlphaDataError::MissingField("test".to_string());
    let _invalid_field_err = AlphaDataError::InvalidFieldValue {
      field: "test".to_string(),
      value: "value".to_string(),
    };
    let _json_access_err = AlphaDataError::JsonFieldAccess("field".to_string());

    // Test error conversion
    let parse_err: Result<f32, _> = "invalid".parse();
    if let Err(e) = parse_err {
      let _alpha_err = AlphaDataError::ParseFloat(e);
    }
  }

  #[test]
  fn test_config_constants() {
    assert_eq!(config::ERROR_STRING, "__Error__");
    assert_eq!(config::DEFAULT_FLOAT, -9.99);
    assert_eq!(config::DEFAULT_DOUBLE, -9.99);
    assert_eq!(config::DEFAULT_INT, -999);

    let default_date = config::default_date();
    assert_eq!(default_date.year(), 1900);
    assert_eq!(default_date.month(), 1);
    assert_eq!(default_date.day(), 1);
  }
}
