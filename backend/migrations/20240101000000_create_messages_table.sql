CREATE TABLE messages (
    id UUID PRIMARY KEY,
    author_id VARCHAR(255) NOT NULL,
    author_tag VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);
