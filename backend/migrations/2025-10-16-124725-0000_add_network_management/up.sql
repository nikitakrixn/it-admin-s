-- ============================================================================
-- Управление сетью
-- ============================================================================

-- Сетевые сегменты (подсети, VLAN)
CREATE TABLE network_segments (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    network_address VARCHAR(45) NOT NULL,
    subnet_mask VARCHAR(45) NOT NULL,
    gateway VARCHAR(45),
    vlan_id INTEGER,
    dns_servers TEXT,
    dhcp_enabled BOOLEAN DEFAULT false,
    dhcp_range_start VARCHAR(45),
    dhcp_range_end VARCHAR(45),
    description TEXT,
    location VARCHAR(100),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Пул IP-адресов
CREATE TABLE ip_addresses (
    id SERIAL PRIMARY KEY,
    network_segment_id INTEGER REFERENCES network_segments(id) ON DELETE CASCADE,
    ip_address VARCHAR(45) NOT NULL UNIQUE,
    status VARCHAR(20) DEFAULT 'available',
    assigned_to_type VARCHAR(50),
    assigned_to_id INTEGER,
    hostname VARCHAR(255),
    mac_address VARCHAR(17),
    is_reserved BOOLEAN DEFAULT false,
    reservation_reason TEXT,
    last_seen TIMESTAMP,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_ip_status CHECK (status IN ('available', 'assigned', 'reserved', 'offline', 'conflict'))
);

-- Сетевые устройства (роутеры, свитчи, точки доступа)
CREATE TABLE network_devices (
    id SERIAL PRIMARY KEY,
    device_type VARCHAR(50) NOT NULL,
    name VARCHAR(255) NOT NULL,
    manufacturer VARCHAR(100),
    model VARCHAR(100),
    serial_number VARCHAR(100) UNIQUE,
    ip_address VARCHAR(45),
    mac_address VARCHAR(17),
    management_url VARCHAR(255),
    location VARCHAR(100),
    firmware_version VARCHAR(100),
    port_count INTEGER,
    status VARCHAR(20) DEFAULT 'active',
    purchase_date DATE,
    warranty_end_date DATE,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_device_type CHECK (device_type IN ('router', 'switch', 'access_point', 'firewall', 'load_balancer', 'vpn_gateway')),
    CONSTRAINT check_device_status CHECK (status IN ('active', 'inactive', 'maintenance', 'failed'))
);

-- Порты сетевых устройств
CREATE TABLE network_device_ports (
    id SERIAL PRIMARY KEY,
    device_id INTEGER NOT NULL REFERENCES network_devices(id) ON DELETE CASCADE,
    port_number INTEGER NOT NULL,
    port_name VARCHAR(50),
    port_type VARCHAR(50),
    speed VARCHAR(20),
    status VARCHAR(20) DEFAULT 'down',
    vlan_id INTEGER,
    connected_device_type VARCHAR(50),
    connected_device_id INTEGER,
    mac_address VARCHAR(17),
    description TEXT,
    is_uplink BOOLEAN DEFAULT false,
    last_activity TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_port_status CHECK (status IN ('up', 'down', 'disabled', 'error')),
    UNIQUE(device_id, port_number)
);

-- История сканирования сети
CREATE TABLE network_scans (
    id SERIAL PRIMARY KEY,
    scan_type VARCHAR(50) NOT NULL,
    network_segment_id INTEGER REFERENCES network_segments(id) ON DELETE SET NULL,
    ip_range VARCHAR(100),
    devices_found INTEGER DEFAULT 0,
    scan_duration INTEGER,
    scan_results JSONB,
    initiated_by UUID REFERENCES users(id) ON DELETE SET NULL,
    started_at TIMESTAMP NOT NULL,
    completed_at TIMESTAMP,
    status VARCHAR(20) DEFAULT 'running',
    error_message TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_scan_type CHECK (scan_type IN ('ping_sweep', 'port_scan', 'full_scan', 'arp_scan')),
    CONSTRAINT check_scan_status CHECK (status IN ('running', 'completed', 'failed', 'cancelled'))
);

-- Индексы
CREATE INDEX idx_network_segments_vlan ON network_segments(vlan_id);
CREATE INDEX idx_ip_addresses_segment ON ip_addresses(network_segment_id);
CREATE INDEX idx_ip_addresses_status ON ip_addresses(status);
CREATE INDEX idx_ip_addresses_mac ON ip_addresses(mac_address);
CREATE INDEX idx_network_devices_type ON network_devices(device_type);
CREATE INDEX idx_network_devices_status ON network_devices(status);
CREATE INDEX idx_network_devices_ip ON network_devices(ip_address);
CREATE INDEX idx_network_device_ports_device ON network_device_ports(device_id);
CREATE INDEX idx_network_device_ports_status ON network_device_ports(status);
CREATE INDEX idx_network_scans_segment ON network_scans(network_segment_id);
CREATE INDEX idx_network_scans_started ON network_scans(started_at DESC);

-- Триггеры
CREATE TRIGGER update_network_segments_updated_at BEFORE UPDATE ON network_segments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_ip_addresses_updated_at BEFORE UPDATE ON ip_addresses
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_network_devices_updated_at BEFORE UPDATE ON network_devices
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_network_device_ports_updated_at BEFORE UPDATE ON network_device_ports
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Комментарии
COMMENT ON TABLE network_segments IS 'Сетевые сегменты (подсети, VLAN)';
COMMENT ON TABLE ip_addresses IS 'Пул IP-адресов с отслеживанием использования';
COMMENT ON TABLE network_devices IS 'Сетевое оборудование (роутеры, свитчи, точки доступа)';
COMMENT ON TABLE network_device_ports IS 'Порты сетевых устройств';
COMMENT ON TABLE network_scans IS 'История сканирования сети';
