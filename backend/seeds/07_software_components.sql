-- Установленное ПО, компоненты компьютеров и данные мониторинга.

SET client_encoding = 'UTF8';

BEGIN;

WITH map(hostname, software_name, version, license_type, install_date, status) AS (VALUES
  ('PC-IT-01',    'Google Chrome',        '129.0.6668.90', 'free',       '2024-01-10', 'active'),
  ('PC-IT-01',    '7-Zip',                '24.08',         'free',       '2024-01-10', 'active'),
  ('PC-IT-01',    'Visual Studio Code',   '1.93.1',        'free',       '2024-02-14', 'active'),
  ('PC-IT-01',    'TeamViewer',           '15.57.3',       'subscription','2024-01-11', 'active'),
  ('PC-IT-01',    'Kaspersky',            '21.18.5.438',   'subscription','2024-01-10', 'active'),
  ('PC-IT-01',    'Notepad++',            '8.6.9',         'free',       '2024-03-02', 'active'),
  ('PC-IT-02',    'Google Chrome',        '129.0.6668.90', 'free',       '2024-02-01', 'active'),
  ('PC-IT-02',    '7-Zip',                '24.08',         'free',       '2024-02-01', 'active'),
  ('PC-IT-02',    'TeamViewer',           '15.57.3',       'subscription','2024-02-01', 'active'),
  ('PC-IT-02',    'Kaspersky',            '21.18.5.438',   'subscription','2024-02-01', 'active'),
  ('PC-ACC-01',   'Microsoft 365',        '2408',          'subscription','2024-01-20', 'active'),
  ('PC-ACC-01',   'Google Chrome',        '129.0.6668.90', 'free',       '2024-01-20', 'active'),
  ('PC-ACC-01',   'Kaspersky',            '21.18.5.438',   'subscription','2024-01-20', 'active'),
  ('PC-ACC-01',   'Adobe Acrobat Reader', '24.003.20112',  'free',       '2024-02-05', 'active'),
  ('PC-ACC-02',   'Microsoft Office',     '2021',          'volume',     '2022-09-12', 'active'),
  ('PC-ACC-02',   'Google Chrome',        '129.0.6668.90', 'free',       '2022-09-12', 'active'),
  ('PC-ACC-02',   'Kaspersky',            '21.18.5.438',   'subscription','2022-09-12', 'active'),
  ('PC-ACC-02',   '7-Zip',                '24.08',         'free',       '2022-09-12', 'active'),
  ('PC-ACC-03',   'Microsoft 365',        '2408',          'subscription','2024-03-10', 'active'),
  ('PC-ACC-03',   'Google Chrome',        '129.0.6668.90', 'free',       '2024-03-10', 'active'),
  ('PC-ACC-03',   'Kaspersky',            '21.18.5.438',   'subscription','2024-03-10', 'active'),
  ('PC-SALES-01', 'Microsoft 365',        '2408',          'subscription','2023-11-15', 'active'),
  ('PC-SALES-01', 'Google Chrome',        '129.0.6668.90', 'free',       '2023-11-15', 'active'),
  ('PC-SALES-01', 'Zoom',                 '6.1.11',        'subscription','2023-11-20', 'active'),
  ('PC-SALES-01', 'Telegram',             '5.4.1',         'free',       '2023-11-20', 'active'),
  ('PC-SALES-02', 'Microsoft 365',        '2408',          'subscription','2024-05-25', 'active'),
  ('PC-SALES-02', 'Google Chrome',        '129.0.6668.90', 'free',       '2024-05-25', 'active'),
  ('PC-SALES-02', 'Zoom',                 '6.1.11',        'subscription','2024-05-25', 'active'),
  ('PC-SALES-02', 'Kaspersky',            '21.18.5.438',   'subscription','2024-05-25', 'active'),
  ('PC-SALES-03', 'Microsoft Office',     '2019',          'volume',     '2022-02-20', 'active'),
  ('PC-SALES-03', 'Google Chrome',        '129.0.6668.90', 'free',       '2022-02-20', 'active'),
  ('PC-SALES-03', '7-Zip',                '24.08',         'free',       '2022-02-20', 'active'),
  ('PC-DIR-01',   'Microsoft 365',        '2408',          'subscription','2024-08-05', 'active'),
  ('PC-DIR-01',   'Google Chrome',        '129.0.6668.90', 'free',       '2024-08-05', 'active'),
  ('PC-DIR-01',   'Zoom',                 '6.1.11',        'subscription','2024-08-05', 'active'),
  ('PC-DIR-01',   'Telegram',             '5.4.1',         'free',       '2024-08-05', 'active'),
  ('PC-HR-01',    'Microsoft 365',        '2408',          'subscription','2024-09-15', 'active'),
  ('PC-HR-01',    'Google Chrome',        '129.0.6668.90', 'free',       '2024-09-15', 'active'),
  ('PC-HR-01',    'Kaspersky',            '21.18.5.438',   'subscription','2024-09-15', 'active'),
  ('PC-ACC-04',   'Microsoft 365',        '2408',          'subscription','2023-09-01', 'active'),
  ('PC-ACC-04',   'Google Chrome',        '129.0.6668.90', 'free',       '2023-09-01', 'active'),
  ('PC-ACC-04',   'Kaspersky',            '21.18.5.438',   'subscription','2023-09-01', 'active'),
  ('PC-SALES-04', 'Microsoft 365',        '2408',          'subscription','2025-03-05', 'active'),
  ('PC-SALES-04', 'Google Chrome',        '129.0.6668.90', 'free',       '2025-03-05', 'active'),
  ('PC-SALES-04', 'Zoom',                 '6.1.11',        'subscription','2025-03-05', 'active'),
  ('SRV-01',      'Windows Defender',     '2022',          'oem',        '2023-01-20', 'active'),
  ('SRV-01',      '7-Zip',                '24.08',         'free',       '2023-01-20', 'active'),
  ('SRV-01',      'Kaspersky',            '21.18.5.438',   'subscription','2023-01-20', 'active'),
  ('SRV-BACKUP',  '7-Zip',                '24.08',         'free',       '2021-07-14', 'active'),
  ('SRV-BACKUP',  'Kaspersky',            '21.18.5.438',   'subscription','2021-07-14', 'active')
)
INSERT INTO computer_software (
  computer_id, software_catalog_id, version, license_type, installation_date,
  last_seen_date, status
)
SELECT c.id, s.id, m.version, m.license_type, m.install_date::date,
       now() - interval '1 hour', m.status
FROM map m
JOIN computers c ON c.hostname = m.hostname
JOIN software_catalog s ON s.name = m.software_name
ON CONFLICT (computer_id, software_catalog_id) DO NOTHING;

-- Компоненты рабочей станции PC-IT-01.
INSERT INTO computer_motherboards (
  computer_id, manufacturer, model, chipset, bios_version, bios_date, form_factor,
  serial_number, notes
)
SELECT c.id, 'Dell', '0WMJ54', 'Intel Q570', '2.18.0', DATE '2024-01-10', 'Micro-ATX',
       'MB-IT-0001', 'Штатная плата Dell OptiPlex 7090'
FROM computers c
WHERE c.hostname = 'PC-IT-01'
ON CONFLICT (computer_id) DO NOTHING;

INSERT INTO computer_processors (
  computer_id, name, manufacturer, cores, threads, base_frequency, max_frequency,
  cache_size, socket, tdp, is_primary, notes
)
SELECT c.id, v.name, v.manufacturer, v.cores, v.threads, v.base_frequency,
       v.max_frequency, v.cache_size, v.socket, v.tdp, v.is_primary, v.notes
FROM (VALUES
  ('PC-IT-01',  'Intel Core i7-11700', 'Intel', 8, 16, '2.5 GHz', '4.9 GHz', '16 MB', 'LGA1200', '65 W', TRUE,  'Штатный процессор'),
  ('PC-ACC-01', 'Intel Core i5-1245U', 'Intel', 10, 12, '1.6 GHz', '4.4 GHz', '12 MB', 'BGA1744', '15 W', TRUE, 'Мобильный процессор')
) AS v(hostname, name, manufacturer, cores, threads, base_frequency, max_frequency,
       cache_size, socket, tdp, is_primary, notes)
JOIN computers c ON c.hostname = v.hostname
WHERE NOT EXISTS (
  SELECT 1 FROM computer_processors p WHERE p.computer_id = c.id
);

INSERT INTO computer_ram (
  computer_id, manufacturer, model, capacity_gb, type, frequency, slot_number,
  serial_number, notes
)
SELECT c.id, v.manufacturer, v.model, v.capacity_gb, v.type, v.frequency,
       v.slot_number, v.serial_number, v.notes
FROM (VALUES
  ('PC-IT-01',  'Kingston', 'KVR32N22D8/16', 16, 'DDR4', '3200 MHz', 1, 'RAM-IT-01',  'Установлена в слот 1'),
  ('PC-IT-01',  'Samsung',  'M378A2K43DB1',  16, 'DDR4', '3200 MHz', 2, 'RAM-IT-02',  'Установлена в слот 2'),
  ('PC-ACC-01', 'Micron',   'MTA8ATF1G64HZ', 8,  'DDR4', '3200 MHz', 1, 'RAM-ACC-01', 'Штатная память ноутбука')
) AS v(hostname, manufacturer, model, capacity_gb, type, frequency, slot_number,
       serial_number, notes)
JOIN computers c ON c.hostname = v.hostname
WHERE NOT EXISTS (
  SELECT 1 FROM computer_ram r WHERE r.computer_id = c.id
);

INSERT INTO computer_storage (
  computer_id, type, manufacturer, model, capacity_gb, interface, serial_number,
  health_status, is_system_drive, mount_point, notes, smart_status, temperature,
  power_on_hours, reallocated_sectors, fragmentation_percent, read_errors,
  write_errors, wear_level_percent, power_cycle_count
)
SELECT c.id, v.type, v.manufacturer, v.model, v.capacity_gb, v.interface,
       v.serial_number, v.health_status, v.is_system_drive, v.mount_point, v.notes,
       v.smart_status, v.temperature, v.power_on_hours, v.reallocated_sectors,
       v.fragmentation_percent, v.read_errors, v.write_errors, v.wear_level_percent,
       v.power_cycle_count
FROM (VALUES
  ('PC-IT-01',  'NVMe', 'Samsung', '980 PRO',       1000, 'PCIe 4.0 x4', 'SSD-IT-0001',  'good',    TRUE,  'C:', 'Системный диск',       'OK', 38, 12480, 0, 2, 0, 0, 4, 412),
  ('PC-ACC-01', 'NVMe', 'Samsung', 'PM9A1',          512, 'PCIe 4.0 x4', 'SSD-ACC-0001', 'good',    TRUE,  'C:', 'Системный диск',       'OK', 41, 6210,  0, 3, 0, 0, 6, 238),
  ('SRV-01',    'SSD',  'Intel',   'DC S4610',       960, 'SATA 6Gb/s',  'SSD-SRV-0001', 'good',    TRUE,  'C:', 'Системный диск сервера','OK', 35, 28410, 0, 1, 0, 0, 9, 96)
) AS v(hostname, type, manufacturer, model, capacity_gb, interface, serial_number,
       health_status, is_system_drive, mount_point, notes, smart_status, temperature,
       power_on_hours, reallocated_sectors, fragmentation_percent, read_errors,
       write_errors, wear_level_percent, power_cycle_count)
JOIN computers c ON c.hostname = v.hostname
WHERE NOT EXISTS (
  SELECT 1 FROM computer_storage s WHERE s.computer_id = c.id
);

INSERT INTO computer_gpus (computer_id, name, manufacturer, memory_gb, memory_type, driver_version, is_primary, notes)
SELECT c.id, v.name, v.manufacturer, v.memory_gb, v.memory_type, v.driver_version, v.is_primary, v.notes
FROM (VALUES
  ('PC-IT-01',  'Intel UHD Graphics 750', 'Intel', 1, 'DDR4', '31.0.101.5333', TRUE,  'Встроенная графика'),
  ('PC-ACC-01', 'Intel Iris Xe Graphics', 'Intel', 1, 'DDR4', '31.0.101.5333', TRUE,  'Встроенная графика')
) AS v(hostname, name, manufacturer, memory_gb, memory_type, driver_version, is_primary, notes)
JOIN computers c ON c.hostname = v.hostname
WHERE NOT EXISTS (
  SELECT 1 FROM computer_gpus g WHERE g.computer_id = c.id
);

INSERT INTO computer_monitors (
  computer_id, name, manufacturer, model, serial_number, size_inches, resolution,
  panel_type, refresh_rate, is_primary, notes
)
SELECT c.id, v.name, v.manufacturer, v.model, v.serial_number, v.size_inches,
       v.resolution, v.panel_type, v.refresh_rate, v.is_primary, v.notes
FROM (VALUES
  ('PC-IT-01',  'Dell P2422H',  'Dell',   'P2422H',  'MON-IT-0001',  24, '1920x1080', 'IPS', 60, TRUE,  'Основной монитор'),
  ('PC-IT-01',  'Dell P2422H',  'Dell',   'P2422H',  'MON-IT-0002',  24, '1920x1080', 'IPS', 60, FALSE, 'Второй монитор'),
  ('PC-ACC-01', 'LG 24MK430H',  'LG',     '24MK430H','MON-ACC-0001', 24, '1920x1080', 'IPS', 75, TRUE,  'Монитор на рабочем месте')
) AS v(hostname, name, manufacturer, model, serial_number, size_inches, resolution,
       panel_type, refresh_rate, is_primary, notes)
JOIN computers c ON c.hostname = v.hostname
WHERE NOT EXISTS (
  SELECT 1 FROM computer_monitors m WHERE m.computer_id = c.id
);

INSERT INTO computer_network_adapters (
  computer_id, name, manufacturer, mac_address, ip_address, adapter_type, speed,
  is_primary, status, notes
)
SELECT c.id, v.name, v.manufacturer, v.mac_address, v.ip_address, v.adapter_type,
       v.speed, v.is_primary, v.status, v.notes
FROM (VALUES
  ('PC-IT-01',  'Intel Ethernet Connection I219-LM', 'Intel', 'A4:BB:6D:10:00:21', '192.168.10.21', 'Ethernet', '1 Gbps', TRUE,  'connected', 'Проводное подключение'),
  ('PC-IT-01',  'Intel Wi-Fi 6 AX201',               'Intel', 'A4:BB:6D:10:00:22', NULL,            'WiFi',     '2.4 Gbps', FALSE, 'disabled',  'Wi-Fi отключён'),
  ('PC-ACC-01', 'Intel Wi-Fi 6E AX211',              'Intel', 'A4:BB:6D:10:00:31', '192.168.10.31', 'WiFi',     '2.4 Gbps', TRUE,  'connected', 'Корпоративная сеть')
) AS v(hostname, name, manufacturer, mac_address, ip_address, adapter_type, speed,
       is_primary, status, notes)
JOIN computers c ON c.hostname = v.hostname
WHERE NOT EXISTS (
  SELECT 1 FROM computer_network_adapters a WHERE a.computer_id = c.id
);

INSERT INTO computer_peripherals (
  computer_id, name, type, manufacturer, model, serial_number, connection_type,
  status, purchase_date, warranty_end_date, notes
)
SELECT c.id, v.name, v.type, v.manufacturer, v.model, v.serial_number,
       v.connection_type, v.status, v.purchase_date::date, v.warranty_end_date::date,
       v.notes
FROM (VALUES
  ('PC-IT-01', 'Клавиатура Logitech K120', 'keyboard', 'Logitech', 'K120', 'PER-IT-001', 'USB', 'active', '2024-01-10', '2027-01-10', NULL),
  ('PC-IT-01', 'Мышь Logitech M90',        'mouse',    'Logitech', 'M90',  'PER-IT-002', 'USB', 'active', '2024-01-10', '2027-01-10', NULL),
  ('PC-IT-01', 'Веб-камера Logitech C920', 'webcam',   'Logitech', 'C920', 'PER-IT-003', 'USB', 'active', '2024-02-01', '2027-02-01', 'Для видеозвонков'),
  ('PC-IT-01', 'Гарнитура Jabra Evolve 20','headset',  'Jabra',    'Evolve 20', 'PER-IT-004', 'USB', 'active', '2024-02-01', '2027-02-01', NULL)
) AS v(hostname, name, type, manufacturer, model, serial_number, connection_type,
       status, purchase_date, warranty_end_date, notes)
JOIN computers c ON c.hostname = v.hostname
WHERE NOT EXISTS (
  SELECT 1 FROM computer_peripherals p WHERE p.computer_id = c.id
);

INSERT INTO computer_antivirus (
  computer_id, product_name, manufacturer, version, status, is_enabled,
  real_time_protection, last_update, definitions_version, definitions_date,
  last_scan, last_scan_type, threats_detected, threats_quarantined,
  license_status, license_expiry_date
)
SELECT c.id, v.product_name, v.manufacturer, v.version, v.status, v.is_enabled,
       v.real_time_protection, now() - (v.last_update_hours || ' hours')::interval,
       v.definitions_version, CURRENT_DATE, now() - (v.last_scan_hours || ' hours')::interval,
       v.last_scan_type, 0, 0, v.license_status, v.license_expiry_date::date
FROM (VALUES
  ('PC-IT-01',  'Kaspersky Endpoint Security', 'Kaspersky Lab', '12.5.0.539', 'enabled', TRUE, TRUE, 1,  '2026-11-13', 18, 'quick', 'valid', '2027-01-15'),
  ('PC-ACC-01', 'Kaspersky Endpoint Security', 'Kaspersky Lab', '12.5.0.539', 'enabled', TRUE, TRUE, 2,  '2026-11-13', 26, 'full',  'valid', '2026-12-05'),
  ('SRV-01',    'Kaspersky Endpoint Security', 'Kaspersky Lab', '12.5.0.539', 'enabled', TRUE, TRUE, 1,  '2026-11-13', 72, 'full',  'valid', '2026-12-05')
) AS v(hostname, product_name, manufacturer, version, status, is_enabled,
       real_time_protection, last_update_hours, definitions_version, last_scan_hours,
       last_scan_type, license_status, license_expiry_date)
JOIN computers c ON c.hostname = v.hostname
ON CONFLICT (computer_id) DO NOTHING;

-- Несколько замеров мониторинга для графиков.
INSERT INTO computer_monitoring (
  computer_id, cpu_usage, ram_usage_percent, ram_total_gb, ram_used_gb, disk_usage,
  cpu_temperature, gpu_temperature, uptime_seconds, network_stats, processes_count,
  logged_users, collected_at
)
SELECT c.id, v.cpu_usage, v.ram_usage_percent, v.ram_total_gb, v.ram_used_gb,
       v.disk_usage::jsonb, v.cpu_temperature, v.gpu_temperature, v.uptime_seconds,
       v.network_stats::jsonb, v.processes_count, v.logged_users::jsonb,
       now() - (v.minutes_ago || ' minutes')::interval
FROM (VALUES
  ('PC-IT-01',  14.2, 46.5, 32, 14.9, '[{"mount": "C:", "total_gb": 1000, "used_gb": 412, "percent": 41.2}]', 44.0, 39.0, 21600,  '{"rx_kbps": 120, "tx_kbps": 45}',  186, '["a.smirnov"]',  2),
  ('PC-IT-01',  22.8, 49.1, 32, 15.7, '[{"mount": "C:", "total_gb": 1000, "used_gb": 413, "percent": 41.3}]', 47.0, 41.0, 25200,  '{"rx_kbps": 240, "tx_kbps": 80}',  191, '["a.smirnov"]',  32),
  ('PC-IT-01',  9.6,  44.3, 32, 14.2, '[{"mount": "C:", "total_gb": 1000, "used_gb": 413, "percent": 41.3}]', 42.0, 37.0, 28800,  '{"rx_kbps": 85, "tx_kbps": 30}',   178, '["a.smirnov"]',  62),
  ('PC-ACC-01', 31.5, 58.2, 16, 9.3,  '[{"mount": "C:", "total_gb": 512, "used_gb": 238, "percent": 46.5}]',  51.0, 44.0, 14400,  '{"rx_kbps": 320, "tx_kbps": 110}', 204, '["m.kuznetsova"]', 4),
  ('PC-ACC-01', 27.1, 56.8, 16, 9.1,  '[{"mount": "C:", "total_gb": 512, "used_gb": 238, "percent": 46.5}]',  49.0, 43.0, 18000,  '{"rx_kbps": 280, "tx_kbps": 95}',  198, '["m.kuznetsova"]', 34),
  ('SRV-01',    38.4, 62.7, 64, 40.1, '[{"mount": "C:", "total_gb": 960, "used_gb": 388, "percent": 40.4}, {"mount": "D:", "total_gb": 4000, "used_gb": 2140, "percent": 53.5}]', 58.0, 46.0, 604800, '{"rx_kbps": 4200, "tx_kbps": 3100}', 96, '[]', 1),
  ('SRV-01',    41.2, 64.3, 64, 41.2, '[{"mount": "C:", "total_gb": 960, "used_gb": 389, "percent": 40.5}, {"mount": "D:", "total_gb": 4000, "used_gb": 2142, "percent": 53.6}]', 59.0, 47.0, 608400, '{"rx_kbps": 5100, "tx_kbps": 3800}', 99, '[]', 31)
) AS v(hostname, cpu_usage, ram_usage_percent, ram_total_gb, ram_used_gb,
       disk_usage, cpu_temperature, gpu_temperature, uptime_seconds, network_stats,
       processes_count, logged_users, minutes_ago)
JOIN computers c ON c.hostname = v.hostname
WHERE NOT EXISTS (
  SELECT 1 FROM computer_monitoring m WHERE m.computer_id = c.id
);

COMMIT;
