-- Откат изменений
DROP VIEW IF EXISTS v_latest_backups;

DROP TRIGGER IF EXISTS update_backup_policies_updated_at ON backup_policies;

DROP TABLE IF EXISTS backup_history;
DROP TABLE IF EXISTS backup_jobs;
DROP TABLE IF EXISTS backup_policies;
