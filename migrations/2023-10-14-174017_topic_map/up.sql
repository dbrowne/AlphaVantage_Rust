-- Your SQL goes here
create table topicmaps
(
    id       serial primary key,
    sid      bigint not null,
    feedid   integer references feeds (id) not null,
    topic    int not null,
    relscore float not null
);