-- Add migration script here
CREATE TABLE IF NOT EXISTS users(
   id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
   name TEXT NOT NULL,
   age INT NOT NULL,
   pwd TEXT NOT NULL
  );