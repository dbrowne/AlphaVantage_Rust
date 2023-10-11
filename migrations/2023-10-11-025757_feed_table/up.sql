-- Your SQL goes here
create table feeds
(
    id             serial primary key,
    sid            bigint           not null,
    overviewid     integer references newsoverviews (id),
    title          text             not null,
    url            text             not null,
    publishedt     timestamp        not null,
    summary        text             not null,
    banner         text             not null,
    source         text             not null,
    sourcecategory text             not null,
    sourcedomain   text             not null,
    osentiment     float            not null,
    sentlabel      text             not null
);
