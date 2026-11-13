-- ============================================================================
-- История изменений заявок
-- ============================================================================

-- Таблица истории изменений заявок
CREATE TABLE request_history (
    id SERIAL PRIMARY KEY,
    request_id INTEGER NOT NULL REFERENCES requests(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    field_name VARCHAR(50) NOT NULL,
    old_value TEXT,
    new_value TEXT,
    change_type VARCHAR(20) NOT NULL,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_change_type CHECK (change_type IN ('status', 'priority', 'assignee', 'category', 'field_update', 'created', 'closed'))
);

-- Таблица связей между заявками
CREATE TABLE request_relations (
    id SERIAL PRIMARY KEY,
    request_id INTEGER NOT NULL REFERENCES requests(id) ON DELETE CASCADE,
    related_request_id INTEGER NOT NULL REFERENCES requests(id) ON DELETE CASCADE,
    relation_type VARCHAR(20) NOT NULL,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_relation_type CHECK (relation_type IN ('duplicate', 'related', 'blocks', 'blocked_by', 'parent', 'child')),
    CONSTRAINT check_not_self_related CHECK (request_id != related_request_id),
    UNIQUE(request_id, related_request_id, relation_type)
);

-- Таблица тегов для заявок
CREATE TABLE request_tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    color VARCHAR(7) DEFAULT '#6B7280',
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Связь заявок и тегов
CREATE TABLE request_tag_relations (
    id SERIAL PRIMARY KEY,
    request_id INTEGER NOT NULL REFERENCES requests(id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES request_tags(id) ON DELETE CASCADE,
    added_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(request_id, tag_id)
);

-- Индексы
CREATE INDEX idx_request_history_request ON request_history(request_id, created_at DESC);
CREATE INDEX idx_request_history_user ON request_history(user_id);
CREATE INDEX idx_request_relations_request ON request_relations(request_id);
CREATE INDEX idx_request_relations_related ON request_relations(related_request_id);
CREATE INDEX idx_request_tag_relations_request ON request_tag_relations(request_id);
CREATE INDEX idx_request_tag_relations_tag ON request_tag_relations(tag_id);

-- Триггер для автоматического логирования изменений заявок
CREATE OR REPLACE FUNCTION log_request_changes()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO request_history (request_id, field_name, new_value, change_type)
        VALUES (NEW.id, 'created', 'Request created', 'created');
    ELSIF TG_OP = 'UPDATE' THEN
        IF OLD.status != NEW.status THEN
            INSERT INTO request_history (request_id, field_name, old_value, new_value, change_type)
            VALUES (NEW.id, 'status', OLD.status, NEW.status, 'status');
        END IF;
        IF OLD.priority != NEW.priority THEN
            INSERT INTO request_history (request_id, field_name, old_value, new_value, change_type)
            VALUES (NEW.id, 'priority', OLD.priority, NEW.priority, 'priority');
        END IF;
        IF (OLD.assignee_id IS DISTINCT FROM NEW.assignee_id) THEN
            INSERT INTO request_history (request_id, field_name, old_value, new_value, change_type)
            VALUES (NEW.id, 'assignee', OLD.assignee_id::TEXT, NEW.assignee_id::TEXT, 'assignee');
        END IF;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER log_request_changes_trigger
AFTER INSERT OR UPDATE ON requests
FOR EACH ROW EXECUTE FUNCTION log_request_changes();

-- Начальные теги
INSERT INTO request_tags (name, color, description) VALUES 
    ('Срочно', '#EF4444', 'Требует немедленного внимания'),
    ('VIP', '#F59E0B', 'VIP пользователь'),
    ('Повторяющаяся', '#8B5CF6', 'Повторяющаяся проблема'),
    ('Требует закупки', '#10B981', 'Необходима закупка оборудования'),
    ('Запланировано', '#3B82F6', 'Запланированная работа');

-- Комментарии
COMMENT ON TABLE request_history IS 'История всех изменений заявок';
COMMENT ON TABLE request_relations IS 'Связи между заявками (дубликаты, зависимости)';
COMMENT ON TABLE request_tags IS 'Теги для категоризации заявок';
COMMENT ON TABLE request_tag_relations IS 'Связь заявок с тегами';
