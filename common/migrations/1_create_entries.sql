CREATE TABLE entries (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  address VARCHAR NOT NULL,
  revenue MONEY NOT NULL DEFAULT 0,
  date DATE NOT NULL
);
