-- Your SQL goes here
CREATE TABLE sessions (
  id               VARCHAR(128) NOT NULL PRIMARY KEY UNIQUE,
  username         VARCHAR(256) NOT NULL UNIQUE,
  secret           VARCHAR(100) NOT NULL,
  created_at       TIMESTAMP NOT NULL,
  user_id          VARCHAR(21)
)
