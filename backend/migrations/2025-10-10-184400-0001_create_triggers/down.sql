-- Удаление триггеров для updated_at
DROP TRIGGER IF EXISTS update_departments_updated_at ON departments;
DROP TRIGGER IF EXISTS update_positions_updated_at ON positions;
DROP TRIGGER IF EXISTS update_employees_updated_at ON employees;
DROP TRIGGER IF EXISTS update_users_updated_at ON users;
DROP TRIGGER IF EXISTS update_computers_updated_at ON computers;
DROP TRIGGER IF EXISTS update_computer_processors_updated_at ON computer_processors;
DROP TRIGGER IF EXISTS update_computer_ram_updated_at ON computer_ram;
DROP TRIGGER IF EXISTS update_computer_storage_updated_at ON computer_storage;
DROP TRIGGER IF EXISTS update_computer_gpus_updated_at ON computer_gpus;
DROP TRIGGER IF EXISTS update_computer_motherboards_updated_at ON computer_motherboards;
DROP TRIGGER IF EXISTS update_computer_network_adapters_updated_at ON computer_network_adapters;
DROP TRIGGER IF EXISTS update_computer_monitors_updated_at ON computer_monitors;
DROP TRIGGER IF EXISTS update_computer_peripherals_updated_at ON computer_peripherals;
DROP TRIGGER IF EXISTS update_software_catalog_updated_at ON software_catalog;
DROP TRIGGER IF EXISTS update_computer_software_updated_at ON computer_software;
DROP TRIGGER IF EXISTS update_hardware_types_updated_at ON hardware_types;
DROP TRIGGER IF EXISTS update_equipment_updated_at ON equipment;
DROP TRIGGER IF EXISTS update_consumables_updated_at ON consumables;
DROP TRIGGER IF EXISTS update_requests_updated_at ON requests;

-- Удаление триггеров для расходников
DROP TRIGGER IF EXISTS update_consumable_quantity_trigger ON consumable_movements;

-- Удаление триггеров для логирования компонентов
DROP TRIGGER IF EXISTS log_processor_changes_trigger ON computer_processors;
DROP TRIGGER IF EXISTS log_ram_changes_trigger ON computer_ram;
DROP TRIGGER IF EXISTS log_storage_changes_trigger ON computer_storage;

-- Удаление триггеров для заявок
DROP TRIGGER IF EXISTS auto_close_resolved_requests_trigger ON requests;

-- Удаление функций
DROP FUNCTION IF EXISTS update_updated_at_column();
DROP FUNCTION IF EXISTS update_consumable_quantity();
DROP FUNCTION IF EXISTS log_processor_changes();
DROP FUNCTION IF EXISTS log_ram_changes();
DROP FUNCTION IF EXISTS log_storage_changes();
DROP FUNCTION IF EXISTS auto_close_resolved_requests();
DROP FUNCTION IF EXISTS update_last_login();
