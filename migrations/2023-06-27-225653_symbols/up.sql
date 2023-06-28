-- Your SQL goes here
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
