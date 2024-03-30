ALTER TABLE entries
    ADD COLUMN by_user_id UUID NOT NULL
    REFERENCES users(id);
