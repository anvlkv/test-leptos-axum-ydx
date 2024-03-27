CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR(250) NOT NULL,
  family_name VARCHAR(250) NOT NULL,
  patronym VARCHAR(250),
  username TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL
);
