-- Откат изменений
DROP INDEX IF EXISTS idx_computers_is_online;
DROP INDEX IF EXISTS idx_computers_last_seen;

ALTER TABLE computers 
    DROP COLUMN IF EXISTS last_seen_online,
    DROP COLUMN IF EXISTS is_online,
    DROP COLUMN IF EXISTS last_user_logged,
    DROP COLUMN IF EXISTS bitlocker_enabled,
    DROP COLUMN IF EXISTS antivirus_status,
    DROP COLUMN IF EXISTS windows_activation_status;
