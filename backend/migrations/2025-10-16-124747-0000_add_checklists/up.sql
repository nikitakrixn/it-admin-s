-- ============================================================================
-- Чек-листы
-- ============================================================================

-- Шаблоны чек-листов
CREATE TABLE checklist_templates (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    category VARCHAR(50),
    is_active BOOLEAN DEFAULT true,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Пункты шаблонов чек-листов
CREATE TABLE checklist_template_items (
    id SERIAL PRIMARY KEY,
    template_id INTEGER NOT NULL REFERENCES checklist_templates(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    sort_order INTEGER DEFAULT 0,
    is_required BOOLEAN DEFAULT false,
    estimated_minutes INTEGER,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Экземпляры чек-листов (привязанные к заявкам или другим объектам)
CREATE TABLE checklists (
    id SERIAL PRIMARY KEY,
    template_id INTEGER REFERENCES checklist_templates(id) ON DELETE SET NULL,
    entity_type VARCHAR(50) NOT NULL,
    entity_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    status VARCHAR(20) DEFAULT 'in_progress',
    assigned_to INTEGER REFERENCES employees(id) ON DELETE SET NULL,
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_checklist_status CHECK (status IN ('not_started', 'in_progress', 'completed', 'cancelled'))
);

-- Пункты чек-листов
CREATE TABLE checklist_items (
    id SERIAL PRIMARY KEY,
    checklist_id INTEGER NOT NULL REFERENCES checklists(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    sort_order INTEGER DEFAULT 0,
    is_required BOOLEAN DEFAULT false,
    is_completed BOOLEAN DEFAULT false,
    completed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    completed_at TIMESTAMP,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Индексы
CREATE INDEX idx_checklist_templates_category ON checklist_templates(category);
CREATE INDEX idx_checklist_templates_active ON checklist_templates(is_active) WHERE is_active = true;
CREATE INDEX idx_checklist_template_items_template ON checklist_template_items(template_id, sort_order);
CREATE INDEX idx_checklists_entity ON checklists(entity_type, entity_id);
CREATE INDEX idx_checklists_template ON checklists(template_id);
CREATE INDEX idx_checklists_assigned ON checklists(assigned_to);
CREATE INDEX idx_checklists_status ON checklists(status);
CREATE INDEX idx_checklist_items_checklist ON checklist_items(checklist_id, sort_order);
CREATE INDEX idx_checklist_items_completed ON checklist_items(is_completed);

-- Триггеры
CREATE TRIGGER update_checklist_templates_updated_at BEFORE UPDATE ON checklist_templates
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_checklists_updated_at BEFORE UPDATE ON checklists
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Функция для автоматического обновления статуса чек-листа
CREATE OR REPLACE FUNCTION update_checklist_status()
RETURNS TRIGGER AS $$
DECLARE
    total_items INTEGER;
    completed_items INTEGER;
    required_items INTEGER;
    completed_required INTEGER;
BEGIN
    SELECT 
        COUNT(*),
        COUNT(*) FILTER (WHERE is_completed = true),
        COUNT(*) FILTER (WHERE is_required = true),
        COUNT(*) FILTER (WHERE is_required = true AND is_completed = true)
    INTO total_items, completed_items, required_items, completed_required
    FROM checklist_items
    WHERE checklist_id = COALESCE(NEW.checklist_id, OLD.checklist_id);
    
    IF completed_items = total_items AND total_items > 0 THEN
        UPDATE checklists 
        SET status = 'completed', completed_at = CURRENT_TIMESTAMP
        WHERE id = COALESCE(NEW.checklist_id, OLD.checklist_id) AND status != 'completed';
    ELSIF completed_required = required_items AND required_items > 0 THEN
        UPDATE checklists 
        SET status = 'completed', completed_at = CURRENT_TIMESTAMP
        WHERE id = COALESCE(NEW.checklist_id, OLD.checklist_id) AND status != 'completed';
    ELSIF completed_items > 0 THEN
        UPDATE checklists 
        SET status = 'in_progress'
        WHERE id = COALESCE(NEW.checklist_id, OLD.checklist_id) AND status = 'not_started';
    END IF;
    
    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_checklist_status_trigger
AFTER INSERT OR UPDATE OR DELETE ON checklist_items
FOR EACH ROW EXECUTE FUNCTION update_checklist_status();

-- Типовые шаблоны чек-листов
INSERT INTO checklist_templates (name, description, category) VALUES 
    ('Настройка нового компьютера', 'Чек-лист для настройки рабочего места нового сотрудника', 'onboarding'),
    ('Увольнение сотрудника', 'Чек-лист для отзыва доступов при увольнении', 'offboarding'),
    ('Установка принтера', 'Чек-лист для подключения и настройки принтера', 'equipment'),
    ('Обновление сервера', 'Чек-лист для планового обновления сервера', 'maintenance');

-- Пункты для чек-листа "Настройка нового компьютера"
INSERT INTO checklist_template_items (template_id, title, sort_order, is_required, estimated_minutes) VALUES 
    (1, 'Установить Windows и драйверы', 1, true, 60),
    (1, 'Присоединить к домену', 2, true, 10),
    (1, 'Установить антивирус', 3, true, 15),
    (1, 'Установить Office', 4, true, 20),
    (1, 'Настроить принтеры', 5, false, 10),
    (1, 'Установить специализированное ПО', 6, false, 30),
    (1, 'Настроить почту', 7, true, 10),
    (1, 'Провести инструктаж пользователя', 8, true, 15);

-- Пункты для чек-листа "Увольнение сотрудника"
INSERT INTO checklist_template_items (template_id, title, sort_order, is_required) VALUES 
    (2, 'Заблокировать учётную запись AD', 1, true),
    (2, 'Отозвать доступ к почте', 2, true),
    (2, 'Отозвать VPN доступ', 3, true),
    (2, 'Забрать компьютер и оборудование', 4, true),
    (2, 'Удалить из групп безопасности', 5, true),
    (2, 'Сделать бэкап данных пользователя', 6, true),
    (2, 'Переназначить лицензии', 7, false);

-- Комментарии
COMMENT ON TABLE checklist_templates IS 'Шаблоны чек-листов для типовых задач';
COMMENT ON TABLE checklist_template_items IS 'Пункты шаблонов чек-листов';
COMMENT ON TABLE checklists IS 'Экземпляры чек-листов, привязанные к заявкам или другим объектам';
COMMENT ON TABLE checklist_items IS 'Пункты чек-листов с отметками о выполнении';
