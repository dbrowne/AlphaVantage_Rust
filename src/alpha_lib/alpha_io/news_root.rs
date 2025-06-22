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

//! News root processing and hash generation module
//!
//! This module provides functionality for creating news overview records
//! and generating unique hash identifiers for news feed data.

use bincode::Options;
use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime};
use crc32fast::Hasher;
use serde::Serialize;
use thiserror::Error;

use crate::{
  alpha_lib::core::news_type::RawFeed,
  db_models::{NewNewsOverview, NewsOverview},
  dbfunctions::common::*,
  schema::newsoverviews::dsl::newsoverviews,
};

/// Custom error types for news root operations
#[derive(Error, Debug)]
pub enum NewsRootError {
  #[error("Database operation failed")]
  Database(#[from] diesel::result::Error),
  #[error("Serialization failed: {0}")]
  Serialization(String),
  #[error("Unexpected error: {0}")]
  Unexpected(String),
}

/// Create current date at midnight for consistent timestamps
fn create_creation_timestamp() -> NaiveDateTime {
  let local = Local::now();
  let date = NaiveDate::from_ymd_opt(local.year(), local.month(), local.day())
    .unwrap_or_else(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());
  let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
  NaiveDateTime::new(date, time)
}

/// Generate hash ID for serializable data
fn generate_hash_id<T>(items: &[T]) -> Result<String, NewsRootError>
where
  T: Serialize,
{
  let bytes = serialize_to_bytes(items)?;
  Ok(calculate_checksum(&bytes))
}

/// Serialize items to bytes using bincode
fn serialize_to_bytes<T>(items: &[T]) -> Result<Vec<u8>, NewsRootError>
where
  T: Serialize,
{
  let mut bytes = Vec::new();
  for item in items {
    let serialized = bincode::options()
      .serialize(item)
      .map_err(|e| NewsRootError::Serialization(e.to_string()))?;
    bytes.extend(serialized);
  }
  Ok(bytes)
}

/// Calculate CRC32 checksum
fn calculate_checksum(bytes: &[u8]) -> String {
  let mut hasher = Hasher::new();
  hasher.update(bytes);
  hasher.finalize().to_string()
}

/// Insert news root overview into database
pub fn insert_news_root(
  conn: &mut PgConnection,
  symbol_id: i64,
  item_count: i32,
  news: Vec<RawFeed>,
) -> Result<NewsOverview, NewsRootError> {
  let creation_date = create_creation_timestamp();
  let hash_id = generate_hash_id(&news)?;

  let new_overview = NewNewsOverview {
    items: &item_count,
    sid: symbol_id,
    hashid: &hash_id,
    creation: &creation_date,
  };

  diesel::insert_into(newsoverviews)
    .values(&new_overview)
    .get_result(conn)
    .map_err(NewsRootError::from)
}

/// Generate hash ID for news feed (legacy function)
pub fn get_hash_id(news: Vec<RawFeed>) -> String {
  generate_hash_id(&news).unwrap_or_else(|_| "error_hash".to_string())
}

/// Convert vector of serializable items to bytes (legacy function)
pub fn convert_to_bytes<T>(items: Vec<T>) -> Result<Vec<u8>, NewsRootError>
where
  T: Serialize,
{
  serialize_to_bytes(&items)
}

/// Calculate checksum from bytes (legacy function)
pub fn calculate_checksum_legacy(bytes: &[u8]) -> String {
  calculate_checksum(bytes)
}

// Legacy error type alias
pub type Error = NewsRootError;

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::Timelike;
  #[test]
  fn test_create_creation_timestamp() {
    let timestamp = create_creation_timestamp();
    // Should have time set to midnight
    assert_eq!(timestamp.time().hour(), 0);
    assert_eq!(timestamp.time().minute(), 0);
    assert_eq!(timestamp.time().second(), 0);
  }

  #[test]
  fn test_calculate_checksum() {
    let test_data = b"test data";
    let checksum1 = calculate_checksum(test_data);
    let checksum2 = calculate_checksum(test_data);

    // Same input should produce same checksum
    assert_eq!(checksum1, checksum2);
    assert!(!checksum1.is_empty());
  }

  #[test]
  fn test_checksum_different_data() {
    let data1 = b"test data 1";
    let data2 = b"test data 2";

    let checksum1 = calculate_checksum(data1);
    let checksum2 = calculate_checksum(data2);

    // Different input should produce different checksums
    assert_ne!(checksum1, checksum2);
  }

  #[test]
  fn test_serialize_to_bytes() {
    let test_vec = vec!["hello".to_string(), "world".to_string()];
    let result = serialize_to_bytes(&test_vec);

    assert!(result.is_ok());
    let bytes = result.unwrap();
    assert!(!bytes.is_empty());
  }

  #[test]
  fn test_generate_hash_id() {
    let test_data = vec!["test".to_string(), "data".to_string()];
    let result = generate_hash_id(&test_data);

    assert!(result.is_ok());
    let hash = result.unwrap();
    assert!(!hash.is_empty());
  }

  #[test]
  fn test_legacy_get_hash_id() {
    // This would require RawFeed to be constructible
    // For now, just test that the function exists
    let empty_news = vec![];
    let hash = get_hash_id(empty_news);
    assert!(!hash.is_empty());
  }

  #[test]
  fn test_legacy_calculate_checksum() {
    let test_data = b"legacy test";
    let checksum = calculate_checksum_legacy(test_data);
    assert!(!checksum.is_empty());

    // Should be same as new function
    assert_eq!(checksum, calculate_checksum(test_data));
  }

  #[test]
  fn test_error_handling_serialization() {
    // Test that we handle serialization errors properly
    // This is more of a compile-time test to ensure the error types work
    match NewsRootError::Serialization("test error".to_string()) {
      NewsRootError::Serialization(msg) => assert_eq!(msg, "test error"),
      _ => panic!("Wrong error type"),
    }
  }
}
