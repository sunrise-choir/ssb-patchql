CREATE TABLE IF NOT EXISTS messages (
  flume_seq BIGINT PRIMARY KEY,
  key_id INTEGER UNIQUE NOT NULL,
  seq INTEGER NOT NULL,
  received_time DOUBLE NOT NULL,
  asserted_time DOUBLE,
  root_key_id INTEGER,
  fork_key_id INTEGER,
  author_id INTEGER NOT NULL,
  content_type TEXT,
  content TEXT,
  is_decrypted BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS root_posts (
  flume_seq BIGINT PRIMARY KEY,
  asserted_timestamp BIGINT,
  key_id INTEGER UNIQUE NOT NULL,
  author_id INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS reply_posts (
  flume_seq BIGINT PRIMARY KEY,
  asserted_timestamp BIGINT,
  key_id INTEGER UNIQUE NOT NULL,
  root_post_id INTEGER NOT NULL,
  author_id INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS messages_author_id_index ON messages(author_id);
CREATE INDEX IF NOT EXISTS messages_content_type_index_flume_seq ON messages(content_type, flume_seq);
CREATE INDEX IF NOT EXISTS messages_root_key_id_index ON messages(root_key_id);
CREATE INDEX IF NOT EXISTS messages_fork_key_id_index ON messages(fork_key_id);

CREATE INDEX IF NOT EXISTS root_posts_timestamp_index ON root_posts(asserted_timestamp);
