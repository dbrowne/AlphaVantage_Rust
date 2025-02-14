-- Your SQL goes here
DROP TABLE IF EXISTS tickersentiments CASCADE;
DROP TABLE IF EXISTS topicmaps CASCADE;
DROP TABLE IF EXISTS authormaps CASCADE;
DROP TABLE IF EXISTS feeds CASCADE;
DROP TABLE IF EXISTS newsoverviews CASCADE;

create table newsoverviews
(
    id       serial primary key,
    sid      bigint    not null,
    items    integer   not null,
    hashid   text      not null,
    creation timestamp not null,
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
    sentlabel      text                                  not null
);

create table authormaps
(
    id       serial primary key,
    feedid   integer not null references feeds (id),
    authorid integer not null references authors (id),
    unique (feedid, authorid)
);

create table topicmaps
(
    id       serial primary key,
    sid      bigint                        not null,
    feedid   integer references feeds (id) not null,
    topicid  int                           not null references topicrefs (id),
    relscore float                         not null
);
create table tickersentiments
(
    id             serial primary key,
    feedid         integer references feeds (id) not null,
    sid            bigint                        not null,
    relevance      float                         not null,
    tsentiment     float                         not null,
    sentimentlable text                          not null
);