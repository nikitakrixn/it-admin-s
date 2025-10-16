-- ============================================================================
-- Улучшение мониторинга дисков
-- ============================================================================

ALTER TABLE computer_storage 
    ADD COLUMN last_defrag_date DATE,
    ADD COLUMN fragmentation_percent INTEGER,
    ADD COLUMN read_errors INTEGER DEFAULT 0,
    ADD COLUMN write_errors INTEGER DEFAULT 0,
    ADD COLUMN wear_level_percent INTEGER,
    ADD COLUMN total_bytes_written BIGINT,
    ADD COLUMN power_cycle_count INTEGER;

-- Индексы
CREATE INDEX idx_computer_storage_health ON computer_storage(health_status) 
    WHERE health_status IN ('warning', 'critical');
CREATE INDEX idx_computer_storage_fragmentation ON computer_storage(fragmentation_percent) 
    WHERE fragmentation_percent > 10;

-- Комментарии
COMMENT ON COLUMN computer_storage.last_defrag_date IS 'Дата последней дефрагментации (для HDD)';
COMMENT ON COLUMN computer_storage.fragmentation_percent IS 'Процент фрагментации диска';
COMMENT ON COLUMN computer_storage.read_errors IS 'Количество ошибок чтения';
COMMENT ON COLUMN computer_storage.write_errors IS 'Количество ошибок записи';
COMMENT ON COLUMN computer_storage.wear_level_percent IS 'Уровень износа (для SSD)';
COMMENT ON COLUMN computer_storage.total_bytes_written IS 'Всего записано байт (для SSD)';
COMMENT ON COLUMN computer_storage.power_cycle_count IS 'Количество циклов включения/выключения';
