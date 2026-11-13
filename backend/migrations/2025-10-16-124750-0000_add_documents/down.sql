-- Откат изменений
DROP TRIGGER IF EXISTS update_documents_updated_at ON documents;

DROP INDEX IF EXISTS idx_documents_title_search;
DROP INDEX IF EXISTS idx_documents_description_search;

DROP TABLE IF EXISTS document_versions;
DROP TABLE IF EXISTS document_permissions;
DROP TABLE IF EXISTS documents;
DROP TABLE IF EXISTS document_categories;
