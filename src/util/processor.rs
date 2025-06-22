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

//! Exchange data file processing utilities
//!
//! This module provides functionality for processing exchange data files from NASDAQ, NYSE,
//! and digital asset sources, reading CSV files and extracting symbol information.

use std::{env, fs::File, io::BufReader};
use thiserror::Error;

use crate::datatypes::data_file_types::{DigitalAsset, NasdaqListed, NyseOtherSymbol};

/// Maximum expected number of symbols per exchange file
pub const MAX_SYMBOLS: usize = 10_000;

/// Supported exchange types for file processing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExchangeType {
  Nasdaq,
  Nyse,
  Digital,
}

impl ExchangeType {
  /// Parse exchange type from string
  pub fn from_str(s: &str) -> Result<Self, FileProcessingError> {
    match s.to_uppercase().as_str() {
      "NASDAQ" => Ok(Self::Nasdaq),
      "NYSE" => Ok(Self::Nyse),
      "DIGITAL" => Ok(Self::Digital),
      _ => Err(FileProcessingError::UnknownExchange(s.to_string())),
    }
  }

  /// Get the string representation
  pub fn as_str(&self) -> &'static str {
    match self {
      Self::Nasdaq => "NASDAQ",
      Self::Nyse => "NYSE",
      Self::Digital => "DIGITAL",
    }
  }
}

/// Errors that can occur during file processing
#[derive(Error, Debug)]
pub enum FileProcessingError {
  #[error("Environment variable '{0}' not found")]
  EnvironmentVariable(String),
  #[error("Failed to open or read file: {0}")]
  FileAccess(#[from] std::io::Error),
  #[error("CSV parsing error: {0}")]
  CsvParsing(#[from] csv::Error),
  #[error("Unknown exchange type: {0}")]
  UnknownExchange(String),
}

/// Configuration for a file to be processed
#[derive(Debug, Clone)]
pub struct FileConfig {
  pub exchange_type: ExchangeType,
  pub env_var_name: String,
}

impl FileConfig {
  /// Create a new file configuration
  pub fn new(exchange_type: ExchangeType, env_var_name: impl Into<String>) -> Self {
    Self {
      exchange_type,
      env_var_name: env_var_name.into(),
    }
  }
}

/// Process multiple exchange data files and extract symbols
///
/// Reads CSV files specified by environment variables and extracts symbol information
/// based on the exchange type. Each file is processed according to its exchange format.
///
/// # Arguments
/// * `file_configs` - Vector of file configurations specifying exchange type and env var
///
/// # Returns
/// * `Ok(Vec<Vec<String>>)` - Nested vector where each inner vector contains symbols from one file
/// * `Err(FileProcessingError)` - Error if any file processing fails
///
/// # Examples
/// ```ignore
/// let configs = vec![
///     FileConfig::new(ExchangeType::Nasdaq, "NASDAQ_FILE"),
///     FileConfig::new(ExchangeType::Nyse, "NYSE_FILE"),
/// ];
/// let results = process_exchange_files(configs)?;
/// ```
pub fn process_exchange_files(
  file_configs: Vec<FileConfig>,
) -> Result<Vec<Vec<String>>, FileProcessingError> {
  let mut results = Vec::with_capacity(file_configs.len());

  for config in file_configs {
    let file_path = env::var(&config.env_var_name)
      .map_err(|_| FileProcessingError::EnvironmentVariable(config.env_var_name.clone()))?;

    let symbols = process_csv_file(&file_path, config.exchange_type)?;
    results.push(symbols);
  }

  Ok(results)
}

/// Process a single CSV file based on exchange type
///
/// Opens and reads a CSV file, deserializing records according to the specified exchange type
/// and extracting symbol information.
///
/// # Arguments
/// * `file_path` - Path to the CSV file
/// * `exchange_type` - Type of exchange data format to expect
///
/// # Returns
/// * `Ok(Vec<String>)` - Vector of extracted symbols
/// * `Err(FileProcessingError)` - Error if file processing fails
pub fn process_csv_file(
  file_path: &str,
  exchange_type: ExchangeType,
) -> Result<Vec<String>, FileProcessingError> {
  let file = File::open(file_path)?;
  let mut reader = csv::Reader::from_reader(BufReader::new(file));
  let mut symbols = Vec::with_capacity(MAX_SYMBOLS);

  match exchange_type {
    ExchangeType::Nasdaq => {
      for result in reader.deserialize() {
        let record: NasdaqListed = result?;
        symbols.push(record.symbol);
      }
    }
    ExchangeType::Nyse => {
      for result in reader.deserialize() {
        let record: NyseOtherSymbol = result?;
        symbols.push(record.actsymbol);
      }
    }
    ExchangeType::Digital => {
      for result in reader.deserialize() {
        let record: DigitalAsset = result?;
        symbols.push(format!("{},{}", record.symbol, record.name));
      }
    }
  }

  Ok(symbols)
}

// Legacy compatibility functions
pub const NASDAQ: &str = "NASDAQ";
pub const NYSE: &str = "NYSE";
pub const DIGITAL: &str = "DIGITAL";

/// Legacy function for backward compatibility
pub fn file_proc(
  file_arr: Vec<(&str, &str)>,
) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
  let mut file_configs = Vec::with_capacity(file_arr.len());

  for (exchange_str, env_var) in file_arr {
    let exchange_type = ExchangeType::from_str(exchange_str)?;
    file_configs.push(FileConfig::new(exchange_type, env_var));
  }

  let results = process_exchange_files(file_configs)?;
  Ok(results)
}

/// Legacy function for backward compatibility  
pub fn csv_proc(
  file_name: String,
  exchange: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
  let exchange_type = ExchangeType::from_str(exchange)?;
  let symbols = process_csv_file(&file_name, exchange_type)?;
  Ok(symbols)
}

/// Legacy error type for backward compatibility
#[derive(Debug, Clone)]
pub struct UnknownExchangeError(pub String);

impl std::fmt::Display for UnknownExchangeError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Unknown exchange type: {}", self.0)
  }
}

impl std::error::Error for UnknownExchangeError {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_exchange_type_parsing() {
    assert_eq!(
      ExchangeType::from_str("NASDAQ").unwrap(),
      ExchangeType::Nasdaq
    );
    assert_eq!(
      ExchangeType::from_str("nasdaq").unwrap(),
      ExchangeType::Nasdaq
    );
    assert_eq!(ExchangeType::from_str("NYSE").unwrap(), ExchangeType::Nyse);
    assert_eq!(
      ExchangeType::from_str("DIGITAL").unwrap(),
      ExchangeType::Digital
    );

    assert!(ExchangeType::from_str("UNKNOWN").is_err());
  }

  #[test]
  fn test_exchange_type_as_str() {
    assert_eq!(ExchangeType::Nasdaq.as_str(), "NASDAQ");
    assert_eq!(ExchangeType::Nyse.as_str(), "NYSE");
    assert_eq!(ExchangeType::Digital.as_str(), "DIGITAL");
  }

  #[test]
  fn test_file_config_creation() {
    let config = FileConfig::new(ExchangeType::Nasdaq, "NASDAQ_FILE");
    assert_eq!(config.exchange_type, ExchangeType::Nasdaq);
    assert_eq!(config.env_var_name, "NASDAQ_FILE");
  }
}
