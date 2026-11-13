-- ============================================================================
-- Отслеживание антивирусного ПО
-- ============================================================================

CREATE TABLE computer_antivirus (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    product_name VARCHAR(255) NOT NULL,
    manufacturer VARCHAR(100),
    version VARCHAR(100),
    status VARCHAR(50) NOT NULL DEFAULT 'unknown',
    is_enabled BOOLEAN DEFAULT true,
    real_time_protection BOOLEAN DEFAULT true,
    last_update TIMESTAMP,
    definitions_version VARCHAR(100),
    definitions_date DATE,
    last_scan TIMESTAMP,
    last_scan_type VARCHAR(50),
    threats_detected INTEGER DEFAULT 0,
    threats_quarantined INTEGER DEFAULT 0,
    license_status VARCHAR(50),
    license_expiry_date DATE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_antivirus_status CHECK (status IN ('enabled', 'disabled', 'not_installed', 'expired', 'updating', 'error')),
    CONSTRAINT check_scan_type CHECK (last_scan_type IN ('quick', 'full', 'custom', 'boot') OR last_scan_type IS NULL),
    CONSTRAINT check_license_status CHECK (license_status IN ('valid', 'expired', 'trial', 'unknown') OR license_status IS NULL),
    UNIQUE(computer_id)
);

-- История сканирований антивируса
CREATE TABLE antivirus_scan_history (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    scan_type VARCHAR(50) NOT NULL,
    started_at TIMESTAMP NOT NULL,
    completed_at TIMESTAMP,
    duration_seconds INTEGER,
    files_scanned INTEGER,
    threats_found INTEGER DEFAULT 0,
    threats_removed INTEGER DEFAULT 0,
    scan_result VARCHAR(50),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_scan_result CHECK (scan_result IN ('clean', 'threats_found', 'threats_removed', 'failed', 'cancelled'))
);

-- Обнаруженные угрозы
CREATE TABLE antivirus_threats (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    threat_name VARCHAR(255) NOT NULL,
    threat_type VARCHAR(50),
    severity VARCHAR(20),
    file_path TEXT,
    detected_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    action_taken VARCHAR(50),
    status VARCHAR(20) DEFAULT 'active',
    resolved_at TIMESTAMP,
    notes TEXT,
    CONSTRAINT check_threat_type CHECK (threat_type IN ('virus', 'trojan', 'worm', 'spyware', 'adware', 'ransomware', 'rootkit', 'pup', 'other')),
    CONSTRAINT check_threat_severity CHECK (severity IN ('low', 'medium', 'high', 'critical')),
    CONSTRAINT check_threat_action CHECK (action_taken IN ('quarantined', 'removed', 'blocked', 'allowed', 'failed') OR action_taken IS NULL),
    CONSTRAINT check_threat_status CHECK (status IN ('active', 'quarantined', 'removed', 'false_positive'))
);

-- Индексы
CREATE INDEX idx_computer_antivirus_computer ON computer_antivirus(computer_id);
CREATE INDEX idx_computer_antivirus_status ON computer_antivirus(status);
CREATE INDEX idx_computer_antivirus_license_expiry ON computer_antivirus(license_expiry_date) WHERE license_expiry_date IS NOT NULL;
CREATE INDEX idx_antivirus_scan_history_computer ON antivirus_scan_history(computer_id, started_at DESC);
CREATE INDEX idx_antivirus_threats_computer ON antivirus_threats(computer_id);
CREATE INDEX idx_antivirus_threats_status ON antivirus_threats(status) WHERE status = 'active';
CREATE INDEX idx_antivirus_threats_severity ON antivirus_threats(severity);

-- Триггеры
CREATE TRIGGER update_computer_antivirus_updated_at BEFORE UPDATE ON computer_antivirus
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Представление: компьютеры с проблемами антивируса
CREATE OR REPLACE VIEW v_antivirus_issues AS
SELECT 
    c.id AS computer_id,
    c.hostname,
    c.inventory_number,
    e.first_name || ' ' || e.last_name AS employee_name,
    ca.product_name,
    ca.status,
    ca.last_update,
    ca.license_expiry_date,
    CASE 
        WHEN ca.status IN ('disabled', 'not_installed') THEN 'critical'
        WHEN ca.status = 'expired' THEN 'high'
        WHEN ca.license_expiry_date < CURRENT_DATE THEN 'high'
        WHEN ca.license_expiry_date < CURRENT_DATE + INTERVAL '30 days' THEN 'medium'
        WHEN ca.last_update < CURRENT_TIMESTAMP - INTERVAL '7 days' THEN 'medium'
        ELSE 'low'
    END AS issue_severity,
    ARRAY_AGG(DISTINCT 
        CASE 
            WHEN ca.status = 'not_installed' THEN 'Антивирус не установлен'
            WHEN ca.status = 'disabled' THEN 'Антивирус отключён'
            WHEN ca.status = 'expired' THEN 'Лицензия истекла'
            WHEN ca.license_expiry_date < CURRENT_DATE THEN 'Лицензия истекла'
            WHEN ca.license_expiry_date < CURRENT_DATE + INTERVAL '30 days' THEN 'Лицензия истекает'
            WHEN ca.last_update < CURRENT_TIMESTAMP - INTERVAL '7 days' THEN 'Давно не обновлялся'
        END
    ) FILTER (WHERE ca.status IS NOT NULL) AS issues
FROM computers c
LEFT JOIN computer_antivirus ca ON c.id = ca.computer_id
LEFT JOIN employees e ON c.employee_id = e.id
WHERE c.status = 'active'
  AND (
    ca.id IS NULL OR
    ca.status IN ('disabled', 'not_installed', 'expired') OR
    ca.license_expiry_date < CURRENT_DATE + INTERVAL '30 days' OR
    ca.last_update < CURRENT_TIMESTAMP - INTERVAL '7 days'
  )
GROUP BY c.id, c.hostname, c.inventory_number, employee_name, ca.product_name, ca.status, ca.last_update, ca.license_expiry_date;

-- Комментарии
COMMENT ON TABLE computer_antivirus IS 'Информация об антивирусном ПО на компьютерах';
COMMENT ON TABLE antivirus_scan_history IS 'История антивирусных сканирований';
COMMENT ON TABLE antivirus_threats IS 'Обнаруженные угрозы';
COMMENT ON VIEW v_antivirus_issues IS 'Компьютеры с проблемами антивирусной защиты';
