-- ============================================================================
-- Управление сертификатами
-- ============================================================================

CREATE TABLE certificates (
    id SERIAL PRIMARY KEY,
    certificate_type VARCHAR(50) NOT NULL,
    subject VARCHAR(500) NOT NULL,
    issuer VARCHAR(500),
    serial_number VARCHAR(100),
    thumbprint VARCHAR(100) UNIQUE,
    algorithm VARCHAR(50),
    key_size INTEGER,
    valid_from DATE NOT NULL,
    valid_to DATE NOT NULL,
    computer_id INTEGER REFERENCES computers(id) ON DELETE CASCADE,
    store_location VARCHAR(50),
    store_name VARCHAR(50),
    has_private_key BOOLEAN DEFAULT false,
    purpose TEXT,
    san_entries TEXT,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_certificate_type CHECK (certificate_type IN (
        'ssl_tls', 'code_signing', 'email', 'user', 'computer', 
        'ca', 'root', 'intermediate', 'self_signed', 'other'
    )),
    CONSTRAINT check_store_location CHECK (store_location IN ('CurrentUser', 'LocalMachine') OR store_location IS NULL),
    CONSTRAINT check_dates CHECK (valid_to >= valid_from)
);

-- История обновления сертификатов
CREATE TABLE certificate_renewals (
    id SERIAL PRIMARY KEY,
    old_certificate_id INTEGER REFERENCES certificates(id) ON DELETE SET NULL,
    new_certificate_id INTEGER REFERENCES certificates(id) ON DELETE SET NULL,
    renewed_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    renewed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    notes TEXT
);

-- Индексы
CREATE INDEX idx_certificates_type ON certificates(certificate_type);
CREATE INDEX idx_certificates_computer ON certificates(computer_id);
CREATE INDEX idx_certificates_expiry ON certificates(valid_to);
CREATE INDEX idx_certificates_thumbprint ON certificates(thumbprint);
CREATE INDEX idx_certificate_renewals_old ON certificate_renewals(old_certificate_id);
CREATE INDEX idx_certificate_renewals_new ON certificate_renewals(new_certificate_id);

-- Триггеры
CREATE TRIGGER update_certificates_updated_at BEFORE UPDATE ON certificates
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Представление: сертификаты требующие внимания
CREATE OR REPLACE VIEW v_certificates_expiring AS
SELECT 
    cert.id,
    cert.certificate_type,
    cert.subject,
    cert.issuer,
    cert.valid_from,
    cert.valid_to,
    cert.valid_to - CURRENT_DATE AS days_until_expiry,
    c.hostname,
    c.inventory_number,
    e.first_name || ' ' || e.last_name AS responsible_person,
    CASE 
        WHEN cert.valid_to < CURRENT_DATE THEN 'expired'
        WHEN cert.valid_to - CURRENT_DATE <= 7 THEN 'critical'
        WHEN cert.valid_to - CURRENT_DATE <= 30 THEN 'warning'
        WHEN cert.valid_to - CURRENT_DATE <= 60 THEN 'notice'
        ELSE 'ok'
    END AS urgency_level
FROM certificates cert
LEFT JOIN computers c ON cert.computer_id = c.id
LEFT JOIN employees e ON c.employee_id = e.id
WHERE cert.valid_to <= CURRENT_DATE + INTERVAL '60 days'
ORDER BY cert.valid_to;

-- Комментарии
COMMENT ON TABLE certificates IS 'Цифровые сертификаты';
COMMENT ON TABLE certificate_renewals IS 'История обновления сертификатов';
COMMENT ON VIEW v_certificates_expiring IS 'Сертификаты, истекающие в ближайшие 60 дней';
