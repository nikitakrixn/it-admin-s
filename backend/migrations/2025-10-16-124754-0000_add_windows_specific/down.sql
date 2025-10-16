-- Откат изменений
DROP VIEW IF EXISTS v_missing_critical_updates;
DROP VIEW IF EXISTS v_critical_windows_events;

DROP TRIGGER IF EXISTS update_windows_services_updated_at ON windows_services;
DROP TRIGGER IF EXISTS update_shared_folders_updated_at ON shared_folders;

DROP TABLE IF EXISTS scheduled_tasks;
DROP TABLE IF EXISTS firewall_rules;
DROP TABLE IF EXISTS rdp_sessions;
DROP TABLE IF EXISTS windows_features;
DROP TABLE IF EXISTS share_permissions;
DROP TABLE IF EXISTS shared_folders;
DROP TABLE IF EXISTS computer_gpo_applications;
DROP TABLE IF EXISTS group_policies;
DROP TABLE IF EXISTS ad_sync_history;
DROP TABLE IF EXISTS ad_group_memberships;
DROP TABLE IF EXISTS ad_groups;
DROP TABLE IF EXISTS windows_events;
DROP TABLE IF EXISTS service_failures;
DROP TABLE IF EXISTS windows_services;
DROP TABLE IF EXISTS windows_missing_updates;
DROP TABLE IF EXISTS windows_updates;
