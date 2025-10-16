-- Откат миграции
DROP VIEW IF EXISTS v_antivirus_issues;
DROP TABLE IF EXISTS antivirus_threats CASCADE;
DROP TABLE IF EXISTS antivirus_scan_history CASCADE;
DROP TABLE IF EXISTS computer_antivirus CASCADE;
