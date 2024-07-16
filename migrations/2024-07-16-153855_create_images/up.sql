-- Your SQL goes here
CREATE TYPE image_format AS ENUM (
  'png',
  'jpeg',
  'webp',
  'gif',
  'heif'
);

CREATE TABLE images (
  id SERIAL PRIMARY KEY,
  token_id SERIAL REFERENCES tokens(id) NOT NULL,
  image bytea NOT NULL,
  format image_format NOT NULL,
  slug bytea NOT NULL
);
