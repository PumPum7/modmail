CREATE INDEX idx_thread_messages_covering 
ON thread_messages (thread_id, message_id);

CREATE INDEX idx_threads_created_at_desc 
ON threads (created_at DESC);

CREATE INDEX idx_messages_created_at_desc 
ON messages (created_at DESC);

CREATE INDEX idx_notes_created_at_desc 
ON notes (created_at DESC);

CREATE INDEX idx_messages_author_created 
ON messages (author_tag, created_at DESC);

CREATE INDEX idx_notes_author_created 
ON notes (author_tag, created_at DESC);

CREATE MATERIALIZED VIEW analytics_summary AS
SELECT 
    DATE(created_at) as date,
    COUNT(*) as thread_count,
    COUNT(*) FILTER (WHERE is_open = true) as open_threads,
    COUNT(*) FILTER (WHERE is_open = false) as closed_threads,
    COUNT(*) FILTER (WHERE urgency = 'Low') as low_urgency,
    COUNT(*) FILTER (WHERE urgency = 'Medium') as medium_urgency,
    COUNT(*) FILTER (WHERE urgency = 'High') as high_urgency,
    COUNT(*) FILTER (WHERE urgency = 'Urgent') as urgent_threads
FROM threads 
WHERE created_at >= CURRENT_DATE - INTERVAL '90 days'
GROUP BY DATE(created_at)
ORDER BY date DESC;

CREATE INDEX idx_analytics_summary_date ON analytics_summary (date);

CREATE OR REPLACE FUNCTION refresh_analytics_summary()
RETURNS void AS $$
BEGIN
    REFRESH MATERIALIZED VIEW analytics_summary;
END;
$$ LANGUAGE plpgsql;

CREATE INDEX idx_threads_user_id_lower 
ON threads (LOWER(user_id));

CREATE INDEX idx_messages_author_tag_lower 
ON messages (LOWER(author_tag));

CREATE INDEX idx_threads_urgency_created 
ON threads (urgency, created_at DESC) 
WHERE urgency IN ('High', 'Urgent') AND is_open = true;