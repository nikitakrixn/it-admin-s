-- ============================================================================
-- Управление патчами и обновлениями
-- ============================================================================

-- Расписания патчей
CREATE TABLE patch_schedules (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    target_type VARCHAR(50) NOT NULL,
    target_id INTEGER,
    patch_day VARCHAR(20),
    maintenance_window_start TIME,
    maintenance_window_end TIME,
    auto_approve_critical BOOLEAN DEFAULT true,
    auto_approve_security BOOLEAN DEFAULT false,
    auto_reboot BOOLEAN DEFAULT false,
    reboot_delay_minutes INTEGER DEFAULT 15,
    notification_before_minutes INTEGER DEFAULT 60,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_target_type CHECK (target_type IN ('computer', 'department', 'all', 'group')),
    CONSTRAINT check_patch_day CHECK (patch_day IN ('monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday', 'sunday', 'any') OR patch_day IS NULL)
);

-- Группы патчей
CREATE TABLE patch_groups (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    auto_approve BOOLEAN DEFAULT false,
    test_group BOOLEAN DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Компьютеры в группах патчей
CREATE TABLE patch_group_members (
    id SERIAL PRIMARY KEY,
    patch_group_id INTEGER NOT NULL REFERENCES patch_groups(id) ON DELETE CASCADE,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(patch_group_id, computer_id)
);

-- Задания на установку патчей
CREATE TABLE patch_deployments (
    id SERIAL PRIMARY KEY,
    deployment_name VARCHAR(255) NOT NULL,
    patch_schedule_id INTEGER REFERENCES patch_schedules(id) ON DELETE SET NULL,
    target_type VARCHAR(50) NOT NULL,
    target_ids INTEGER[],
    update_ids TEXT[],
    scheduled_at TIMESTAMP NOT NULL,
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    status VARCHAR(20) DEFAULT 'scheduled',
    total_targets INTEGER DEFAULT 0,
    successful_count INTEGER DEFAULT 0,
    failed_count INTEGER DEFAULT 0,
    pending_count INTEGER DEFAULT 0,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_deployment_status CHECK (status IN ('scheduled', 'running', 'completed', 'failed', 'cancelled'))
);

-- Результаты установки патчей
CREATE TABLE patch_deployment_results (
    id SERIAL PRIMARY KEY,
    deployment_id INTEGER NOT NULL REFERENCES patch_deployments(id) ON DELETE CASCADE,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    update_id VARCHAR(100) NOT NULL,
    update_title VARCHAR(500),
    status VARCHAR(20) NOT NULL,
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    error_code INTEGER,
    error_message TEXT,
    reboot_required BOOLEAN DEFAULT false,
    rebooted BOOLEAN DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_result_status CHECK (status IN ('pending', 'downloading', 'installing', 'success', 'failed', 'cancelled'))
);

-- Индексы
CREATE INDEX idx_patch_schedules_active ON patch_schedules(is_active) WHERE is_active = true;
CREATE INDEX idx_patch_schedules_target ON patch_schedules(target_type, target_id);
CREATE INDEX idx_patch_group_members_group ON patch_group_members(patch_group_id);
CREATE INDEX idx_patch_group_members_computer ON patch_group_members(computer_id);
CREATE INDEX idx_patch_deployments_status ON patch_deployments(status);
CREATE INDEX idx_patch_deployments_scheduled ON patch_deployments(scheduled_at);
CREATE INDEX idx_patch_deployment_results_deployment ON patch_deployment_results(deployment_id);
CREATE INDEX idx_patch_deployment_results_computer ON patch_deployment_results(computer_id);
CREATE INDEX idx_patch_deployment_results_status ON patch_deployment_results(status);

-- Триггеры
CREATE TRIGGER update_patch_schedules_updated_at BEFORE UPDATE ON patch_schedules
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Представление: статистика патчей
CREATE OR REPLACE VIEW v_patch_compliance AS
SELECT 
    d.name AS department_name,
    COUNT(DISTINCT c.id) AS total_computers,
    COUNT(DISTINCT CASE WHEN wmu.id IS NULL THEN c.id END) AS fully_patched,
    COUNT(DISTINCT CASE WHEN wmu.severity = 'Critical' THEN c.id END) AS missing_critical,
    COUNT(DISTINCT CASE WHEN wmu.severity = 'Important' THEN c.id END) AS missing_important,
    ROUND(
        100.0 * COUNT(DISTINCT CASE WHEN wmu.id IS NULL THEN c.id END) / 
        NULLIF(COUNT(DISTINCT c.id), 0), 
        2
    ) AS compliance_percentage
FROM computers c
LEFT JOIN employees e ON c.employee_id = e.id
LEFT JOIN departments d ON e.department_id = d.id
LEFT JOIN windows_missing_updates wmu ON c.id = wmu.computer_id
WHERE c.status = 'active'
GROUP BY d.name;

-- Комментарии
COMMENT ON TABLE patch_schedules IS 'Расписания установки патчей';
COMMENT ON TABLE patch_groups IS 'Группы компьютеров для тестирования патчей';
COMMENT ON TABLE patch_group_members IS 'Компьютеры в группах патчей';
COMMENT ON TABLE patch_deployments IS 'Задания на развёртывание патчей';
COMMENT ON TABLE patch_deployment_results IS 'Результаты установки патчей на компьютеры';
COMMENT ON VIEW v_patch_compliance IS 'Статистика соответствия патчам по отделам';
