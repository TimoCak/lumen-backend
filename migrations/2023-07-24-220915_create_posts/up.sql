-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    author VARCHAR NOT NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP,
    title VARCHAR NOT NULL,
    text VARCHAR NOT NULL,
    likes INTEGER DEFAULT 0,
    dislikes INTEGER DEFAULT 0
);