-- Сессии инвентаризации создают позиции со статусом "pending": оборудование
-- включено в список, но ещё не проверено. Раньше такого статуса не было,
-- поэтому ограничение нужно расширить, а не пересоздавать таблицу.

ALTER TABLE inventory_audit_items DROP CONSTRAINT IF EXISTS check_item_status;

ALTER TABLE inventory_audit_items
    ADD CONSTRAINT check_item_status
    CHECK (status IN ('pending', 'found', 'missing', 'damaged', 'relocated', 'surplus', 'disposed'));

ALTER TABLE inventory_audit_items ALTER COLUMN status SET DEFAULT 'pending';

-- Список позиций сессии фильтруется по статусу при каждом открытии,
-- поэтому составной индекс экономит последовательное чтение.
CREATE INDEX IF NOT EXISTS idx_inventory_audit_items_audit_status
    ON inventory_audit_items(audit_id, status);
