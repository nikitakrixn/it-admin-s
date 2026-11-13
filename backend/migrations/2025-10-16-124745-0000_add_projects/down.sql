-- Откат изменений
DROP TRIGGER IF EXISTS update_projects_updated_at ON projects;

DROP TABLE IF EXISTS project_assets;
DROP TABLE IF EXISTS project_requests;
DROP TABLE IF EXISTS project_members;
DROP TABLE IF EXISTS projects;
