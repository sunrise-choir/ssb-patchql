-- Your SQL goes here
CREATE TABLE IF NOT EXISTS branches (
  id INTEGER PRIMARY KEY,
  link_from_key_id INTEGER NOT NULL,
  link_to_key_id INTEGER NOT NULL
)
