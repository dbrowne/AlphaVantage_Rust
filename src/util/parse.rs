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
use chrono::naive::NaiveTime;

use crate::{alpha_lib::core::alpha_data_types::AlphaSymbol, dbfunctions::common::Error};

/// Parses a time string into a `NaiveTime` struct.
///
/// This function expects `time_str` to be in the format "HH:MM", where HH represents the hour in
/// 24-hour format and MM represents minutes. For example, "13:45" represents 1:45 PM. If the string
/// is not in this format, the function will return an error.
///
/// # Arguments
///
/// * `time_str`: A string slice that holds the time to be parsed.
/// * `error_message`: A string slice used as the error message in case the `time_str` parsing
///   fails.
/// * `a_sym`: A reference to an `AlphaSymbol` struct, which is included in the error message if
///   parsing fails.
///
/// # Returns
///
/// * `Ok(NaiveTime)`: If the `time_str` is successfully parsed into `NaiveTime`.
/// * `Error)`: If the `time_str` cannot be parsed into `NaiveTime`.
///
/// # Errors
///
/// This function will return an error if `time_str` cannot be parsed into a `NaiveTime` struct.
///
/// # Example
///
///
/// let time_str = "13:45";
/// let a_sym = AlphaSymbol::new();
/// match parse_time(time_str, "Failed to parse time string", &a_sym) {
///     Ok(time) => println!("Parsed time: {:?}", time),
///     Err(e) => println!("Error: {}", e),
/// }
pub fn parse_time(
  time_str: &str,
  error_message: &str,
  a_sym: &AlphaSymbol,
) -> Result<NaiveTime, Error> {
  NaiveTime::parse_from_str(time_str, "%H:%M").map_err(|e| {
    let msg = format!("{}: {}. Symbol: {}", error_message, e, a_sym.symbol);
    Error::TimeParse(msg)
  })
}
#[cfg(test)]
mod tests {
  use chrono::NaiveTime;

  use super::*;

  #[test]
  fn test_parse_time_valid_time() {
    // Create a valid AlphaSymbol
    let alpha_symbol = AlphaSymbol {
      symbol: "AAPL".to_string(),
      name: "Apple Inc.".to_string(),
      s_type: "Equity".to_string(),
      region: "US".to_string(),
      marketOpen: "09:30".to_string(),
      marketClose: "16:00".to_string(),
      timezone: "EST".to_string(),
      currency: "USD".to_string(),
      matchScore: 1.0,
    };

    // Test valid marketOpen time
    let result = parse_time(
      &alpha_symbol.marketOpen,
      "Failed to parse marketOpen",
      &alpha_symbol,
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), NaiveTime::from_hms_opt(9, 30, 0).unwrap());

    // Test valid marketClose time
    let result = parse_time(
      &alpha_symbol.marketClose,
      "Failed to parse marketClose",
      &alpha_symbol,
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), NaiveTime::from_hms_opt(16, 0, 0).unwrap());
  }

  #[test]
  fn test_parse_time_invalid_time() {
    let alpha_symbol = AlphaSymbol {
      symbol: "GOOG".to_string(),
      name: "Alphabet Inc.".to_string(),
      s_type: "Equity".to_string(),
      region: "US".to_string(),
      marketOpen: "9:30 AM".to_string(), // Invalid time format
      marketClose: "16:00".to_string(),
      timezone: "PST".to_string(),
      currency: "USD".to_string(),
      matchScore: 0.9,
    };

    let result = parse_time(
      &alpha_symbol.marketOpen,
      "Failed to parse marketOpen",
      &alpha_symbol,
    );
    assert!(result.is_err());

    if let Err(Error::TimeParse(msg)) = result {
      assert!(msg.contains("Failed to parse marketOpen"));
      assert!(msg.contains("GOOG")); // Symbol should be in the error message
    } else {
      panic!("Expected Error::TimeParse");
    }
  }

  #[test]
  fn test_parse_time_empty_time() {
    let alpha_symbol = AlphaSymbol {
      symbol: "TSLA".to_string(),
      name: "Tesla Inc.".to_string(),
      s_type: "Equity".to_string(),
      region: "US".to_string(),
      marketOpen: "".to_string(), // Empty string
      marketClose: "16:00".to_string(),
      timezone: "PST".to_string(),
      currency: "USD".to_string(),
      matchScore: 0.95,
    };

    let result = parse_time(
      &alpha_symbol.marketOpen,
      "Failed to parse marketOpen",
      &alpha_symbol,
    );
    assert!(result.is_err());

    if let Err(Error::TimeParse(msg)) = result {
      assert!(msg.contains("Failed to parse marketOpen"));
      assert!(msg.contains("TSLA")); // Symbol should be in the error message
    } else {
      panic!("Expected Error::TimeParse");
    }
  }
}
