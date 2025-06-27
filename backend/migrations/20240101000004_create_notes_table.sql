CREATE TABLE notes (
    id UUID PRIMARY KEY,
    thread_id INTEGER REFERENCES threads(id),
    author_id VARCHAR(255) NOT NULL,
    author_tag VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);
