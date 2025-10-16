-- ============================================================================
-- Отслеживание резервного копирования
-- ============================================================================

-- Политики резервного копирования
CREATE TABLE backup_policies (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    backup_type VARCHAR(50) NOT NULL,
    frequency VARCHAR(50) NOT NULL,
    retention_days INTEGER NOT NULL,
    backup_location VARCHAR(500),
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_backup_type CHECK (backup_type IN ('full', 'incremental', 'differential', 'snapshot')),
    CONSTRAINT check_frequency CHECK (frequency IN ('hourly', 'daily', 'weekly', 'monthly', 'manual'))
);

-- Задания резервного копирования
CREATE TABLE backup_jobs (
    id SERIAL PRIMARY KEY,
    policy_id INTEGER REFERENCES backup_policies(id) ON DELETE SET NULL,
    entity_type VARCHAR(50) NOT NULL,
    entity_id INTEGER NOT NULL,
    job_name VARCHAR(255) NOT NULL,
    backup_type VARCHAR(50) NOT NULL,
    status VARCHAR(20) DEFAULT 'pending',
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    duration_seconds INTEGER,
    backup_size_mb BIGINT,
    backup_location VARCHAR(500),
    error_message TEXT,
    next_run_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_job_status CHECK (status IN ('pending', 'running', 'completed', 'failed', 'cancelled'))
);

-- История резервного копирования
CREATE TABLE backup_history (
    id SERIAL PRIMARY KEY,
    job_id INTEGER REFERENCES backup_jobs(id) ON DELETE SET NULL,
    entity_type VARCHAR(50) NOT NULL,
    entity_id INTEGER NOT NULL,
    backup_type VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL,
    started_at TIMESTAMP NOT NULL,
    completed_at TIMESTAMP,
    duration_seconds INTEGER,
    backup_size_mb BIGINT,
    backup_location VARCHAR(500),
    files_count INTEGER,
    success_rate DECIMAL(5,2),
    error_message TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_history_status CHECK (status IN ('success', 'partial', 'failed'))
);

-- Индексы
CREATE INDEX idx_backup_policies_active ON backup_policies(is_active) WHERE is_active = true;
CREATE INDEX idx_backup_jobs_entity ON backup_jobs(entity_type, entity_id);
CREATE INDEX idx_backup_jobs_status ON backup_jobs(status);
CREATE INDEX idx_backup_jobs_next_run ON backup_jobs(next_run_at) WHERE status = 'pending';
CREATE INDEX idx_backup_history_entity ON backup_history(entity_type, entity_id);
CREATE INDEX idx_backup_history_started ON backup_history(started_at DESC);
CREATE INDEX idx_backup_history_status ON backup_history(status);

-- Триггеры
CREATE TRIGGER update_backup_policies_updated_at BEFORE UPDATE ON backup_policies
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Представление: последние бэкапы для каждого объекта
CREATE OR REPLACE VIEW v_latest_backups AS
SELECT DISTINCT ON (bh.entity_type, bh.entity_id)
    bh.entity_type,
    bh.entity_id,
    bh.backup_type,
    bh.status,
    bh.started_at,
    bh.completed_at,
    bh.backup_size_mb,
    bh.backup_location,
    CASE 
        WHEN bh.completed_at < CURRENT_TIMESTAMP - INTERVAL '7 days' THEN 'outdated'
        WHEN bh.status = 'failed' THEN 'failed'
        WHEN bh.status = 'partial' THEN 'warning'
        ELSE 'ok'
    END AS backup_health
FROM backup_history bh
ORDER BY bh.entity_type, bh.entity_id, bh.started_at DESC;

-- Комментарии
COMMENT ON TABLE backup_policies IS 'Политики резервного копирования';
COMMENT ON TABLE backup_jobs IS 'Задания резервного копирования';
COMMENT ON TABLE backup_history IS 'История выполнения резервного копирования';
COMMENT ON VIEW v_latest_backups IS 'Последние бэкапы для каждого объекта с оценкой состояния';
