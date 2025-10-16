-- Откат изменений
DROP VIEW IF EXISTS v_request_statistics;
DROP VIEW IF EXISTS v_computers_need_attention;

-- Откат изменений equipment
ALTER TABLE equipment 
    DROP COLUMN IF EXISTS last_maintenance_date,
    DROP COLUMN IF EXISTS next_maintenance_date,
    DROP COLUMN IF EXISTS toner_level,
    DROP COLUMN IF EXISTS page_count;

-- Откат изменений computer_storage
ALTER TABLE computer_storage 
    DROP COLUMN IF EXISTS smart_status,
    DROP COLUMN IF EXISTS temperature,
    DROP COLUMN IF EXISTS power_on_hours,
    DROP COLUMN IF EXISTS reallocated_sectors;

-- Откат изменений users
DROP INDEX IF EXISTS idx_users_locked;

ALTER TABLE users 
    DROP COLUMN IF EXISTS two_factor_enabled,
    DROP COLUMN IF EXISTS two_factor_secret,
    DROP COLUMN IF EXISTS password_changed_at,
    DROP COLUMN IF EXISTS must_change_password,
    DROP COLUMN IF EXISTS failed_login_attempts,
    DROP COLUMN IF EXISTS locked_until;

-- Откат изменений computer_monitoring
DROP INDEX IF EXISTS idx_monitoring_latest;

-- Откат изменений software_catalog
DROP INDEX IF EXISTS idx_software_catalog_deprecated;
DROP INDEX IF EXISTS idx_software_catalog_risk;
DROP INDEX IF EXISTS idx_software_catalog_vendor;

ALTER TABLE software_catalog 
    DROP COLUMN IF EXISTS latest_version,
    DROP COLUMN IF EXISTS is_deprecated,
    DROP COLUMN IF EXISTS security_risk_level,
    DROP COLUMN IF EXISTS vendor_id;

-- Откат изменений requests
DROP INDEX IF EXISTS idx_requests_sla_deadline;
DROP INDEX IF EXISTS idx_requests_sla_breached;
DROP INDEX IF EXISTS idx_requests_parent;

ALTER TABLE requests 
    DROP COLUMN IF EXISTS first_response_at,
    DROP COLUMN IF EXISTS sla_deadline,
    DROP COLUMN IF EXISTS sla_breached,
    DROP COLUMN IF EXISTS estimated_time,
    DROP COLUMN IF EXISTS actual_time,
    DROP COLUMN IF EXISTS parent_request_id;

-- Откат изменений employees
DROP INDEX IF EXISTS idx_employees_vip;

ALTER TABLE employees 
    DROP COLUMN IF EXISTS photo_url,
    DROP COLUMN IF EXISTS telegram_username,
    DROP COLUMN IF EXISTS is_vip;
