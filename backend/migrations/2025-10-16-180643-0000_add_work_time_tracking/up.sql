-- ============================================================================
-- Учёт времени работы
-- ============================================================================

CREATE TABLE work_time_tracking (
    id SERIAL PRIMARY KEY,
    request_id INTEGER REFERENCES requests(id) ON DELETE CASCADE,
    employee_id INTEGER NOT NULL REFERENCES employees(id) ON DELETE CASCADE,
    work_type VARCHAR(50) NOT NULL,
    started_at TIMESTAMP NOT NULL,
    ended_at TIMESTAMP,
    duration_minutes INTEGER,
    work_description TEXT NOT NULL,
    is_billable BOOLEAN DEFAULT false,
    hourly_rate DECIMAL(10,2),
    total_cost DECIMAL(10,2),
    approved_by UUID REFERENCES users(id) ON DELETE SET NULL,
    approved_at TIMESTAMP,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_work_type CHECK (work_type IN (
        'installation', 'configuration', 'troubleshooting', 'maintenance', 
        'consultation', 'training', 'documentation', 'other'
    )),
    CONSTRAINT check_time_range CHECK (ended_at IS NULL OR ended_at >= started_at)
);

-- Индексы
CREATE INDEX idx_work_time_tracking_request ON work_time_tracking(request_id);
CREATE INDEX idx_work_time_tracking_employee ON work_time_tracking(employee_id);
CREATE INDEX idx_work_time_tracking_started ON work_time_tracking(started_at DESC);
CREATE INDEX idx_work_time_tracking_billable ON work_time_tracking(is_billable) WHERE is_billable = true;

-- Триггеры
CREATE TRIGGER update_work_time_tracking_updated_at BEFORE UPDATE ON work_time_tracking
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Функция для автоматического расчёта длительности
CREATE OR REPLACE FUNCTION calculate_work_duration()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.ended_at IS NOT NULL AND NEW.started_at IS NOT NULL THEN
        NEW.duration_minutes := EXTRACT(EPOCH FROM (NEW.ended_at - NEW.started_at)) / 60;
        
        IF NEW.hourly_rate IS NOT NULL THEN
            NEW.total_cost := (NEW.duration_minutes / 60.0) * NEW.hourly_rate;
        END IF;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER calculate_work_duration_trigger
BEFORE INSERT OR UPDATE ON work_time_tracking
FOR EACH ROW EXECUTE FUNCTION calculate_work_duration();

-- Представление: сводка по времени работы
CREATE OR REPLACE VIEW v_work_time_summary AS
SELECT 
    e.id AS employee_id,
    e.first_name || ' ' || e.last_name AS employee_name,
    d.name AS department_name,
    DATE_TRUNC('month', wtt.started_at) AS month,
    COUNT(*) AS total_entries,
    SUM(wtt.duration_minutes) AS total_minutes,
    ROUND(SUM(wtt.duration_minutes) / 60.0, 2) AS total_hours,
    SUM(CASE WHEN wtt.is_billable THEN wtt.duration_minutes ELSE 0 END) AS billable_minutes,
    SUM(wtt.total_cost) AS total_revenue
FROM work_time_tracking wtt
JOIN employees e ON wtt.employee_id = e.id
LEFT JOIN departments d ON e.department_id = d.id
WHERE wtt.ended_at IS NOT NULL
GROUP BY e.id, employee_name, d.name, DATE_TRUNC('month', wtt.started_at);

-- Комментарии
COMMENT ON TABLE work_time_tracking IS 'Учёт времени работы сотрудников';
COMMENT ON VIEW v_work_time_summary IS 'Сводка по времени работы по месяцам';
COMMENT ON COLUMN work_time_tracking.duration_minutes IS 'Автоматически вычисляется при заполнении ended_at';
COMMENT ON COLUMN work_time_tracking.total_cost IS 'Автоматически вычисляется на основе duration и hourly_rate';
