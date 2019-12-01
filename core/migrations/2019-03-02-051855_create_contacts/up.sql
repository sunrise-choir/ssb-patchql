-- Your SQL goes here
CREATE TABLE IF NOT EXISTS contacts (
  id INTEGER PRIMARY KEY,
  author_id INTEGER NOT NULL,
  contact_author_id INTEGER NOT NULL,
  is_decrypted BOOLEAN NOT NULL,
  state INTEGER,
  UNIQUE(author_id, contact_author_id, is_decrypted)
);
CREATE INDEX IF NOT EXISTS contacts_contact_author_id_state_index ON contacts(contact_author_id);
CREATE INDEX IF NOT EXISTS contacts_author_id_state_index ON contacts(author_id, state);
