-- ============================================================================
-- WireGuard Peers Management
-- ============================================================================

-- Таблица WireGuard интерфейсов на MikroTik
CREATE TABLE wireguard_interfaces (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    listen_port INTEGER NOT NULL,
    public_key TEXT NOT NULL,
    private_key TEXT,
    mtu INTEGER DEFAULT 1420,
    is_active BOOLEAN DEFAULT true,
    mikrotik_id VARCHAR(100),
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_listen_port CHECK (listen_port > 0 AND listen_port <= 65535)
);

-- Таблица WireGuard пиров (клиентов)
CREATE TABLE wireguard_peers (
    id SERIAL PRIMARY KEY,
    interface_id INTEGER NOT NULL REFERENCES wireguard_interfaces(id) ON DELETE CASCADE,
    employee_id INTEGER REFERENCES employees(id) ON DELETE SET NULL,
    name VARCHAR(100) NOT NULL,
    public_key TEXT NOT NULL UNIQUE,
    private_key TEXT,
    preshared_key TEXT,
    allowed_ips TEXT NOT NULL DEFAULT '0.0.0.0/0,::/0',
    client_address VARCHAR(50) NOT NULL,
    client_dns VARCHAR(255),
    endpoint_address VARCHAR(255),
    endpoint_port INTEGER,
    persistent_keepalive INTEGER DEFAULT 25,
    mikrotik_peer_id VARCHAR(100),
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    last_handshake TIMESTAMP,
    rx_bytes BIGINT DEFAULT 0,
    tx_bytes BIGINT DEFAULT 0,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    CONSTRAINT check_peer_status CHECK (status IN ('active', 'disabled', 'expired', 'revoked')),
    CONSTRAINT check_endpoint_port CHECK (endpoint_port IS NULL OR (endpoint_port > 0 AND endpoint_port <= 65535)),
    CONSTRAINT check_keepalive CHECK (persistent_keepalive >= 0 AND persistent_keepalive <= 65535)
);

-- Таблица истории подключений WireGuard
CREATE TABLE wireguard_connection_history (
    id SERIAL PRIMARY KEY,
    peer_id INTEGER NOT NULL REFERENCES wireguard_peers(id) ON DELETE CASCADE,
    connected_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    disconnected_at TIMESTAMP,
    client_ip VARCHAR(45),
    endpoint_ip VARCHAR(45),
    rx_bytes BIGINT DEFAULT 0,
    tx_bytes BIGINT DEFAULT 0,
    duration_seconds INTEGER
);

-- Таблица конфигурационных файлов
CREATE TABLE wireguard_config_downloads (
    id SERIAL PRIMARY KEY,
    peer_id INTEGER NOT NULL REFERENCES wireguard_peers(id) ON DELETE CASCADE,
    downloaded_by UUID REFERENCES users(id) ON DELETE SET NULL,
    downloaded_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ip_address VARCHAR(45),
    user_agent TEXT
);

-- Индексы для производительности
CREATE INDEX idx_wireguard_peers_interface ON wireguard_peers(interface_id);
CREATE INDEX idx_wireguard_peers_employee ON wireguard_peers(employee_id);
CREATE INDEX idx_wireguard_peers_status ON wireguard_peers(status);
CREATE INDEX idx_wireguard_peers_mikrotik_id ON wireguard_peers(mikrotik_peer_id);
CREATE INDEX idx_wireguard_connection_history_peer ON wireguard_connection_history(peer_id);
CREATE INDEX idx_wireguard_connection_history_connected_at ON wireguard_connection_history(connected_at DESC);
CREATE INDEX idx_wireguard_config_downloads_peer ON wireguard_config_downloads(peer_id);

-- Комментарии к таблицам
COMMENT ON TABLE wireguard_interfaces IS 'WireGuard интерфейсы на MikroTik роутере';
COMMENT ON TABLE wireguard_peers IS 'WireGuard пиры (клиенты VPN)';
COMMENT ON TABLE wireguard_connection_history IS 'История подключений WireGuard пиров';
COMMENT ON TABLE wireguard_config_downloads IS 'Журнал скачиваний конфигурационных файлов';
