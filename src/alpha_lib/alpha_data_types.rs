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


impl FullOverview {
    pub fn new(sid: i64, json_txt: Value) -> Option<Self> {
        const ERROR: &str = "__Error__";
        // Would have used serede but fields beginning with numbers are not allowed
// /JSON: Object {"200DayMovingAverage": String("151.55"),
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

        Some(Self {
            sid,
            symbol: json_txt["Symbol"].as_str().unwrap_or(ERROR).to_string(),
            name: json_txt["Name"].as_str().unwrap_or(ERROR).to_string(),
            description: json_txt["Description"].as_str().unwrap_or(ERROR).to_string(),
            cik: json_txt["CIK"].as_str().unwrap_or(ERROR).to_string(),
            exch: json_txt["Exchange"].as_str().unwrap_or("ERROR").to_string(),
            curr: json_txt["Currency"].as_str().unwrap_or("ERROR").to_string(),
            country: json_txt["Country"].as_str().unwrap_or("ERROR").to_string(),
            sector: json_txt["Sector"].as_str().unwrap_or("ERROR").to_string(),
            industry: json_txt["Industry"].as_str().unwrap_or("ERROR").to_string(),
            address: json_txt["Address"].as_str().unwrap_or("ERROR").to_string(),
            fiscalyearend: json_txt["FiscalYearEnd"].as_str().unwrap_or("ERROR").to_string(),
            latestquarter: NaiveDate::parse_from_str(json_txt["LatestQuarter"].as_str().unwrap_or(""), "%Y-%m-%d").unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()),
            marketcapitalization: json_txt["MarketCapitalization"].as_str().unwrap_or("").parse::<i32>().unwrap_or(-9),
            ebitda: json_txt["EBITDA"].as_str().unwrap_or("").parse::<i32>().unwrap_or(-9),
            peratio: json_txt["PERatio"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            pegratio: json_txt["PEGRatio"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            bookvalue: json_txt["BookValue"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            dividendpershare: json_txt["DividendPerShare"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            dividendyield: json_txt["DividendYield"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            eps: json_txt["EPS"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            revenuepersharettm: json_txt["RevenuePerShareTTM"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            profitmargin: json_txt["ProfitMargin"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            operatingmarginttm: json_txt["OperatingMarginTTM"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            returnonassetsttm: json_txt["ReturnOnAssetsTTM"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            returnonequityttm: json_txt["ReturnOnEquityTTM"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            revenuettm: json_txt["RevenueTTM"].as_str().unwrap_or("").parse::<i32>().unwrap_or(0),
            grossprofitttm: json_txt["GrossProfitTTM"].as_str().unwrap_or("").parse::<i32>().unwrap_or(0),
            dilutedepsttm: json_txt["DilutedEPSTTM"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            quarterlyearningsgrowthyoy: json_txt["QuarterlyEarningsGrowthYOY"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            quarterlyrevenuegrowthyoy: json_txt["QuarterlyRevenueGrowthYOY"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            analysttargetprice: json_txt["AnalystTargetPrice"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            trailingpe: json_txt["TrailingPE"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            forwardpe: json_txt["ForwardPE"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            pricetosalesratiottm: json_txt["PriceToSalesRatioTTM"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            pricetobookratio: json_txt["PriceToBookRatio"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            evtorevenue: json_txt["EVToRevenue"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            evtoebitda: json_txt["EVToEBITDA"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            beta: json_txt["Beta"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            annweekhigh: json_txt["52WeekHigh"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            annweeklow: json_txt["52WeekLow"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            fiftydaymovingaverage: json_txt["50DayMovingAverage"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            twohdaymovingaverage: json_txt["200DayMovingAverage"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            sharesoutstanding: json_txt["SharesOutstanding"].as_str().unwrap_or("").parse::<f32>().unwrap_or(-9.99),
            dividenddate: json_txt["DividendDate"].as_str().unwrap_or("").parse::<NaiveDate>().unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()),
            exdividenddate: json_txt["ExDividendDate"].as_str().unwrap_or("ERROR").parse::<NaiveDate>().unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()),
        })
    }
}
