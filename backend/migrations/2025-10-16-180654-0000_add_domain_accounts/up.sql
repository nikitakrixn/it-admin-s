-- ============================================================================
-- Доменные учётные записи
-- ============================================================================

CREATE TABLE domain_accounts (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER REFERENCES employees(id) ON DELETE SET NULL,
    username VARCHAR(100) UNIQUE NOT NULL,
    display_name VARCHAR(255),
    email VARCHAR(255),
    account_type VARCHAR(50) NOT NULL DEFAULT 'user',
    distinguished_name TEXT,
    user_principal_name VARCHAR(255),
    sam_account_name VARCHAR(100),
    password_last_set TIMESTAMP,
    password_expires_at TIMESTAMP,
    password_never_expires BOOLEAN DEFAULT false,
    must_change_password BOOLEAN DEFAULT false,
    cannot_change_password BOOLEAN DEFAULT false,
    account_expires_at TIMESTAMP,
    is_enabled BOOLEAN DEFAULT true,
    is_locked BOOLEAN DEFAULT false,
    locked_at TIMESTAMP,
    lockout_time TIMESTAMP,
    last_logon TIMESTAMP,
    last_logon_ip VARCHAR(45),
    logon_count INTEGER DEFAULT 0,
    bad_password_count INTEGER DEFAULT 0,
    bad_password_time TIMESTAMP,
    home_directory VARCHAR(500),
    profile_path VARCHAR(500),
    script_path VARCHAR(500),
    description TEXT,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_account_type CHECK (account_type IN ('user', 'service', 'admin', 'computer', 'guest'))
);

-- История изменений доменных учёток
CREATE TABLE domain_account_history (
    id SERIAL PRIMARY KEY,
    account_id INTEGER NOT NULL REFERENCES domain_accounts(id) ON DELETE CASCADE,
    action VARCHAR(50) NOT NULL,
    field_name VARCHAR(100),
    old_value TEXT,
    new_value TEXT,
    changed_by VARCHAR(255),
    changed_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    notes TEXT,
    CONSTRAINT check_account_action CHECK (action IN (
        'created', 'enabled', 'disabled', 'locked', 'unlocked', 
        'password_reset', 'password_changed', 'moved', 'renamed', 
        'group_added', 'group_removed', 'attribute_changed'
    ))
);

-- Индексы
CREATE INDEX idx_domain_accounts_employee ON domain_accounts(employee_id);
CREATE INDEX idx_domain_accounts_username ON domain_accounts(username);
CREATE INDEX idx_domain_accounts_type ON domain_accounts(account_type);
CREATE INDEX idx_domain_accounts_enabled ON domain_accounts(is_enabled);
CREATE INDEX idx_domain_accounts_locked ON domain_accounts(is_locked) WHERE is_locked = true;
CREATE INDEX idx_domain_accounts_password_expiry ON domain_accounts(password_expires_at) 
    WHERE password_expires_at IS NOT NULL;
CREATE INDEX idx_domain_accounts_account_expiry ON domain_accounts(account_expires_at) 
    WHERE account_expires_at IS NOT NULL;
CREATE INDEX idx_domain_account_history_account ON domain_account_history(account_id, changed_at DESC);
CREATE INDEX idx_domain_account_history_action ON domain_account_history(action);

-- Триггеры
CREATE TRIGGER update_domain_accounts_updated_at BEFORE UPDATE ON domain_accounts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Представление: учётки требующие внимания
CREATE OR REPLACE VIEW v_domain_accounts_expiring AS
SELECT 
    da.id,
    da.username,
    da.display_name,
    e.first_name || ' ' || e.last_name AS employee_name,
    d.name AS department_name,
    da.account_type,
    da.password_expires_at,
    da.account_expires_at,
    da.last_logon,
    da.is_locked,
    CASE 
        WHEN da.is_locked THEN 'locked'
        WHEN NOT da.is_enabled THEN 'disabled'
        WHEN da.account_expires_at < CURRENT_TIMESTAMP THEN 'account_expired'
        WHEN da.password_expires_at < CURRENT_TIMESTAMP THEN 'password_expired'
        WHEN da.account_expires_at < CURRENT_TIMESTAMP + INTERVAL '7 days' THEN 'account_expiring_soon'
        WHEN da.password_expires_at < CURRENT_TIMESTAMP + INTERVAL '7 days' THEN 'password_expiring_soon'
        WHEN da.last_logon < CURRENT_TIMESTAMP - INTERVAL '90 days' THEN 'inactive'
        ELSE 'ok'
    END AS status,
    CASE 
        WHEN da.password_expires_at IS NOT NULL 
        THEN EXTRACT(DAY FROM (da.password_expires_at - CURRENT_TIMESTAMP))::INTEGER 
    END AS days_until_password_expiry,
    CASE 
        WHEN da.account_expires_at IS NOT NULL 
        THEN EXTRACT(DAY FROM (da.account_expires_at - CURRENT_TIMESTAMP))::INTEGER 
    END AS days_until_account_expiry
FROM domain_accounts da
LEFT JOIN employees e ON da.employee_id = e.id
LEFT JOIN departments d ON e.department_id = d.id
WHERE 
    da.is_locked = true OR
    da.is_enabled = false OR
    da.account_expires_at < CURRENT_TIMESTAMP + INTERVAL '30 days' OR
    da.password_expires_at < CURRENT_TIMESTAMP + INTERVAL '14 days' OR
    (da.last_logon IS NOT NULL AND da.last_logon < CURRENT_TIMESTAMP - INTERVAL '90 days')
ORDER BY 
    CASE 
        WHEN da.is_locked THEN 1
        WHEN NOT da.is_enabled THEN 2
        WHEN da.account_expires_at < CURRENT_TIMESTAMP THEN 3
        WHEN da.password_expires_at < CURRENT_TIMESTAMP THEN 4
        ELSE 5
    END,
    da.password_expires_at NULLS LAST;

-- Комментарии
COMMENT ON TABLE domain_accounts IS 'Доменные учётные записи Active Directory';
COMMENT ON TABLE domain_account_history IS 'История изменений доменных учёток';
COMMENT ON VIEW v_domain_accounts_expiring IS 'Учётные записи требующие внимания (истекающие, заблокированные, неактивные)';
