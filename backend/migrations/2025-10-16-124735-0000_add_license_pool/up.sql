-- ============================================================================
-- Пул лицензий
-- ============================================================================

-- Пулы лицензий
CREATE TABLE license_pools (
    id SERIAL PRIMARY KEY,
    software_catalog_id INTEGER NOT NULL REFERENCES software_catalog(id) ON DELETE CASCADE,
    license_type VARCHAR(50) NOT NULL,
    total_licenses INTEGER NOT NULL,
    used_licenses INTEGER DEFAULT 0,
    available_licenses INTEGER GENERATED ALWAYS AS (total_licenses - used_licenses) STORED,
    license_key VARCHAR(500),
    purchase_date DATE,
    expiration_date DATE,
    cost_per_license DECIMAL(10,2),
    total_cost DECIMAL(15,2),
    vendor_id INTEGER REFERENCES vendors(id) ON DELETE SET NULL,
    contract_id INTEGER REFERENCES contracts(id) ON DELETE SET NULL,
    auto_renewal BOOLEAN DEFAULT false,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_license_type CHECK (license_type IN ('perpetual', 'subscription', 'volume', 'oem', 'trial', 'educational')),
    CONSTRAINT check_licenses CHECK (total_licenses > 0 AND used_licenses >= 0 AND used_licenses <= total_licenses)
);

-- Назначение лицензий
CREATE TABLE license_assignments (
    id SERIAL PRIMARY KEY,
    license_pool_id INTEGER NOT NULL REFERENCES license_pools(id) ON DELETE CASCADE,
    computer_id INTEGER REFERENCES computers(id) ON DELETE CASCADE,
    employee_id INTEGER REFERENCES employees(id) ON DELETE SET NULL,
    assigned_date DATE NOT NULL DEFAULT CURRENT_DATE,
    revoked_date DATE,
    status VARCHAR(20) DEFAULT 'active',
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_assignment_status CHECK (status IN ('active', 'revoked', 'expired')),
    CONSTRAINT check_one_assignment CHECK (
        (computer_id IS NOT NULL AND employee_id IS NULL) OR
        (computer_id IS NULL AND employee_id IS NOT NULL)
    )
);

-- Индексы
CREATE INDEX idx_license_pools_software ON license_pools(software_catalog_id);
CREATE INDEX idx_license_pools_vendor ON license_pools(vendor_id);
CREATE INDEX idx_license_pools_contract ON license_pools(contract_id);
CREATE INDEX idx_license_pools_expiration ON license_pools(expiration_date) WHERE expiration_date IS NOT NULL;
CREATE INDEX idx_license_assignments_pool ON license_assignments(license_pool_id);
CREATE INDEX idx_license_assignments_computer ON license_assignments(computer_id);
CREATE INDEX idx_license_assignments_employee ON license_assignments(employee_id);
CREATE INDEX idx_license_assignments_status ON license_assignments(status);

-- Триггеры
CREATE TRIGGER update_license_pools_updated_at BEFORE UPDATE ON license_pools
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Функция для автоматического обновления счётчика использованных лицензий
CREATE OR REPLACE FUNCTION update_license_pool_usage()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' AND NEW.status = 'active' THEN
        UPDATE license_pools 
        SET used_licenses = used_licenses + 1 
        WHERE id = NEW.license_pool_id;
    ELSIF TG_OP = 'UPDATE' THEN
        IF OLD.status = 'active' AND NEW.status != 'active' THEN
            UPDATE license_pools 
            SET used_licenses = used_licenses - 1 
            WHERE id = OLD.license_pool_id;
        ELSIF OLD.status != 'active' AND NEW.status = 'active' THEN
            UPDATE license_pools 
            SET used_licenses = used_licenses + 1 
            WHERE id = NEW.license_pool_id;
        END IF;
    ELSIF TG_OP = 'DELETE' AND OLD.status = 'active' THEN
        UPDATE license_pools 
        SET used_licenses = used_licenses - 1 
        WHERE id = OLD.license_pool_id;
        RETURN OLD;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_license_pool_usage_trigger
AFTER INSERT OR UPDATE OR DELETE ON license_assignments
FOR EACH ROW EXECUTE FUNCTION update_license_pool_usage();

-- Представление: статус лицензий
CREATE OR REPLACE VIEW v_license_status AS
SELECT 
    lp.id,
    sc.name AS software_name,
    sc.manufacturer,
    lp.license_type,
    lp.total_licenses,
    lp.used_licenses,
    lp.available_licenses,
    lp.expiration_date,
    CASE 
        WHEN lp.expiration_date IS NULL THEN 'perpetual'
        WHEN lp.expiration_date < CURRENT_DATE THEN 'expired'
        WHEN lp.expiration_date < CURRENT_DATE + INTERVAL '30 days' THEN 'expiring_soon'
        ELSE 'valid'
    END AS license_health,
    CASE 
        WHEN lp.available_licenses = 0 THEN 'full'
        WHEN lp.available_licenses <= lp.total_licenses * 0.1 THEN 'low'
        ELSE 'ok'
    END AS availability_status,
    v.name AS vendor_name,
    lp.total_cost
FROM license_pools lp
JOIN software_catalog sc ON lp.software_catalog_id = sc.id
LEFT JOIN vendors v ON lp.vendor_id = v.id;

-- Комментарии
COMMENT ON TABLE license_pools IS 'Пулы лицензий на программное обеспечение';
COMMENT ON TABLE license_assignments IS 'Назначение лицензий компьютерам или сотрудникам';
COMMENT ON VIEW v_license_status IS 'Статус лицензий с оценкой доступности и срока действия';
COMMENT ON COLUMN license_pools.available_licenses IS 'Автоматически вычисляемое поле: total_licenses - used_licenses';
