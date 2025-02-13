-- Drop the newly added unique constraint
ALTER TABLE topstats
DROP CONSTRAINT IF EXISTS topstats_date_event_type_unique;

-- Re-add the original constraint if necessary
ALTER TABLE topstats
    ADD CONSTRAINT topstats_date_key UNIQUE (date);
