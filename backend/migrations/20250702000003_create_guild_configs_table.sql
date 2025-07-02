CREATE TABLE guild_configs (
    id SERIAL PRIMARY KEY,
    guild_id VARCHAR(255) NOT NULL UNIQUE,
    modmail_category_id VARCHAR(255),
    log_channel_id VARCHAR(255),
    randomize_names BOOLEAN DEFAULT FALSE,
    auto_close_hours INTEGER DEFAULT NULL,
    welcome_message TEXT DEFAULT NULL,
    moderator_role_ids TEXT[] DEFAULT '{}',
    blocked_words TEXT[] DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
