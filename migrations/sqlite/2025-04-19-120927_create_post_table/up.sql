PRAGMA FOREIGN_KEYS = on;

-- POSTS TABLE (Sqlite)
-- + id string
-- + title string
-- + content string (default empty)
-- + author_id string
-- + created_at datetime
-- + updated_at datetime
CREATE TABLE
  posts (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    content TEXT DEFAULT '',
    draft BOOLEAN NOT NULL DEFAULT 1,
    author_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (author_id) REFERENCES users (id) ON DELETE CASCADE
  );

