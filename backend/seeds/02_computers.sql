-- Демонстрационный парк компьютеров и серверов.

SET client_encoding = 'UTF8';

BEGIN;

-- Тестовый компьютер превращается в рабочую станцию IT-отдела.
UPDATE computers
SET inventory_number          = 'IT-0001',
    hostname                  = 'PC-IT-01',
    computer_type             = 'desktop',
    manufacturer              = 'Dell',
    model                     = 'OptiPlex 7090',
    serial_number             = 'SN-IT-0001',
    os                        = 'Windows 11 Pro',
    os_version                = '23H2',
    os_build                  = '22631',
    os_architecture           = 'x64',
    domain_joined             = TRUE,
    last_boot_time            = now() - interval '6 hours',
    employee_id               = (SELECT id FROM employees WHERE email = 'a.smirnov@company.ru'),
    location                  = 'IT отдел',
    location_id               = (SELECT id FROM locations WHERE name = 'IT отдел'),
    purchase_date             = DATE '2023-04-12',
    warranty_end_date         = DATE '2026-04-12',
    status                    = 'active',
    notes                     = 'Рабочая станция системного администратора',
    last_seen_online          = now() - interval '2 minutes',
    is_online                 = TRUE,
    last_user_logged          = 'a.smirnov',
    bitlocker_enabled         = TRUE,
    antivirus_status          = 'Защищено',
    windows_activation_status = 'Активирована',
    updated_at                = now()
WHERE hostname = 'QA-COMPONENT-01';

INSERT INTO computers (
  inventory_number, hostname, computer_type, manufacturer, model, serial_number,
  os, os_version, os_build, os_architecture, domain_joined, last_boot_time,
  employee_id, location, location_id, purchase_date, warranty_end_date, status,
  notes, last_seen_online, is_online, last_user_logged, bitlocker_enabled,
  antivirus_status, windows_activation_status
)
SELECT v.inventory_number,
       v.hostname,
       v.computer_type,
       v.manufacturer,
       v.model,
       v.serial_number,
       v.os,
       v.os_version,
       v.os_build,
       v.os_architecture,
       v.domain_joined,
       now() - (v.boot_hours || ' hours')::interval,
       e.id,
       l.name,
       l.id,
       v.purchase_date::date,
       v.warranty_end_date::date,
       v.status,
       v.notes,
       now() - (v.seen_minutes || ' minutes')::interval,
       v.is_online,
       v.last_user_logged,
       v.bitlocker_enabled,
       v.antivirus_status,
       v.windows_activation_status
FROM (VALUES
  ('IT-0001',  'PC-IT-01',    'desktop', 'Dell',    'OptiPlex 7090',      'SN-IT-0001',    'Windows 11 Pro',     '23H2', '22631', 'x64',   TRUE,  6,   'a.smirnov@company.ru',   'IT отдел',      '2023-04-12', '2026-04-12', 'active',  'Рабочая станция системного администратора', 2,   TRUE,  'a.smirnov',   TRUE,  'Защищено', 'Активирована'),
  ('IT-0002',  'PC-IT-02',    'desktop', 'HP',      'ProDesk 600 G6',     'SN-IT-0002',    'Windows 11 Pro',     '23H2', '22631', 'x64',   TRUE,  14,  'n.grigoriev@company.ru', 'IT отдел',      '2023-06-20', '2026-06-20', 'active',  'Рабочая станция техника IT',                3,   TRUE,  'n.grigoriev', TRUE,  'Защищено', 'Активирована'),
  ('IT-0003',  'PC-ACC-01',   'laptop',  'Lenovo',  'ThinkPad T14 Gen 3', 'SN-ACC-0001',   'Windows 11 Pro',     '23H2', '22631', 'x64',   TRUE,  4,   'm.kuznetsova@company.ru','Бухгалтерия',   '2024-01-15', '2027-01-15', 'active',  'Ноутбук главного бухгалтера',               1,   TRUE,  'm.kuznetsova',TRUE,  'Защищено', 'Активирована'),
  ('IT-0004',  'PC-ACC-02',   'desktop', 'HP',      'EliteDesk 800 G8',   'SN-ACC-0002',   'Windows 11 Pro',     '23H2', '22631', 'x64',   TRUE,  26,  'd.orlov@company.ru',     'Бухгалтерия',   '2022-09-10', '2025-09-10', 'active',  'Гарантия закончилась, требуется диагностика', 1,  TRUE,  'd.orlov',     TRUE,  'Защищено', 'Активирована'),
  ('IT-0005',  'PC-ACC-03',   'laptop',  'Dell',    'Latitude 5430',      'SN-ACC-0003',   'Windows 11 Pro',     '23H2', '22631', 'x64',   TRUE,  48,  'e.volkova@company.ru',   'Бухгалтерия',   '2024-03-05', '2027-03-05', 'repair',  'Синий экран при загрузке, в ремонте',        180, FALSE, 'e.volkova',   TRUE,  'Защищено', 'Активирована'),
  ('IT-0006',  'PC-SALES-01', 'laptop',  'ASUS',    'ExpertBook B1',      'SN-SALES-0001', 'Windows 11 Pro',     '23H2', '22631', 'x64',   TRUE,  9,   'p.sokolov@company.ru',   'Отдел продаж',  '2023-11-11', '2026-11-11', 'active',  NULL,                                         4,   TRUE,  'p.sokolov',   TRUE,  'Защищено', 'Активирована'),
  ('IT-0007',  'PC-SALES-02', 'laptop',  'Acer',    'TravelMate P2',      'SN-SALES-0002', 'Windows 11 Pro',     '23H2', '22631', 'x64',   TRUE,  2,   'a.lebedeva@company.ru',  'Отдел продаж',  '2024-05-22', '2027-05-22', 'active',  NULL,                                         1,   TRUE,  'a.lebedeva',  TRUE,  'Защищено', 'Активирована'),
  ('IT-0008',  'PC-SALES-03', 'desktop', 'Custom',  'Office Build',       'SN-SALES-0003', 'Windows 10 Pro',     '22H2', '19045', 'x64',   TRUE,  52,  's.morozov@company.ru',   'Отдел продаж',  '2022-02-18', '2025-02-18', 'active',  'Сборка 2022 года, морально устарела',        6,   TRUE,  's.morozov',   FALSE, 'Защищено', 'Активирована'),
  ('IT-0009',  'PC-DIR-01',   'laptop',  'Apple',   'MacBook Pro 14',     'SN-DIR-0001',   'macOS Sonoma',       '14.5', '23F79', 'arm64', FALSE, 12,  'i.novikov@company.ru',   'Главный офис',  '2024-08-01', '2027-08-01', 'active',  'Ноутбук директора',                          7,   TRUE,  'i.novikov',   TRUE,  'Защищено', 'Не применимо'),
  ('IT-0010',  'PC-HR-01',    'laptop',  'Lenovo',  'ThinkPad E14 Gen 5', 'SN-HR-0001',    'Windows 11 Pro',     '23H2', '22631', 'x64',   TRUE,  5,   'o.fedorova@company.ru',  'Главный офис',  '2024-09-12', '2027-09-12', 'active',  NULL,                                         2,   TRUE,  'o.fedorova',  TRUE,  'Защищено', 'Активирована'),
  ('IT-0011',  'SRV-01',      'server',  'Dell',    'PowerEdge R750',     'SN-SRV-0001',   'Windows Server 2022','21H2', '20348', 'x64',   TRUE,  720, NULL,                     'Серверная',     '2023-01-20', '2028-01-20', 'active',  'Контроллер домена, файловый сервер, 1С',      1,   TRUE,  NULL,          TRUE,  'Защищено', 'Активирована'),
  ('IT-0012',  'SRV-BACKUP',  'server',  'HP',      'ProLiant DL380 Gen10','SN-SRV-0002', 'Windows Server 2019','1809', '17763', 'x64',   TRUE,  96,  NULL,                     'Серверная',     '2021-07-14', '2026-07-14', 'active',  'Сервер резервного копирования',               35,  FALSE, NULL,          TRUE,  'Защищено', 'Активирована'),
  ('IT-0013',  'PC-ACC-04',   'desktop', 'Lenovo',  'ThinkCentre M70q',   'SN-ACC-0004',   'Windows 11 Pro',     '23H2', '22631', 'x64',   TRUE,  18,  'v.zakharova@company.ru', 'Бухгалтерия',   '2023-08-30', '2026-08-30', 'active',  NULL,                                         15,  TRUE,  'v.zakharova', TRUE,  'Защищено', 'Активирована'),
  ('IT-0014',  'PC-SALES-04', 'laptop',  'HP',      'ProBook 450 G10',    'SN-SALES-0004', 'Windows 11 Pro',     '23H2', '22631', 'x64',   TRUE,  30,  'a.pavlov@company.ru',    'Отдел продаж',  '2025-03-03', '2028-03-03', 'active',  NULL,                                         20,  TRUE,  'a.pavlov',    TRUE,  'Защищено', 'Активирована')
) AS v(inventory_number, hostname, computer_type, manufacturer, model, serial_number,
       os, os_version, os_build, os_architecture, domain_joined, boot_hours,
       employee_email, location_name, purchase_date, warranty_end_date, status, notes,
       seen_minutes, is_online, last_user_logged, bitlocker_enabled, antivirus_status,
       windows_activation_status)
LEFT JOIN employees e ON e.email = v.employee_email
JOIN locations l ON l.name = v.location_name
ON CONFLICT (hostname) DO NOTHING;

COMMIT;
