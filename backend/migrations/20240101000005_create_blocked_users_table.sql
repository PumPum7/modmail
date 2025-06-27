CREATE TABLE blocked_users (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL UNIQUE,
    user_tag VARCHAR(255) NOT NULL,
    blocked_by VARCHAR(255) NOT NULL,
    blocked_by_tag VARCHAR(255) NOT NULL,
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
