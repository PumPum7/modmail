-- Update unique constraints to include guild_id
ALTER TABLE threads DROP CONSTRAINT IF EXISTS idx_threads_user_open;
CREATE UNIQUE INDEX idx_threads_user_guild_open ON threads (user_id, guild_id) WHERE is_open = true;

-- Add guild-specific indexes for performance
CREATE INDEX idx_threads_guild_id ON threads (guild_id);
CREATE INDEX idx_messages_guild_id ON messages (guild_id);
CREATE INDEX idx_macros_guild_id ON macros (guild_id);
