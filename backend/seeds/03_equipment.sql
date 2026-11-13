-- Демонстрационная оргтехника: принтеры, МФУ, сканеры, ИБП, телефоны.

SET client_encoding = 'UTF8';

BEGIN;

INSERT INTO equipment (
  inventory_number, name, type_id, manufacturer, model, serial_number, employee_id,
  location, location_id, ip_address, mac_address, purchase_date, warranty_end_date,
  status, notes, last_maintenance_date, next_maintenance_date, toner_level,
  page_count, paper_jam_count, total_pages_printed, color_pages_printed,
  mono_pages_printed, last_error_code, last_error_time, supplies_status
)
SELECT v.inventory_number,
       v.name,
       ht.id,
       v.manufacturer,
       v.model,
       v.serial_number,
       e.id,
       l.name,
       l.id,
       v.ip_address,
       v.mac_address,
       v.purchase_date::date,
       v.warranty_end_date::date,
       v.status,
       v.notes,
       v.last_maintenance_date::date,
       v.next_maintenance_date::date,
       v.toner_level,
       v.page_count,
       v.paper_jam_count,
       v.total_pages_printed,
       v.color_pages_printed,
       v.mono_pages_printed,
       v.last_error_code,
       CASE WHEN v.last_error_days IS NULL THEN NULL ELSE now() - (v.last_error_days || ' days')::interval END,
       v.supplies_status::jsonb
FROM (VALUES
  ('EQ-0001', 'МФУ Kyocera ECOSYS M3145dn',        'МФУ',                'Kyocera', 'ECOSYS M3145dn',      'KDC-M3145-001', 'm.kuznetsova@company.ru', 'Бухгалтерия',   '192.168.10.51', '00:17:C8:3A:11:01', '2023-02-10', '2026-02-10', 'active',  'Основное МФУ бухгалтерии',                   '2026-09-15', '2027-03-15', 34, 48210,  7, 48210,     0, 48210, NULL,    NULL, '{"toner": 34, "drum": 78}'),
  ('EQ-0002', 'Принтер HP LaserJet Pro M404dn',    'Лазерный принтер',   'HP',      'LaserJet Pro M404dn', 'HP-M404-002',  'p.sokolov@company.ru',    'Отдел продаж',  '192.168.10.52', '00:1B:78:2C:22:02', '2022-05-18', '2025-05-18', 'active',  'Низкий уровень тонера, нужен картридж',      '2026-08-01', '2027-02-01',  8, 73540, 12, 73540,     0, 73540, NULL,    NULL, '{"toner": 8, "drum": 54}'),
  ('EQ-0003', 'Принтер Canon i-SENSYS LBP6030',    'Лазерный принтер',   'Canon',   'i-SENSYS LBP6030',    'CN-LBP-003',   'o.fedorova@company.ru',   'Главный офис',  '192.168.10.53', '00:1E:8F:44:33:03', '2021-10-05', '2024-10-05', 'active',  'Принтер HR-отдела',                          '2026-06-20', '2026-12-20', 56, 22110,  3, 22110,     0, 22110, NULL,    NULL, '{"toner": 56, "drum": 82}'),
  ('EQ-0004', 'Сканер Epson Perfection V39',       'Сканер',             'Epson',   'Perfection V39',      'EP-V39-004',   'm.kuznetsova@company.ru', 'Бухгалтерия',   NULL,            NULL,                '2022-03-14', '2025-03-14', 'active',  'Сканирование первичных документов',          NULL,         NULL,         NULL, NULL,   NULL, NULL,  NULL,   NULL,   NULL,   NULL,    NULL),
  ('EQ-0005', 'ИБП APC Smart-UPS 1500VA',          'ИБП',                'APC',     'Smart-UPS SMT1500I',  'APC-SMT-005',  NULL,                      'Серверная',     NULL,            NULL,                '2021-12-01', '2024-12-01', 'active',  'Питание серверной стойки',                   '2026-10-01', '2027-04-01', NULL, NULL,   NULL, NULL,  NULL,   NULL,   NULL,   NULL,    NULL),
  ('EQ-0006', 'Проектор Epson EB-FH52',            'Проектор',           'Epson',   'EB-FH52',             'EP-FH52-006',  NULL,                      'Главный офис',  NULL,            NULL,                '2023-09-20', '2026-09-20', 'storage', 'Хранится в переговорной, используется на встречах', NULL,  NULL,         NULL, NULL,   NULL, NULL,  NULL,   NULL,   NULL,   NULL,    NULL),
  ('EQ-0007', 'IP-телефон Yealink T31P (директор)','Телефон IP',         'Yealink', 'SIP-T31P',            'YL-T31-007',   'i.novikov@company.ru',    'Главный офис',  '192.168.10.61', '80:5E:C0:77:44:07', '2024-02-12', '2027-02-12', 'active',  'Кабинет директора',                          NULL,         NULL,         NULL, NULL,   NULL, NULL,  NULL,   NULL,   NULL,   NULL,    NULL),
  ('EQ-0008', 'IP-телефон Yealink T31P (продажи)', 'Телефон IP',         'Yealink', 'SIP-T31P',            'YL-T31-008',   'p.sokolov@company.ru',    'Отдел продаж',  '192.168.10.62', '80:5E:C0:77:44:08', '2024-02-12', '2027-02-12', 'active',  'Общий телефон отдела продаж',                NULL,         NULL,         NULL, NULL,   NULL, NULL,  NULL,   NULL,   NULL,   NULL,    NULL),
  ('EQ-0009', 'IP-телефон Yealink T31P (HR)',      'Телефон IP',         'Yealink', 'SIP-T31P',            'YL-T31-009',   'o.fedorova@company.ru',   'Главный офис',  '192.168.10.63', '80:5E:C0:77:44:09', '2024-02-12', '2027-02-12', 'active',  'Телефон HR-отдела',                          NULL,         NULL,         NULL, NULL,   NULL, NULL,  NULL,   NULL,   NULL,   NULL,    NULL),
  ('EQ-0010', 'МФУ Xerox VersaLink C405',          'МФУ',                'Xerox',   'VersaLink C405',      'XR-C405-010',  NULL,                      'Администрация', '192.168.10.54', '9C:93:4E:55:66:10', '2022-11-08', '2025-11-08', 'repair',  'Ошибка подачи бумаги, ожидает ремонта',       '2026-07-12', '2027-01-12', 41, 15890, 23, 15890,  6400,  9490, 'E-042', 2,    '{"toner": {"cyan": 62, "magenta": 58, "yellow": 71, "black": 41}}')
) AS v(inventory_number, name, type_name, manufacturer, model, serial_number,
       employee_email, location_name, ip_address, mac_address, purchase_date,
       warranty_end_date, status, notes, last_maintenance_date, next_maintenance_date,
       toner_level, page_count, paper_jam_count, total_pages_printed,
       color_pages_printed, mono_pages_printed, last_error_code, last_error_days,
       supplies_status)
JOIN hardware_types ht ON ht.name = v.type_name
LEFT JOIN employees e ON e.email = v.employee_email
JOIN locations l ON l.name = v.location_name
ON CONFLICT (inventory_number) DO NOTHING;

COMMIT;
