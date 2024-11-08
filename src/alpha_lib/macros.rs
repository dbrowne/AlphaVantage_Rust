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

/// The current supported AlphaVantage API functions.
#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash)]
pub enum FuncType {
  TsIntra,
  TsDaily,
  Overview,
  SymSearch,
  TopQuery,
  NewsQuery,
}

/// `create_url!` is a macro used for constructing request URLs to various endpoints of the
/// AlphaVantage API. It is necessary because macros run before name resolution
/// see https://github.com/rust-lang/rust/issues/69133 for more details
///
/// This macro takes a `FuncType`, which denotes the AlphaVantage API endpoint to construct a URL
/// for, and two expression parameters representing the symbol and API key. The order of the
/// expression parameters is always: symbol then API key.
///
/// The available `FuncType`s are:
///
/// `TsIntraExt`: Constructs a URL for the TIME_SERIES_INTRADAY_EXTENDED endpoint.
/// `TsDaily`: Constructs a URL for the TIME_SERIES_DAILY endpoint.
/// `Overview`: Constructs a URL for the OVERVIEW endpoint.
/// `SymSearch`: Constructs a URL for the SYMBOL_SEARCH endpoint.
///
/// # Example
///
///
/// let url = create_url!(FuncType:TsDaily, "AAPL", "demo");
/// assert_eq!(url, "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&datatype=json&symbol=AAPL&apikey=demo");
///
///
/// If an unrecognized `FuncType` is passed, it returns a string saying "Unknown function type
/// received".
///
/// # Panics
///
/// This macro does not panic.
#[macro_export]
macro_rules! create_url {
    (FuncType:TsIntra,$string1:expr, $string2:expr) =>{
        format!("https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&datatype=csv&symbol={}&interval=1min&apikey={}",$string1,$string2)
    };
    (FuncType:TsDaily,$string1:expr, $string2:expr) =>{
        format!("https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&datatype=json&symbol={}&apikey={}",$string1,$string2)
    };
    (FuncType:Overview,$string1:expr, $string2:expr) =>{
        format!("https://www.alphavantage.co/query?function=OVERVIEW&symbol={}&apikey={}",$string1,$string2)
    };
    (FuncType:SymSearch,$string1:expr, $string2:expr) =>{
        format!("https://www.alphavantage.co/query?function=SYMBOL_SEARCH&keywords={}&apikey={}&datatype=csv",$string1,$string2)
    };
    (FuncType:TopQuery,$string1:expr, $string2:expr) =>{
        format!("https://www.alphavantage.co/query?function=TOP_GAINERS_LOSERS&apikey={}",$string2)
    };
    (FuncType:NewsQuery,$string1:expr, $string2:expr) =>{
        format!("https://www.alphavantage.co/query?function=NEWS_SENTIMENT&tickers={}&apikey={}",$string1,$string2)
    };
    ($other:expr,$string1:expr, $string2:expr) =>{
        format!("Unknown function type received {:?}",$other)
    };
}

#[cfg(test)]
mod test {
  #[test]
  fn t_01() {
    let (sym, api_key) = ("AAPL", "123456789");
    let url = create_url!(FuncType:TsIntra,sym,api_key);
    assert_eq!(url, "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&datatype=csv&symbol=AAPL&interval=1min&apikey=123456789");
  }

  #[test]
  fn t_02() {
    let (sym, api_key) = ("AAPL", "123456789");
    let url = create_url!(FuncType:TsDaily,sym,api_key);
    assert_eq!(url, "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&datatype=json&symbol=AAPL&apikey=123456789");
  }

  #[test]
  fn t_03() {
    let (sym, api_key) = ("AAPL", "123456789");
    let url = create_url!(FuncType:Overview,sym,api_key);
    assert_eq!(
      url,
      "https://www.alphavantage.co/query?function=OVERVIEW&symbol=AAPL&apikey=123456789"
    );
  }

  #[test]
  fn t_04() {
    let (sym, api_key) = ("AAPL", "123456789");
    let url = create_url!(FuncType:SymSearch,sym,api_key);
    assert_eq!(url, "https://www.alphavantage.co/query?function=SYMBOL_SEARCH&keywords=AAPL&apikey=123456789&datatype=csv");
  }

  #[test]
  fn t_05() {
    let url = create_url!(55, "AAPL", "123456789");
    assert_eq!(url, "Unknown function type received 55");
  }

  #[test]
  fn t_06() {
    let url = create_url!(FuncType:TopQuery,"NONE","12345678");
    assert_eq!(
      url,
      "https://www.alphavantage.co/query?function=TOP_GAINERS_LOSERS&apikey=12345678"
    );
  }

  #[test]
  fn t_09() {
    let url = create_url!(FuncType:NewsQuery,"AAPL","12345678");
    assert_eq!(
      url,
      "https://www.alphavantage.co/query?function=NEWS_SENTIMENT&tickers=AAPL&apikey=12345678"
    );
  }
}
