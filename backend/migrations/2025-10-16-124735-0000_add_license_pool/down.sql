-- Откат изменений
DROP VIEW IF EXISTS v_license_status;

DROP TRIGGER IF EXISTS update_license_pool_usage_trigger ON license_assignments;
DROP FUNCTION IF EXISTS update_license_pool_usage();

DROP TRIGGER IF EXISTS update_license_pools_updated_at ON license_pools;

DROP TABLE IF EXISTS license_assignments;
DROP TABLE IF EXISTS license_pools;
