ALTER TABLE topstats DROP CONSTRAINT IF EXISTS topstats_pk;
alter table topstats
    add constraint topstats_pk
        unique (sid, event_type, date);-- Your SQL goes here