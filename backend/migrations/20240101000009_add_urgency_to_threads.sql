-- Add urgency column to threads table
ALTER TABLE threads ADD COLUMN urgency VARCHAR(20) DEFAULT 'Medium';

-- Add index for urgency filtering
CREATE INDEX idx_threads_urgency ON threads(urgency);
