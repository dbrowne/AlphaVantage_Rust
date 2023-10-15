-- Your SQL goes here
create table tickersentiments (
    id serial primary key ,
    feedid integer references feeds(id) not null,
    ticker text not null,
    sid bigint not null,
    relevance float not null ,
    tsentiment float not null,
    sentimentlable text not null
);