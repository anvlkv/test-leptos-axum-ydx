CREATE TABLE permissions (
    id SERIAL PRIMARY KEY,
    token TEXT NOT NULL,
    user_id INT,
    FOREIGN KEY(user_id) REFERENCES users(id)
        ON DELETE CASCADE
);
