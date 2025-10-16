-- ============================================================================
-- Инвентаризация
-- ============================================================================

CREATE TABLE inventory_audits (
    id SERIAL PRIMARY KEY,
    audit_name VARCHAR(255) NOT NULL,
    audit_date DATE NOT NULL,
    audit_type VARCHAR(50) NOT NULL,
    location VARCHAR(100),
    department_id INTEGER REFERENCES departments(id) ON DELETE SET NULL,
    audited_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    status VARCHAR(20) DEFAULT 'in_progress',
    total_items INTEGER DEFAULT 0,
    found_items INTEGER DEFAULT 0,
    missing_items INTEGER DEFAULT 0,
    damaged_items INTEGER DEFAULT 0,
    notes TEXT,
    started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_audit_type CHECK (audit_type IN ('full', 'partial', 'spot_check', 'annual')),
    CONSTRAINT check_audit_status CHECK (status IN ('planned', 'in_progress', 'completed', 'cancelled'))
);

-- Элементы инвентаризации
CREATE TABLE inventory_audit_items (
    id SERIAL PRIMARY KEY,
    audit_id INTEGER NOT NULL REFERENCES inventory_audits(id) ON DELETE CASCADE,
    entity_type VARCHAR(50) NOT NULL,
    entity_id INTEGER NOT NULL,
    expected_location VARCHAR(100),
    actual_location VARCHAR(100),
    expected_condition VARCHAR(50),
    actual_condition VARCHAR(50),
    status VARCHAR(20) NOT NULL,
    discrepancy_notes TEXT,
    photo_url VARCHAR(500),
    checked_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    checked_by UUID REFERENCES users(id) ON DELETE SET NULL,
    CONSTRAINT check_entity_type CHECK (entity_type IN ('computer', 'equipment', 'consumable', 'peripheral')),
    CONSTRAINT check_item_status CHECK (status IN ('found', 'missing', 'damaged', 'relocated', 'surplus', 'disposed')),
    CONSTRAINT check_condition CHECK (expected_condition IN ('new', 'good', 'fair', 'poor', 'broken') OR expected_condition IS NULL),
    CONSTRAINT check_actual_condition CHECK (actual_condition IN ('new', 'good', 'fair', 'poor', 'broken') OR actual_condition IS NULL)
);

-- Индексы
CREATE INDEX idx_inventory_audits_date ON inventory_audits(audit_date DESC);
CREATE INDEX idx_inventory_audits_status ON inventory_audits(status);
CREATE INDEX idx_inventory_audits_department ON inventory_audits(department_id);
CREATE INDEX idx_inventory_audits_audited_by ON inventory_audits(audited_by);
CREATE INDEX idx_inventory_audit_items_audit ON inventory_audit_items(audit_id);
CREATE INDEX idx_inventory_audit_items_entity ON inventory_audit_items(entity_type, entity_id);
CREATE INDEX idx_inventory_audit_items_status ON inventory_audit_items(status);

-- Представление: последняя инвентаризация для каждого объекта
CREATE OR REPLACE VIEW v_last_inventory_check AS
SELECT DISTINCT ON (iai.entity_type, iai.entity_id)
    iai.entity_type,
    iai.entity_id,
    ia.audit_date,
    ia.audit_name,
    iai.status,
    iai.actual_location,
    iai.actual_condition,
    u.email AS audited_by_email,
    CURRENT_DATE - ia.audit_date AS days_since_audit
FROM inventory_audit_items iai
JOIN inventory_audits ia ON iai.audit_id = ia.id
LEFT JOIN users u ON ia.audited_by = u.id
WHERE ia.status = 'completed'
ORDER BY iai.entity_type, iai.entity_id, ia.audit_date DESC;

-- Комментарии
COMMENT ON TABLE inventory_audits IS 'Инвентаризации';
COMMENT ON TABLE inventory_audit_items IS 'Элементы инвентаризации';
COMMENT ON VIEW v_last_inventory_check IS 'Последняя инвентаризация для каждого объекта';
