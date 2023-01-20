-- Add migration script here
CREATE TABLE IF NOT EXISTS users(
   id serial PRIMARY KEY,
   name TEXT NOT NULL,
   age INT NOT NULL,
   pwd TEXT NOT NULL
  );