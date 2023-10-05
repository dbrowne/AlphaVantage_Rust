-- Your SQL goes here
create table topstats
(
    eventid    serial primary key,
    date       timestamp   not null unique ,
    event_type text   not null,
    sid        bigint not null,
    symbol     text   not null,
    price      real   not null,
    change_val real   not null,
    change_pct real   not null,
    volume     int    not null
);
