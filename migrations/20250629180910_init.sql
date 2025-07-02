-- PostgreSQL schema for StackScribe server
-- Multi-tenant architecture with user-based data separation

-- Users table - each user has their own data
CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(255) PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP,
    is_active BOOLEAN DEFAULT true
);

-- Archives belong to specific users
CREATE TABLE IF NOT EXISTS archives (
    id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Tomes belong to archives (and indirectly to users)
CREATE TABLE IF NOT EXISTS tomes (
    id VARCHAR(255) PRIMARY KEY,
    archive_id VARCHAR(255) NOT NULL,
    user_id VARCHAR(255) NOT NULL, -- Denormalized for faster queries
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (archive_id) REFERENCES archives(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Entries belong to tomes (and indirectly to users)
CREATE TABLE IF NOT EXISTS entries (
    id VARCHAR(255) PRIMARY KEY,
    tome_id VARCHAR(255) NOT NULL,
    user_id VARCHAR(255) NOT NULL, -- Denormalized for faster queries
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (tome_id) REFERENCES tomes(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Indexes for better performance
CREATE INDEX IF NOT EXISTS idx_archives_user_id ON archives(user_id);
CREATE INDEX IF NOT EXISTS idx_tomes_user_id ON tomes(user_id);
CREATE INDEX IF NOT EXISTS idx_entries_user_id ON entries(user_id);
CREATE INDEX IF NOT EXISTS idx_tomes_archive_id ON tomes(archive_id);
CREATE INDEX IF NOT EXISTS idx_entries_tome_id ON entries(tome_id);
CREATE INDEX IF NOT EXISTS idx_archives_updated_at ON archives(updated_at);
CREATE INDEX IF NOT EXISTS idx_tomes_updated_at ON tomes(updated_at);
CREATE INDEX IF NOT EXISTS idx_entries_updated_at ON entries(updated_at);

-- Combined indexes for sync queries
CREATE INDEX IF NOT EXISTS idx_archives_user_updated ON archives(user_id, updated_at);
CREATE INDEX IF NOT EXISTS idx_tomes_user_updated ON tomes(user_id, updated_at);
CREATE INDEX IF NOT EXISTS idx_entries_user_updated ON entries(user_id, updated_at);

-- Per-user sync metadata table for server-side tracking
CREATE TABLE IF NOT EXISTS user_sync_metadata (
    user_id VARCHAR(255) NOT NULL,
    key VARCHAR(255) NOT NULL,
    value TEXT NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, key),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
