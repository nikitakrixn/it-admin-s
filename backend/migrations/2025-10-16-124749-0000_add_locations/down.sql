-- Откат изменений
DROP VIEW IF EXISTS v_location_paths;

DROP INDEX IF EXISTS idx_computers_location_id;
DROP INDEX IF EXISTS idx_equipment_location_id;
DROP INDEX IF EXISTS idx_employees_location_id;

ALTER TABLE computers DROP COLUMN IF EXISTS location_id;
ALTER TABLE equipment DROP COLUMN IF EXISTS location_id;
ALTER TABLE employees DROP COLUMN IF EXISTS location_id;

DROP TRIGGER IF EXISTS update_locations_updated_at ON locations;

DROP TABLE IF EXISTS locations;
