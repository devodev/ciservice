-- Your SQL goes here

CREATE TABLE job_parameter (
  id SERIAL PRIMARY KEY,
  job_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  value TEXT NOT NULL,
  type TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  CONSTRAINT fk_job_parameter_job FOREIGN KEY (job_id) REFERENCES job (id)
)
