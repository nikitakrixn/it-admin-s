-- ============================================================================
-- Система быстрых заметок
-- ============================================================================

-- Таблица заметок (можно привязать к любой сущности)
CREATE TABLE notes (
    id SERIAL PRIMARY KEY,
    entity_type VARCHAR(50) NOT NULL,
    entity_id INTEGER NOT NULL,
    title VARCHAR(255),
    content TEXT NOT NULL,
    note_type VARCHAR(20) DEFAULT 'info',
    is_pinned BOOLEAN DEFAULT false,
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_note_type CHECK (note_type IN ('info', 'warning', 'important', 'tip', 'issue'))
);

-- Индексы
CREATE INDEX idx_notes_entity ON notes(entity_type, entity_id);
CREATE INDEX idx_notes_created_by ON notes(created_by);
CREATE INDEX idx_notes_pinned ON notes(is_pinned) WHERE is_pinned = true;
CREATE INDEX idx_notes_created_at ON notes(created_at DESC);

-- Триггер для updated_at
CREATE TRIGGER update_notes_updated_at BEFORE UPDATE ON notes
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Комментарии
COMMENT ON TABLE notes IS 'Быстрые заметки к любым объектам системы';
COMMENT ON COLUMN notes.entity_type IS 'Тип объекта: computer, employee, equipment, request, etc.';
COMMENT ON COLUMN notes.entity_id IS 'ID объекта';
COMMENT ON COLUMN notes.is_pinned IS 'Закреплённая заметка (показывается первой)';
