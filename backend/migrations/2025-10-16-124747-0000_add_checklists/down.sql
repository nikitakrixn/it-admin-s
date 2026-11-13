-- Откат изменений
DROP TRIGGER IF EXISTS update_checklist_status_trigger ON checklist_items;
DROP FUNCTION IF EXISTS update_checklist_status();

DROP TRIGGER IF EXISTS update_checklist_templates_updated_at ON checklist_templates;
DROP TRIGGER IF EXISTS update_checklists_updated_at ON checklists;

DROP TABLE IF EXISTS checklist_items;
DROP TABLE IF EXISTS checklists;
DROP TABLE IF EXISTS checklist_template_items;
DROP TABLE IF EXISTS checklist_templates;
