-- ============================================================================
-- Windows-специфичные таблицы
-- ============================================================================

-- Windows Updates
CREATE TABLE windows_updates (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    update_id VARCHAR(100) NOT NULL,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    kb_number VARCHAR(20),
    installed_date TIMESTAMP,
    is_security_update BOOLEAN DEFAULT false,
    requires_reboot BOOLEAN DEFAULT false,
    update_classification VARCHAR(50),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(computer_id, update_id)
);

-- Отсутствующие обновления
CREATE TABLE windows_missing_updates (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    update_id VARCHAR(100) NOT NULL,
    title VARCHAR(500) NOT NULL,
    kb_number VARCHAR(20),
    severity VARCHAR(20),
    is_security_update BOOLEAN DEFAULT false,
    release_date DATE,
    detected_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(computer_id, update_id),
    CONSTRAINT check_severity CHECK (severity IN ('Critical', 'Important', 'Moderate', 'Low', 'Unspecified'))
);

-- Windows Services
CREATE TABLE windows_services (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    service_name VARCHAR(255) NOT NULL,
    display_name VARCHAR(500),
    status VARCHAR(50),
    startup_type VARCHAR(50),
    account VARCHAR(255),
    is_critical BOOLEAN DEFAULT false,
    last_checked TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_service_status CHECK (status IN ('Running', 'Stopped', 'Paused', 'StartPending', 'StopPending')),
    CONSTRAINT check_startup_type CHECK (startup_type IN ('Automatic', 'Manual', 'Disabled', 'AutomaticDelayedStart')),
    UNIQUE(computer_id, service_name)
);

-- История падений служб
CREATE TABLE service_failures (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    service_name VARCHAR(255) NOT NULL,
    failure_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    error_message TEXT,
    auto_restarted BOOLEAN DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Windows Event Log
CREATE TABLE windows_events (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    event_id INTEGER NOT NULL,
    log_name VARCHAR(100),
    level VARCHAR(20),
    source VARCHAR(255),
    message TEXT,
    event_time TIMESTAMP NOT NULL,
    collected_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_log_name CHECK (log_name IN ('Application', 'System', 'Security', 'Setup')),
    CONSTRAINT check_level CHECK (level IN ('Critical', 'Error', 'Warning', 'Information', 'Verbose'))
);

-- Active Directory Groups
CREATE TABLE ad_groups (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    distinguished_name TEXT NOT NULL,
    description TEXT,
    group_type VARCHAR(50),
    group_scope VARCHAR(50),
    member_count INTEGER DEFAULT 0,
    last_synced TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_group_type CHECK (group_type IN ('Security', 'Distribution')),
    CONSTRAINT check_group_scope CHECK (group_scope IN ('Global', 'DomainLocal', 'Universal'))
);

-- Членство в группах AD
CREATE TABLE ad_group_memberships (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL REFERENCES employees(id) ON DELETE CASCADE,
    ad_group_id INTEGER NOT NULL REFERENCES ad_groups(id) ON DELETE CASCADE,
    added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(employee_id, ad_group_id)
);

-- История синхронизации с AD
CREATE TABLE ad_sync_history (
    id SERIAL PRIMARY KEY,
    sync_type VARCHAR(50),
    status VARCHAR(20),
    records_added INTEGER DEFAULT 0,
    records_updated INTEGER DEFAULT 0,
    records_removed INTEGER DEFAULT 0,
    error_message TEXT,
    sync_started_at TIMESTAMP NOT NULL,
    sync_completed_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_sync_type CHECK (sync_type IN ('users', 'groups', 'computers', 'full')),
    CONSTRAINT check_sync_status CHECK (status IN ('success', 'failed', 'partial'))
);

-- Group Policy Objects
CREATE TABLE group_policies (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    guid VARCHAR(50) UNIQUE,
    description TEXT,
    gpo_status VARCHAR(50),
    created_date TIMESTAMP,
    modified_date TIMESTAMP,
    version INTEGER,
    last_synced TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_gpo_status CHECK (gpo_status IN ('Enabled', 'Disabled', 'AllSettingsDisabled', 'ComputerSettingsDisabled', 'UserSettingsDisabled'))
);

-- Применение GPO к компьютерам
CREATE TABLE computer_gpo_applications (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    gpo_id INTEGER NOT NULL REFERENCES group_policies(id) ON DELETE CASCADE,
    applied_at TIMESTAMP,
    status VARCHAR(50),
    error_message TEXT,
    CONSTRAINT check_gpo_application_status CHECK (status IN ('Applied', 'Failed', 'Pending', 'NotApplied')),
    UNIQUE(computer_id, gpo_id)
);

-- Shared Folders
CREATE TABLE shared_folders (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER REFERENCES computers(id) ON DELETE CASCADE,
    share_name VARCHAR(255) NOT NULL,
    path TEXT NOT NULL,
    description TEXT,
    max_users INTEGER,
    current_users INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Share Permissions
CREATE TABLE share_permissions (
    id SERIAL PRIMARY KEY,
    shared_folder_id INTEGER NOT NULL REFERENCES shared_folders(id) ON DELETE CASCADE,
    principal_name VARCHAR(255) NOT NULL,
    permission_type VARCHAR(50),
    is_allowed BOOLEAN DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_permission_type CHECK (permission_type IN ('Read', 'Change', 'FullControl'))
);

-- Windows Features & Roles
CREATE TABLE windows_features (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    feature_name VARCHAR(255) NOT NULL,
    display_name VARCHAR(500),
    feature_type VARCHAR(50),
    install_state VARCHAR(50),
    restart_needed BOOLEAN DEFAULT false,
    last_checked TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_feature_type CHECK (feature_type IN ('Role', 'RoleService', 'Feature')),
    CONSTRAINT check_install_state CHECK (install_state IN ('Installed', 'Available', 'Removed', 'InstallPending', 'UninstallPending')),
    UNIQUE(computer_id, feature_name)
);

-- RDP Sessions
CREATE TABLE rdp_sessions (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    username VARCHAR(255) NOT NULL,
    session_id INTEGER,
    client_name VARCHAR(255),
    client_ip VARCHAR(45),
    logon_time TIMESTAMP NOT NULL,
    logoff_time TIMESTAMP,
    session_duration INTEGER,
    disconnect_reason VARCHAR(100),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Windows Firewall Rules
CREATE TABLE firewall_rules (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    rule_name VARCHAR(255) NOT NULL,
    direction VARCHAR(20),
    action VARCHAR(20),
    protocol VARCHAR(50),
    local_port VARCHAR(100),
    remote_port VARCHAR(100),
    remote_address TEXT,
    enabled BOOLEAN DEFAULT true,
    profile VARCHAR(50),
    last_checked TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_direction CHECK (direction IN ('Inbound', 'Outbound')),
    CONSTRAINT check_action CHECK (action IN ('Allow', 'Block')),
    CONSTRAINT check_profile CHECK (profile IN ('Domain', 'Private', 'Public', 'Any'))
);

-- Scheduled Tasks
CREATE TABLE scheduled_tasks (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    task_name VARCHAR(500) NOT NULL,
    task_path VARCHAR(500),
    status VARCHAR(50),
    last_run_time TIMESTAMP,
    last_result INTEGER,
    next_run_time TIMESTAMP,
    trigger_info TEXT,
    action_info TEXT,
    run_as_user VARCHAR(255),
    is_enabled BOOLEAN DEFAULT true,
    last_checked TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_task_status CHECK (status IN ('Ready', 'Running', 'Disabled', 'Queued'))
);

-- Индексы
CREATE INDEX idx_windows_updates_computer ON windows_updates(computer_id);
CREATE INDEX idx_windows_updates_kb ON windows_updates(kb_number);
CREATE INDEX idx_windows_missing_updates_computer ON windows_missing_updates(computer_id);
CREATE INDEX idx_windows_missing_updates_severity ON windows_missing_updates(severity);
CREATE INDEX idx_windows_services_computer ON windows_services(computer_id);
CREATE INDEX idx_windows_services_critical ON windows_services(is_critical) WHERE is_critical = true;
CREATE INDEX idx_service_failures_computer ON service_failures(computer_id, failure_time DESC);
CREATE INDEX idx_windows_events_computer ON windows_events(computer_id, event_time DESC);
CREATE INDEX idx_windows_events_level ON windows_events(level) WHERE level IN ('Critical', 'Error');
CREATE INDEX idx_ad_groups_name ON ad_groups(name);
CREATE INDEX idx_ad_group_memberships_employee ON ad_group_memberships(employee_id);
CREATE INDEX idx_ad_group_memberships_group ON ad_group_memberships(ad_group_id);
CREATE INDEX idx_ad_sync_history_started ON ad_sync_history(sync_started_at DESC);
CREATE INDEX idx_group_policies_guid ON group_policies(guid);
CREATE INDEX idx_computer_gpo_applications_computer ON computer_gpo_applications(computer_id);
CREATE INDEX idx_shared_folders_computer ON shared_folders(computer_id);
CREATE INDEX idx_share_permissions_folder ON share_permissions(shared_folder_id);
CREATE INDEX idx_windows_features_computer ON windows_features(computer_id);
CREATE INDEX idx_rdp_sessions_computer ON rdp_sessions(computer_id, logon_time DESC);
CREATE INDEX idx_rdp_sessions_user ON rdp_sessions(username);
CREATE INDEX idx_firewall_rules_computer ON firewall_rules(computer_id);
CREATE INDEX idx_scheduled_tasks_computer ON scheduled_tasks(computer_id);

-- Триггеры
CREATE TRIGGER update_windows_services_updated_at BEFORE UPDATE ON windows_services
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_shared_folders_updated_at BEFORE UPDATE ON shared_folders
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Представления
CREATE OR REPLACE VIEW v_critical_windows_events AS
SELECT 
    we.id,
    c.hostname,
    c.inventory_number,
    we.event_id,
    we.log_name,
    we.level,
    we.source,
    we.message,
    we.event_time,
    e.first_name || ' ' || e.last_name AS employee_name
FROM windows_events we
JOIN computers c ON we.computer_id = c.id
LEFT JOIN employees e ON c.employee_id = e.id
WHERE we.level IN ('Critical', 'Error')
  AND we.event_time >= CURRENT_TIMESTAMP - INTERVAL '7 days'
ORDER BY we.event_time DESC;

CREATE OR REPLACE VIEW v_missing_critical_updates AS
SELECT 
    wmu.id,
    c.hostname,
    c.inventory_number,
    wmu.title,
    wmu.kb_number,
    wmu.severity,
    wmu.detected_at,
    e.first_name || ' ' || e.last_name AS employee_name,
    d.name AS department_name
FROM windows_missing_updates wmu
JOIN computers c ON wmu.computer_id = c.id
LEFT JOIN employees e ON c.employee_id = e.id
LEFT JOIN departments d ON e.department_id = d.id
WHERE wmu.severity IN ('Critical', 'Important')
  AND wmu.is_security_update = true
ORDER BY wmu.severity, wmu.detected_at DESC;

-- Комментарии
COMMENT ON TABLE windows_updates IS 'Установленные обновления Windows';
COMMENT ON TABLE windows_missing_updates IS 'Отсутствующие обновления Windows';
COMMENT ON TABLE windows_services IS 'Службы Windows';
COMMENT ON TABLE service_failures IS 'История падений служб';
COMMENT ON TABLE windows_events IS 'События из журнала Windows';
COMMENT ON TABLE ad_groups IS 'Группы Active Directory';
COMMENT ON TABLE ad_group_memberships IS 'Членство в группах AD';
COMMENT ON TABLE ad_sync_history IS 'История синхронизации с AD';
COMMENT ON TABLE group_policies IS 'Групповые политики (GPO)';
COMMENT ON TABLE computer_gpo_applications IS 'Применение GPO к компьютерам';
COMMENT ON TABLE shared_folders IS 'Сетевые папки';
COMMENT ON TABLE share_permissions IS 'Права доступа к сетевым папкам';
COMMENT ON TABLE windows_features IS 'Установленные роли и компоненты Windows';
COMMENT ON TABLE rdp_sessions IS 'История RDP подключений';
COMMENT ON TABLE firewall_rules IS 'Правила Windows Firewall';
COMMENT ON TABLE scheduled_tasks IS 'Запланированные задачи Windows';
COMMENT ON VIEW v_critical_windows_events IS 'Критичные события Windows за последние 7 дней';
COMMENT ON VIEW v_missing_critical_updates IS 'Отсутствующие критичные обновления безопасности';
