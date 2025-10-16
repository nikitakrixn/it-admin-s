-- ============================================================================
-- VPN подключения
-- ============================================================================

CREATE TABLE vpn_connections (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL REFERENCES employees(id) ON DELETE CASCADE,
    username VARCHAR(255) NOT NULL,
    connection_name VARCHAR(255),
    vpn_server VARCHAR(255),
    vpn_protocol VARCHAR(50),
    client_ip VARCHAR(45),
    assigned_ip VARCHAR(45),
    connected_at TIMESTAMP NOT NULL,
    disconnected_at TIMESTAMP,
    session_duration_seconds INTEGER,
    bytes_sent BIGINT DEFAULT 0,
    bytes_received BIGINT DEFAULT 0,
    disconnect_reason VARCHAR(100),
    status VARCHAR(20) DEFAULT 'active',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_vpn_protocol CHECK (vpn_protocol IN ('PPTP', 'L2TP', 'SSTP', 'IKEv2', 'OpenVPN', 'WireGuard') OR vpn_protocol IS NULL),
    CONSTRAINT check_vpn_status CHECK (status IN ('active', 'disconnected', 'failed'))
);

-- Конфигурации VPN
CREATE TABLE vpn_configurations (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    server_address VARCHAR(255) NOT NULL,
    protocol VARCHAR(50) NOT NULL,
    port INTEGER,
    encryption VARCHAR(50),
    authentication_method VARCHAR(50),
    split_tunneling BOOLEAN DEFAULT false,
    dns_servers TEXT,
    is_active BOOLEAN DEFAULT true,
    max_concurrent_connections INTEGER,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Доступ к VPN
CREATE TABLE vpn_access (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL REFERENCES employees(id) ON DELETE CASCADE,
    vpn_config_id INTEGER NOT NULL REFERENCES vpn_configurations(id) ON DELETE CASCADE,
    granted_at DATE NOT NULL DEFAULT CURRENT_DATE,
    expires_at DATE,
    granted_by UUID REFERENCES users(id) ON DELETE SET NULL,
    revoked_at DATE,
    revoked_by UUID REFERENCES users(id) ON DELETE SET NULL,
    revoke_reason TEXT,
    is_active BOOLEAN DEFAULT true,
    notes TEXT,
    UNIQUE(employee_id, vpn_config_id)
);

-- Индексы
CREATE INDEX idx_vpn_connections_employee ON vpn_connections(employee_id);
CREATE INDEX idx_vpn_connections_connected ON vpn_connections(connected_at DESC);
CREATE INDEX idx_vpn_connections_status ON vpn_connections(status);
CREATE INDEX idx_vpn_configurations_active ON vpn_configurations(is_active) WHERE is_active = true;
CREATE INDEX idx_vpn_access_employee ON vpn_access(employee_id);
CREATE INDEX idx_vpn_access_config ON vpn_access(vpn_config_id);
CREATE INDEX idx_vpn_access_active ON vpn_access(is_active) WHERE is_active = true;

-- Триггеры
CREATE TRIGGER update_vpn_configurations_updated_at BEFORE UPDATE ON vpn_configurations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Представление: активные VPN сессии
CREATE OR REPLACE VIEW v_active_vpn_sessions AS
SELECT 
    vc.id,
    e.first_name || ' ' || e.last_name AS employee_name,
    e.email,
    d.name AS department_name,
    vc.vpn_server,
    vc.client_ip,
    vc.assigned_ip,
    vc.connected_at,
    EXTRACT(EPOCH FROM (CURRENT_TIMESTAMP - vc.connected_at))::INTEGER AS session_duration_seconds,
    vc.bytes_sent,
    vc.bytes_received
FROM vpn_connections vc
JOIN employees e ON vc.employee_id = e.id
LEFT JOIN departments d ON e.department_id = d.id
WHERE vc.status = 'active'
ORDER BY vc.connected_at DESC;

-- Комментарии
COMMENT ON TABLE vpn_connections IS 'История VPN подключений';
COMMENT ON TABLE vpn_configurations IS 'Конфигурации VPN серверов';
COMMENT ON TABLE vpn_access IS 'Права доступа к VPN';
COMMENT ON VIEW v_active_vpn_sessions IS 'Активные VPN сессии';
