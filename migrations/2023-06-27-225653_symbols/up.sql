-- Your SQL goes here
create table symbols
(
    sid       bigint primary key,
    symbol    text      not null,
    compname  text      not null,
    secname   text      not null,
    exch      text      not null,
    cqssym    text      not null,
    etf       boolean   not null,
    rltsz     real      not null,
    istest    boolean   not null,
    nasdaqsym text      not null,
    hasprice  boolean   not null,
    c_time    timestamp not null ,
    m_time    timestamp not null
);
