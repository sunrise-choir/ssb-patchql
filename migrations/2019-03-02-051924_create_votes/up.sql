CREATE TABLE IF NOT EXISTS votes (
  link_from_author_id INTEGER NOT NULL,
  link_to_key_id INTEGER NOT NULL,
  value INTEGER,
  PRIMARY KEY(link_from_author_id, link_to_key_id)
);
CREATE INDEX IF NOT EXISTS votes_link_from_author_id_index on votes (link_from_author_id);
CREATE INDEX IF NOT EXISTS votes_link_to_key_id_index on votes (link_to_key_id);
