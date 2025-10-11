PRAGMA FOREIGN_KEYS = on;

-- COMMENTS TABLE (Sqlite)
-- + id string
-- + content string
-- + post_id string
-- + author_id string
-- + parent_id string|null
-- + created_at datetime
-- + updated_at datetime
CREATE TABLE
  comments (
    id TEXT PRIMARY KEY NOT NULL,
    content TEXT NOT NULL,
    post_id TEXT NOT NULL,
    author_id TEXT NOT NULL,
    parent_id TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (post_id) REFERENCES posts (id) ON DELETE CASCADE,
    FOREIGN KEY (author_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES comments (id) ON DELETE CASCADE
  );