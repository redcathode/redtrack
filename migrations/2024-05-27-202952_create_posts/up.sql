-- Your SQL goes here
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  userid BIGINT NOT NULL DEFAULT -1,
  fieldname VARCHAR(128) NOT NULL DEFAULT 'mood',
  fieldtype VARCHAR(128) NOT NULL DEFAULT 'numerical_rating',
  fieldval TEXT DEFAULT NULL,
  timestamp BIGINT NOT NULL DEFAULT -1
);