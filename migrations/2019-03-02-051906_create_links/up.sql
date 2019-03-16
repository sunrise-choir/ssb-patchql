CREATE TABLE IF NOT EXISTS links (
  id INTEGER PRIMARY KEY,
  link_from_key_id INTEGER NOT NULL,
  link_to_key_id INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS links_to_id_index ON links (link_to_key_id, link_from_key_id);
CREATE INDEX IF NOT EXISTS links_from_id_index ON links (link_from_key_id, link_to_key_id);
