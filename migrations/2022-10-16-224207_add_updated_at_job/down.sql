-- This file should undo anything in `up.sql`

ALTER TABLE job
DROP COLUMN updated_at;
