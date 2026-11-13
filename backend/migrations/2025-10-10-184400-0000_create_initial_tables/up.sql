-- Создание расширений
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ============================================================================
-- ОСНОВНЫЕ СПРАВОЧНИКИ
-- ============================================================================

-- Таблица отделов
CREATE TABLE departments (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Таблица должностей
CREATE TABLE positions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    department_id INTEGER REFERENCES departments(id) ON DELETE SET NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(name, department_id)
);

-- Таблица сотрудников
CREATE TABLE employees (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    middle_name VARCHAR(50),
    position_id INTEGER REFERENCES positions(id) ON DELETE SET NULL,
    department_id INTEGER REFERENCES departments(id) ON DELETE SET NULL,
    email VARCHAR(100) UNIQUE,
    phone VARCHAR(20),
    ad_username VARCHAR(50) UNIQUE,
    hire_date DATE,
    termination_date DATE,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_employee_status CHECK (status IN ('active', 'inactive', 'vacation', 'sick_leave')),
    CONSTRAINT check_termination_date CHECK (termination_date IS NULL OR termination_date >= hire_date)
);

-- Таблица пользователей (для аутентификации)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    employee_id INTEGER UNIQUE REFERENCES employees(id) ON DELETE SET NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    is_active BOOLEAN NOT NULL DEFAULT true,
    last_login_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_user_role CHECK (role IN ('admin', 'technician', 'user', 'viewer'))
);

-- Таблица API токенов
CREATE TABLE api_tokens (
    id SERIAL PRIMARY KEY,
    token VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at TIMESTAMP,
    last_used_at TIMESTAMP,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_token_expiry CHECK (expires_at IS NULL OR expires_at > created_at)
);

-- ============================================================================
-- КОМПЬЮТЕРЫ И КОМПОНЕНТЫ
-- ============================================================================

-- Таблица компьютеров (основная информация)
CREATE TABLE computers (
    id SERIAL PRIMARY KEY,
    inventory_number VARCHAR(50) UNIQUE,
    hostname VARCHAR(100) UNIQUE,
    computer_type VARCHAR(50) NOT NULL DEFAULT 'desktop',
    manufacturer VARCHAR(100),
    model VARCHAR(100),
    serial_number VARCHAR(100) UNIQUE,
    os VARCHAR(100),
    os_version VARCHAR(50),
    os_build VARCHAR(50),
    os_architecture VARCHAR(20),
    domain_joined BOOLEAN DEFAULT false,
    last_boot_time TIMESTAMP,
    employee_id INTEGER REFERENCES employees(id) ON DELETE SET NULL,
    location VARCHAR(100),
    purchase_date DATE,
    warranty_end_date DATE,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_computer_status CHECK (status IN ('active', 'inactive', 'repair', 'storage', 'decommissioned')),
    CONSTRAINT check_computer_type CHECK (computer_type IN ('desktop', 'laptop', 'server', 'workstation', 'thin_client'))
);

-- Таблица процессоров
CREATE TABLE computer_processors (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    name VARCHAR(200) NOT NULL,
    manufacturer VARCHAR(100),
    cores INTEGER,
    threads INTEGER,
    base_frequency VARCHAR(50),
    max_frequency VARCHAR(50),
    cache_size VARCHAR(50),
    socket VARCHAR(50),
    tdp VARCHAR(50),
    is_primary BOOLEAN DEFAULT true,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Таблица оперативной памяти
CREATE TABLE computer_ram (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    manufacturer VARCHAR(100),
    model VARCHAR(100),
    capacity_gb INTEGER NOT NULL,
    type VARCHAR(50),
    frequency VARCHAR(50),
    slot_number INTEGER,
    serial_number VARCHAR(100),
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_ram_capacity CHECK (capacity_gb > 0)
);

-- Таблица накопителей
CREATE TABLE computer_storage (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    type VARCHAR(50) NOT NULL,
    manufacturer VARCHAR(100),
    model VARCHAR(100),
    capacity_gb INTEGER NOT NULL,
    interface VARCHAR(50),
    serial_number VARCHAR(100),
    health_status VARCHAR(50) DEFAULT 'good',
    is_system_drive BOOLEAN DEFAULT false,
    mount_point VARCHAR(100),
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_storage_type CHECK (type IN ('HDD', 'SSD', 'NVMe', 'M.2', 'eMMC')),
    CONSTRAINT check_storage_health CHECK (health_status IN ('good', 'warning', 'critical', 'unknown')),
    CONSTRAINT check_storage_capacity CHECK (capacity_gb > 0)
);

-- Таблица видеокарт
CREATE TABLE computer_gpus (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    name VARCHAR(200) NOT NULL,
    manufacturer VARCHAR(100),
    memory_gb INTEGER,
    memory_type VARCHAR(50),
    driver_version VARCHAR(100),
    is_primary BOOLEAN DEFAULT true,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Таблица материнских плат
CREATE TABLE computer_motherboards (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    manufacturer VARCHAR(100),
    model VARCHAR(100),
    chipset VARCHAR(100),
    bios_version VARCHAR(100),
    bios_date DATE,
    form_factor VARCHAR(50),
    serial_number VARCHAR(100),
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(computer_id)
);

-- Таблица сетевых адаптеров
CREATE TABLE computer_network_adapters (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    manufacturer VARCHAR(100),
    mac_address VARCHAR(17),
    ip_address VARCHAR(45),
    adapter_type VARCHAR(50),
    speed VARCHAR(50),
    is_primary BOOLEAN DEFAULT false,
    status VARCHAR(20) DEFAULT 'active',
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_adapter_type CHECK (adapter_type IN ('Ethernet', 'WiFi', 'Bluetooth', 'Virtual'))
);

-- Таблица мониторов
CREATE TABLE computer_monitors (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL DEFAULT 'Unknown Monitor',
    manufacturer VARCHAR(100),
    model VARCHAR(100),
    serial_number VARCHAR(100),
    size_inches DECIMAL(4,1),
    resolution VARCHAR(50),
    panel_type VARCHAR(50),
    refresh_rate INTEGER,
    is_primary BOOLEAN DEFAULT false,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Таблица периферийных устройств
CREATE TABLE computer_peripherals (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL DEFAULT 'Unknown Device',
    type VARCHAR(50) NOT NULL,
    manufacturer VARCHAR(100),
    model VARCHAR(100),
    serial_number VARCHAR(100),
    connection_type VARCHAR(50),
    status VARCHAR(20) DEFAULT 'active',
    purchase_date DATE,
    warranty_end_date DATE,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_peripheral_type CHECK (type IN ('keyboard', 'mouse', 'webcam', 'headset', 'microphone', 'speaker', 'ups', 'docking_station', 'other')),
    CONSTRAINT check_connection_type CHECK (connection_type IN ('USB', 'Bluetooth', 'PS/2', 'Wireless', 'Wired', 'Thunderbolt', 'USB-C'))
);

-- История изменений компонентов
CREATE TABLE component_history (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    component_type VARCHAR(50) NOT NULL,
    component_id INTEGER NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_value JSONB,
    new_value JSONB,
    changed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_component_action CHECK (action IN ('added', 'removed', 'replaced', 'updated'))
);

-- ============================================================================
-- МОНИТОРИНГ КОМПЬЮТЕРОВ
-- ============================================================================

-- Таблица данных мониторинга
CREATE TABLE computer_monitoring (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    cpu_usage DECIMAL(5,2),
    ram_usage_percent DECIMAL(5,2),
    ram_total_gb INTEGER,
    ram_used_gb DECIMAL(6,2),
    disk_usage JSONB,
    cpu_temperature DECIMAL(5,2),
    gpu_temperature DECIMAL(5,2),
    uptime_seconds BIGINT,
    network_stats JSONB,
    processes_count INTEGER,
    logged_users JSONB,
    collected_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_cpu_usage CHECK (cpu_usage >= 0 AND cpu_usage <= 100),
    CONSTRAINT check_ram_usage CHECK (ram_usage_percent >= 0 AND ram_usage_percent <= 100)
);

-- ============================================================================
-- ПРОГРАММНОЕ ОБЕСПЕЧЕНИЕ
-- ============================================================================

-- Справочник программного обеспечения
CREATE TABLE software_catalog (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    manufacturer VARCHAR(255),
    category VARCHAR(100),
    description TEXT,
    website VARCHAR(255),
    is_system BOOLEAN DEFAULT false,
    requires_license BOOLEAN DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(name, manufacturer)
);

-- Маппинг названий ПО (для нормализации)
CREATE TABLE software_name_mappings (
    id SERIAL PRIMARY KEY,
    original_name VARCHAR(255) NOT NULL,
    normalized_name VARCHAR(255) NOT NULL,
    software_catalog_id INTEGER NOT NULL REFERENCES software_catalog(id) ON DELETE CASCADE,
    publisher VARCHAR(255),
    confidence_score DECIMAL(3,2) DEFAULT 1.0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(original_name, publisher)
);

-- Установленное ПО на компьютерах
CREATE TABLE computer_software (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    software_catalog_id INTEGER NOT NULL REFERENCES software_catalog(id) ON DELETE CASCADE,
    version VARCHAR(100),
    license_key VARCHAR(255),
    license_type VARCHAR(50) DEFAULT 'unknown',
    license_end_date DATE,
    installation_date DATE,
    last_seen_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    status VARCHAR(20) DEFAULT 'active',
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_software_status CHECK (status IN ('active', 'inactive', 'trial', 'expired')),
    CONSTRAINT check_license_type CHECK (license_type IN ('perpetual', 'subscription', 'trial', 'free', 'oem', 'volume', 'unknown')),
    UNIQUE(computer_id, software_catalog_id)
);

-- История изменений ПО
CREATE TABLE software_history (
    id SERIAL PRIMARY KEY,
    computer_id INTEGER NOT NULL REFERENCES computers(id) ON DELETE CASCADE,
    software_catalog_id INTEGER NOT NULL REFERENCES software_catalog(id) ON DELETE CASCADE,
    action VARCHAR(20) NOT NULL,
    old_version VARCHAR(100),
    new_version VARCHAR(100),
    detected_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_software_action CHECK (action IN ('installed', 'updated', 'removed'))
);

-- ============================================================================
-- ОБОРУДОВАНИЕ И РАСХОДНИКИ
-- ============================================================================

-- Типы оборудования
CREATE TABLE hardware_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    category VARCHAR(50) NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(name, category)
);

-- Оргтехника и оборудование
CREATE TABLE equipment (
    id SERIAL PRIMARY KEY,
    inventory_number VARCHAR(50) UNIQUE,
    type_id INTEGER REFERENCES hardware_types(id) ON DELETE SET NULL,
    name VARCHAR(255) NOT NULL,
    manufacturer VARCHAR(100),
    model VARCHAR(100),
    serial_number VARCHAR(100) UNIQUE,
    employee_id INTEGER REFERENCES employees(id) ON DELETE SET NULL,
    location VARCHAR(100),
    ip_address VARCHAR(45),
    mac_address VARCHAR(17),
    purchase_date DATE,
    warranty_end_date DATE,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_equipment_status CHECK (status IN ('active', 'inactive', 'repair', 'storage', 'decommissioned'))
);

-- Расходные материалы
CREATE TABLE consumables (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    manufacturer VARCHAR(100),
    model VARCHAR(100),
    category VARCHAR(50),
    compatible_with INTEGER REFERENCES hardware_types(id) ON DELETE SET NULL,
    quantity INTEGER NOT NULL DEFAULT 0,
    min_quantity INTEGER NOT NULL DEFAULT 1,
    unit VARCHAR(20) DEFAULT 'шт',
    price_per_unit DECIMAL(10,2),
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_consumable_quantity CHECK (quantity >= 0),
    CONSTRAINT check_min_quantity CHECK (min_quantity >= 0)
);

-- История движения расходников
CREATE TABLE consumable_movements (
    id SERIAL PRIMARY KEY,
    consumable_id INTEGER NOT NULL REFERENCES consumables(id) ON DELETE CASCADE,
    equipment_id INTEGER REFERENCES equipment(id) ON DELETE SET NULL,
    computer_id INTEGER REFERENCES computers(id) ON DELETE SET NULL,
    quantity INTEGER NOT NULL,
    movement_type VARCHAR(20) NOT NULL,
    employee_id INTEGER REFERENCES employees(id) ON DELETE SET NULL,
    performed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_movement_type CHECK (movement_type IN ('purchase', 'usage', 'return', 'write_off', 'transfer')),
    CONSTRAINT check_movement_quantity CHECK (quantity != 0)
);

-- ============================================================================
-- ЗАЯВКИ И ИНЦИДЕНТЫ
-- ============================================================================

-- Таблица заявок
CREATE TABLE requests (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    requester_id INTEGER NOT NULL REFERENCES employees(id) ON DELETE CASCADE,
    assignee_id INTEGER REFERENCES employees(id) ON DELETE SET NULL,
    computer_id INTEGER REFERENCES computers(id) ON DELETE SET NULL,
    equipment_id INTEGER REFERENCES equipment(id) ON DELETE SET NULL,
    priority VARCHAR(20) NOT NULL DEFAULT 'medium',
    status VARCHAR(20) NOT NULL DEFAULT 'new',
    category VARCHAR(50),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    resolved_at TIMESTAMP,
    closed_at TIMESTAMP,
    CONSTRAINT check_request_priority CHECK (priority IN ('low', 'medium', 'high', 'critical')),
    CONSTRAINT check_request_status CHECK (status IN ('new', 'in_progress', 'waiting', 'resolved', 'closed', 'cancelled')),
    CONSTRAINT check_request_category CHECK (category IN ('hardware', 'software', 'network', 'access', 'consultation', 'other') OR category IS NULL)
);

-- Комментарии к заявкам
CREATE TABLE request_comments (
    id SERIAL PRIMARY KEY,
    request_id INTEGER NOT NULL REFERENCES requests(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    comment TEXT NOT NULL,
    is_internal BOOLEAN DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Вложения к заявкам
CREATE TABLE request_attachments (
    id SERIAL PRIMARY KEY,
    request_id INTEGER NOT NULL REFERENCES requests(id) ON DELETE CASCADE,
    filename VARCHAR(255) NOT NULL,
    file_path VARCHAR(500) NOT NULL,
    file_size INTEGER,
    mime_type VARCHAR(100),
    uploaded_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- ЖУРНАЛИРОВАНИЕ И АУДИТ
-- ============================================================================

-- Журнал активности
CREATE TABLE activity_log (
    id SERIAL PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(50) NOT NULL,
    entity_type VARCHAR(50) NOT NULL,
    entity_id INTEGER NOT NULL,
    details JSONB,
    ip_address VARCHAR(45),
    user_agent TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Уведомления
CREATE TABLE notifications (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    type VARCHAR(50) NOT NULL,
    entity_type VARCHAR(50),
    entity_id INTEGER,
    is_read BOOLEAN DEFAULT false,
    read_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_notification_type CHECK (type IN ('info', 'warning', 'error', 'success', 'request', 'system'))
);

-- ============================================================================
-- ИНДЕКСЫ ДЛЯ ПРОИЗВОДИТЕЛЬНОСТИ
-- ============================================================================

-- Индексы для employees
CREATE INDEX idx_employees_email ON employees(email);
CREATE INDEX idx_employees_ad_username ON employees(ad_username);
CREATE INDEX idx_employees_status ON employees(status);
CREATE INDEX idx_employees_department ON employees(department_id);

-- Индексы для users
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_employee ON users(employee_id);
CREATE INDEX idx_users_is_active ON users(is_active);

-- Индексы для api_tokens
CREATE INDEX idx_api_tokens_token ON api_tokens(token);
CREATE INDEX idx_api_tokens_user ON api_tokens(user_id);
CREATE INDEX idx_api_tokens_active ON api_tokens(is_active) WHERE is_active = true;

-- Индексы для computers
CREATE INDEX idx_computers_hostname ON computers(hostname);
CREATE INDEX idx_computers_inventory ON computers(inventory_number);
CREATE INDEX idx_computers_employee ON computers(employee_id);
CREATE INDEX idx_computers_status ON computers(status);

-- Индексы для компонентов
CREATE INDEX idx_processors_computer ON computer_processors(computer_id);
CREATE INDEX idx_ram_computer ON computer_ram(computer_id);
CREATE INDEX idx_storage_computer ON computer_storage(computer_id);
CREATE INDEX idx_gpus_computer ON computer_gpus(computer_id);
CREATE INDEX idx_motherboards_computer ON computer_motherboards(computer_id);
CREATE INDEX idx_network_adapters_computer ON computer_network_adapters(computer_id);
CREATE INDEX idx_monitors_computer ON computer_monitors(computer_id);
CREATE INDEX idx_peripherals_computer ON computer_peripherals(computer_id);

-- Индексы для мониторинга
CREATE INDEX idx_monitoring_computer ON computer_monitoring(computer_id);
CREATE INDEX idx_monitoring_collected_at ON computer_monitoring(collected_at);
CREATE INDEX idx_monitoring_computer_time ON computer_monitoring(computer_id, collected_at DESC);

-- Индексы для ПО
CREATE INDEX idx_software_catalog_name ON software_catalog(name);
CREATE INDEX idx_software_catalog_manufacturer ON software_catalog(manufacturer);
CREATE INDEX idx_software_mappings_original ON software_name_mappings(original_name);
CREATE INDEX idx_computer_software_computer ON computer_software(computer_id);
CREATE INDEX idx_computer_software_catalog ON computer_software(software_catalog_id);
CREATE INDEX idx_computer_software_status ON computer_software(status);
CREATE INDEX idx_software_history_computer ON software_history(computer_id);
CREATE INDEX idx_software_history_detected_at ON software_history(detected_at);

-- Индексы для equipment
CREATE INDEX idx_equipment_employee ON equipment(employee_id);
CREATE INDEX idx_equipment_type ON equipment(type_id);
CREATE INDEX idx_equipment_status ON equipment(status);
CREATE INDEX idx_equipment_inventory ON equipment(inventory_number);

-- Индексы для consumables
CREATE INDEX idx_consumables_category ON consumables(category);
CREATE INDEX idx_consumable_movements_consumable ON consumable_movements(consumable_id);
CREATE INDEX idx_consumable_movements_created_at ON consumable_movements(created_at);

-- Индексы для requests
CREATE INDEX idx_requests_requester ON requests(requester_id);
CREATE INDEX idx_requests_assignee ON requests(assignee_id);
CREATE INDEX idx_requests_status ON requests(status);
CREATE INDEX idx_requests_priority ON requests(priority);
CREATE INDEX idx_requests_created_at ON requests(created_at DESC);
CREATE INDEX idx_request_comments_request ON request_comments(request_id);

-- Индексы для activity_log
CREATE INDEX idx_activity_log_user ON activity_log(user_id);
CREATE INDEX idx_activity_log_entity ON activity_log(entity_type, entity_id);
CREATE INDEX idx_activity_log_created_at ON activity_log(created_at DESC);

-- Индексы для notifications
CREATE INDEX idx_notifications_user ON notifications(user_id);
CREATE INDEX idx_notifications_is_read ON notifications(is_read) WHERE is_read = false;
CREATE INDEX idx_notifications_created_at ON notifications(created_at DESC);

-- ============================================================================
-- НАЧАЛЬНЫЕ ДАННЫЕ
-- ============================================================================

-- Отделы
INSERT INTO departments (name, description) VALUES 
    ('IT отдел', 'Отдел информационных технологий'),
    ('Бухгалтерия', 'Финансовый отдел'),
    ('Отдел продаж', 'Коммерческий отдел'),
    ('Администрация', 'Руководство компании'),
    ('HR отдел', 'Отдел кадров');

-- Должности
INSERT INTO positions (name, department_id) VALUES 
    ('Системный администратор', 1),
    ('Техник IT', 1),
    ('Главный бухгалтер', 2),
    ('Бухгалтер', 2),
    ('Менеджер по продажам', 3),
    ('Директор', 4),
    ('HR менеджер', 5);

-- Типы оборудования
INSERT INTO hardware_types (name, category, description) VALUES 
    ('Лазерный принтер', 'printer', 'Лазерный принтер для офиса'),
    ('Струйный принтер', 'printer', 'Струйный принтер'),
    ('МФУ', 'mfu', 'Многофункциональное устройство'),
    ('Сканер', 'scanner', 'Сканер документов'),
    ('Роутер', 'network', 'Сетевой маршрутизатор'),
    ('Коммутатор', 'network', 'Сетевой коммутатор'),
    ('ИБП', 'ups', 'Источник бесперебойного питания'),
    ('Проектор', 'projector', 'Мультимедийный проектор'),
    ('Телефон IP', 'phone', 'IP-телефон');

-- Популярное ПО в справочник
INSERT INTO software_catalog (name, manufacturer, category, requires_license) VALUES 
    ('Microsoft Office', 'Microsoft Corporation', 'Офисные пакеты', true),
    ('Microsoft 365', 'Microsoft Corporation', 'Офисные пакеты', true),
    ('Google Chrome', 'Google LLC', 'Браузеры', false),
    ('Mozilla Firefox', 'Mozilla Foundation', 'Браузеры', false),
    ('Adobe Acrobat Reader', 'Adobe Inc.', 'Программы для PDF', false),
    ('7-Zip', '7-Zip', 'Архиваторы', false),
    ('WinRAR', 'RARLAB', 'Архиваторы', true),
    ('VLC Media Player', 'VideoLAN', 'Медиаплееры', false),
    ('Notepad++', 'Don Ho', 'Текстовые редакторы', false),
    ('Visual Studio Code', 'Microsoft Corporation', 'Среды разработки', false),
    ('TeamViewer', 'TeamViewer GmbH', 'Удаленный доступ', true),
    ('AnyDesk', 'AnyDesk Software GmbH', 'Удаленный доступ', false),
    ('Zoom', 'Zoom Video Communications', 'Видеоконференции', true),
    ('Microsoft Teams', 'Microsoft Corporation', 'Коммуникации', true),
    ('Telegram', 'Telegram FZ-LLC', 'Мессенджеры', false),
    ('Kaspersky', 'Kaspersky Lab', 'Антивирусы', true),
    ('Windows Defender', 'Microsoft Corporation', 'Антивирусы', false);

-- ============================================================================
-- КОММЕНТАРИИ К ТАБЛИЦАМ
-- ============================================================================

COMMENT ON TABLE departments IS 'Отделы организации';
COMMENT ON TABLE positions IS 'Должности сотрудников';
COMMENT ON TABLE employees IS 'Сотрудники организации';
COMMENT ON TABLE users IS 'Пользователи системы (для аутентификации)';
COMMENT ON TABLE api_tokens IS 'API токены для интеграции с клиентскими приложениями';
COMMENT ON TABLE computers IS 'Компьютеры и серверы';
COMMENT ON TABLE computer_processors IS 'Процессоры компьютеров';
COMMENT ON TABLE computer_ram IS 'Модули оперативной памяти';
COMMENT ON TABLE computer_storage IS 'Накопители (HDD/SSD/NVMe)';
COMMENT ON TABLE computer_gpus IS 'Видеокарты';
COMMENT ON TABLE computer_motherboards IS 'Материнские платы';
COMMENT ON TABLE computer_network_adapters IS 'Сетевые адаптеры';
COMMENT ON TABLE computer_monitors IS 'Подключенные мониторы';
COMMENT ON TABLE computer_peripherals IS 'Периферийные устройства';
COMMENT ON TABLE component_history IS 'История изменений компонентов';
COMMENT ON TABLE computer_monitoring IS 'Данные мониторинга состояния компьютеров';
COMMENT ON TABLE software_catalog IS 'Справочник программного обеспечения';
COMMENT ON TABLE software_name_mappings IS 'Маппинг названий ПО для нормализации';
COMMENT ON TABLE computer_software IS 'Установленное ПО на компьютерах';
COMMENT ON TABLE software_history IS 'История изменений ПО';
COMMENT ON TABLE hardware_types IS 'Типы оборудования';
COMMENT ON TABLE equipment IS 'Оргтехника и оборудование';
COMMENT ON TABLE consumables IS 'Расходные материалы';
COMMENT ON TABLE consumable_movements IS 'История движения расходников';
COMMENT ON TABLE requests IS 'Заявки на обслуживание';
COMMENT ON TABLE request_comments IS 'Комментарии к заявкам';
COMMENT ON TABLE request_attachments IS 'Вложения к заявкам';
COMMENT ON TABLE activity_log IS 'Журнал активности пользователей';
COMMENT ON TABLE notifications IS 'Уведомления пользователей';
