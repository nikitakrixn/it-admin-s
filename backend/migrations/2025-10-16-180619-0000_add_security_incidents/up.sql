-- ============================================================================
-- Инциденты безопасности
-- ============================================================================

CREATE TABLE security_incidents (
    id SERIAL PRIMARY KEY,
    incident_number VARCHAR(50) UNIQUE NOT NULL,
    incident_type VARCHAR(50) NOT NULL,
    severity VARCHAR(20) NOT NULL,
    computer_id INTEGER REFERENCES computers(id) ON DELETE SET NULL,
    employee_id INTEGER REFERENCES employees(id) ON DELETE SET NULL,
    detected_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    detected_by VARCHAR(100),
    resolved_at TIMESTAMP,
    resolved_by UUID REFERENCES users(id) ON DELETE SET NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    actions_taken TEXT,
    root_cause TEXT,
    status VARCHAR(20) DEFAULT 'open',
    impact_level VARCHAR(20),
    affected_systems TEXT,
    data_compromised BOOLEAN DEFAULT false,
    reported_to_authorities BOOLEAN DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_incident_type CHECK (incident_type IN (
        'malware', 'ransomware', 'phishing', 'unauthorized_access', 
        'data_leak', 'ddos', 'brute_force', 'insider_threat', 
        'policy_violation', 'suspicious_activity', 'other'
    )),
    CONSTRAINT check_severity CHECK (severity IN ('low', 'medium', 'high', 'critical')),
    CONSTRAINT check_incident_status CHECK (status IN ('open', 'investigating', 'contained', 'resolved', 'closed', 'false_positive')),
    CONSTRAINT check_impact_level CHECK (impact_level IN ('none', 'minimal', 'moderate', 'significant', 'severe') OR impact_level IS NULL)
);

-- Связанные файлы/доказательства
CREATE TABLE incident_evidence (
    id SERIAL PRIMARY KEY,
    incident_id INTEGER NOT NULL REFERENCES security_incidents(id) ON DELETE CASCADE,
    evidence_type VARCHAR(50),
    file_name VARCHAR(255),
    file_path VARCHAR(500),
    file_hash VARCHAR(128),
    description TEXT,
    collected_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    collected_by UUID REFERENCES users(id) ON DELETE SET NULL,
    CONSTRAINT check_evidence_type CHECK (evidence_type IN ('log', 'screenshot', 'memory_dump', 'network_capture', 'file_sample', 'other'))
);

-- Индексы
CREATE INDEX idx_security_incidents_type ON security_incidents(incident_type);
CREATE INDEX idx_security_incidents_severity ON security_incidents(severity);
CREATE INDEX idx_security_incidents_status ON security_incidents(status);
CREATE INDEX idx_security_incidents_computer ON security_incidents(computer_id);
CREATE INDEX idx_security_incidents_employee ON security_incidents(employee_id);
CREATE INDEX idx_security_incidents_detected ON security_incidents(detected_at DESC);
CREATE INDEX idx_incident_evidence_incident ON incident_evidence(incident_id);

-- Триггеры
CREATE TRIGGER update_security_incidents_updated_at BEFORE UPDATE ON security_incidents
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Функция для генерации номера инцидента
CREATE OR REPLACE FUNCTION generate_incident_number()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.incident_number IS NULL THEN
        NEW.incident_number := 'INC-' || TO_CHAR(CURRENT_DATE, 'YYYYMMDD') || '-' || 
                               LPAD(NEXTVAL('security_incidents_id_seq')::TEXT, 4, '0');
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER generate_incident_number_trigger
BEFORE INSERT ON security_incidents
FOR EACH ROW EXECUTE FUNCTION generate_incident_number();

-- Комментарии
COMMENT ON TABLE security_incidents IS 'Инциденты информационной безопасности';
COMMENT ON TABLE incident_evidence IS 'Доказательства и файлы, связанные с инцидентами';
