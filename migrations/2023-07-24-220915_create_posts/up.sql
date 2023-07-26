-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    thread_id INTEGER NOT NULL,
    answer_id INTEGER DEFAULT -1,
    author VARCHAR NOT NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP,
    title VARCHAR NOT NULL,
    text VARCHAR NOT NULL,
    likes INTEGER DEFAULT 0,
    dislikes INTEGER DEFAULT 0
);