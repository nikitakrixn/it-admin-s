-- Откат изменений
DROP TRIGGER IF EXISTS log_request_changes_trigger ON requests;
DROP FUNCTION IF EXISTS log_request_changes();

DROP TABLE IF EXISTS request_tag_relations;
DROP TABLE IF EXISTS request_tags;
DROP TABLE IF EXISTS request_relations;
DROP TABLE IF EXISTS request_history;
