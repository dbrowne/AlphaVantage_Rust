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

// from http://www.nasdaqtrader.com/trader.aspx?id=symboldirdefs

// NASDAQ Symbol Directory
/*
Symbol	The one to four or five character identifier for each NASDAQ-listed security.
Security Name	Company issuing the security.
Market Category	The category assigned to the issue by NASDAQ based on Listing Requirements. Values:
Q = NASDAQ Global Select MarketSM
G = NASDAQ Global MarketSM
S = NASDAQ Capital Market

Test Issue	Indicates whether or not the security is a test security. Values: Y = yes, it is a test issue. N = no, it is not a test issue.
Financial Status	Indicates when an issuer has failed to submit its regulatory filings on a timely basis, has failed to meet NASDAQ's continuing listing standards, and/or has filed for bankruptcy. Values include:
D = Deficient: Issuer Failed to Meet NASDAQ Continued Listing Requirements
E = Delinquent: Issuer Missed Regulatory Filing Deadline
Q = Bankrupt: Issuer Has Filed for Bankruptcy
N = Normal (Default): Issuer Is NOT Deficient, Delinquent, or Bankrupt.
G = Deficient and Bankrupt
H = Deficient and Delinquent
J = Delinquent and Bankrupt
K = Deficient, Delinquent, and Bankrupt
Round Lot	Indicates the number of shares that make up a round lot for the given security.

ACT Symbol

Field Name Change effective 2/12/2010

Identifier for each security used in ACT and CTCI connectivity protocol. Typical identifiers have 1-5 character root symbol and then 1-3 characters for suffixes. Allow up to 14 characters.

More information regarding the symbology convention can be found on this website.

Security Name	The name of the security including additional information, if applicable. Examples are security type (common stock, preferred stock, etc.) or class (class A or B, etc.). Allow up to 255 characters.
Exchange
The listing stock exchange or market of a security.

Allowed values are:
A = NYSE MKT
N = New York Stock Exchange (NYSE)
P = NYSE ARCA
Z = BATS Global Markets (BATS)
V = Investors' Exchange, LLC (IEXG)
CQS Symbol
Identifier of the security used to disseminate data via the SIAC Consolidated Quotation System (CQS) and Consolidated Tape System (CTS) data feeds. Typical identifiers have 1-5 character root symbol and then 1-3 characters for suffixes. Allow up to 14 characters.

More information regarding the symbology convention can be found on this website.

ETF	Identifies whether the security is an exchange traded fund (ETF). Possible values:
Y = Yes, security is an ETF
N = No, security is not an ETF
For new ETFs added to the file, the ETF field for the record will be updated to a value of "Y".
Round Lot Size	Indicates the number of shares that make up a round lot for the given security. Allow up to 6 digits.
Test Issue	Indicates whether the security is a test security.
Y = Yes, it is a test issue.
N = No, it is not a test issue.

*/

/*
See https://www.tickdata.com/product/nbbo/
Exchange codes
Exchange on which the trade occurred.
A AMEX (NYSE MKT)
B NASDAQ OMX BX (Boston)
C National Stock Exchange (Cincinnati)
D/1 NASD ADF (FINRA)
E Market Independent (SIP - Generated)
I ISE (International Securities Exchange)
J DirectEdge A
K DirectEdge X
L Long-Term Stock Exchange (Starting 8/1/2020)
M Chicago
N NYSE
O Instinet (Valid only during January and February 1993)
P ARCA (formerly Pacific)
S Consolidated Tape System
T/Q NASDAQ
V IEX
W CBOE (Valid through 04/30/14)
X NASDAQ OMX PSX (Philadelphia)
Y BATS Y-Exchange, Inc.
Z BATS
*/

//! Security type classification and encoding system
//!
//! This module provides functionality for classifying and encoding different types
//! of financial securities based on NASDAQ Symbol Directory specifications.

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Exchange codes for different trading venues
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Exchange {
  /// AMEX (NYSE MKT)
  Amex,
  /// NASDAQ OMX BX (Boston)
  NasdaqBx,
  /// National Stock Exchange (Cincinnati)
  Nse,
  /// NASD ADF (FINRA)
  NasdAdf,
  /// Market Independent (SIP - Generated)
  MarketIndependent,
  /// ISE (International Securities Exchange)
  Ise,
  /// DirectEdge A
  DirectEdgeA,
  /// DirectEdge X
  DirectEdgeX,
  /// Long-Term Stock Exchange
  Ltse,
  /// Chicago
  Chicago,
  /// NYSE
  Nyse,
  /// Instinet (Historical)
  Instinet,
  /// ARCA (formerly Pacific)
  Arca,
  /// Consolidated Tape System
  Cts,
  /// NASDAQ
  Nasdaq,
  /// IEX
  Iex,
  /// CBOE (Historical)
  Cboe,
  /// NASDAQ OMX PSX (Philadelphia)
  NasdaqPsx,
  /// BATS Y-Exchange
  BatsY,
  /// BATS
  Bats,
}

impl Exchange {
  /// Convert from exchange code character to Exchange enum
  pub fn from_code(code: char) -> Option<Self> {
    match code {
      'A' => Some(Self::Amex),
      'B' => Some(Self::NasdaqBx),
      'C' => Some(Self::Nse),
      'D' => Some(Self::NasdAdf),
      'E' => Some(Self::MarketIndependent),
      'I' => Some(Self::Ise),
      'J' => Some(Self::DirectEdgeA),
      'K' => Some(Self::DirectEdgeX),
      'L' => Some(Self::Ltse),
      'M' => Some(Self::Chicago),
      'N' => Some(Self::Nyse),
      'O' => Some(Self::Instinet),
      'P' => Some(Self::Arca),
      'S' => Some(Self::Cts),
      'T' | 'Q' => Some(Self::Nasdaq),
      'V' => Some(Self::Iex),
      'W' => Some(Self::Cboe),
      'X' => Some(Self::NasdaqPsx),
      'Y' => Some(Self::BatsY),
      'Z' => Some(Self::Bats),
      _ => None,
    }
  }

  /// Convert Exchange enum to exchange code character
  pub fn to_code(self) -> char {
    match self {
      Self::Amex => 'A',
      Self::NasdaqBx => 'B',
      Self::Nse => 'C',
      Self::NasdAdf => 'D',
      Self::MarketIndependent => 'E',
      Self::Ise => 'I',
      Self::DirectEdgeA => 'J',
      Self::DirectEdgeX => 'K',
      Self::Ltse => 'L',
      Self::Chicago => 'M',
      Self::Nyse => 'N',
      Self::Instinet => 'O',
      Self::Arca => 'P',
      Self::Cts => 'S',
      Self::Nasdaq => 'T',
      Self::Iex => 'V',
      Self::Cboe => 'W',
      Self::NasdaqPsx => 'X',
      Self::BatsY => 'Y',
      Self::Bats => 'Z',
    }
  }
}

/// Market categories for NASDAQ-listed securities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum MarketCategory {
  /// NASDAQ Global Select Market
  GlobalSelect,
  /// NASDAQ Global Market
  Global,
  /// NASDAQ Capital Market
  Capital,
}

impl MarketCategory {
  pub fn from_code(code: char) -> Option<Self> {
    match code {
      'Q' => Some(Self::GlobalSelect),
      'G' => Some(Self::Global),
      'S' => Some(Self::Capital),
      _ => None,
    }
  }

  pub fn to_code(self) -> char {
    match self {
      Self::GlobalSelect => 'Q',
      Self::Global => 'G',
      Self::Capital => 'S',
    }
  }
}

/// Financial status indicators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum FinancialStatus {
  /// Normal status
  Normal,
  /// Deficient: Failed to meet NASDAQ continued listing requirements
  Deficient,
  /// Delinquent: Missed regulatory filing deadline
  Delinquent,
  /// Bankrupt: Filed for bankruptcy
  Bankrupt,
  /// Deficient and Bankrupt
  DeficientBankrupt,
  /// Deficient and Delinquent
  DeficientDelinquent,
  /// Delinquent and Bankrupt
  DelinquentBankrupt,
  /// Deficient, Delinquent, and Bankrupt
  DeficientDelinquentBankrupt,
}

impl FinancialStatus {
  pub fn from_code(code: char) -> Option<Self> {
    match code {
      'N' => Some(Self::Normal),
      'D' => Some(Self::Deficient),
      'E' => Some(Self::Delinquent),
      'Q' => Some(Self::Bankrupt),
      'G' => Some(Self::DeficientBankrupt),
      'H' => Some(Self::DeficientDelinquent),
      'J' => Some(Self::DelinquentBankrupt),
      'K' => Some(Self::DeficientDelinquentBankrupt),
      _ => None,
    }
  }

  pub fn to_code(self) -> char {
    match self {
      Self::Normal => 'N',
      Self::Deficient => 'D',
      Self::Delinquent => 'E',
      Self::Bankrupt => 'Q',
      Self::DeficientBankrupt => 'G',
      Self::DeficientDelinquent => 'H',
      Self::DelinquentBankrupt => 'J',
      Self::DeficientDelinquentBankrupt => 'K',
    }
  }
}

/// Types of financial securities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum SecurityType {
  Equity,
  Bond,
  Option,
  Future,
  Etf,
  MutualFund,
  Crypto,
  Fx,
  Swap,
  Warrant,
  Adr,
  Preferred,
  Other,
}

impl fmt::Display for SecurityType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let name = match self {
      Self::Equity => "Equity",
      Self::Bond => "Bond",
      Self::Option => "Option",
      Self::Future => "Future",
      Self::Etf => "ETF",
      Self::MutualFund => "Mutual Fund",
      Self::Crypto => "Crypto",
      Self::Fx => "FX",
      Self::Swap => "Swap",
      Self::Warrant => "Warrant",
      Self::Adr => "ADR",
      Self::Preferred => "Preferred",
      Self::Other => "Other",
    };
    write!(f, "{}", name)
  }
}

/// Symbol processing flags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum SymbolFlag {
  Overview,
  Intraday,
  Summary,
  All,
}

/// Security identifier with embedded type information
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SecurityIdentifier {
  pub security_type: SecurityType,
  pub raw_id: u32,
}

/// Type alias for counting securities by type
pub type SecurityTypeCounts = HashMap<SecurityType, u32>;

/// Security type classification and encoding functionality
pub mod classification {
  use super::*;

  // Security type string mappings
  const SECURITY_TYPE_STRINGS: &[(&str, SecurityType)] = &[
    ("equity", SecurityType::Equity),
    ("common stock", SecurityType::Equity),
    ("ordinary shares", SecurityType::Equity),
    ("common shares", SecurityType::Equity),
    ("option", SecurityType::Option),
    ("future", SecurityType::Future),
    ("warrant", SecurityType::Warrant),
    ("wt", SecurityType::Warrant),
    ("wrnt", SecurityType::Warrant),
    ("mutual fund", SecurityType::MutualFund),
    ("american depositary shares", SecurityType::Adr),
    ("adr", SecurityType::Adr),
    ("depositary sh", SecurityType::Adr),
    ("dep shs", SecurityType::Adr),
    ("bond", SecurityType::Bond),
    ("subordinated debentures", SecurityType::Bond),
    ("senior notes", SecurityType::Bond),
    ("floating rate", SecurityType::Bond),
    ("notes", SecurityType::Bond),
    ("preferred", SecurityType::Preferred),
    ("pfd", SecurityType::Preferred),
    ("etf", SecurityType::Etf),
    ("etn", SecurityType::Etf),
    ("exchange traded note", SecurityType::Etf),
    (
      "lp common units representing limited partner interests",
      SecurityType::MutualFund,
    ),
    (
      "common units representing limited partner interests",
      SecurityType::MutualFund,
    ),
    ("trust", SecurityType::MutualFund),
    ("fund", SecurityType::MutualFund),
    ("crypto", SecurityType::Crypto),
    ("forei", SecurityType::Fx),
    ("fx", SecurityType::Fx),
    ("swap", SecurityType::Swap),
  ];

  lazy_static! {
    static ref SECURITY_TYPE_MAP: HashMap<&'static str, SecurityType> =
      SECURITY_TYPE_STRINGS.iter().cloned().collect();
  }

  impl SecurityType {
    /// Classify security type from string representation
    pub fn from_string(s: &str) -> Self {
      let lower = s.to_lowercase();

      // Check exact matches first
      if let Some(&sec_type) = SECURITY_TYPE_MAP.get(lower.as_str()) {
        return sec_type;
      }

      // Check for substring matches
      for (pattern, sec_type) in SECURITY_TYPE_STRINGS {
        if lower.contains(pattern) {
          return *sec_type;
        }
      }

      Self::Other
    }

    /// Get detailed security type classification from type and name
    pub fn classify_detailed(type_str: &str, name: &str) -> (Self, String) {
      let name_lower = name.to_lowercase();
      let type_lower = type_str.to_lowercase();

      // Start with type classification
      let mut security_type = match type_lower.as_str() {
        "equity" => Self::Equity,
        "etf" => Self::Etf,
        "mutual fund" => Self::MutualFund,
        _ => Self::Other,
      };

      let mut type_string = security_type.to_string();

      // Override based on name patterns (these take precedence)
      if name_lower.contains("adr") {
        security_type = Self::Adr;
        type_string = "ADR".to_string();
      } else if name_lower.contains("warrant") || name_lower.contains("wrnt") {
        security_type = Self::Warrant;
        type_string = "Warrant".to_string();
      } else if name_lower.contains("pfd") || name_lower.contains("preferred") {
        security_type = Self::Preferred;
        type_string = "Preferred".to_string();
      }

      (security_type, type_string)
    }
  }
}

/// Security identifier encoding and decoding functionality
pub mod encoding {
  use super::*;

  // Bit manipulation constants
  const MASK_32: i64 = 0x7FFF_FFFF;
  const SHIFT: u8 = 48;

  // Type masks for encoding
  const TYPE_MASKS: &[(SecurityType, i64)] = &[
    (SecurityType::Equity, 0b0000_0000),
    (SecurityType::Preferred, 0b0000_0010),
    (SecurityType::Adr, 0b0000_0100),
    (SecurityType::Warrant, 0b0000_0110),
    (SecurityType::Bond, 0b0001_0000),
    (SecurityType::Option, 0b0010_0000),
    (SecurityType::Future, 0b0011_0000),
    (SecurityType::Etf, 0b0100_0000),
    (SecurityType::MutualFund, 0b0101_0000),
    (SecurityType::Crypto, 0b0110_0000),
    (SecurityType::Fx, 0b0111_0000),
    (SecurityType::Swap, 0b1000_0000),
    (SecurityType::Other, 0b1111_0000),
  ];

  lazy_static! {
    static ref ENCODE_MAP: HashMap<SecurityType, i64> = TYPE_MASKS.iter().cloned().collect();
    static ref DECODE_MAP: HashMap<i64, SecurityType> =
      TYPE_MASKS.iter().map(|(t, m)| (*m, *t)).collect();
  }

  impl SecurityType {
    /// Encode security type and ID into a single i64
    pub fn encode(security_type: SecurityType, id: u32) -> i64 {
      let mask = ENCODE_MAP
        .get(&security_type)
        .copied()
        .unwrap_or(0b1111_0000);
      (mask << SHIFT) | (id as i64)
    }

    /// Decode security type from encoded i64
    pub fn decode_type(encoded: i64) -> Self {
      let type_bits = encoded >> SHIFT;
      DECODE_MAP.get(&type_bits).copied().unwrap_or(Self::Other)
    }
  }

  impl SecurityIdentifier {
    /// Create a new SecurityIdentifier
    pub fn new(security_type: SecurityType, raw_id: u32) -> Self {
      Self {
        security_type,
        raw_id,
      }
    }

    /// Encode SecurityIdentifier into i64
    pub fn encode(&self) -> i64 {
      SecurityType::encode(self.security_type, self.raw_id)
    }

    /// Decode SecurityIdentifier from i64
    pub fn decode(encoded: i64) -> Option<Self> {
      let security_type = SecurityType::decode_type(encoded);
      let raw_id = (encoded & MASK_32) as u32;

      // Validate that the decoded type is not Other due to invalid encoding
      if matches!(security_type, SecurityType::Other) {
        let type_bits = encoded >> SHIFT;
        if !DECODE_MAP.contains_key(&type_bits) {
          return None;
        }
      }

      Some(Self::new(security_type, raw_id))
    }
  }
}

// Re-export commonly used items

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_exchange_codes() {
    assert_eq!(Exchange::from_code('A'), Some(Exchange::Amex));
    assert_eq!(Exchange::Nyse.to_code(), 'N');
    assert_eq!(Exchange::from_code('Z'), Some(Exchange::Bats));
  }

  #[test]
  fn test_security_type_classification() {
    assert_eq!(SecurityType::from_string("equity"), SecurityType::Equity);
    assert_eq!(SecurityType::from_string("ETF"), SecurityType::Etf);
    assert_eq!(SecurityType::from_string("unknown"), SecurityType::Other);
  }

  #[test]
  fn test_detailed_classification() {
    let (sec_type, type_str) = SecurityType::classify_detailed("Equity", "Apple Inc");
    assert_eq!(sec_type, SecurityType::Equity);
    assert_eq!(type_str, "Equity");

    let (sec_type, type_str) = SecurityType::classify_detailed("Equity", "Tesla ADR");
    assert_eq!(sec_type, SecurityType::Adr);
    assert_eq!(type_str, "ADR");
  }

  #[test]
  fn test_encoding_decoding() {
    let identifier = SecurityIdentifier::new(SecurityType::Equity, 12345);
    let encoded = identifier.encode();
    let decoded = SecurityIdentifier::decode(encoded).unwrap();

    assert_eq!(decoded, identifier);
  }

  #[test]
  fn test_all_security_types_encode_decode() {
    let test_cases = [
      SecurityType::Equity,
      SecurityType::Bond,
      SecurityType::Option,
      SecurityType::Future,
      SecurityType::Etf,
      SecurityType::MutualFund,
      SecurityType::Crypto,
      SecurityType::Fx,
      SecurityType::Swap,
      SecurityType::Warrant,
      SecurityType::Adr,
      SecurityType::Preferred,
      SecurityType::Other,
    ];

    for &sec_type in &test_cases {
      let encoded = SecurityType::encode(sec_type, 12345);
      let decoded_type = SecurityType::decode_type(encoded);
      assert_eq!(decoded_type, sec_type);

      let identifier = SecurityIdentifier::new(sec_type, 12345);
      let decoded_identifier = SecurityIdentifier::decode(identifier.encode()).unwrap();
      assert_eq!(decoded_identifier, identifier);
    }
  }

  #[test]
  fn test_max_id_values() {
    let max_id = u32::MAX >> 1; // Use max value that fits in MASK_32
    let identifier = SecurityIdentifier::new(SecurityType::Equity, max_id);
    let decoded = SecurityIdentifier::decode(identifier.encode()).unwrap();
    assert_eq!(decoded.raw_id, max_id);
  }

  #[test]
  fn test_invalid_decode() {
    // Test with invalid type bits (use u64 literal and cast to i64)
    assert_eq!(
      SecurityIdentifier::decode(0x8100_0000_0000_00FFu64 as i64),
      None
    );

    // Test with another invalid pattern
    assert_eq!(
      SecurityIdentifier::decode(0x9000_0000_0000_0001u64 as i64),
      None
    );

    // Test with valid but unrecognized pattern
    assert_eq!(
      SecurityIdentifier::decode(0x1100_0000_0000_0001u64 as i64),
      None
    );
  }
}
