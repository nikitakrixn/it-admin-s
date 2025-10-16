-- ============================================================================
-- База знаний
-- ============================================================================

-- Категории базы знаний
CREATE TABLE kb_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    parent_id INTEGER REFERENCES kb_categories(id) ON DELETE CASCADE,
    description TEXT,
    icon VARCHAR(50),
    sort_order INTEGER DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Статьи базы знаний
CREATE TABLE knowledge_base_articles (
    id SERIAL PRIMARY KEY,
    category_id INTEGER REFERENCES kb_categories(id) ON DELETE SET NULL,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    summary TEXT,
    keywords TEXT,
    is_published BOOLEAN DEFAULT true,
    view_count INTEGER DEFAULT 0,
    helpful_count INTEGER DEFAULT 0,
    not_helpful_count INTEGER DEFAULT 0,
    author_id UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Связь статей с заявками
CREATE TABLE kb_article_requests (
    id SERIAL PRIMARY KEY,
    article_id INTEGER NOT NULL REFERENCES knowledge_base_articles(id) ON DELETE CASCADE,
    request_id INTEGER NOT NULL REFERENCES requests(id) ON DELETE CASCADE,
    linked_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(article_id, request_id)
);

-- Вложения к статьям
CREATE TABLE kb_article_attachments (
    id SERIAL PRIMARY KEY,
    article_id INTEGER NOT NULL REFERENCES knowledge_base_articles(id) ON DELETE CASCADE,
    filename VARCHAR(255) NOT NULL,
    file_path VARCHAR(500) NOT NULL,
    file_size INTEGER,
    mime_type VARCHAR(100),
    uploaded_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Индексы
CREATE INDEX idx_kb_categories_parent ON kb_categories(parent_id);
CREATE INDEX idx_kb_articles_category ON knowledge_base_articles(category_id);
CREATE INDEX idx_kb_articles_published ON knowledge_base_articles(is_published) WHERE is_published = true;
CREATE INDEX idx_kb_articles_author ON knowledge_base_articles(author_id);
CREATE INDEX idx_kb_article_requests_article ON kb_article_requests(article_id);
CREATE INDEX idx_kb_article_requests_request ON kb_article_requests(request_id);
CREATE INDEX idx_kb_article_attachments_article ON kb_article_attachments(article_id);

-- Полнотекстовый поиск
CREATE INDEX idx_kb_articles_title_search ON knowledge_base_articles USING gin(to_tsvector('russian', title));
CREATE INDEX idx_kb_articles_content_search ON knowledge_base_articles USING gin(to_tsvector('russian', content));

-- Триггеры
CREATE TRIGGER update_kb_categories_updated_at BEFORE UPDATE ON kb_categories
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_kb_articles_updated_at BEFORE UPDATE ON knowledge_base_articles
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Начальные категории
INSERT INTO kb_categories (name, description, icon, sort_order) VALUES 
    ('Аппаратное обеспечение', 'Проблемы с железом', 'hardware', 1),
    ('Программное обеспечение', 'Установка и настройка ПО', 'software', 2),
    ('Сеть', 'Сетевые проблемы', 'network', 3),
    ('Принтеры', 'Проблемы с печатью', 'printer', 4),
    ('Учётные записи', 'Пароли и доступы', 'account', 5),
    ('Инструкции', 'Пошаговые руководства', 'guide', 6);

-- Комментарии
COMMENT ON TABLE kb_categories IS 'Категории базы знаний';
COMMENT ON TABLE knowledge_base_articles IS 'Статьи базы знаний с решениями типовых проблем';
COMMENT ON TABLE kb_article_requests IS 'Связь статей базы знаний с заявками';
COMMENT ON TABLE kb_article_attachments IS 'Вложения к статьям (скриншоты, инструкции)';
