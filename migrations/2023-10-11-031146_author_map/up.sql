-- Your SQL goes here
create table authormaps
(
    id       serial primary key,
    feedid   integer not null references feeds (id),
    authorid integer not null references authors (id),
    unique (feedid, authorid)
);