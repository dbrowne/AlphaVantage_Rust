/*
 *
 *
 *
 *
 * MIT License
 * Copyright (c) 2023. Dwight J. Browne
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

use serde::Deserialize;
use chrono::{NaiveDate};
use serde_json::{Value};

// based on https://www.alphavantage.co/query?function=SYMBOL_SEARCH&keywords=BA&apikey=demo&datatype=csv
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
    pub marketcapitalization: i32,
    pub ebitda: i32,
    pub peratio: f32,
    pub pegratio: f32,
    pub bookvalue: f32,
    pub dividendpershare: f32,
    pub dividendyield: f32,
    pub eps: f32,
    pub revenuepersharettm: f32,
    pub profitmargin: f32,
    pub operatingmarginttm: f32,
    pub returnonassetsttm: f32,
    pub returnonequityttm: f32,
    pub revenuettm: i32,
    pub grossprofitttm: i32,
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
    pub beta: f32,
    pub annweekhigh: f32,
    pub annweeklow: f32,
    pub fiftydaymovingaverage: f32,
    pub twohdaymovingaverage: f32,
    pub sharesoutstanding: f32,
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
// "Description": String("Apple Inc. is an American multinational technology company that specializes in consumer electronics, computer software, and online services.
// Apple is the world's largest technology company by revenue (totalling $274.5 billion in 2020) and, since January 2021, the world's most valuable company.
// As of 2021, Apple is the world's fourth-largest PC vendor by unit sales, and fourth-largest smartphone manufacturer.
// It is one of the Big Five American information technology companies, along with Amazon, Google, Microsoft, and Facebook."),
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
    fn get_string_field(json_txt: &Value, field: &str) -> String {
        const ERROR: &str = "__Error__";
        json_txt[field].as_str().unwrap_or(ERROR).to_string()
    }

    fn get_i32_field(json_txt: &Value, field: &str) -> i32 {
        json_txt[field].as_str().unwrap_or("").parse::<i32>().unwrap_or(-9)
    }

    fn get_f32_field(json_txt: &Value, field: &str) -> f32 {
        json_txt[field].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99)
    }

    fn get_date_field(json_txt: &Value, field: &str) -> NaiveDate {
        NaiveDate::parse_from_str(json_txt[field].as_str().unwrap_or(""), "%Y-%m-%d").unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap())
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
            marketcapitalization: Self::get_i32_field(&json_txt, "MarketCapitalization"),
            ebitda: Self::get_i32_field(&json_txt, "EBITDA"),
            peratio: Self::get_f32_field(&json_txt, "PERatio"),
            pegratio: Self::get_f32_field(&json_txt, "PEGRatio"),
            bookvalue: Self::get_f32_field(&json_txt, "BookValue"),
            dividendpershare: Self::get_f32_field(&json_txt, "DividendPerShare"),
            dividendyield: Self::get_f32_field(&json_txt, "DividendYield"),
            eps: Self::get_f32_field(&json_txt, "EPS"),
            revenuepersharettm: Self::get_f32_field(&json_txt, "RevenuePerShareTTM"),
            profitmargin: Self::get_f32_field(&json_txt, "ProfitMargin"),
            operatingmarginttm: Self::get_f32_field(&json_txt, "OperatingMarginTTM"),
            returnonassetsttm: Self::get_f32_field(&json_txt, "ReturnOnAssetsTTM"),
            returnonequityttm: Self::get_f32_field(&json_txt, "ReturnOnEquityTTM"),
            revenuettm: Self::get_i32_field(&json_txt, "RevenueTTM"),
            grossprofitttm: Self::get_i32_field(&json_txt, "GrossProfitTTM"),
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
            beta: Self::get_f32_field(&json_txt, "Beta"),
            annweekhigh: Self::get_f32_field(&json_txt, "52WeekHigh"),
            annweeklow: Self::get_f32_field(&json_txt, "52WeekLow"),
            fiftydaymovingaverage: Self::get_f32_field(&json_txt, "50DayMovingAverage"),
            twohdaymovingaverage: Self::get_f32_field(&json_txt, "200DayMovingAverage"),
            sharesoutstanding: Self::get_f32_field(&json_txt, "SharesOutstanding"),
            dividenddate: Self::get_date_field(&json_txt, "DividendDate"),
            exdividenddate: Self::get_date_field(&json_txt, "ExDividendDate"),
        })
    }
}
