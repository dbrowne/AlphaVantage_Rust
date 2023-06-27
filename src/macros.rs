#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash)]
pub  enum FuncType {
    TsIntraExt,
    TsDaily,
    Overview,
    SymSearch,
}

/// `create_url!` is a macro used for constructing request URLs to various endpoints of the AlphaVantage API.
/// It is necessary because macros wun becfore name resolution
/// see https://github.com/rust-lang/rust/issues/69133 for more details
///
/// This macro takes a `FuncType`, which denotes the AlphaVantage API endpoint to construct a URL for, and
/// two expression parameters representing the symbol and API key. The order of the expression parameters is
/// always: symbol then API key.
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
/// ```rust
/// let url = create_url!(FuncType:TsDaily, "AAPL", "demo");
/// assert_eq!(url, "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&datatype=json&symbol=AAPL&apikey=demo");
/// ```
///
/// If an unrecognized `FuncType` is passed, it returns a string saying "Unknown function type received".
///
/// # Panics
///
/// This macro does not panic.

#[macro_export]
macro_rules! create_url{
    (FuncType:TsIntraExt,$string1:expr, $string2:expr) =>{
        format!("https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY_EXTENDED&datatype=json&symbol={}&interval=1min&slice=year1month1&apikey={}",$string1,$string2)
    };
    (FuncType:TsDaily,$string1:expr, $string2:expr) =>{
        format!("https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&datatype=json&symbol={}&apikey={}",$string1,$string2)
    };
    (FuncType:Overview,$string1:expr, $string2:expr) =>{
        format!("https://www.alphavantage.co/query?function=OVERVIEW&symbol={}&apikey={}",$string1,$string2)
    };
    (FuncType:SymSearch,$string1:expr, $string2:expr) =>{
        format!("https://www.alphavantage.co/query?function=SYMBOL_SEARCH&keywords={}&apikey={}",$string1,$string2)
    };

    ($other:expr,$string1:expr, $string2:expr) =>{
        format!("Unknown function type received {:?}",$other)
    };
}



#[cfg(test)]
mod  test{
    #[test]
    fn t_01(){
        let  url = create_url!(FuncType:TsIntraExt,"AAPL","123456789");
        assert_eq!(url,"https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY_EXTENDED&datatype=json&symbol=AAPL&interval=1min&slice=year1month1&apikey=123456789");
    }
    #[test]
    fn t_02(){
        let  url = create_url!(FuncType:TsDaily,"AAPL","123456789");
        assert_eq!(url,"https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&datatype=json&symbol=AAPL&apikey=123456789");
    }
    #[test]
    fn t_03(){
        let  url = create_url!(FuncType:Overview,"AAPL","123456789");
        assert_eq!(url,"https://www.alphavantage.co/query?function=OVERVIEW&symbol=AAPL&apikey=123456789");
    }
    #[test]
    fn  t_04(){
        let  url = create_url!(FuncType:SymSearch,"AAPL","123456789");
        assert_eq!(url,"https://www.alphavantage.co/query?function=SYMBOL_SEARCH&keywords=AAPL&apikey=123456789");
    }
    #[test]
    fn  t_05(){
        let  url = create_url!(55,"AAPL","123456789");
        assert_eq!(url,"Unknown function type received 55");
    }
}