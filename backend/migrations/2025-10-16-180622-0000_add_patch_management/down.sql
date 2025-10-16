-- Откат миграции
DROP VIEW IF EXISTS v_patch_compliance;
DROP TABLE IF EXISTS patch_deployment_results CASCADE;
DROP TABLE IF EXISTS patch_deployments CASCADE;
DROP TABLE IF EXISTS patch_group_members CASCADE;
DROP TABLE IF EXISTS patch_groups CASCADE;
DROP TABLE IF EXISTS patch_schedules CASCADE;
