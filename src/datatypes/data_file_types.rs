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
/// Data File Types
/// Contain the corresponding data structures for the symbol data files.

/// A structure representing other symbols on the New York Stock Exchange (NYSE).
/// The data is based on the dataset provided at: https://datahub.io/core/nyse-other-listings
#[derive(Debug, Clone, Default, Deserialize)]
pub struct NyseOtherSymbol {
    /// The actual symbol of the company listed on the NYSE.
    pub actsymbol: String,

    /// The official registered name of the company.
    pub companyname: String,

    /// The name of the security that the symbol represents.
    pub securityname: String,

    /// The exchange on which the security is traded.
    pub exchange: String,

    /// The CQS symbol for the security. The Consolidated Quotation System (CQS) provides
    /// price quotations for securities traded on NYSE, American Stock Exchange, Nasdaq
    /// and regional stock exchanges.
    pub cqssymbol: String,

    /// A flag indicating whether the security is an ETF (Exchange Traded Fund).
    /// "Y" if the security is an ETF; "N" otherwise.
    pub etf: String,

    /// The standard lot size for round lot trades.
    /// This is usually 100 shares for most securities.
    pub roundlotsize: f32,

    /// A flag indicating whether this is a test security.
    /// "Y" if the security is a test issue; "N" otherwise.
    pub testissue: String,

    /// The symbol used for this security on the Nasdaq exchange.
    pub nasdaqsymbol: String,
}

/// A structure representing listed symbols on the NASDAQ exchange.
/// The data is based on the dataset provided at: https://datahub.io/core/nasdaq-listings#resource-nasdaq-listed
#[derive(Debug, Clone, Default, Deserialize)]
pub struct NasdaqListed {
    /// The unique identifier used to identify the listed company on the NASDAQ exchange.
    pub symbol: String,

    /// The official registered name of the company.
    pub companyname: String,

    /// The name of the security that the symbol represents.
    pub securityname: String,

    /// The category of the market that the security is traded on.
    /// Examples include "N" for NASDAQ Global Select MarketSM, "G" for Global Market,
    /// and "S" for NASDAQ Capital Market.
    pub marketcategory: String,

    /// A flag indicating whether this is a test security.
    /// "Y" if the security is a test issue; "N" otherwise.
    pub testissue: String,

    /// A flag indicating the financial status of the company.
    /// For instance, "D" signifies Deficient, "E" for Delinquent,
    /// "Q" for Bankrupt, "N" for Normal, "G" for Deficient and Bankrupt,
    /// and "H" for Deficient and Delinquent.
    pub financialstatus: String,

    /// The standard lot size for round lot trades.
    /// This is usually 100 shares for most securities.
    pub roundlotsize: f32,
}
