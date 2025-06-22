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

//! Alpha Vantage utility functions
//!
//! Simple utility functions for normalizing region names and getting type constants.

use crate::alpha_lib::core::alpha_data_types::TopType;

/// Normalize region names to abbreviated forms.
///
/// Converts common region names to their standard abbreviated forms used
/// throughout the application. Unknown regions are returned unchanged.
pub fn normalize_alpha_region(region: &str) -> String {
  match region {
    "United States" => "USA",
    "United Kingdom" => "UK",
    "Frankfurt" => "Frank",
    "Toronto Venture" => "TOR",
    "India/Bombay" => "Bomb",
    "Brazil/Sao Paolo" => "SaoP",
    _ => region,
  }
  .to_string()
}

/// Get database constant for TopType.
///
/// Returns the string constant used in the database to represent
/// different types of top performer lists.
pub fn top_constants(top_type: &TopType) -> String {
  match top_type {
    TopType::TopGainer => "GAIN",
    TopType::TopLoser => "LOSE",
    TopType::TopActive => "ACTV",
  }
  .to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_normalize_alpha_region_known() {
    assert_eq!(normalize_alpha_region("United States"), "USA");
    assert_eq!(normalize_alpha_region("United Kingdom"), "UK");
    assert_eq!(normalize_alpha_region("Frankfurt"), "Frank");
    assert_eq!(normalize_alpha_region("Toronto Venture"), "TOR");
    assert_eq!(normalize_alpha_region("India/Bombay"), "Bomb");
    assert_eq!(normalize_alpha_region("Brazil/Sao Paolo"), "SaoP");
  }

  #[test]
  fn test_normalize_alpha_region_unknown() {
    assert_eq!(normalize_alpha_region("Whatever"), "Whatever");
    assert_eq!(normalize_alpha_region("Mars"), "Mars");
    assert_eq!(normalize_alpha_region(""), "");
  }

  #[test]
  fn test_top_constants() {
    assert_eq!(top_constants(&TopType::TopGainer), "GAIN");
    assert_eq!(top_constants(&TopType::TopLoser), "LOSE");
    assert_eq!(top_constants(&TopType::TopActive), "ACTV");
  }
}
