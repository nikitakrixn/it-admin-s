-- ============================================================================
-- Контракты и поставщики
-- ============================================================================

-- Поставщики
CREATE TABLE vendors (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    legal_name VARCHAR(255),
    vendor_type VARCHAR(50),
    inn VARCHAR(20),
    kpp VARCHAR(20),
    contact_person VARCHAR(255),
    email VARCHAR(255),
    phone VARCHAR(50),
    website VARCHAR(255),
    address TEXT,
    rating INTEGER,
    is_active BOOLEAN DEFAULT true,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_vendor_type CHECK (vendor_type IN ('hardware', 'software', 'service', 'telecom', 'other')),
    CONSTRAINT check_rating CHECK (rating >= 1 AND rating <= 5)
);

-- Контракты
CREATE TABLE contracts (
    id SERIAL PRIMARY KEY,
    vendor_id INTEGER REFERENCES vendors(id) ON DELETE SET NULL,
    contract_number VARCHAR(100) NOT NULL UNIQUE,
    contract_type VARCHAR(50) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    start_date DATE NOT NULL,
    end_date DATE,
    contract_value DECIMAL(15,2),
    currency VARCHAR(3) DEFAULT 'RUB',
    payment_terms TEXT,
    auto_renewal BOOLEAN DEFAULT false,
    status VARCHAR(20) DEFAULT 'active',
    responsible_employee_id INTEGER REFERENCES employees(id) ON DELETE SET NULL,
    document_path VARCHAR(500),
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_contract_type CHECK (contract_type IN ('purchase', 'lease', 'service', 'support', 'license', 'maintenance', 'other')),
    CONSTRAINT check_contract_status CHECK (status IN ('draft', 'active', 'expired', 'terminated', 'renewed'))
);

-- Связь контрактов с оборудованием
CREATE TABLE contract_assets (
    id SERIAL PRIMARY KEY,
    contract_id INTEGER NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    asset_type VARCHAR(50) NOT NULL,
    asset_id INTEGER NOT NULL,
    quantity INTEGER DEFAULT 1,
    unit_price DECIMAL(10,2),
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(contract_id, asset_type, asset_id)
);

-- Индексы
CREATE INDEX idx_vendors_type ON vendors(vendor_type);
CREATE INDEX idx_vendors_active ON vendors(is_active) WHERE is_active = true;
CREATE INDEX idx_contracts_vendor ON contracts(vendor_id);
CREATE INDEX idx_contracts_status ON contracts(status);
CREATE INDEX idx_contracts_end_date ON contracts(end_date) WHERE status = 'active';
CREATE INDEX idx_contracts_responsible ON contracts(responsible_employee_id);
CREATE INDEX idx_contract_assets_contract ON contract_assets(contract_id);
CREATE INDEX idx_contract_assets_asset ON contract_assets(asset_type, asset_id);

-- Триггеры
CREATE TRIGGER update_vendors_updated_at BEFORE UPDATE ON vendors
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_contracts_updated_at BEFORE UPDATE ON contracts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Представление: истекающие контракты
CREATE OR REPLACE VIEW v_expiring_contracts AS
SELECT 
    c.id,
    c.contract_number,
    c.title,
    c.contract_type,
    c.end_date,
    c.contract_value,
    c.auto_renewal,
    v.name AS vendor_name,
    v.contact_person,
    v.phone AS vendor_phone,
    e.first_name || ' ' || e.last_name AS responsible_person,
    CURRENT_DATE - c.end_date AS days_until_expiry
FROM contracts c
LEFT JOIN vendors v ON c.vendor_id = v.id
LEFT JOIN employees e ON c.responsible_employee_id = e.id
WHERE c.status = 'active'
  AND c.end_date IS NOT NULL
  AND c.end_date BETWEEN CURRENT_DATE AND CURRENT_DATE + INTERVAL '90 days'
ORDER BY c.end_date ASC;

-- Комментарии
COMMENT ON TABLE vendors IS 'Поставщики оборудования и услуг';
COMMENT ON TABLE contracts IS 'Контракты на закупку, обслуживание, лицензии';
COMMENT ON TABLE contract_assets IS 'Связь контрактов с оборудованием и ПО';
COMMENT ON VIEW v_expiring_contracts IS 'Контракты, истекающие в ближайшие 90 дней';
