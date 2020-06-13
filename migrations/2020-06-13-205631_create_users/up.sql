-- Your SQL goes here
CREATE TABLE users (
  username VARCHAR(100) NOT NULL,
  email VARCHAR(100) NOT NULL PRIMARY KEY,
  password VARCHAR(64) NOT NULL,
  created_at TIMESTAMP NOT NULL
)
