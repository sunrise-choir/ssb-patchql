CREATE TABLE IF NOT EXISTS abouts (
  id INTEGER PRIMARY KEY,
  link_from_key_id INTEGER,
  link_to_author_id INTEGER,
  link_to_key_id INTEGER,
  UNIQUE(link_from_key_id, link_to_author_id, link_to_key_id)
);
CREATE INDEX IF NOT EXISTS abouts_from_key_index on abouts (link_from_key_id);
CREATE INDEX IF NOT EXISTS abouts_to_key_index on abouts (link_to_key_id);
CREATE INDEX IF NOT EXISTS abouts_to_author_index on abouts (link_to_author_id);

