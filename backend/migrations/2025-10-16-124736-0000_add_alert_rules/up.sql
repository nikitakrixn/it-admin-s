-- ============================================================================
-- Правила алертов и уведомлений
-- ============================================================================

-- Правила алертов
CREATE TABLE alert_rules (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    rule_type VARCHAR(50) NOT NULL,
    entity_type VARCHAR(50),
    condition_field VARCHAR(100) NOT NULL,
    condition_operator VARCHAR(20) NOT NULL,
    condition_value VARCHAR(255) NOT NULL,
    severity VARCHAR(20) DEFAULT 'warning',
    is_active BOOLEAN DEFAULT true,
    check_interval_minutes INTEGER DEFAULT 60,
    notification_channels JSONB,
    notify_users JSONB,
    last_checked_at TIMESTAMP,
    last_triggered_at TIMESTAMP,
    trigger_count INTEGER DEFAULT 0,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_rule_type CHECK (rule_type IN ('threshold', 'status_change', 'missing_data', 'pattern', 'schedule')),
    CONSTRAINT check_operator CHECK (condition_operator IN ('equals', 'not_equals', 'greater_than', 'less_than', 'contains', 'not_contains', 'is_null', 'is_not_null')),
    CONSTRAINT check_severity CHECK (severity IN ('info', 'warning', 'error', 'critical'))
);

-- История срабатывания алертов
CREATE TABLE alert_history (
    id SERIAL PRIMARY KEY,
    alert_rule_id INTEGER REFERENCES alert_rules(id) ON DELETE SET NULL,
    entity_type VARCHAR(50),
    entity_id INTEGER,
    severity VARCHAR(20) NOT NULL,
    message TEXT NOT NULL,
    details JSONB,
    status VARCHAR(20) DEFAULT 'active',
    acknowledged_by UUID REFERENCES users(id) ON DELETE SET NULL,
    acknowledged_at TIMESTAMP,
    resolved_at TIMESTAMP,
    resolution_notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_alert_status CHECK (status IN ('active', 'acknowledged', 'resolved', 'auto_resolved'))
);

-- Подписки на алерты
CREATE TABLE alert_subscriptions (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    alert_rule_id INTEGER REFERENCES alert_rules(id) ON DELETE CASCADE,
    notification_method VARCHAR(50) NOT NULL,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_notification_method CHECK (notification_method IN ('email', 'telegram', 'web', 'sms')),
    UNIQUE(user_id, alert_rule_id, notification_method)
);

-- Индексы
CREATE INDEX idx_alert_rules_type ON alert_rules(rule_type);
CREATE INDEX idx_alert_rules_active ON alert_rules(is_active) WHERE is_active = true;
CREATE INDEX idx_alert_rules_entity ON alert_rules(entity_type);
CREATE INDEX idx_alert_history_rule ON alert_history(alert_rule_id);
CREATE INDEX idx_alert_history_entity ON alert_history(entity_type, entity_id);
CREATE INDEX idx_alert_history_status ON alert_history(status);
CREATE INDEX idx_alert_history_created ON alert_history(created_at DESC);
CREATE INDEX idx_alert_subscriptions_user ON alert_subscriptions(user_id);
CREATE INDEX idx_alert_subscriptions_rule ON alert_subscriptions(alert_rule_id);

-- Триггеры
CREATE TRIGGER update_alert_rules_updated_at BEFORE UPDATE ON alert_rules
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Типовые правила алертов
INSERT INTO alert_rules (name, description, rule_type, entity_type, condition_field, condition_operator, condition_value, severity, check_interval_minutes) VALUES 
    ('Диск заполнен > 90%', 'Предупреждение о заполнении диска', 'threshold', 'computer', 'disk_usage_percent', 'greater_than', '90', 'warning', 30),
    ('Диск заполнен > 95%', 'Критическое заполнение диска', 'threshold', 'computer', 'disk_usage_percent', 'greater_than', '95', 'critical', 15),
    ('Высокая загрузка CPU', 'CPU загружен более 90%', 'threshold', 'computer', 'cpu_usage', 'greater_than', '90', 'warning', 15),
    ('Компьютер офлайн > 24ч', 'Компьютер не выходил на связь более суток', 'missing_data', 'computer', 'last_seen_online', 'less_than', '24', 'error', 60),
    ('Истекает гарантия', 'Гарантия истекает через 30 дней', 'threshold', 'computer', 'warranty_end_date', 'less_than', '30', 'info', 1440),
    ('Истекает лицензия', 'Лицензия истекает через 30 дней', 'threshold', 'license_pool', 'expiration_date', 'less_than', '30', 'warning', 1440),
    ('Низкий остаток расходников', 'Расходники ниже минимального уровня', 'threshold', 'consumable', 'quantity', 'less_than', 'min_quantity', 'warning', 1440);

-- Комментарии
COMMENT ON TABLE alert_rules IS 'Правила для автоматических алертов и уведомлений';
COMMENT ON TABLE alert_history IS 'История срабатывания алертов';
COMMENT ON TABLE alert_subscriptions IS 'Подписки пользователей на алерты';
COMMENT ON COLUMN alert_rules.notification_channels IS 'JSON массив каналов уведомлений: ["email", "telegram", "web"]';
COMMENT ON COLUMN alert_rules.notify_users IS 'JSON массив UUID пользователей для уведомления';
