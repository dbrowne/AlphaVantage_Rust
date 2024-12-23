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

use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;

// based on https://www.alphavantage.co/query?function=SYMBOL_SEARCH&keywords=BA&apikey=demo&datatype=csv

#[derive(Error, Debug)]
pub enum Error {
  #[error(transparent)]
  ParseFloat(#[from] std::num::ParseFloatError),
  #[error(transparent)]
  ParseInt(#[from] std::num::ParseIntError),
}

#[derive(Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AlphaSymbol {
  /// Based on the Alpha Vantage csv format from the Ticker Search endpoint
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
}

/// mapping of Company Overview from Alpha Vantage
/// This is split into two structs to allow for easier use of the data and to avoid long
///compile times with the  Diesel 64-column-tables feature enabled
#[derive(Debug, Clone)]
pub struct FullOverview {
  /// based on the Alpha Vantage Company Overview endpoint https://www.alphavantage.co/query?function=OVERVIEW&symbol=IBM&apikey=demo
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

// JSON: Object {"200DayMovingAverage": String("151.55"),
// "50DayMovingAverage": String("160.75"),
// "52WeekHigh": String("175.59"),
// "52WeekLow": String("123.98"),
// "Address": String("ONE INFINITE LOOP, CUPERTINO, CA, US"),
// "AnalystTargetPrice": String("177.94"),
// "AssetType": String("Common Stock"),
// "Beta": String("1.297"),
// "BookValue": String("3.178"),
// "CIK": String("320193"),
// "Country": String("USA"),
// "Currency": String("USD"),
// "Description": String("Apple Inc. is an American multinational technology company that
// specializes in consumer electronics, computer software, and online services. Apple is the world's
// largest technology company by revenue (totalling $274.5 billion in 2020) and, since January 2021,
// the world's most valuable company. As of 2021, Apple is the world's fourth-largest PC vendor by
// unit sales, and fourth-largest smartphone manufacturer. It is one of the Big Five American
// information technology companies, along with Amazon, Google, Microsoft, and Facebook."),
// "DilutedEPSTTM": String("5.9"),
// "DividendDate": String("2023-05-18"),
// "DividendPerShare": String("0"),
// "DividendYield": String("0.0055"),
// "EBITDA": String("125287997000"),
// "EPS": String("5.9"),
// "EVToEBITDA": String("17.53"),
// "EVToRevenue": String("5.92"),
// "ExDividendDate": String("2023-05-12"),
// "Exchange": String("NASDAQ"),
// "FiscalYearEnd": String("September"),
// "ForwardPE": String("25.4"),
// "GrossProfitTTM": String("170782000000"),
// "Industry": String("ELECTRONIC COMPUTERS"),
// "LatestQuarter": String("2023-03-31"),
// "MarketCapitalization": String("2701718979000"),
// "Name": String("Apple Inc"),
// "OperatingMarginTTM": String("0"),
// "PEGRatio": String("2.75"),
// "PERatio": String("29.11"),
// "PriceToBookRatio": String("44.63"),
// "PriceToSalesRatioTTM": String("5.51"),
// "ProfitMargin": String("0"),
// "QuarterlyEarningsGrowthYOY": String("-0.105"),
// "QuarterlyRevenueGrowthYOY": String("-0.055"),
// "ReturnOnAssetsTTM": String("0.196"),
// "ReturnOnEquityTTM": String("147.94"),
// "RevenuePerShareTTM": String("24.32"),
// "RevenueTTM": String("387537011000"),
// "Sector": String("TECHNOLOGY"),
// "SharesOutstanding": String("15728700000"),
// "Symbol": String("AAPL"),
// "TrailingPE": String("29.11")}
impl FullOverview {
  /// a set of helper functions
  ///  TO BE REFACTORED!!!
  fn get_string_field(json_txt: &Value, field: &str) -> String {
    const ERROR: &str = "__Error__";
    json_txt[field].as_str().unwrap_or(ERROR).to_string()
  }

  fn get_f32_field(json_txt: &Value, field: &str) -> f32 {
    json_txt[field]
      .as_str()
      .unwrap_or("")
      .parse::<f32>()
      .unwrap_or(-9.99)
  }

  fn get_i64_field(json_txt: &Value, field: &str) -> i64 {
    json_txt[field]
      .as_str()
      .unwrap_or("")
      .parse::<i64>()
      .unwrap_or(-999)
  }

  fn get_f64_field(json_txt: &Value, field: &str) -> f64 {
    json_txt[field]
      .as_str()
      .unwrap_or("")
      .parse::<f64>()
      .unwrap_or(-9.99)
  }

  fn get_date_field(json_txt: &Value, field: &str) -> NaiveDate {
    NaiveDate::parse_from_str(json_txt[field].as_str().unwrap_or(""), "%Y-%m-%d")
      .unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap())
  }
  pub fn new(sid: i64, json_txt: Value) -> Option<Self> {
    Some(Self {
      sid,
      symbol: Self::get_string_field(&json_txt, "Symbol"),
      name: Self::get_string_field(&json_txt, "Name"),
      description: Self::get_string_field(&json_txt, "Description"),
      cik: Self::get_string_field(&json_txt, "CIK"),
      exch: Self::get_string_field(&json_txt, "Exchange"),
      curr: Self::get_string_field(&json_txt, "Currency"),
      country: Self::get_string_field(&json_txt, "Country"),
      sector: Self::get_string_field(&json_txt, "Sector"),
      industry: Self::get_string_field(&json_txt, "Industry"),
      address: Self::get_string_field(&json_txt, "Address"),
      fiscalyearend: Self::get_string_field(&json_txt, "FiscalYearEnd"),
      latestquarter: Self::get_date_field(&json_txt, "LatestQuarter"),
      marketcapitalization: Self::get_i64_field(&json_txt, "MarketCapitalization"),
      ebitda: Self::get_i64_field(&json_txt, "EBITDA"),
      peratio: Self::get_f32_field(&json_txt, "PERatio"),
      pegratio: Self::get_f32_field(&json_txt, "PEGRatio"),
      bookvalue: Self::get_f64_field(&json_txt, "BookValue"),
      dividendpershare: Self::get_f32_field(&json_txt, "DividendPerShare"),
      dividendyield: Self::get_f32_field(&json_txt, "DividendYield"),
      eps: Self::get_f32_field(&json_txt, "EPS"),
      revenuepersharettm: Self::get_f32_field(&json_txt, "RevenuePerShareTTM"),
      profitmargin: Self::get_f32_field(&json_txt, "ProfitMargin"),
      operatingmarginttm: Self::get_f32_field(&json_txt, "OperatingMarginTTM"),
      returnonassetsttm: Self::get_f32_field(&json_txt, "ReturnOnAssetsTTM"),
      returnonequityttm: Self::get_f32_field(&json_txt, "ReturnOnEquityTTM"),
      revenuettm: Self::get_i64_field(&json_txt, "RevenueTTM"),
      grossprofitttm: Self::get_i64_field(&json_txt, "GrossProfitTTM"),
      dilutedepsttm: Self::get_f32_field(&json_txt, "DilutedEPSTTM"),
      quarterlyearningsgrowthyoy: Self::get_f32_field(&json_txt, "QuarterlyEarningsGrowthYOY"),
      quarterlyrevenuegrowthyoy: Self::get_f32_field(&json_txt, "QuarterlyRevenueGrowthYOY"),
      analysttargetprice: Self::get_f32_field(&json_txt, "AnalystTargetPrice"),
      trailingpe: Self::get_f32_field(&json_txt, "TrailingPE"),
      forwardpe: Self::get_f32_field(&json_txt, "ForwardPE"),
      pricetosalesratiottm: Self::get_f32_field(&json_txt, "PriceToSalesRatioTTM"),
      pricetobookratio: Self::get_f32_field(&json_txt, "PriceToBookRatio"),
      evtorevenue: Self::get_f32_field(&json_txt, "EVToRevenue"),
      evtoebitda: Self::get_f32_field(&json_txt, "EVToEBITDA"),
      beta: Self::get_f64_field(&json_txt, "Beta"),
      annweekhigh: Self::get_f64_field(&json_txt, "52WeekHigh"),
      annweeklow: Self::get_f64_field(&json_txt, "52WeekLow"),
      fiftydaymovingaverage: Self::get_f64_field(&json_txt, "50DayMovingAverage"),
      twohdaymovingaverage: Self::get_f64_field(&json_txt, "200DayMovingAverage"),
      sharesoutstanding: Self::get_f64_field(&json_txt, "SharesOutstanding"),
      dividenddate: Self::get_date_field(&json_txt, "DividendDate"),
      exdividenddate: Self::get_date_field(&json_txt, "ExDividendDate"),
    })
  }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RawIntraDayPrice {
  ///This is for the TIME_SERIES_INTRADAY endpoint
  /// based on https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol=IBM&interval=5min&apikey=demo&datatype=csv
  pub timestamp: String,
  pub open: f32,
  pub high: f32,
  pub low: f32,
  pub close: f32,
  pub volume: i32,
}

#[derive(Debug, Clone, Default)]
pub struct RawDailyPrice {
  /// This is for the TIME_SERIES_DAILY endpoint based on
  /// https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol=IBM&apikey=demo
  pub date: NaiveDate,
  pub symbol: String,
  pub open: f32,
  pub high: f32,
  pub low: f32,
  pub close: f32,
  pub volume: i32,
}

pub enum TopType {
  TopGainer,
  TopLoser,
  TopActive,
}

/// used to parse https://www.alphavantage.co/query?function=TOP_GAINERS_LOSERS&apikey=demo
/// generated by https://transform.tools/json-to-rust-serde
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

#[derive(Debug, Clone)]
pub struct GTopStat {
  ///Generic Top Statistics had to change name to avoid confusion with dbmodeal::TopSat
  pub ticker: String,
  pub price: f32,
  pub change_amount: f32,
  pub change_percentage: f32,
  pub volume: i32,
}

pub trait Convert {
  fn make_top_stat(&self) -> Result<GTopStat, Error>;
}

macro_rules! impl_convert {
  ($type:ty) => {
    impl Convert for $type {
      fn make_top_stat(&self) -> Result<GTopStat, Error> {
        let cleaned = self.change_percentage.trim_end_matches('%');
        Ok(GTopStat {
          ticker: self.ticker.clone(),
          price: self.price.parse::<f32>()?,
          change_amount: self.change_amount.parse::<f32>()?,
          change_percentage: cleaned.parse::<f32>()?,
          volume: self.volume.parse::<i32>()?,
        })
      }
    }
  };
}

impl_convert!(TopGainer);
impl_convert!(TopLoser);
impl_convert!(MostActivelyTraded);

// fn process_top_type<T: Convert>(item: T) -> Result<GTopStat, Box<dyn Error>> {
//     item.make_top_stat()
// }

#[cfg(test)]
mod tests {
  use super::*;
  #[derive(Debug, Clone)]
  struct MockTopStat {
    pub ticker: String,
    pub price: String,
    pub change_amount: String,
    pub change_percentage: String,
    pub volume: String,
  }

  // Implement Convert for MockTopStat using the macro
  impl_convert!(MockTopStat);

  #[test]
  fn test_top_gainer_conversion() {
    let gainer = TopGainer {
      ticker: "TEST".to_string(),
      price: "10.5".to_string(),
      change_amount: "1.5".to_string(),
      change_percentage: "15.0".to_string(),
      volume: "1000".to_string(),
    };

    let stat = gainer.make_top_stat().unwrap();
    assert_eq!(stat.ticker, "TEST");
    assert_eq!(stat.price, 10.5);
    assert_eq!(stat.change_amount, 1.5);
    assert_eq!(stat.change_percentage, 15.0);
    assert_eq!(stat.volume, 1000);
  }

  #[test]
  fn test_make_top_stat_valid_data() {
    let mock = MockTopStat {
      ticker: "AAPL".to_string(),
      price: "150.75".to_string(),
      change_amount: "5.25".to_string(),
      change_percentage: "3.5%".to_string(),
      volume: "1000000".to_string(),
    };

    let result = mock.make_top_stat().unwrap();

    assert_eq!(result.ticker, "AAPL");
    assert_eq!(result.price, 150.75);
    assert_eq!(result.change_amount, 5.25);
    assert_eq!(result.change_percentage, 3.5);
    assert_eq!(result.volume, 1000000);
  }

  #[test]
  fn test_make_top_stat_invalid_price() {
    let mock = MockTopStat {
      ticker: "AAPL".to_string(),
      price: "invalid_price".to_string(),
      change_amount: "5.25".to_string(),
      change_percentage: "3.5%".to_string(),
      volume: "1000000".to_string(),
    };

    let result = mock.make_top_stat();
    assert!(
      result.is_err(),
      "Expected error for invalid price, got {:?}",
      result
    );
  }

  #[test]
  fn test_make_top_stat_invalid_change_amount() {
    let mock = MockTopStat {
      ticker: "AAPL".to_string(),
      price: "150.75".to_string(),
      change_amount: "invalid_change".to_string(),
      change_percentage: "3.5%".to_string(),
      volume: "1000000".to_string(),
    };

    let result = mock.make_top_stat();
    assert!(
      result.is_err(),
      "Expected error for invalid change amount, got {:?}",
      result
    );
  }

  #[test]
  fn test_make_top_stat_invalid_change_percentage() {
    let mock = MockTopStat {
      ticker: "AAPL".to_string(),
      price: "150.75".to_string(),
      change_amount: "5.25".to_string(),
      change_percentage: "invalid_percentage".to_string(),
      volume: "1000000".to_string(),
    };

    let result = mock.make_top_stat();
    assert!(
      result.is_err(),
      "Expected error for invalid change percentage, got {:?}",
      result
    );
  }

  #[test]
  fn test_make_top_stat_invalid_volume() {
    let mock = MockTopStat {
      ticker: "AAPL".to_string(),
      price: "150.75".to_string(),
      change_amount: "5.25".to_string(),
      change_percentage: "3.5%".to_string(),
      volume: "invalid_volume".to_string(),
    };

    let result = mock.make_top_stat();
    assert!(
      result.is_err(),
      "Expected error for invalid volume, got {:?}",
      result
    );
  }

  #[test]
  fn test_make_top_stat_no_percentage_symbol() {
    let mock = MockTopStat {
      ticker: "AAPL".to_string(),
      price: "150.75".to_string(),
      change_amount: "5.25".to_string(),
      change_percentage: "3.5".to_string(), // No '%' symbol
      volume: "1000000".to_string(),
    };

    let result = mock.make_top_stat().unwrap();

    assert_eq!(result.change_percentage, 3.5);
  }
}
