-- Add migration script here
CREATE TABLE IF NOT EXISTS books (
  id UUID PRIMARY KEY,
  title varchar NOT NULL,
  author varchar NOT NULL,
  quote TEXT NOT NULL,
  inserted_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,
  UNIQUE (book, quote)  
);