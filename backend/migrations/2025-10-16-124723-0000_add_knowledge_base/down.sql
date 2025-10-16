-- Откат изменений
DROP TRIGGER IF EXISTS update_kb_categories_updated_at ON kb_categories;
DROP TRIGGER IF EXISTS update_kb_articles_updated_at ON knowledge_base_articles;

DROP INDEX IF EXISTS idx_kb_articles_title_search;
DROP INDEX IF EXISTS idx_kb_articles_content_search;

DROP TABLE IF EXISTS kb_article_attachments;
DROP TABLE IF EXISTS kb_article_requests;
DROP TABLE IF EXISTS knowledge_base_articles;
DROP TABLE IF EXISTS kb_categories;
