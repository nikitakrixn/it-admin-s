-- ============================================================================
-- Удалённый доступ
-- ============================================================================

-- Учётные данные для удалённого доступа
CREATE TABLE remote_access_credentials (
    id SERIAL PRIMARY KEY,
    entity_type VARCHAR(50) NOT NULL,
    entity_id INTEGER NOT NULL,
    access_type VARCHAR(50) NOT NULL,
    hostname VARCHAR(255),
    port INTEGER,
    username VARCHAR(255),
    password_encrypted TEXT,
    private_key_path VARCHAR(500),
    connection_string TEXT,
    notes TEXT,
    is_active BOOLEAN DEFAULT true,
    last_used_at TIMESTAMP,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_access_type CHECK (access_type IN ('rdp', 'ssh', 'vnc', 'teamviewer', 'anydesk', 'ipmi', 'ilo', 'idrac', 'web_interface', 'vpn'))
);

-- История подключений
CREATE TABLE remote_access_sessions (
    id SERIAL PRIMARY KEY,
    credential_id INTEGER REFERENCES remote_access_credentials(id) ON DELETE SET NULL,
    entity_type VARCHAR(50) NOT NULL,
    entity_id INTEGER NOT NULL,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    access_type VARCHAR(50) NOT NULL,
    client_ip VARCHAR(45),
    session_started_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    session_ended_at TIMESTAMP,
    duration_seconds INTEGER,
    status VARCHAR(20) DEFAULT 'active',
    disconnect_reason VARCHAR(100),
    notes TEXT,
    CONSTRAINT check_session_status CHECK (status IN ('active', 'completed', 'failed', 'timeout'))
);

-- Индексы
CREATE INDEX idx_remote_credentials_entity ON remote_access_credentials(entity_type, entity_id);
CREATE INDEX idx_remote_credentials_type ON remote_access_credentials(access_type);
CREATE INDEX idx_remote_credentials_active ON remote_access_credentials(is_active) WHERE is_active = true;
CREATE INDEX idx_remote_sessions_credential ON remote_access_sessions(credential_id);
CREATE INDEX idx_remote_sessions_entity ON remote_access_sessions(entity_type, entity_id);
CREATE INDEX idx_remote_sessions_user ON remote_access_sessions(user_id);
CREATE INDEX idx_remote_sessions_started ON remote_access_sessions(session_started_at DESC);

-- Триггер
CREATE TRIGGER update_remote_credentials_updated_at BEFORE UPDATE ON remote_access_credentials
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Комментарии
COMMENT ON TABLE remote_access_credentials IS 'Учётные данные для удалённого доступа к компьютерам и оборудованию';
COMMENT ON TABLE remote_access_sessions IS 'История сеансов удалённого доступа';
COMMENT ON COLUMN remote_access_credentials.entity_type IS 'Тип объекта: computer, equipment, network_device';
COMMENT ON COLUMN remote_access_credentials.password_encrypted IS 'Зашифрованный пароль (использовать шифрование на уровне приложения)';
