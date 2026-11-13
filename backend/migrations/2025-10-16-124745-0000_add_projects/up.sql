-- ============================================================================
-- Проекты
-- ============================================================================

-- Проекты
CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    project_type VARCHAR(50),
    status VARCHAR(20) DEFAULT 'planning',
    priority VARCHAR(20) DEFAULT 'medium',
    start_date DATE,
    end_date DATE,
    estimated_budget DECIMAL(15,2),
    actual_cost DECIMAL(15,2) DEFAULT 0,
    progress_percent INTEGER DEFAULT 0,
    project_manager_id INTEGER REFERENCES employees(id) ON DELETE SET NULL,
    department_id INTEGER REFERENCES departments(id) ON DELETE SET NULL,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_project_type CHECK (project_type IN ('infrastructure', 'migration', 'upgrade', 'deployment', 'maintenance', 'other')),
    CONSTRAINT check_project_status CHECK (status IN ('planning', 'in_progress', 'on_hold', 'completed', 'cancelled')),
    CONSTRAINT check_priority CHECK (priority IN ('low', 'medium', 'high', 'critical')),
    CONSTRAINT check_progress CHECK (progress_percent >= 0 AND progress_percent <= 100)
);

-- Участники проекта
CREATE TABLE project_members (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    employee_id INTEGER NOT NULL REFERENCES employees(id) ON DELETE CASCADE,
    role VARCHAR(100),
    joined_at DATE DEFAULT CURRENT_DATE,
    left_at DATE,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(project_id, employee_id)
);

-- Связь проектов с заявками
CREATE TABLE project_requests (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    request_id INTEGER NOT NULL REFERENCES requests(id) ON DELETE CASCADE,
    added_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(project_id, request_id)
);

-- Связь проектов с активами
CREATE TABLE project_assets (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    asset_type VARCHAR(50) NOT NULL,
    asset_id INTEGER NOT NULL,
    purpose TEXT,
    added_at DATE DEFAULT CURRENT_DATE,
    removed_at DATE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(project_id, asset_type, asset_id)
);

-- Индексы
CREATE INDEX idx_projects_status ON projects(status);
CREATE INDEX idx_projects_manager ON projects(project_manager_id);
CREATE INDEX idx_projects_department ON projects(department_id);
CREATE INDEX idx_projects_dates ON projects(start_date, end_date);
CREATE INDEX idx_project_members_project ON project_members(project_id);
CREATE INDEX idx_project_members_employee ON project_members(employee_id);
CREATE INDEX idx_project_requests_project ON project_requests(project_id);
CREATE INDEX idx_project_requests_request ON project_requests(request_id);
CREATE INDEX idx_project_assets_project ON project_assets(project_id);
CREATE INDEX idx_project_assets_asset ON project_assets(asset_type, asset_id);

-- Триггеры
CREATE TRIGGER update_projects_updated_at BEFORE UPDATE ON projects
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Комментарии
COMMENT ON TABLE projects IS 'IT-проекты (миграции, обновления, развёртывания)';
COMMENT ON TABLE project_members IS 'Участники проектов';
COMMENT ON TABLE project_requests IS 'Связь проектов с заявками';
COMMENT ON TABLE project_assets IS 'Связь проектов с оборудованием и компьютерами';
