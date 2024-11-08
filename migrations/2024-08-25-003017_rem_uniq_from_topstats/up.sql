
alter table topstats
    add constraint topstats_pk
        unique (sid, event_type, date);