-- Откат миграции
DROP VIEW IF EXISTS v_work_time_summary;
DROP TABLE IF EXISTS work_time_tracking CASCADE;
