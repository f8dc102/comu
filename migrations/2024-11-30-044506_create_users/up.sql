-- up.sql
CREATE TABLE users (
    uuid UUID PRIMARY KEY UNIQUE,
    email VARCHAR(320) UNIQUE NOT NULL,
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT current_timestamp,
    updated_at TIMESTAMP DEFAULT current_timestamp,
    deleted_at TIMESTAMP NULL DEFAULT NULL
);
