-- Функция для обновления updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Триггеры для всех таблиц с полем updated_at

-- Основные справочники
CREATE TRIGGER update_departments_updated_at BEFORE UPDATE ON departments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_positions_updated_at BEFORE UPDATE ON positions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_employees_updated_at BEFORE UPDATE ON employees
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Компьютеры и компоненты
CREATE TRIGGER update_computers_updated_at BEFORE UPDATE ON computers
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_computer_processors_updated_at BEFORE UPDATE ON computer_processors
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_computer_ram_updated_at BEFORE UPDATE ON computer_ram
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_computer_storage_updated_at BEFORE UPDATE ON computer_storage
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_computer_gpus_updated_at BEFORE UPDATE ON computer_gpus
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_computer_motherboards_updated_at BEFORE UPDATE ON computer_motherboards
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_computer_network_adapters_updated_at BEFORE UPDATE ON computer_network_adapters
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_computer_monitors_updated_at BEFORE UPDATE ON computer_monitors
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_computer_peripherals_updated_at BEFORE UPDATE ON computer_peripherals
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Программное обеспечение
CREATE TRIGGER update_software_catalog_updated_at BEFORE UPDATE ON software_catalog
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_computer_software_updated_at BEFORE UPDATE ON computer_software
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Оборудование и расходники
CREATE TRIGGER update_hardware_types_updated_at BEFORE UPDATE ON hardware_types
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_equipment_updated_at BEFORE UPDATE ON equipment
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_consumables_updated_at BEFORE UPDATE ON consumables
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Заявки
CREATE TRIGGER update_requests_updated_at BEFORE UPDATE ON requests
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- Триггер для автоматического обновления количества расходников
-- ============================================================================

CREATE OR REPLACE FUNCTION update_consumable_quantity()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        -- При добавлении движения обновляем количество
        IF NEW.movement_type IN ('purchase', 'return') THEN
            UPDATE consumables 
            SET quantity = quantity + NEW.quantity 
            WHERE id = NEW.consumable_id;
        ELSIF NEW.movement_type IN ('usage', 'write_off', 'transfer') THEN
            UPDATE consumables 
            SET quantity = quantity - ABS(NEW.quantity)
            WHERE id = NEW.consumable_id;
        END IF;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_consumable_quantity_trigger 
AFTER INSERT ON consumable_movements
FOR EACH ROW EXECUTE FUNCTION update_consumable_quantity();

-- ============================================================================
-- Триггер для логирования изменений компонентов
-- ============================================================================

-- Функция для логирования изменений процессоров
CREATE OR REPLACE FUNCTION log_processor_changes()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO component_history (computer_id, component_type, component_id, action, new_value)
        VALUES (NEW.computer_id, 'processor', NEW.id, 'added', to_jsonb(NEW));
    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO component_history (computer_id, component_type, component_id, action, old_value, new_value)
        VALUES (NEW.computer_id, 'processor', NEW.id, 'updated', to_jsonb(OLD), to_jsonb(NEW));
    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO component_history (computer_id, component_type, component_id, action, old_value)
        VALUES (OLD.computer_id, 'processor', OLD.id, 'removed', to_jsonb(OLD));
        RETURN OLD;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER log_processor_changes_trigger
AFTER INSERT OR UPDATE OR DELETE ON computer_processors
FOR EACH ROW EXECUTE FUNCTION log_processor_changes();

-- Аналогичные триггеры для других компонентов
CREATE OR REPLACE FUNCTION log_ram_changes()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO component_history (computer_id, component_type, component_id, action, new_value)
        VALUES (NEW.computer_id, 'ram', NEW.id, 'added', to_jsonb(NEW));
    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO component_history (computer_id, component_type, component_id, action, old_value, new_value)
        VALUES (NEW.computer_id, 'ram', NEW.id, 'updated', to_jsonb(OLD), to_jsonb(NEW));
    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO component_history (computer_id, component_type, component_id, action, old_value)
        VALUES (OLD.computer_id, 'ram', OLD.id, 'removed', to_jsonb(OLD));
        RETURN OLD;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER log_ram_changes_trigger
AFTER INSERT OR UPDATE OR DELETE ON computer_ram
FOR EACH ROW EXECUTE FUNCTION log_ram_changes();

CREATE OR REPLACE FUNCTION log_storage_changes()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO component_history (computer_id, component_type, component_id, action, new_value)
        VALUES (NEW.computer_id, 'storage', NEW.id, 'added', to_jsonb(NEW));
    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO component_history (computer_id, component_type, component_id, action, old_value, new_value)
        VALUES (NEW.computer_id, 'storage', NEW.id, 'updated', to_jsonb(OLD), to_jsonb(NEW));
    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO component_history (computer_id, component_type, component_id, action, old_value)
        VALUES (OLD.computer_id, 'storage', OLD.id, 'removed', to_jsonb(OLD));
        RETURN OLD;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER log_storage_changes_trigger
AFTER INSERT OR UPDATE OR DELETE ON computer_storage
FOR EACH ROW EXECUTE FUNCTION log_storage_changes();

-- ============================================================================
-- Триггер для автоматического закрытия заявок
-- ============================================================================

CREATE OR REPLACE FUNCTION auto_close_resolved_requests()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.status = 'resolved' AND OLD.status != 'resolved' THEN
        NEW.resolved_at = CURRENT_TIMESTAMP;
    ELSIF NEW.status = 'closed' AND OLD.status != 'closed' THEN
        NEW.closed_at = CURRENT_TIMESTAMP;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER auto_close_resolved_requests_trigger
BEFORE UPDATE ON requests
FOR EACH ROW EXECUTE FUNCTION auto_close_resolved_requests();

-- ============================================================================
-- Триггер для обновления last_login_at
-- ============================================================================

CREATE OR REPLACE FUNCTION update_last_login()
RETURNS TRIGGER AS $$
BEGIN
    -- Этот триггер будет вызываться из приложения при логине
    -- Здесь просто заглушка для будущего использования
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- Комментарии
-- ============================================================================

COMMENT ON FUNCTION update_updated_at_column() IS 'Автоматически обновляет поле updated_at при изменении записи';
COMMENT ON FUNCTION update_consumable_quantity() IS 'Автоматически обновляет количество расходников при добавлении движения';
COMMENT ON FUNCTION log_processor_changes() IS 'Логирует изменения процессоров в component_history';
COMMENT ON FUNCTION log_ram_changes() IS 'Логирует изменения RAM в component_history';
COMMENT ON FUNCTION log_storage_changes() IS 'Логирует изменения накопителей в component_history';
COMMENT ON FUNCTION auto_close_resolved_requests() IS 'Автоматически устанавливает resolved_at и closed_at для заявок';
