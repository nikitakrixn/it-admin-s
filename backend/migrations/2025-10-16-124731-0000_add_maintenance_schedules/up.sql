-- ============================================================================
-- Планирование обслуживания
-- ============================================================================

-- Расписание обслуживания
CREATE TABLE maintenance_schedules (
    id SERIAL PRIMARY KEY,
    entity_type VARCHAR(50) NOT NULL,
    entity_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    maintenance_type VARCHAR(50) NOT NULL,
    frequency VARCHAR(50) NOT NULL,
    next_maintenance_date DATE NOT NULL,
    last_maintenance_date DATE,
    estimated_duration INTEGER,
    assigned_to INTEGER REFERENCES employees(id) ON DELETE SET NULL,
    is_active BOOLEAN DEFAULT true,
    priority VARCHAR(20) DEFAULT 'medium',
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_maintenance_type CHECK (maintenance_type IN ('cleaning', 'inspection', 'update', 'replacement', 'calibration', 'testing', 'other')),
    CONSTRAINT check_frequency CHECK (frequency IN ('daily', 'weekly', 'monthly', 'quarterly', 'yearly', 'once')),
    CONSTRAINT check_priority CHECK (priority IN ('low', 'medium', 'high', 'critical'))
);

-- История обслуживания
CREATE TABLE maintenance_history (
    id SERIAL PRIMARY KEY,
    schedule_id INTEGER REFERENCES maintenance_schedules(id) ON DELETE SET NULL,
    entity_type VARCHAR(50) NOT NULL,
    entity_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    maintenance_type VARCHAR(50) NOT NULL,
    performed_by INTEGER REFERENCES employees(id) ON DELETE SET NULL,
    scheduled_date DATE,
    actual_date DATE NOT NULL,
    duration_minutes INTEGER,
    status VARCHAR(20) NOT NULL,
    work_performed TEXT,
    parts_replaced TEXT,
    cost DECIMAL(10,2),
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_maintenance_status CHECK (status IN ('completed', 'partial', 'cancelled', 'failed'))
);

-- Индексы
CREATE INDEX idx_maintenance_schedules_entity ON maintenance_schedules(entity_type, entity_id);
CREATE INDEX idx_maintenance_schedules_next_date ON maintenance_schedules(next_maintenance_date) WHERE is_active = true;
CREATE INDEX idx_maintenance_schedules_assigned ON maintenance_schedules(assigned_to);
CREATE INDEX idx_maintenance_history_entity ON maintenance_history(entity_type, entity_id);
CREATE INDEX idx_maintenance_history_schedule ON maintenance_history(schedule_id);
CREATE INDEX idx_maintenance_history_date ON maintenance_history(actual_date DESC);

-- Триггеры
CREATE TRIGGER update_maintenance_schedules_updated_at BEFORE UPDATE ON maintenance_schedules
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Функция для автоматического обновления следующей даты обслуживания
CREATE OR REPLACE FUNCTION update_next_maintenance_date()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.status = 'completed' AND OLD.schedule_id IS NOT NULL THEN
        UPDATE maintenance_schedules
        SET 
            last_maintenance_date = NEW.actual_date,
            next_maintenance_date = CASE frequency
                WHEN 'daily' THEN NEW.actual_date + INTERVAL '1 day'
                WHEN 'weekly' THEN NEW.actual_date + INTERVAL '1 week'
                WHEN 'monthly' THEN NEW.actual_date + INTERVAL '1 month'
                WHEN 'quarterly' THEN NEW.actual_date + INTERVAL '3 months'
                WHEN 'yearly' THEN NEW.actual_date + INTERVAL '1 year'
                ELSE next_maintenance_date
            END
        WHERE id = OLD.schedule_id AND frequency != 'once';
        
        -- Деактивировать одноразовые задачи
        UPDATE maintenance_schedules
        SET is_active = false
        WHERE id = OLD.schedule_id AND frequency = 'once';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_next_maintenance_trigger
AFTER INSERT OR UPDATE ON maintenance_history
FOR EACH ROW EXECUTE FUNCTION update_next_maintenance_date();

-- Представление: предстоящее обслуживание
CREATE OR REPLACE VIEW v_upcoming_maintenance AS
SELECT 
    ms.id,
    ms.entity_type,
    ms.entity_id,
    ms.title,
    ms.maintenance_type,
    ms.next_maintenance_date,
    ms.priority,
    e.first_name || ' ' || e.last_name AS assigned_to_name,
    CASE 
        WHEN ms.next_maintenance_date < CURRENT_DATE THEN 'overdue'
        WHEN ms.next_maintenance_date = CURRENT_DATE THEN 'today'
        WHEN ms.next_maintenance_date <= CURRENT_DATE + INTERVAL '7 days' THEN 'this_week'
        WHEN ms.next_maintenance_date <= CURRENT_DATE + INTERVAL '30 days' THEN 'this_month'
        ELSE 'future'
    END AS urgency
FROM maintenance_schedules ms
LEFT JOIN employees e ON ms.assigned_to = e.id
WHERE ms.is_active = true
ORDER BY ms.next_maintenance_date ASC;

-- Комментарии
COMMENT ON TABLE maintenance_schedules IS 'Расписание планового обслуживания';
COMMENT ON TABLE maintenance_history IS 'История выполненного обслуживания';
COMMENT ON VIEW v_upcoming_maintenance IS 'Предстоящее обслуживание с оценкой срочности';
