-- Откат миграции
DROP VIEW IF EXISTS v_active_vpn_sessions;
DROP TABLE IF EXISTS vpn_access CASCADE;
DROP TABLE IF EXISTS vpn_configurations CASCADE;
DROP TABLE IF EXISTS vpn_connections CASCADE;
