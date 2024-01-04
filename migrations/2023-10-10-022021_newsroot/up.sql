-- Your SQL goes here
create table newsoverviews
(
    id        serial primary key,
    sid       bigint   not null,
    items     integer   not null,
    sentiment text      not null,
    relevance text      not null,
    creation  timestamp not null,
    constraint unique_creation_sid unique (creation, sid)
);