-- Откат миграции
ALTER TABLE computer_storage 
    DROP COLUMN IF EXISTS last_defrag_date,
    DROP COLUMN IF EXISTS fragmentation_percent,
    DROP COLUMN IF EXISTS read_errors,
    DROP COLUMN IF EXISTS write_errors,
    DROP COLUMN IF EXISTS wear_level_percent,
    DROP COLUMN IF EXISTS total_bytes_written,
    DROP COLUMN IF EXISTS power_cycle_count;

DROP INDEX IF EXISTS idx_computer_storage_health;
DROP INDEX IF EXISTS idx_computer_storage_fragmentation;
