ALTER TABLE messages SET (
    autovacuum_vacuum_scale_factor = 0.1,
    autovacuum_analyze_scale_factor = 0.05
);

ALTER TABLE threads SET (
    autovacuum_vacuum_scale_factor = 0.1,
    autovacuum_analyze_scale_factor = 0.05
);

ALTER TABLE threads SET (fillfactor = 90);
ALTER TABLE messages SET (fillfactor = 95);

CREATE OR REPLACE FUNCTION update_table_stats()
RETURNS TRIGGER AS $$
BEGIN
    -- 1% chance to update statistics on insert
    IF TG_OP = 'INSERT' AND random() < 0.01 THEN
        IF TG_TABLE_NAME = 'threads' THEN
            ANALYZE threads;
        ELSIF TG_TABLE_NAME = 'messages' THEN
            ANALYZE messages;
        END IF;
    END IF;
    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_stats_threads
    AFTER INSERT ON threads
    FOR EACH ROW
    EXECUTE FUNCTION update_table_stats();

CREATE TRIGGER update_stats_messages
    AFTER INSERT ON messages
    FOR EACH ROW
    EXECUTE FUNCTION update_table_stats();