-- Your SQL goes here
create table sources
(
    id          serial primary key,
    source_name text not null,
    domain      text not null
);

create table articles
(
    hashid   bigint primary key not null,
    sourceid int references sources (id) not null,
    category text      not null,
    title    text      not null,
    url      text      not null,
    summary  text      not null,
    banner   text      not null,
    author   int references authors (id) not null,
    ct       timestamp not null
);


create table feeds
(
    id             serial primary key,
    sid            bigint not null,
    newsoverviewid integer references newsoverviews (id) not null,
    articleid      bigint not null references articles (hashid),
    sourceid       int references sources (id) not null,
    osentiment     float  not null,
    sentlabel      text   not null
);
