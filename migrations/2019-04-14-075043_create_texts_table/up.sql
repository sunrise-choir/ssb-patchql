CREATE VIEW IF NOT EXISTS message_texts AS
  SELECT 
    key_id,
    json_extract(content, '$.text') AS text
  FROM messages;

CREATE VIRTUAL TABLE texts USING FTS5(text, content=message_texts, content_rowid=key_id);
