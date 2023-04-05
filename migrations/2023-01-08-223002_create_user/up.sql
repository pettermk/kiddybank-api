CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  first_name text NOT NULL,
  last_name text NOT NULL,
  external_id text NOT NULL
)
