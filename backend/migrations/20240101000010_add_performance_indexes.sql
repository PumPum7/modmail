CREATE INDEX idx_threads_id_desc ON threads (id DESC);

CREATE INDEX idx_threads_user_id ON threads (user_id);

CREATE INDEX idx_threads_is_open ON threads (is_open) WHERE is_open = true;

CREATE INDEX idx_threads_open_created ON threads (is_open, created_at DESC) WHERE is_open = true;

CREATE INDEX idx_threads_created_at ON threads (created_at);

CREATE INDEX idx_messages_author_id ON messages (author_id);

CREATE INDEX idx_messages_created_at ON messages (created_at);

CREATE INDEX idx_thread_messages_message_id ON thread_messages (message_id);

CREATE INDEX idx_thread_messages_thread_id ON thread_messages (thread_id);

CREATE INDEX idx_notes_thread_id ON notes (thread_id);

CREATE INDEX idx_notes_created_at ON notes (created_at);

CREATE INDEX idx_notes_author_id ON notes (author_id);

CREATE INDEX idx_blocked_users_user_id ON blocked_users (user_id);

CREATE INDEX idx_macros_quick_access ON macros (quick_access) WHERE quick_access = true;

CREATE INDEX idx_macros_name ON macros (name);