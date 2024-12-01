-- create_comments, up.sql
CREATE TABLE comments (
    uuid UUID PRIMARY KEY UNIQUE DEFAULT gen_random_uuid(),
    content TEXT NOT NULL,
    post_id UUID NOT NULL REFERENCES posts (uuid) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users (uuid) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);
