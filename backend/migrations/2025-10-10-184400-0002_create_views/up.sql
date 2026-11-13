-- ============================================================================
-- Представление: Полная информация о сотрудниках
-- ============================================================================
CREATE OR REPLACE VIEW v_employees_full AS
SELECT 
    e.id,
    e.first_name,
    e.last_name,
    e.middle_name,
    e.first_name || ' ' || e.last_name AS full_name,
    e.email,
    e.phone,
    e.ad_username,
    e.hire_date,
    e.termination_date,
    e.status,
    p.name AS position_name,
    d.name AS department_name,
    d.id AS department_id,
    p.id AS position_id,
    u.id AS user_id,
    u.role AS user_role,
    u.is_active AS user_is_active,
    e.created_at,
    e.updated_at
FROM employees e
LEFT JOIN positions p ON e.position_id = p.id
LEFT JOIN departments d ON e.department_id = d.id
LEFT JOIN users u ON u.employee_id = e.id;

-- ============================================================================
-- Представление: Компьютеры с суммарными характеристиками
-- ============================================================================
CREATE OR REPLACE VIEW v_computers_summary AS
SELECT 
    c.id,
    c.inventory_number,
    c.hostname,
    c.computer_type,
    c.manufacturer,
    c.model,
    c.serial_number,
    c.os,
    c.os_version,
    c.status,
    c.location,
    -- Процессор (первый primary)
    (SELECT name FROM computer_processors WHERE computer_id = c.id AND is_primary = true LIMIT 1) AS processor_name,
    -- Суммарная RAM
    COALESCE((SELECT SUM(capacity_gb) FROM computer_ram WHERE computer_id = c.id), 0) AS total_ram_gb,
    -- Количество планок RAM
    (SELECT COUNT(*) FROM computer_ram WHERE computer_id = c.id) AS ram_modules_count,
    -- Суммарный объем дисков
    COALESCE((SELECT SUM(capacity_gb) FROM computer_storage WHERE computer_id = c.id), 0) AS total_storage_gb,
    -- Количество дисков
    (SELECT COUNT(*) FROM computer_storage WHERE computer_id = c.id) AS storage_count,
    -- Видеокарта (первая primary)
    (SELECT name FROM computer_gpus WHERE computer_id = c.id AND is_primary = true LIMIT 1) AS gpu_name,
    -- Сотрудник
    e.first_name || ' ' || e.last_name AS employee_name,
    e.id AS employee_id,
    e.email AS employee_email,
    d.name AS department_name,
    -- Даты
    c.purchase_date,
    c.warranty_end_date,
    c.last_boot_time,
    c.created_at,
    c.updated_at
FROM computers c
LEFT JOIN employees e ON c.employee_id = e.id
LEFT JOIN departments d ON e.department_id = d.id;

-- ============================================================================
-- Представление: Детальная информация о компьютере
-- ============================================================================
CREATE OR REPLACE VIEW v_computers_detailed AS
SELECT 
    c.*,
    e.first_name || ' ' || e.last_name AS employee_name,
    e.email AS employee_email,
    e.phone AS employee_phone,
    d.name AS department_name,
    p.name AS position_name,
    -- Процессоры (JSON)
    (SELECT json_agg(json_build_object(
        'id', id,
        'name', name,
        'manufacturer', manufacturer,
        'cores', cores,
        'threads', threads,
        'is_primary', is_primary
    )) FROM computer_processors WHERE computer_id = c.id) AS processors,
    -- RAM (JSON)
    (SELECT json_agg(json_build_object(
        'id', id,
        'manufacturer', manufacturer,
        'capacity_gb', capacity_gb,
        'type', type,
        'frequency', frequency
    )) FROM computer_ram WHERE computer_id = c.id) AS ram_modules,
    -- Диски (JSON)
    (SELECT json_agg(json_build_object(
        'id', id,
        'type', type,
        'manufacturer', manufacturer,
        'model', model,
        'capacity_gb', capacity_gb,
        'health_status', health_status,
        'is_system_drive', is_system_drive
    )) FROM computer_storage WHERE computer_id = c.id) AS storage_devices,
    -- Сетевые адаптеры (JSON)
    (SELECT json_agg(json_build_object(
        'id', id,
        'name', name,
        'mac_address', mac_address,
        'ip_address', ip_address,
        'is_primary', is_primary
    )) FROM computer_network_adapters WHERE computer_id = c.id) AS network_adapters
FROM computers c
LEFT JOIN employees e ON c.employee_id = e.id
LEFT JOIN departments d ON e.department_id = d.id
LEFT JOIN positions p ON e.position_id = p.id;

-- ============================================================================
-- Представление: Установленное ПО с деталями
-- ============================================================================
CREATE OR REPLACE VIEW v_computer_software_full AS
SELECT 
    cs.id,
    cs.computer_id,
    c.hostname,
    c.inventory_number,
    sc.name AS software_name,
    sc.manufacturer AS software_manufacturer,
    sc.category AS software_category,
    cs.version,
    cs.license_key,
    cs.license_type,
    cs.license_end_date,
    cs.installation_date,
    cs.last_seen_date,
    cs.status,
    -- Проверка истечения лицензии
    CASE 
        WHEN cs.license_end_date IS NULL THEN 'no_expiry'
        WHEN cs.license_end_date < CURRENT_DATE THEN 'expired'
        WHEN cs.license_end_date < CURRENT_DATE + INTERVAL '30 days' THEN 'expiring_soon'
        ELSE 'valid'
    END AS license_status,
    e.first_name || ' ' || e.last_name AS employee_name,
    d.name AS department_name,
    cs.created_at,
    cs.updated_at
FROM computer_software cs
JOIN software_catalog sc ON cs.software_catalog_id = sc.id
JOIN computers c ON cs.computer_id = c.id
LEFT JOIN employees e ON c.employee_id = e.id
LEFT JOIN departments d ON e.department_id = d.id;

-- ============================================================================
-- Представление: Оборудование с деталями
-- ============================================================================
CREATE OR REPLACE VIEW v_equipment_full AS
SELECT 
    eq.id,
    eq.inventory_number,
    eq.name,
    eq.manufacturer,
    eq.model,
    eq.serial_number,
    ht.name AS type_name,
    ht.category AS type_category,
    eq.location,
    eq.ip_address,
    eq.mac_address,
    eq.status,
    e.first_name || ' ' || e.last_name AS employee_name,
    e.id AS employee_id,
    e.email AS employee_email,
    d.name AS department_name,
    eq.purchase_date,
    eq.warranty_end_date,
    -- Проверка гарантии
    CASE 
        WHEN eq.warranty_end_date IS NULL THEN 'no_warranty'
        WHEN eq.warranty_end_date < CURRENT_DATE THEN 'expired'
        WHEN eq.warranty_end_date < CURRENT_DATE + INTERVAL '30 days' THEN 'expiring_soon'
        ELSE 'valid'
    END AS warranty_status,
    eq.notes,
    eq.created_at,
    eq.updated_at
FROM equipment eq
LEFT JOIN hardware_types ht ON eq.type_id = ht.id
LEFT JOIN employees e ON eq.employee_id = e.id
LEFT JOIN departments d ON e.department_id = d.id;

-- ============================================================================
-- Представление: Расходники с низким остатком
-- ============================================================================
CREATE OR REPLACE VIEW v_consumables_low_stock AS
SELECT 
    c.id,
    c.name,
    c.manufacturer,
    c.model,
    c.category,
    c.quantity,
    c.min_quantity,
    c.quantity - c.min_quantity AS quantity_diff,
    ht.name AS compatible_type,
    c.unit,
    c.price_per_unit,
    c.quantity * c.price_per_unit AS total_value,
    c.updated_at
FROM consumables c
LEFT JOIN hardware_types ht ON c.compatible_with = ht.id
WHERE c.quantity <= c.min_quantity
ORDER BY c.quantity - c.min_quantity ASC;

-- ============================================================================
-- Представление: Активные заявки с деталями
-- ============================================================================
CREATE OR REPLACE VIEW v_requests_active AS
SELECT 
    r.id,
    r.title,
    r.description,
    r.priority,
    r.status,
    r.category,
    -- Заявитель
    req.first_name || ' ' || req.last_name AS requester_name,
    req.email AS requester_email,
    req.phone AS requester_phone,
    req_dept.name AS requester_department,
    -- Исполнитель
    asn.first_name || ' ' || asn.last_name AS assignee_name,
    asn.email AS assignee_email,
    -- Оборудование
    c.hostname AS computer_hostname,
    c.inventory_number AS computer_inventory,
    eq.name AS equipment_name,
    eq.inventory_number AS equipment_inventory,
    -- Даты
    r.created_at,
    r.updated_at,
    r.resolved_at,
    -- Время обработки
    CASE 
        WHEN r.resolved_at IS NOT NULL THEN 
            EXTRACT(EPOCH FROM (r.resolved_at - r.created_at)) / 3600
        ELSE 
            EXTRACT(EPOCH FROM (CURRENT_TIMESTAMP - r.created_at)) / 3600
    END AS hours_elapsed,
    -- Количество комментариев
    (SELECT COUNT(*) FROM request_comments WHERE request_id = r.id) AS comments_count
FROM requests r
JOIN employees req ON r.requester_id = req.id
LEFT JOIN departments req_dept ON req.department_id = req_dept.id
LEFT JOIN employees asn ON r.assignee_id = asn.id
LEFT JOIN computers c ON r.computer_id = c.id
LEFT JOIN equipment eq ON r.equipment_id = eq.id
WHERE r.status NOT IN ('closed', 'cancelled')
ORDER BY 
    CASE r.priority
        WHEN 'critical' THEN 1
        WHEN 'high' THEN 2
        WHEN 'medium' THEN 3
        WHEN 'low' THEN 4
    END,
    r.created_at ASC;

-- ============================================================================
-- Представление: Последние данные мониторинга
-- ============================================================================
CREATE OR REPLACE VIEW v_monitoring_latest AS
SELECT DISTINCT ON (cm.computer_id)
    cm.computer_id,
    c.hostname,
    c.inventory_number,
    c.status AS computer_status,
    cm.cpu_usage,
    cm.ram_usage_percent,
    cm.ram_total_gb,
    cm.ram_used_gb,
    cm.cpu_temperature,
    cm.gpu_temperature,
    cm.uptime_seconds,
    cm.uptime_seconds / 3600 AS uptime_hours,
    cm.processes_count,
    cm.collected_at,
    -- Статус здоровья
    CASE 
        WHEN cm.cpu_usage > 90 OR cm.ram_usage_percent > 95 THEN 'critical'
        WHEN cm.cpu_usage > 80 OR cm.ram_usage_percent > 85 THEN 'warning'
        ELSE 'good'
    END AS health_status,
    e.first_name || ' ' || e.last_name AS employee_name,
    d.name AS department_name
FROM computer_monitoring cm
JOIN computers c ON cm.computer_id = c.id
LEFT JOIN employees e ON c.employee_id = e.id
LEFT JOIN departments d ON e.department_id = d.id
ORDER BY cm.computer_id, cm.collected_at DESC;

-- ============================================================================
-- Представление: Статистика по отделам
-- ============================================================================
CREATE OR REPLACE VIEW v_department_stats AS
SELECT 
    d.id,
    d.name,
    -- Сотрудники
    COUNT(DISTINCT e.id) AS employees_count,
    COUNT(DISTINCT CASE WHEN e.status = 'active' THEN e.id END) AS active_employees_count,
    -- Компьютеры
    COUNT(DISTINCT c.id) AS computers_count,
    COUNT(DISTINCT CASE WHEN c.status = 'active' THEN c.id END) AS active_computers_count,
    -- Оборудование
    COUNT(DISTINCT eq.id) AS equipment_count,
    -- Заявки
    COUNT(DISTINCT r.id) AS total_requests,
    COUNT(DISTINCT CASE WHEN r.status IN ('new', 'in_progress') THEN r.id END) AS active_requests,
    d.created_at
FROM departments d
LEFT JOIN employees e ON d.id = e.department_id
LEFT JOIN computers c ON e.id = c.employee_id
LEFT JOIN equipment eq ON e.id = eq.employee_id
LEFT JOIN requests r ON e.id = r.requester_id
GROUP BY d.id, d.name, d.created_at;

-- ============================================================================
-- Представление: Истекающие гарантии
-- ============================================================================
CREATE OR REPLACE VIEW v_expiring_warranties AS
SELECT 
    'computer' AS item_type,
    c.id,
    c.inventory_number,
    c.hostname AS name,
    c.manufacturer,
    c.model,
    c.warranty_end_date,
    CURRENT_DATE - c.warranty_end_date AS days_until_expiry,
    e.first_name || ' ' || e.last_name AS employee_name,
    d.name AS department_name
FROM computers c
LEFT JOIN employees e ON c.employee_id = e.id
LEFT JOIN departments d ON e.department_id = d.id
WHERE c.warranty_end_date BETWEEN CURRENT_DATE AND CURRENT_DATE + INTERVAL '90 days'
    AND c.status = 'active'

UNION ALL

SELECT 
    'equipment' AS item_type,
    eq.id,
    eq.inventory_number,
    eq.name,
    eq.manufacturer,
    eq.model,
    eq.warranty_end_date,
    CURRENT_DATE - eq.warranty_end_date AS days_until_expiry,
    e.first_name || ' ' || e.last_name AS employee_name,
    d.name AS department_name
FROM equipment eq
LEFT JOIN employees e ON eq.employee_id = e.id
LEFT JOIN departments d ON e.department_id = d.id
WHERE eq.warranty_end_date BETWEEN CURRENT_DATE AND CURRENT_DATE + INTERVAL '90 days'
    AND eq.status = 'active'

ORDER BY warranty_end_date ASC;

-- ============================================================================
-- Комментарии к представлениям
-- ============================================================================

COMMENT ON VIEW v_employees_full IS 'Полная информация о сотрудниках с должностями и отделами';
COMMENT ON VIEW v_computers_summary IS 'Компьютеры с суммарными характеристиками (RAM, диски)';
COMMENT ON VIEW v_computers_detailed IS 'Детальная информация о компьютерах с компонентами в JSON';
COMMENT ON VIEW v_computer_software_full IS 'Установленное ПО с информацией о лицензиях';
COMMENT ON VIEW v_equipment_full IS 'Оборудование с полной информацией';
COMMENT ON VIEW v_consumables_low_stock IS 'Расходники с низким остатком';
COMMENT ON VIEW v_requests_active IS 'Активные заявки с деталями';
COMMENT ON VIEW v_monitoring_latest IS 'Последние данные мониторинга для каждого компьютера';
COMMENT ON VIEW v_department_stats IS 'Статистика по отделам';
COMMENT ON VIEW v_expiring_warranties IS 'Истекающие гарантии на компьютеры и оборудование';
