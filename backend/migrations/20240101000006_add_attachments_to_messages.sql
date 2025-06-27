ALTER TABLE messages ADD COLUMN attachments JSONB DEFAULT '[]'::jsonb;
