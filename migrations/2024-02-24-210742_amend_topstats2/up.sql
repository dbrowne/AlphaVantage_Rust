-- Your SQL goes here
ALTER TABLE topstats
    DROP CONSTRAINT IF EXISTS topstats_date_event_type_unique;

ALTER TABLE topstats
    ADD CONSTRAINT topstats_date_event_type_sid_unique UNIQUE (date, event_type, sid);


