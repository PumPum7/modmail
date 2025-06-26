CREATE TABLE thread_messages (
    thread_id INTEGER REFERENCES threads(id),
    message_id UUID REFERENCES messages(id),
    PRIMARY KEY (thread_id, message_id)
);
