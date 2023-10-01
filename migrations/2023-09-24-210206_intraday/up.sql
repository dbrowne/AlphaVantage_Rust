create table intradayprices
(
    eventid    serial primary key,
    tstamp        timestamp without time zone   not null,
    sid          bigint not null,
    symbol      text   not null,
    open  real   not null,
    high  real   not null,
    low   real   not null,
    close real   not null,
    volume      int    not null

);