-- Your SQL goes here
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  notes TEXT DEFAULT NULL,
  overall FLOAT DEFAULT NULL,
  psychomotor FLOAT DEFAULT NULL,
  energy FLOAT DEFAULT NULL,
  mood FLOAT DEFAULT NULL,
  thoughts_slowed_racing FLOAT DEFAULT NULL,
  concentration_difficulty FLOAT DEFAULT NULL,
  time_submitted BIGINT NOT NULL DEFAULT -1
);