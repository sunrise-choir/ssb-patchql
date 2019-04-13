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

CREATE VIEW IF NOT EXISTS threads AS 
  SELECT 
    roots.flume_seq as flume_seq,
    roots.key_id as key_id,
    roots.seq as seq,
    roots.received_time as received_time,
    roots.asserted_time as asserted_time,
    roots.root_key_id as root_key_id,
    roots.fork_key_id as fork_key_id,
    roots.author_id,
    roots.content_type,
    roots.content,
    roots.is_decrypted,
    replies.key_id as reply_key_id,
    replies.author_id as reply_author_id
  FROM messages AS roots
  INNER JOIN messages AS replies ON roots.key_id = replies.root_key_id
  WHERE replies.content_type = 'post';

CREATE INDEX IF NOT EXISTS messages_author_id_index ON messages(author_id);
CREATE INDEX IF NOT EXISTS messages_content_type_index_flume_seq ON messages(content_type, flume_seq);
CREATE INDEX IF NOT EXISTS messages_key_id_index ON messages(root_key_id);
