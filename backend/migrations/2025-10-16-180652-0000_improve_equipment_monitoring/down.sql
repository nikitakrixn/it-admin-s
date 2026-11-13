-- Откат миграции
ALTER TABLE equipment 
    DROP COLUMN IF EXISTS paper_jam_count,
    DROP COLUMN IF EXISTS total_pages_printed,
    DROP COLUMN IF EXISTS color_pages_printed,
    DROP COLUMN IF EXISTS mono_pages_printed,
    DROP COLUMN IF EXISTS last_error_code,
    DROP COLUMN IF EXISTS last_error_time,
    DROP COLUMN IF EXISTS supplies_status;

DROP INDEX IF EXISTS idx_equipment_maintenance_due;
