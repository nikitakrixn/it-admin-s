-- Демонстрационная сеть: сегменты, устройства, IP-адреса и история сканирований.

SET client_encoding = 'UTF8';

BEGIN;

INSERT INTO network_segments (
  name, network_address, subnet_mask, gateway, vlan_id, dns_servers, dhcp_enabled,
  dhcp_range_start, dhcp_range_end, description, location
)
SELECT v.name, v.network_address, v.subnet_mask, v.gateway, v.vlan_id,
       v.dns_servers, v.dhcp_enabled, v.dhcp_range_start, v.dhcp_range_end,
       v.description, v.location
FROM (VALUES
  ('Офисная сеть',   '192.168.10.0', '255.255.255.0', '192.168.10.1', 10, '192.168.10.1, 8.8.8.8', TRUE,  '192.168.10.100', '192.168.10.200', 'Рабочие станции и оргтехника', 'Главный офис'),
  ('Серверная сеть', '192.168.20.0', '255.255.255.0', '192.168.20.1', 20, '192.168.10.1',          FALSE, NULL,             NULL,             'Серверы и системы хранения',    'Серверная'),
  ('Гостевая сеть',  '192.168.30.0', '255.255.255.0', '192.168.30.1', 30, '8.8.8.8, 1.1.1.1',      TRUE,  '192.168.30.50',  '192.168.30.150', 'Wi-Fi для гостей, изолирована',  'Главный офис')
) AS v(name, network_address, subnet_mask, gateway, vlan_id, dns_servers,
       dhcp_enabled, dhcp_range_start, dhcp_range_end, description, location)
WHERE NOT EXISTS (SELECT 1 FROM network_segments s WHERE s.name = v.name);

INSERT INTO network_devices (
  device_type, name, manufacturer, model, serial_number, ip_address, mac_address,
  management_url, location, firmware_version, port_count, status, purchase_date,
  warranty_end_date, notes
)
VALUES
  ('router',       'MikroTik CCR2004',          'MikroTik',  'CCR2004-1G-12S+2XS', 'SN-NET-001', '192.168.10.1', '48:8F:5A:00:01:01', 'https://192.168.10.1', 'Серверная',    'RouterOS 7.14', 14, 'active',      '2022-01-10', '2027-01-10', 'Основной маршрутизатор офиса'),
  ('firewall',     'Fortinet FortiGate 60F',    'Fortinet',  'FortiGate 60F',      'SN-NET-002', '192.168.10.2', '90:6C:AC:00:02:02', 'https://192.168.10.2', 'Серверная',    'FortiOS 7.4.3', 10, 'active',      '2022-01-10', '2027-01-10', 'Межсетевой экран и VPN-концентратор'),
  ('switch',       'Cisco Catalyst 2960X-48',   'Cisco',     'WS-C2960X-48FPD-L',  'SN-NET-003', '192.168.10.3', '00:1E:14:00:03:03', 'https://192.168.10.3', 'Серверная',    '15.2(7)E',      48, 'active',      '2021-04-05', '2026-04-05', 'Коммутатор доступа, 1 этаж'),
  ('switch',       'Cisco Catalyst 2960X-24',   'Cisco',     'WS-C2960X-24TS-L',   'SN-NET-004', '192.168.10.4', '00:1E:14:00:04:04', 'https://192.168.10.4', '2 этаж',       '15.2(7)E',      24, 'active',      '2021-04-05', '2026-04-05', 'Коммутатор доступа, 2 этаж'),
  ('access_point', 'Ubiquiti UniFi U6 Pro',     'Ubiquiti',  'U6-Pro',             'SN-NET-005', '192.168.10.5', '74:AC:B9:00:05:05', 'https://192.168.10.5', 'Главный офис', '6.6.65',         1, 'active',      '2023-03-14', '2026-03-14', 'Wi-Fi в переговорной и на ресепшене'),
  ('access_point', 'Ubiquiti UniFi U6 Lite',    'Ubiquiti',  'U6-Lite',            'SN-NET-006', '192.168.10.6', '74:AC:B9:00:06:06', 'https://192.168.10.6', 'Отдел продаж', '6.6.65',         1, 'maintenance', '2023-03-14', '2026-03-14', 'Плановое обновление прошивки'),
  ('vpn_gateway',  'MikroTik hEX S',            'MikroTik',  'RB760iGS',           'SN-NET-007', '192.168.20.1', '48:8F:5A:00:07:07', 'https://192.168.20.1', 'Серверная',    'RouterOS 7.14',  5, 'active',      '2022-06-01', '2027-06-01', 'Шлюз для удалённого доступа')
ON CONFLICT (serial_number) DO NOTHING;

-- Адреса рабочих станций.
WITH map(hostname, ip, mac) AS (VALUES
  ('PC-IT-01',    '192.168.10.21', 'A4:BB:6D:10:00:21'),
  ('PC-IT-02',    '192.168.10.22', 'A4:BB:6D:10:00:22'),
  ('PC-ACC-01',   '192.168.10.31', 'A4:BB:6D:10:00:31'),
  ('PC-ACC-02',   '192.168.10.32', 'A4:BB:6D:10:00:32'),
  ('PC-ACC-03',   '192.168.10.33', 'A4:BB:6D:10:00:33'),
  ('PC-ACC-04',   '192.168.10.34', 'A4:BB:6D:10:00:34'),
  ('PC-SALES-01', '192.168.10.41', 'A4:BB:6D:10:00:41'),
  ('PC-SALES-02', '192.168.10.42', 'A4:BB:6D:10:00:42'),
  ('PC-SALES-03', '192.168.10.43', 'A4:BB:6D:10:00:43'),
  ('PC-SALES-04', '192.168.10.44', 'A4:BB:6D:10:00:44'),
  ('PC-DIR-01',   '192.168.10.11', 'A4:BB:6D:10:00:11'),
  ('PC-HR-01',    '192.168.10.12', 'A4:BB:6D:10:00:12')
)
INSERT INTO ip_addresses (
  network_segment_id, ip_address, status, assigned_to_type, assigned_to_id,
  hostname, mac_address, is_reserved, last_seen, notes
)
SELECT s.id, m.ip, 'assigned', 'computer', c.id, c.hostname, m.mac, FALSE,
       now() - interval '3 minutes', NULL
FROM map m
JOIN computers c ON c.hostname = m.hostname
JOIN network_segments s ON s.name = 'Офисная сеть'
ON CONFLICT (ip_address) DO NOTHING;

-- Серверы.
WITH map(hostname, ip, mac) AS (VALUES
  ('SRV-01',     '192.168.20.10', 'A4:BB:6D:20:00:10'),
  ('SRV-BACKUP', '192.168.20.11', 'A4:BB:6D:20:00:11')
)
INSERT INTO ip_addresses (
  network_segment_id, ip_address, status, assigned_to_type, assigned_to_id,
  hostname, mac_address, is_reserved, last_seen, notes
)
SELECT s.id, m.ip, 'assigned', 'computer', c.id, c.hostname, m.mac, FALSE,
       now() - interval '1 minute', NULL
FROM map m
JOIN computers c ON c.hostname = m.hostname
JOIN network_segments s ON s.name = 'Серверная сеть'
ON CONFLICT (ip_address) DO NOTHING;

-- Оргтехника.
WITH map(inventory_number, ip, mac) AS (VALUES
  ('EQ-0001', '192.168.10.51', '00:17:C8:3A:11:01'),
  ('EQ-0002', '192.168.10.52', '00:1B:78:2C:22:02'),
  ('EQ-0003', '192.168.10.53', '00:1E:8F:44:33:03'),
  ('EQ-0007', '192.168.10.61', '80:5E:C0:77:44:07'),
  ('EQ-0008', '192.168.10.62', '80:5E:C0:77:44:08'),
  ('EQ-0009', '192.168.10.63', '80:5E:C0:77:44:09'),
  ('EQ-0010', '192.168.10.54', '9C:93:4E:55:66:10')
)
INSERT INTO ip_addresses (
  network_segment_id, ip_address, status, assigned_to_type, assigned_to_id,
  hostname, mac_address, is_reserved, last_seen, notes
)
SELECT s.id, m.ip, 'assigned', 'equipment', e.id, e.name, m.mac, FALSE,
       now() - interval '10 minutes', NULL
FROM map m
JOIN equipment e ON e.inventory_number = m.inventory_number
JOIN network_segments s ON s.name = 'Офисная сеть'
ON CONFLICT (ip_address) DO NOTHING;

-- Служебные и свободные адреса.
INSERT INTO ip_addresses (
  network_segment_id, ip_address, status, hostname, is_reserved,
  reservation_reason, last_seen, notes
)
SELECT s.id, v.ip, v.status, v.hostname, v.is_reserved, v.reason,
       CASE WHEN v.status = 'offline' THEN now() - interval '3 days' ELSE NULL END,
       v.notes
FROM (VALUES
  ('Офисная сеть',   '192.168.10.1',   'reserved',  'gateway-office',  TRUE,  'Шлюз офисной сети',           'Служебный адрес'),
  ('Серверная сеть', '192.168.20.1',   'reserved',  'gateway-servers', TRUE,  'Шлюз серверной сети',         'Служебный адрес'),
  ('Гостевая сеть',  '192.168.30.1',   'reserved',  'gateway-guest',   TRUE,  'Шлюз гостевой сети',          'Служебный адрес'),
  ('Офисная сеть',   '192.168.10.100', 'available', NULL,              FALSE, NULL,                          'Свободный адрес из DHCP-пула'),
  ('Офисная сеть',   '192.168.10.101', 'available', NULL,              FALSE, NULL,                          'Свободный адрес из DHCP-пула'),
  ('Офисная сеть',   '192.168.10.102', 'available', NULL,              FALSE, NULL,                          'Свободный адрес из DHCP-пула'),
  ('Офисная сеть',   '192.168.10.200', 'reserved',  'printer-pool',    TRUE,  'Пул для сетевой печати',      'Зарезервировано администратором'),
  ('Офисная сеть',   '192.168.10.99',  'conflict',  'unknown-device',  FALSE, NULL,                          'Конфликт адресов: отвечают два MAC'),
  ('Офисная сеть',   '192.168.10.60',  'offline',   'old-scanner',     FALSE, NULL,                          'Старый сканер, выведен из эксплуатации'),
  ('Гостевая сеть',  '192.168.30.50',  'available', NULL,              FALSE, NULL,                          'Свободный адрес гостевой сети'),
  ('Гостевая сеть',  '192.168.30.51',  'available', NULL,              FALSE, NULL,                          'Свободный адрес гостевой сети'),
  ('Серверная сеть', '192.168.20.20',  'available', NULL,              FALSE, NULL,                          'Резерв для нового сервера')
) AS v(segment_name, ip, status, hostname, is_reserved, reason, notes)
JOIN network_segments s ON s.name = v.segment_name
ON CONFLICT (ip_address) DO NOTHING;

-- История сканирований сети.
INSERT INTO network_scans (
  scan_type, network_segment_id, ip_range, devices_found, scan_duration,
  scan_results, initiated_by, started_at, completed_at, status, created_at
)
SELECT v.scan_type,
       s.id,
       v.ip_range,
       v.devices_found,
       v.scan_duration,
       v.scan_results::jsonb,
       '549b358e-b0bf-42a0-961b-4b58d956aa99'::uuid,
       now() - (v.hours_ago || ' hours')::interval,
       now() - (v.hours_ago || ' hours')::interval + (v.scan_duration || ' seconds')::interval,
       'completed',
       now() - (v.hours_ago || ' hours')::interval
FROM (VALUES
  ('ping_sweep', 'Офисная сеть',   '192.168.10.1-192.168.10.254', 18, 42, '[{"ip": "192.168.10.1", "ports": [443]}, {"ip": "192.168.10.21", "ports": [135, 445]}, {"ip": "192.168.10.31", "ports": [445]}]', 26),
  ('port_scan',  'Серверная сеть', '192.168.20.10-192.168.20.11', 2,  9,  '[{"ip": "192.168.20.10", "ports": [80, 443, 3389]}, {"ip": "192.168.20.11", "ports": [445, 3389]}]', 5)
) AS v(scan_type, segment_name, ip_range, devices_found, scan_duration, scan_results, hours_ago)
JOIN network_segments s ON s.name = v.segment_name
WHERE NOT EXISTS (
  SELECT 1 FROM network_scans n
  WHERE n.ip_range = v.ip_range AND n.scan_type = v.scan_type
);

COMMIT;
