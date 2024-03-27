ALTER TABLE entries
    ADD COLUMN by_user_id INT NOT NULL
    REFERENCES users(id);
