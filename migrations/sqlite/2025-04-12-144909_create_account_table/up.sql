PRAGMA foreign_keys = on;

-- ACCOUNTS TABLE (Sqlite)
-- + id string
-- + provider_id string
-- + provider_account_id string
-- + user_id string
-- + provider_type string
-- + refresh_token string|null
-- + access_token string|null
-- + expires_at datetime|null
-- + token_type string|null
-- + scope string|null
-- + id_token string|null
-- + session_state string|null
-- + created_at datetime
-- + updated_at datetime
CREATE TABLE
  accounts (
    id TEXT PRIMARY KEY NOT NULL,
    provider_id TEXT NOT NULL,
    provider_account_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    provider_type TEXT NOT NULL,
    refresh_token TEXT,
    access_token TEXT,
    expires_at DATETIME,
    token_type TEXT,
    scope TEXT,
    id_token TEXT,
    session_state TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
  );

-- Unique (provider_id, provider_account_id) index
CREATE UNIQUE INDEX idx_accounts_provider_id_provider_account_id ON accounts (provider_id, provider_account_id);