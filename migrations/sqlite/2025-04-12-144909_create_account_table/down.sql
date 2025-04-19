-- Down.sql for accounts table
DROP TABLE IF EXISTS accounts;

-- Indexes
DROP INDEX IF EXISTS idx_accounts_provider_id_provider_account_id;