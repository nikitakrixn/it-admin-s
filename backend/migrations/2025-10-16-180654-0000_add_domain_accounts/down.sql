-- Откат миграции
DROP VIEW IF EXISTS v_domain_accounts_expiring;
DROP TABLE IF EXISTS domain_account_history CASCADE;
DROP TABLE IF EXISTS domain_accounts CASCADE;
