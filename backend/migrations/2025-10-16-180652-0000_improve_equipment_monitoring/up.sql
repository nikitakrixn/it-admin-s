-- ============================================================================
-- Улучшение мониторинга оборудования (принтеры, МФУ)
-- ============================================================================

ALTER TABLE equipment 
    ADD COLUMN paper_jam_count INTEGER DEFAULT 0,
    ADD COLUMN total_pages_printed INTEGER DEFAULT 0,
    ADD COLUMN color_pages_printed INTEGER DEFAULT 0,
    ADD COLUMN mono_pages_printed INTEGER DEFAULT 0,
    ADD COLUMN last_error_code VARCHAR(50),
    ADD COLUMN last_error_time TIMESTAMP,
    ADD COLUMN supplies_status JSONB;

-- Индексы
CREATE INDEX idx_equipment_maintenance_due ON equipment(next_maintenance_date) 
    WHERE next_maintenance_date IS NOT NULL;

-- Комментарии
COMMENT ON COLUMN equipment.paper_jam_count IS 'Количество замятий бумаги';
COMMENT ON COLUMN equipment.total_pages_printed IS 'Общее количество напечатанных страниц';
COMMENT ON COLUMN equipment.color_pages_printed IS 'Количество цветных страниц';
COMMENT ON COLUMN equipment.mono_pages_printed IS 'Количество чёрно-белых страниц';
COMMENT ON COLUMN equipment.last_error_code IS 'Код последней ошибки';
COMMENT ON COLUMN equipment.last_error_time IS 'Время последней ошибки';
COMMENT ON COLUMN equipment.supplies_status IS 'Статус расходников (JSON: {toner_black: 50, toner_cyan: 75, ...})';
