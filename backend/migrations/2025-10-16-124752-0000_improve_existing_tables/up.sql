-- ============================================================================
-- Улучшения существующих таблиц
-- ============================================================================

-- Улучшения таблицы employees
ALTER TABLE employees 
    ADD COLUMN photo_url VARCHAR(500),
    ADD COLUMN telegram_username VARCHAR(100),
    ADD COLUMN is_vip BOOLEAN DEFAULT false;

CREATE INDEX idx_employees_vip ON employees(is_vip) WHERE is_vip = true;

COMMENT ON COLUMN employees.photo_url IS 'URL фотографии сотрудника';
COMMENT ON COLUMN employees.telegram_username IS 'Telegram для уведомлений';
COMMENT ON COLUMN employees.is_vip IS 'VIP пользователь (приоритетная поддержка)';

-- Улучшения таблицы requests
ALTER TABLE requests 
    ADD COLUMN first_response_at TIMESTAMP,
    ADD COLUMN sla_deadline TIMESTAMP,
    ADD COLUMN sla_breached BOOLEAN DEFAULT false,
    ADD COLUMN estimated_time INTEGER,
    ADD COLUMN actual_time INTEGER,
    ADD COLUMN parent_request_id INTEGER REFERENCES requests(id) ON DELETE SET NULL;

CREATE INDEX idx_requests_sla_deadline ON requests(sla_deadline) WHERE status NOT IN ('closed', 'cancelled');
CREATE INDEX idx_requests_sla_breached ON requests(sla_breached) WHERE sla_breached = true;
CREATE INDEX idx_requests_parent ON requests(parent_request_id);

COMMENT ON COLUMN requests.first_response_at IS 'Время первого ответа на заявку';
COMMENT ON COLUMN requests.sla_deadline IS 'Дедлайн по SLA';
COMMENT ON COLUMN requests.sla_breached IS 'Нарушен ли SLA';
COMMENT ON COLUMN requests.estimated_time IS 'Оценка времени выполнения (минуты)';
COMMENT ON COLUMN requests.actual_time IS 'Фактическое время выполнения (минуты)';
COMMENT ON COLUMN requests.parent_request_id IS 'Родительская заявка (для подзадач)';

-- Улучшения таблицы software_catalog
ALTER TABLE software_catalog 
    ADD COLUMN latest_version VARCHAR(100),
    ADD COLUMN is_deprecated BOOLEAN DEFAULT false,
    ADD COLUMN security_risk_level VARCHAR(20),
    ADD COLUMN vendor_id INTEGER REFERENCES vendors(id) ON DELETE SET NULL;

CREATE INDEX idx_software_catalog_deprecated ON software_catalog(is_deprecated) WHERE is_deprecated = true;
CREATE INDEX idx_software_catalog_risk ON software_catalog(security_risk_level);
CREATE INDEX idx_software_catalog_vendor ON software_catalog(vendor_id);

COMMENT ON COLUMN software_catalog.latest_version IS 'Последняя доступная версия';
COMMENT ON COLUMN software_catalog.is_deprecated IS 'Устаревшее ПО (не рекомендуется к использованию)';
COMMENT ON COLUMN software_catalog.security_risk_level IS 'Уровень риска безопасности (low, medium, high, critical)';

-- Улучшения таблицы computer_monitoring
CREATE INDEX idx_monitoring_latest ON computer_monitoring(computer_id, collected_at DESC);

-- Улучшения таблицы users
ALTER TABLE users 
    ADD COLUMN two_factor_enabled BOOLEAN DEFAULT false,
    ADD COLUMN two_factor_secret VARCHAR(255),
    ADD COLUMN password_changed_at TIMESTAMP,
    ADD COLUMN must_change_password BOOLEAN DEFAULT false,
    ADD COLUMN failed_login_attempts INTEGER DEFAULT 0,
    ADD COLUMN locked_until TIMESTAMP;

CREATE INDEX idx_users_locked ON users(locked_until) WHERE locked_until IS NOT NULL;

COMMENT ON COLUMN users.two_factor_enabled IS 'Включена ли двухфакторная аутентификация';
COMMENT ON COLUMN users.two_factor_secret IS 'Секретный ключ для 2FA';
COMMENT ON COLUMN users.password_changed_at IS 'Дата последней смены пароля';
COMMENT ON COLUMN users.must_change_password IS 'Требуется смена пароля при следующем входе';
COMMENT ON COLUMN users.failed_login_attempts IS 'Количество неудачных попыток входа';
COMMENT ON COLUMN users.locked_until IS 'Учётная запись заблокирована до';

-- Улучшения таблицы computer_storage
ALTER TABLE computer_storage 
    ADD COLUMN smart_status VARCHAR(50),
    ADD COLUMN temperature INTEGER,
    ADD COLUMN power_on_hours INTEGER,
    ADD COLUMN reallocated_sectors INTEGER;

COMMENT ON COLUMN computer_storage.smart_status IS 'Статус SMART (good, warning, critical)';
COMMENT ON COLUMN computer_storage.temperature IS 'Температура диска (°C)';
COMMENT ON COLUMN computer_storage.power_on_hours IS 'Часы работы диска';
COMMENT ON COLUMN computer_storage.reallocated_sectors IS 'Количество переназначенных секторов';

-- Улучшения таблицы equipment
ALTER TABLE equipment 
    ADD COLUMN last_maintenance_date DATE,
    ADD COLUMN next_maintenance_date DATE,
    ADD COLUMN toner_level INTEGER,
    ADD COLUMN page_count INTEGER;

COMMENT ON COLUMN equipment.last_maintenance_date IS 'Дата последнего обслуживания';
COMMENT ON COLUMN equipment.next_maintenance_date IS 'Дата следующего планового обслуживания';
COMMENT ON COLUMN equipment.toner_level IS 'Уровень тонера (для принтеров, %)';
COMMENT ON COLUMN equipment.page_count IS 'Счётчик страниц (для принтеров)';

-- Представление: компьютеры требующие внимания
CREATE OR REPLACE VIEW v_computers_need_attention AS
SELECT 
    c.id,
    c.hostname,
    c.inventory_number,
    e.first_name || ' ' || e.last_name AS employee_name,
    ARRAY_AGG(DISTINCT issue) AS issues
FROM computers c
LEFT JOIN employees e ON c.employee_id = e.id
CROSS JOIN LATERAL (
    SELECT 'Офлайн более 7 дней' AS issue
    WHERE c.last_seen_online < CURRENT_TIMESTAMP - INTERVAL '7 days'
    UNION ALL
    SELECT 'Не активирована Windows'
    WHERE c.windows_activation_status = 'not_activated'
    UNION ALL
    SELECT 'Антивирус отключён'
    WHERE c.antivirus_status IN ('disabled', 'not_installed')
    UNION ALL
    SELECT 'Истекает гарантия'
    WHERE c.warranty_end_date BETWEEN CURRENT_DATE AND CURRENT_DATE + INTERVAL '30 days'
) AS issues
GROUP BY c.id, c.hostname, c.inventory_number, employee_name;

-- Представление: статистика по заявкам
CREATE OR REPLACE VIEW v_request_statistics AS
SELECT 
    DATE_TRUNC('day', created_at) AS date,
    status,
    priority,
    category,
    COUNT(*) AS count,
    AVG(EXTRACT(EPOCH FROM (COALESCE(resolved_at, CURRENT_TIMESTAMP) - created_at)) / 3600) AS avg_resolution_hours,
    COUNT(*) FILTER (WHERE sla_breached = true) AS sla_breached_count
FROM requests
WHERE created_at >= CURRENT_DATE - INTERVAL '90 days'
GROUP BY DATE_TRUNC('day', created_at), status, priority, category;

COMMENT ON VIEW v_computers_need_attention IS 'Компьютеры, требующие внимания (офлайн, проблемы с антивирусом и т.д.)';
COMMENT ON VIEW v_request_statistics IS 'Статистика по заявкам за последние 90 дней';
