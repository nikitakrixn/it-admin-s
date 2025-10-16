-- ============================================================================
-- Добавление полей для отслеживания онлайн-статуса компьютеров
-- ============================================================================

-- Добавляем поля в таблицу computers
ALTER TABLE computers 
    ADD COLUMN last_seen_online TIMESTAMP,
    ADD COLUMN is_online BOOLEAN DEFAULT false,
    ADD COLUMN last_user_logged VARCHAR(255),
    ADD COLUMN bitlocker_enabled BOOLEAN DEFAULT false,
    ADD COLUMN antivirus_status VARCHAR(50),
    ADD COLUMN windows_activation_status VARCHAR(50);

-- Индекс для быстрого поиска онлайн компьютеров
CREATE INDEX idx_computers_is_online ON computers(is_online) WHERE is_online = true;
CREATE INDEX idx_computers_last_seen ON computers(last_seen_online DESC);

-- Комментарии
COMMENT ON COLUMN computers.last_seen_online IS 'Время последнего подключения компьютера к системе мониторинга';
COMMENT ON COLUMN computers.is_online IS 'Текущий статус: онлайн или офлайн';
COMMENT ON COLUMN computers.last_user_logged IS 'Последний пользователь, который входил в систему';
COMMENT ON COLUMN computers.bitlocker_enabled IS 'Включено ли шифрование BitLocker';
COMMENT ON COLUMN computers.antivirus_status IS 'Статус антивируса (active, disabled, outdated, not_installed)';
COMMENT ON COLUMN computers.windows_activation_status IS 'Статус активации Windows (activated, not_activated, grace_period)';
