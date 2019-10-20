-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
  id UUID PRIMARY KEY default uuid_generate_v4(),
  username VARCHAR NOT NULL,
  email VARCHAR(254) NOT NULL,
  salt VARCHAR NOT NULL,
  password VARCHAR NOT NULL
);

CREATE UNIQUE INDEX email_idx ON users (email);
