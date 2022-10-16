-- Your SQL goes here

ALTER TABLE job
ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT now();

UPDATE job
SET updated_at = created_at;
