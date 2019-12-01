CREATE TABLE IF NOT EXISTS mentions (
  id INTEGER PRIMARY KEY,
  link_from_key_id INTEGER NOT NULL,
  link_to_author_id INTEGER NOT NULL 
);
CREATE INDEX IF NOT EXISTS mentions_id_to_index ON mentions (link_to_author_id, link_from_key_id);
CREATE INDEX IF NOT EXISTS mentions_id_from_index ON mentions (link_from_key_id, link_to_author_id);
