-- Up.sql for users table
PRAGMA foreign_keys = on;

-- USERS TABLE
-- + id string
-- + name string|null
-- + email string|null
-- + email_verified datetime|null
-- + image string|null
-- + created_at datetime
-- + updated_at datetime
CREATE TABLE
  users (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT,
    email TEXT,
    email_verified DATETIME,
    image TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
  );