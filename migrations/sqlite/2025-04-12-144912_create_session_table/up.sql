PRAGMA FOREIGN_KEYS = on;

-- SESSIONS TABLE (Sqlite)
-- + id string 
-- + user_id string
-- + token string
-- + expires_at datetime
-- + created_at datetime
-- + updated_at datetime
CREATE TABLE
  sessions (
    id TEXT PRIMARY KEY NOT NULL,
    token TEXT NOT NULL,
    expires_at DATETIME NOT NULL,
    user_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
  );

-- + Unique (user_id, token) index
CREATE UNIQUE INDEX idx_sessions_user_id_token ON sessions (user_id, token);