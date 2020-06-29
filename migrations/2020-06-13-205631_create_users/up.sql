-- Your SQL goes here
CREATE TABLE users (
  id VARCHAR(21) NOT NULL PRIMARY KEY UNIQUE,
  username VARCHAR(100) NOT NULL UNIQUE,
  email VARCHAR(100) NOT NULL UNIQUE,
  password VARCHAR(64) NOT NULL,
  created_at TIMESTAMP NOT NULL
)
