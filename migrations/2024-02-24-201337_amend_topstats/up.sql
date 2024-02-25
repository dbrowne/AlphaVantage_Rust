-- Your SQL goes here

ALTER TABLE topstats
    DROP CONSTRAINT IF EXISTS topstats_date_key;

ALTER TABLE topstats
    ADD CONSTRAINT topstats_date_event_type_unique UNIQUE (date, event_type);