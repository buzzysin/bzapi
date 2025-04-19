-- Down.sql for sessions table
DROP TABLE IF EXISTS verification_tokens;

-- Indexes
DROP INDEX IF EXISTS idx_verification_tokens_email_token;