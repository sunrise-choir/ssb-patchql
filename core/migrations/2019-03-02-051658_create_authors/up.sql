-- Your SQL goes here
CREATE TABLE IF NOT EXISTS authors (
  id INTEGER PRIMARY KEY,
  author TEXT UNIQUE NOT NULL,
  is_me BOOLEAN 
);
CREATE INDEX IF NOT EXISTS authors_is_me_index ON authors (is_me);
