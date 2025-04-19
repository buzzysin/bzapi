PRAGMA FOREIGN_KEYS = on;

-- VERIFICATION_TOKENS TABLE (Sqlite)
-- + id string 
-- + email string
-- + token string
-- + expires_at datetime
CREATE TABLE
  verification_tokens (
    id TEXT PRIMARY KEY NOT NULL,
    email TEXT NOT NULL,
    token TEXT NOT NULL,
    expires_at DATETIME NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
  );

-- + Unique (email, token) index
CREATE UNIQUE INDEX idx_verification_tokens_email_token ON verification_tokens (email, token);