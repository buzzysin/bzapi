-- Down.sql for sessions table
DROP TABLE IF EXISTS sessions;

-- Indexes
DROP INDEX IF EXISTS idx_sessions_user_id_token;