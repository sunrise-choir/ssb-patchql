CREATE TABLE IF NOT EXISTS messages (
  flume_seq BIGINT PRIMARY KEY,
  key_id INTEGER UNIQUE, 
  seq INTEGER,
  received_time DOUBLE,
  asserted_time DOUBLE,
  root_key_id INTEGER,
  fork_key_id INTEGER,
  author_id INTEGER,
  content_type TEXT,
  content TEXT,
  is_decrypted BOOLEAN 
);
CREATE INDEX IF NOT EXISTS messages_author_id_index ON messages(author_id);
CREATE INDEX IF NOT EXISTS messages_content_type_index ON messages(content_type);
CREATE INDEX IF NOT EXISTS messages_key_id_index ON messages(root_key_id);
