-- Откат изменений
DROP VIEW IF EXISTS v_upcoming_maintenance;

DROP TRIGGER IF EXISTS update_next_maintenance_trigger ON maintenance_history;
DROP FUNCTION IF EXISTS update_next_maintenance_date();

DROP TRIGGER IF EXISTS update_maintenance_schedules_updated_at ON maintenance_schedules;

DROP TABLE IF EXISTS maintenance_history;
DROP TABLE IF EXISTS maintenance_schedules;
