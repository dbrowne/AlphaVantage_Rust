-- Your SQL goes here

DROP TABLE IF EXISTS tickersentiments CASCADE;
DROP TABLE IF EXISTS topicmaps CASCADE;
DROP TABLE IF EXISTS topicrefs CASCADE;
DROP TABLE IF EXISTS authormaps CASCADE;
DROP TABLE IF EXISTS feeds CASCADE;
DROP TABLE IF EXISTS newsoverviews CASCADE;
DROP TABLE IF EXISTS topstats CASCADE;
DROP TABLE IF EXISTS summaryprices CASCADE;
DROP TABLE IF EXISTS intradayprices CASCADE;
DROP TABLE IF EXISTS overviewexts CASCADE;
DROP TABLE IF EXISTS overviews CASCADE;
DROP TABLE IF EXISTS symbols CASCADE;

create table symbols
(
    sid         bigint primary key not null,
    symbol      text               not null,
    name        text               not null,
    sec_type    text               not null,
    region      text               not null,
    marketOpen  time               not null,
    marketClose time               not null,
    timezone    text               not null,
    currency    text               not null,
    overview    boolean            not null,
    intraday    boolean            not null,
    summary     boolean            not null,
    c_time      timestamp          not null,
    m_time      timestamp          not null
);


create table overviews
(
    sid                  bigint references symbols (sid),
    primary key (sid),
    symbol               text      not null,
    name                 text      not null,
    description          text      not null,
    CIK                  text      not null,
    Exch                 text      not null,
    CURR                 text      not null,
    country              text      not null,
    sector               text      not null,
    Industry             text      not null,
    address              text      not null,
    FiscalYearEnd        text      not null,
    LatestQuarter        date      not null,
    MarketCapitalization int       not null,
    EBITDA               int       not null,
    PERatio              real      not null,
    PEGRatio             real      not null,
    BookValue            real      not null,
    DividendPerShare     real      not null,
    DividendYield        real      not null,
    EPS                  real      not null,
    c_time               timestamp not null,
    mod_time             timestamp not null

);

create table overviewexts
(
    sid                        bigint references symbols (sid),
    primary key (sid),
    RevenuePerShareTTM         real      not null,
    ProfitMargin               real      not null,
    OperatingMarginTTM         real      not null,
    ReturnOnAssetsTTM          real      not null,
    ReturnOnEquityTTM          real      not null,
    RevenueTTM                 int       not null,
    GrossProfitTTM             int       not null,
    DilutedEPSTTM              real      not null,
    QuarterlyEarningsGrowthYOY real      not null,
    QuarterlyRevenueGrowthYOY  real      not null,
    AnalystTargetPrice         real      not null,
    TrailingPE                 real      not null,
    ForwardPE                  real      not null,
    PriceToSalesRatioTTM       real      not null,
    PriceToBookRatio           real      not null,
    EVToRevenue                real      not null,
    EVToEBITDA                 real      not null,
    Beta                       real      not null,
    annWeekHigh                real      not null,
    annWeekLow                 real      not null,
    fiftyDayMovingAverage      real      not null,
    twohDayMovingAverage       real      not null,
    SharesOutstanding          real      not null,
    DividendDate               date      not null,
    ExDividendDate             date      not null,
    c_time                     timestamp not null,
    mod_time                   timestamp not null
);

create table intradayprices
(
    eventid serial primary key,
    tstamp  timestamp without time zone not null,
    sid     bigint                      not null,
    symbol  text                        not null,
    open    real                        not null,
    high    real                        not null,
    low     real                        not null,
    close   real                        not null,
    volume  int                         not null,
    foreign key (sid) references symbols (sid)
);

create table summaryprices
(
    eventid serial primary key,
    date    date   not null,
    sid     bigint not null,
    symbol  text   not null,
    open    real   not null,
    high    real   not null,
    low     real   not null,
    close   real   not null,
    volume  int    not null,
    foreign key (sid) references symbols (sid)
);

create table topstats
(
    eventid    serial primary key,
    date       timestamp not null unique,
    event_type text      not null,
    sid        bigint    not null,
    symbol     text      not null,
    price      real      not null,
    change_val real      not null,
    change_pct real      not null,
    volume     int       not null,
    foreign key (sid) references symbols (sid),
    constraint topstats_date_event_type_unique UNIQUE (date, event_type, sid)

);

create table newsoverviews
(
    id       serial primary key,
    sid      bigint    not null,
    items    integer   not null,
    hashid   text      not null,
    creation timestamp not null,
    foreign key (sid) references symbols (sid),
    constraint unique_creation_sid unique (hashid, sid)
);

create table feeds
(
    id             serial primary key,
    sid            bigint                                not null,
    newsoverviewid integer references newsoverviews (id) not null,
    articleid      text                                  not null references articles (hashid),
    sourceid       int references sources (id)           not null,
    osentiment     float                                 not null,
    sentlabel      text                                  not null,
    foreign key (sid) references symbols (sid)
);

create table authormaps
(
    id       serial primary key,
    feedid   integer not null references feeds (id),
    authorid integer not null references authors (id),
    unique (feedid, authorid)
);

CREATE TABLE topicrefs (
                           id SERIAL PRIMARY KEY,
                           name TEXT NOT NULL UNIQUE
);

-- Populate the topic_reference table with the predefined topics will add more as needed
INSERT INTO topicrefs (name) VALUES
                                 ('Blockchain'),
                                 ('Earnings'),
                                 ('Economy - Fiscal'),
                                 ('Economy - Macro'),
                                 ('Economy - Monetary'),
                                 ('Energy & Transportation'),
                                 ('Finance'),
                                 ('Financial Markets'),
                                 ('IPO'),
                                 ('Life Sciences'),
                                 ('Manufacturing'),
                                 ('Real Estate & Construction'),
                                 ('Retail & Wholesale'),
                                 ('Technology');

create table topicmaps
(
    id       serial primary key,
    sid      bigint                        not null,
    feedid   integer references feeds (id) not null,
    topicid  int                           not null references topicrefs (id),
    relscore float                         not null,
    foreign key (sid) references symbols (sid)
);
create table tickersentiments
(
    id             serial primary key,
    feedid         integer references feeds (id) not null,
    sid            bigint                        not null,
    relevance      float                         not null,
    tsentiment     float                         not null,
    sentimentlable text                          not null,
    foreign key (sid) references symbols (sid)
);