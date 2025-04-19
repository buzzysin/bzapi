-- PRAGMA FOREIGN_KEYS = on;

-- -- TAGS TABLE (Sqlite)
-- -- + id string
-- -- + name string
-- -- + description string|null
-- -- + created_at datetime
-- -- + updated_at datetime

CREATE TABLE
  tags (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
  );