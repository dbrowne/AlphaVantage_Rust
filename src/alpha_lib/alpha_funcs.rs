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

use crate::alpha_lib::alpha_data_types::TopType;

/// Normalizes region names to their respective short forms.
///
/// This function takes a string slice (`&str`) that represents a region name and
/// returns a `String` that contains a shortened form of the region name.
///
/// The function recognizes the following region names and their respective short forms:
/// - "United States" -> "USA"
/// - "United Kingdom" -> "UK"
/// - "Frankfurt" -> "Frank"
/// - "Toronto Venture" -> "TOR"
/// - "India/Bombay" -> "Bomb"
/// - "Brazil/Sao Paolo" -> "SaoP"
///
/// If a region name is not recognized, the function simply returns the original name.
///
/// # Arguments
///
/// * `reg` - A string slice that holds the name of the region to be normalized.
///
/// # Returns
///
/// * A `String` containing the shortened form of the region name. If the region name is not
///   recognized, the function returns the original name.
///
/// # Examples
///
/// ```ignore
/// let region = "United States";
/// let short_region = normalize_alpha_region(region);
/// assert_eq!(short_region, "USA");
/// ```
pub fn normalize_alpha_region(reg: &str) -> String {
  match reg {
    "United States" => "USA",
    "United Kingdom" => "UK",
    "Frankfurt" => "Frank",
    "Toronto Venture" => "TOR",
    "India/Bombay" => "Bomb",
    "Brazil/Sao Paolo" => "SaoP",
    _ => reg,
  }
  .to_string()
}

pub fn top_constants(act: &TopType) -> String {
  match act {
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
  fn test_normalize_alpha_region() {
    let region = "United States";
    let short_region = normalize_alpha_region(region);
    assert_eq!(short_region, "USA");
  }

  #[test]
  fn test_normalize_alpha_region_2() {
    let region = "Whatever";
    let short_region = normalize_alpha_region(region);
    assert_eq!(short_region, "Whatever");
  }
}
