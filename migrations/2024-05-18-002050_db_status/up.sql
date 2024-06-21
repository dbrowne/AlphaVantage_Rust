CREATE TABLE proctypes (
     id SERIAL PRIMARY KEY,
     name TEXT NOT NULL UNIQUE
);

INSERT INTO proctypes (name) VALUES
('load_intraday'),
('load_missed'),
('load_missed_overviews'),
('load_news'),
('load_open_close'),
('load_overviews'),
('load_symbols'),
('load_tops');

create table states
(
    id         Serial primary key,
    name       text not null unique
);

insert into states (name) values
('running'),
('success'),
('failed');


create table procstates
(
    spid         Serial primary key,
    proc_id    integer references proctypes (id),
    start_time timestamp not null default now(),
    end_state   integer references states (id) default null,
    end_time   timestamp default null
);