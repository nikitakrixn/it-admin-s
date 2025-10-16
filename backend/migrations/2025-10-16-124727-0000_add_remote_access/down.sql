-- Откат изменений
DROP TRIGGER IF EXISTS update_remote_credentials_updated_at ON remote_access_credentials;

DROP TABLE IF EXISTS remote_access_sessions;
DROP TABLE IF EXISTS remote_access_credentials;
