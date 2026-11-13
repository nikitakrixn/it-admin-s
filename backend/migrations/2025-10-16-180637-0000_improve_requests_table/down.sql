-- Откат миграции
ALTER TABLE requests 
    DROP CONSTRAINT IF EXISTS check_satisfaction_rating,
    DROP COLUMN IF EXISTS recurring_schedule,
    DROP COLUMN IF EXISTS knowledge_base_article_id,
    DROP COLUMN IF EXISTS satisfaction_rating,
    DROP COLUMN IF EXISTS satisfaction_comment;

DROP INDEX IF EXISTS idx_requests_kb_article;
DROP INDEX IF EXISTS idx_requests_recurring;
