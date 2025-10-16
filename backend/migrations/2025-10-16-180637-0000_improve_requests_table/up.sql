-- ============================================================================
-- Улучшения таблицы заявок
-- ============================================================================

ALTER TABLE requests 
    ADD COLUMN recurring_schedule VARCHAR(50),
    ADD COLUMN knowledge_base_article_id INTEGER,
    ADD COLUMN satisfaction_rating INTEGER,
    ADD COLUMN satisfaction_comment TEXT;

-- Индексы
CREATE INDEX idx_requests_kb_article ON requests(knowledge_base_article_id) WHERE knowledge_base_article_id IS NOT NULL;
CREATE INDEX idx_requests_recurring ON requests(recurring_schedule) WHERE recurring_schedule IS NOT NULL;

-- Комментарии
COMMENT ON COLUMN requests.recurring_schedule IS 'Расписание для повторяющихся задач (daily, weekly, monthly)';
COMMENT ON COLUMN requests.knowledge_base_article_id IS 'Ссылка на статью базы знаний';
COMMENT ON COLUMN requests.satisfaction_rating IS 'Оценка удовлетворённости (1-5)';
COMMENT ON COLUMN requests.satisfaction_comment IS 'Комментарий к оценке';

-- Добавляем constraint
ALTER TABLE requests 
    ADD CONSTRAINT check_satisfaction_rating 
    CHECK (satisfaction_rating IS NULL OR (satisfaction_rating BETWEEN 1 AND 5));
