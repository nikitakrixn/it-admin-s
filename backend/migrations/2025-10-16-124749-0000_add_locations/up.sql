-- ============================================================================
-- Справочник местоположений
-- ============================================================================

-- Местоположения (иерархическая структура)
CREATE TABLE locations (
    id SERIAL PRIMARY KEY,
    parent_id INTEGER REFERENCES locations(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    location_type VARCHAR(50) NOT NULL,
    address TEXT,
    floor INTEGER,
    room_number VARCHAR(50),
    capacity INTEGER,
    description TEXT,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_location_type CHECK (location_type IN ('building', 'floor', 'room', 'rack', 'desk', 'other'))
);

-- Индексы
CREATE INDEX idx_locations_parent ON locations(parent_id);
CREATE INDEX idx_locations_type ON locations(location_type);
CREATE INDEX idx_locations_active ON locations(is_active) WHERE is_active = true;

-- Триггер
CREATE TRIGGER update_locations_updated_at BEFORE UPDATE ON locations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Обновляем существующие таблицы для использования location_id
ALTER TABLE computers ADD COLUMN location_id INTEGER REFERENCES locations(id) ON DELETE SET NULL;
ALTER TABLE equipment ADD COLUMN location_id INTEGER REFERENCES locations(id) ON DELETE SET NULL;
ALTER TABLE employees ADD COLUMN location_id INTEGER REFERENCES locations(id) ON DELETE SET NULL;

-- Индексы для новых полей
CREATE INDEX idx_computers_location_id ON computers(location_id);
CREATE INDEX idx_equipment_location_id ON equipment(location_id);
CREATE INDEX idx_employees_location_id ON employees(location_id);

-- Представление: полный путь местоположения
CREATE OR REPLACE VIEW v_location_paths AS
WITH RECURSIVE location_tree AS (
    SELECT 
        id,
        parent_id,
        name,
        location_type,
        CAST(name AS TEXT) AS full_path,
        1 AS level
    FROM locations
    WHERE parent_id IS NULL
    
    UNION ALL
    
    SELECT 
        l.id,
        l.parent_id,
        l.name,
        l.location_type,
        CAST(lt.full_path || ' → ' || l.name AS TEXT) AS full_path,
        lt.level + 1 AS level
    FROM locations l
    INNER JOIN location_tree lt ON l.parent_id = lt.id
)
SELECT * FROM location_tree;

-- Примеры местоположений
INSERT INTO locations (name, location_type, address) VALUES 
    ('Главный офис', 'building', 'г. Москва, ул. Примерная, д. 1');

INSERT INTO locations (parent_id, name, location_type, floor) VALUES 
    (1, '1 этаж', 'floor', 1),
    (1, '2 этаж', 'floor', 2),
    (1, '3 этаж', 'floor', 3);

INSERT INTO locations (parent_id, name, location_type, room_number, capacity) VALUES 
    (2, 'Серверная', 'room', '101', 10),
    (2, 'IT отдел', 'room', '102', 5),
    (3, 'Бухгалтерия', 'room', '201', 8),
    (3, 'Отдел продаж', 'room', '202', 12);

-- Комментарии
COMMENT ON TABLE locations IS 'Иерархический справочник местоположений (здания, этажи, кабинеты)';
COMMENT ON VIEW v_location_paths IS 'Полные пути местоположений (например: Главный офис → 2 этаж → IT отдел)';
COMMENT ON COLUMN computers.location_id IS 'Ссылка на справочник местоположений (заменяет текстовое поле location)';
COMMENT ON COLUMN equipment.location_id IS 'Ссылка на справочник местоположений (заменяет текстовое поле location)';
COMMENT ON COLUMN employees.location_id IS 'Рабочее место сотрудника';
