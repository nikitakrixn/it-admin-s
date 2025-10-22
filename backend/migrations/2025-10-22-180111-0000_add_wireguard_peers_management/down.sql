-- Удаление таблиц WireGuard в обратном порядке
DROP TABLE IF EXISTS wireguard_config_downloads;
DROP TABLE IF EXISTS wireguard_connection_history;
DROP TABLE IF EXISTS wireguard_peers;
DROP TABLE IF EXISTS wireguard_interfaces;
