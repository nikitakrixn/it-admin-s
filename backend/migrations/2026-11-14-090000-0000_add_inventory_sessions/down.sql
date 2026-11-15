-- Откат: статус "pending" относится только к сессиям инвентаризации.
-- Перед возвратом старого ограничения такие позиции нужно убрать.

DROP INDEX IF EXISTS idx_inventory_audit_items_audit_status;

DELETE FROM inventory_audit_items WHERE status = 'pending';

ALTER TABLE inventory_audit_items ALTER COLUMN status DROP DEFAULT;

ALTER TABLE inventory_audit_items DROP CONSTRAINT IF EXISTS check_item_status;

ALTER TABLE inventory_audit_items
    ADD CONSTRAINT check_item_status
    CHECK (status IN ('found', 'missing', 'damaged', 'relocated', 'surplus', 'disposed'));
