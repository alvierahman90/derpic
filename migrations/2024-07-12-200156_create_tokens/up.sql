-- Your SQL goes here
CREATE TABLE tokens (
  id SERIAL PRIMARY KEY,
  token bytea NOT NULL,
  name VARCHAR NOT NULL,
  revoked BOOLEAN NOT NULL
);
