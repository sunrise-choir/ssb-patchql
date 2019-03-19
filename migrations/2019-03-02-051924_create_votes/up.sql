CREATE TABLE IF NOT EXISTS votes (
  id INTEGER PRIMARY KEY,
  link_from_author_id INTEGER,
  link_to_key_id INTEGER,
  value INTEGER,
  UNIQUE (link_from_author_id, link_to_key_id)
);
CREATE INDEX IF NOT EXISTS votes_link_from_author_id_index on votes (link_from_author_id);
CREATE INDEX IF NOT EXISTS votes_link_to_key_id_index on votes (link_to_key_id);
