-- ============================================================================
-- Документы и файлы
-- ============================================================================

-- Категории документов
CREATE TABLE document_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    parent_id INTEGER REFERENCES document_categories(id) ON DELETE CASCADE,
    description TEXT,
    icon VARCHAR(50),
    sort_order INTEGER DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Документы
CREATE TABLE documents (
    id SERIAL PRIMARY KEY,
    category_id INTEGER REFERENCES document_categories(id) ON DELETE SET NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    document_type VARCHAR(50),
    filename VARCHAR(255) NOT NULL,
    file_path VARCHAR(500) NOT NULL,
    file_size BIGINT,
    mime_type VARCHAR(100),
    version VARCHAR(20) DEFAULT '1.0',
    is_confidential BOOLEAN DEFAULT false,
    entity_type VARCHAR(50),
    entity_id INTEGER,
    uploaded_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_document_type CHECK (document_type IN ('manual', 'instruction', 'policy', 'contract', 'invoice', 'diagram', 'report', 'other'))
);

-- Права доступа к документам
CREATE TABLE document_permissions (
    id SERIAL PRIMARY KEY,
    document_id INTEGER NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(50),
    permission_type VARCHAR(20) NOT NULL,
    granted_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_permission_type CHECK (permission_type IN ('view', 'edit', 'delete', 'share')),
    UNIQUE(document_id, user_id, permission_type),
    CONSTRAINT check_user_or_role CHECK (
        (user_id IS NOT NULL AND role IS NULL) OR
        (user_id IS NULL AND role IS NOT NULL)
    )
);

-- История версий документов
CREATE TABLE document_versions (
    id SERIAL PRIMARY KEY,
    document_id INTEGER NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    version VARCHAR(20) NOT NULL,
    filename VARCHAR(255) NOT NULL,
    file_path VARCHAR(500) NOT NULL,
    file_size BIGINT,
    change_notes TEXT,
    uploaded_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Индексы
CREATE INDEX idx_document_categories_parent ON document_categories(parent_id);
CREATE INDEX idx_documents_category ON documents(category_id);
CREATE INDEX idx_documents_type ON documents(document_type);
CREATE INDEX idx_documents_entity ON documents(entity_type, entity_id);
CREATE INDEX idx_documents_uploaded_by ON documents(uploaded_by);
CREATE INDEX idx_documents_confidential ON documents(is_confidential);
CREATE INDEX idx_document_permissions_document ON document_permissions(document_id);
CREATE INDEX idx_document_permissions_user ON document_permissions(user_id);
CREATE INDEX idx_document_versions_document ON document_versions(document_id, created_at DESC);

-- Полнотекстовый поиск
CREATE INDEX idx_documents_title_search ON documents USING gin(to_tsvector('russian', title));
CREATE INDEX idx_documents_description_search ON documents USING gin(to_tsvector('russian', description));

-- Триггер
CREATE TRIGGER update_documents_updated_at BEFORE UPDATE ON documents
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Категории документов
INSERT INTO document_categories (name, description, icon, sort_order) VALUES 
    ('Инструкции', 'Инструкции по работе с оборудованием и ПО', 'book', 1),
    ('Схемы сети', 'Схемы сетевой инфраструктуры', 'network', 2),
    ('Политики безопасности', 'Документы по информационной безопасности', 'shield', 3),
    ('Контракты', 'Договоры с поставщиками', 'file-text', 4),
    ('Отчёты', 'Отчёты и аналитика', 'chart', 5),
    ('Пароли', 'Зашифрованные файлы с паролями', 'lock', 6);

-- Комментарии
COMMENT ON TABLE document_categories IS 'Категории документов';
COMMENT ON TABLE documents IS 'Документы и файлы (инструкции, схемы, контракты)';
COMMENT ON TABLE document_permissions IS 'Права доступа к документам';
COMMENT ON TABLE document_versions IS 'История версий документов';
COMMENT ON COLUMN documents.is_confidential IS 'Конфиденциальный документ (требует дополнительных прав)';
COMMENT ON COLUMN documents.entity_type IS 'Тип объекта, к которому привязан документ';
COMMENT ON COLUMN documents.entity_id IS 'ID объекта, к которому привязан документ';
