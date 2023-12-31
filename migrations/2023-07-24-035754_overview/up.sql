-- Your SQL goes here
create table overviews
(
    sid bigint references symbols(sid),
    primary key (sid),
    symbol text not null,
    name text not null,
    description text not null,
    CIK text not null,
    Exch text not null,
    CURR text not null,
    country text not null,
    sector text not null,
    Industry text not null,
    address text not null,
    FiscalYearEnd text not null ,
    LatestQuarter date not null,
    MarketCapitalization int not null ,
    EBITDA int not null,
    PERatio real not null ,
    PEGRatio real not null ,
    BookValue real not null ,
    DividendPerShare real not null ,
    DividendYield real not null ,
    EPS real not null ,
    c_time timestamp not null,
    mod_time timestamp not null

);

create table overviewexts
(
    sid bigint references symbols(sid),
    primary key(sid),
    RevenuePerShareTTM real not null ,
    ProfitMargin real not null ,
    OperatingMarginTTM real not null ,
    ReturnOnAssetsTTM real not null ,
    ReturnOnEquityTTM real not null ,
    RevenueTTM int not null ,
    GrossProfitTTM int not null,
    DilutedEPSTTM real not null,
    QuarterlyEarningsGrowthYOY real not null,
    QuarterlyRevenueGrowthYOY real not null,
    AnalystTargetPrice real not null ,
    TrailingPE real not null ,
    ForwardPE real not null ,
    PriceToSalesRatioTTM real not null ,
    PriceToBookRatio real not null,
    EVToRevenue real not null,
    EVToEBITDA real not null,
    Beta real not null ,
    annWeekHigh real not null ,
    annWeekLow real not null ,
    fiftyDayMovingAverage real not null ,
    twohDayMovingAverage real not null ,
    SharesOutstanding real not null,
    DividendDate date not null,
    ExDividendDate date not null,
    c_time timestamp not null,
    mod_time timestamp not null
);