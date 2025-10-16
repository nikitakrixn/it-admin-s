-- Откат изменений
DROP TRIGGER IF EXISTS update_network_segments_updated_at ON network_segments;
DROP TRIGGER IF EXISTS update_ip_addresses_updated_at ON ip_addresses;
DROP TRIGGER IF EXISTS update_network_devices_updated_at ON network_devices;
DROP TRIGGER IF EXISTS update_network_device_ports_updated_at ON network_device_ports;

DROP TABLE IF EXISTS network_scans;
DROP TABLE IF EXISTS network_device_ports;
DROP TABLE IF EXISTS network_devices;
DROP TABLE IF EXISTS ip_addresses;
DROP TABLE IF EXISTS network_segments;
