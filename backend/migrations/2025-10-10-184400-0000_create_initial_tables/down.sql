-- Журналирование и аудит
DROP TABLE IF EXISTS notifications CASCADE;
DROP TABLE IF EXISTS activity_log CASCADE;

-- Заявки и инциденты
DROP TABLE IF EXISTS request_attachments CASCADE;
DROP TABLE IF EXISTS request_comments CASCADE;
DROP TABLE IF EXISTS requests CASCADE;

-- Расходники
DROP TABLE IF EXISTS consumable_movements CASCADE;
DROP TABLE IF EXISTS consumables CASCADE;

-- Оборудование
DROP TABLE IF EXISTS equipment CASCADE;
DROP TABLE IF EXISTS hardware_types CASCADE;

-- Программное обеспечение
DROP TABLE IF EXISTS software_history CASCADE;
DROP TABLE IF EXISTS computer_software CASCADE;
DROP TABLE IF EXISTS software_name_mappings CASCADE;
DROP TABLE IF EXISTS software_catalog CASCADE;

-- Мониторинг
DROP TABLE IF EXISTS computer_monitoring CASCADE;

-- Компоненты компьютеров
DROP TABLE IF EXISTS component_history CASCADE;
DROP TABLE IF EXISTS computer_peripherals CASCADE;
DROP TABLE IF EXISTS computer_monitors CASCADE;
DROP TABLE IF EXISTS computer_network_adapters CASCADE;
DROP TABLE IF EXISTS computer_motherboards CASCADE;
DROP TABLE IF EXISTS computer_gpus CASCADE;
DROP TABLE IF EXISTS computer_storage CASCADE;
DROP TABLE IF EXISTS computer_ram CASCADE;
DROP TABLE IF EXISTS computer_processors CASCADE;

-- Компьютеры
DROP TABLE IF EXISTS computers CASCADE;

-- API токены
DROP TABLE IF EXISTS api_tokens CASCADE;

-- Пользователи и сотрудники
DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS employees CASCADE;
DROP TABLE IF EXISTS positions CASCADE;
DROP TABLE IF EXISTS departments CASCADE;

-- Удаление расширений
DROP EXTENSION IF EXISTS "uuid-ossp";
