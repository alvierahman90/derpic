-- Your SQL goes here
CREATE TABLE images (
  id SERIAL PRIMARY KEY,
  token_id SERIAL REFERENCES tokens(id) NOT NULL,
  image bytea NOT NULL,
  slug bytea NOT NULL
);
