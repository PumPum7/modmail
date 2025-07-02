-- Add guild_id column to all main tables
ALTER TABLE threads ADD COLUMN guild_id VARCHAR(255) NOT NULL DEFAULT 'default';
ALTER TABLE messages ADD COLUMN guild_id VARCHAR(255) NOT NULL DEFAULT 'default';
ALTER TABLE macros ADD COLUMN guild_id VARCHAR(255) NOT NULL DEFAULT 'default';
ALTER TABLE notes ADD COLUMN guild_id VARCHAR(255) NOT NULL DEFAULT 'default';
ALTER TABLE blocked_users ADD COLUMN guild_id VARCHAR(255) NOT NULL DEFAULT 'default';
