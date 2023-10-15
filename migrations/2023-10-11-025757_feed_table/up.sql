-- Your SQL goes here
create table sources
(
    id serial primary key,
    source text not null,
    domain text not null
);

create table articles
(
    hashid     bigint primary key,
    sourceid   int    references sources(id),
    category text    not null,
    title    text    not null,
    url      text    not null,
    summary  text    not null,
    banner  text    not null,
    author   int references authors (id),
    ct       timestamp not null
);


create table feeds
(
    id             serial primary key,
    sid            bigint           not null,
    newsoverviewid     integer references newsoverviews (id),
    articleid          bigint not null references articles (hashid),
    sourceid         int references sources(id),
    osentiment     float            not null,
    sentlabel      text             not null
);
