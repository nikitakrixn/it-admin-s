-- Откат миграции
DROP VIEW IF EXISTS v_certificates_expiring;
DROP TABLE IF EXISTS certificate_renewals CASCADE;
DROP TABLE IF EXISTS certificates CASCADE;
