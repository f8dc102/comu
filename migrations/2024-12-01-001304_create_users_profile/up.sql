-- create_users_profile, up.sql
CREATE TABLE users_profile (
    uuid UUID PRIMARY KEY UNIQUE DEFAULT gen_random_uuid(),
    user_uuid UUID NOT NULL REFERENCES users (uuid),
    handle VARCHAR(16) NOT NULL,
    username VARCHAR(255) DEFAULT NULL,
    private BOOLEAN DEFAULT FALSE,
    bio TEXT NULL DEFAULT NULL,
    profile_image TEXT NULL DEFAULT NULL,
    cover_image TEXT NULL DEFAULT NULL,
    posts_count INT DEFAULT 0,
    likes_count INT DEFAULT 0,
    comments_count INT DEFAULT 0,
    followers_count INT DEFAULT 0,
    following_count INT DEFAULT 0,
    postings JSONB DEFAULT '[]',
    comments JSONB DEFAULT '[]',
    verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT current_timestamp,
    updated_at TIMESTAMP DEFAULT current_timestamp,
    deleted_at TIMESTAMP NULL DEFAULT NULL
);
