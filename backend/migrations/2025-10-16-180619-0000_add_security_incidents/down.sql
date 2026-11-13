-- Откат миграции
DROP TRIGGER IF EXISTS generate_incident_number_trigger ON security_incidents;
DROP FUNCTION IF EXISTS generate_incident_number();
DROP TABLE IF EXISTS incident_evidence CASCADE;
DROP TABLE IF EXISTS security_incidents CASCADE;
