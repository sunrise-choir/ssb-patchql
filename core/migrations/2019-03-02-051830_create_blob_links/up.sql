CREATE TABLE IF NOT EXISTS blob_links (
  id INTEGER PRIMARY KEY,
  link_from_key_id INTEGER NOT NULL,
  link_to_blob_id INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS blob_links_index_to ON blob_links(link_to_blob_id);
CREATE INDEX IF NOT EXISTS blob_links_index_from ON blob_links(link_from_key_id);
