-- ============================================================================
-- Исправление индексов для таблицы ip_addresses
-- ============================================================================

-- Добавляем составной индекс для связи assigned_to
CREATE INDEX idx_ip_addresses_assigned ON ip_addresses(assigned_to_type, assigned_to_id);

-- Комментарии для ясности
COMMENT ON COLUMN ip_addresses.assigned_to_type IS 'Тип объекта (computer, equipment, network_device)';
COMMENT ON COLUMN ip_addresses.assigned_to_id IS 'ID объекта в соответствующей таблице';
