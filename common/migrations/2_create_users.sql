CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(250) NOT NULL,
  family_name VARCHAR(250) NOT NULL,
  patronym VARCHAR(250),
  username TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL
);
