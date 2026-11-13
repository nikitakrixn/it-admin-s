-- Откат изменений
DROP VIEW IF EXISTS v_expiring_contracts;

DROP TRIGGER IF EXISTS update_vendors_updated_at ON vendors;
DROP TRIGGER IF EXISTS update_contracts_updated_at ON contracts;

DROP TABLE IF EXISTS contract_assets;
DROP TABLE IF EXISTS contracts;
DROP TABLE IF EXISTS vendors;
