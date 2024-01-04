-- Your SQL goes here
create table topicmaps
(
    id       serial primary key,
    sid      bigint not null,
    feedid   integer references feeds (id) not null,
    topicid    int not null references topicrefs (id),
    relscore float not null
);