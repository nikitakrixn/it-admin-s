-- Демонстрационный журнал активности.

SET client_encoding = 'UTF8';

BEGIN;

DELETE FROM activity_log WHERE details->>'seed' = 'true';

DELETE FROM activity_log
WHERE entity_type = 'employee'
  AND entity_id = 1
  AND details->>'employee_name' = 'тест тестов';

INSERT INTO activity_log (
  user_id, action, entity_type, entity_id, details, ip_address, user_agent, created_at
)
SELECT '549b358e-b0bf-42a0-961b-4b58d956aa99'::uuid, v.action, v.entity_type, v.entity_id,
       v.details::jsonb, '192.168.10.21', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)',
       now() - v.age
FROM (
  SELECT 'created' AS action, 'employee' AS entity_type, e.id AS entity_id,
         jsonb_build_object('seed', TRUE, 'name', e.last_name || ' ' || e.first_name,
                            'email', e.email)::text AS details,
         interval '6 days' AS age
  FROM employees e WHERE e.email = 'm.kuznetsova@company.ru'

  UNION ALL
  SELECT 'created', 'computer', c.id,
         jsonb_build_object('seed', TRUE, 'name', c.hostname, 'inventory_number', c.inventory_number)::text,
         interval '6 days'
  FROM computers c WHERE c.hostname = 'PC-ACC-01'

  UNION ALL
  SELECT 'created', 'equipment', eq.id,
         jsonb_build_object('seed', TRUE, 'name', eq.name, 'inventory_number', eq.inventory_number)::text,
         interval '5 days'
  FROM equipment eq WHERE eq.inventory_number = 'EQ-0001'

  UNION ALL
  SELECT 'created', 'license', l.id,
         jsonb_build_object('seed', TRUE, 'name', 'Microsoft 365', 'license_type', l.license_type)::text,
         interval '5 days'
  FROM license_pools l WHERE l.id = 1

  UNION ALL
  SELECT 'created', 'network', s.id,
         jsonb_build_object('seed', TRUE, 'name', s.name, 'network_address', s.network_address)::text,
         interval '4 days'
  FROM network_segments s WHERE s.name = 'Офисная сеть'

  UNION ALL
  SELECT 'created', 'network', d.id,
         jsonb_build_object('seed', TRUE, 'name', d.name, 'ip_address', d.ip_address)::text,
         interval '4 days'
  FROM network_devices d WHERE d.serial_number = 'SN-NET-001'

  UNION ALL
  SELECT 'created', 'ad', g.id,
         jsonb_build_object('seed', TRUE, 'name', g.name, 'group_type', g.group_type)::text,
         interval '3 days'
  FROM ad_groups g WHERE g.name = 'IT-Administrators'

  UNION ALL
  SELECT 'created', 'ad', a.id,
         jsonb_build_object('seed', TRUE, 'name', a.username, 'account_type', a.account_type)::text,
         interval '3 days'
  FROM domain_accounts a WHERE a.username = 'a.smirnov'

  UNION ALL
  SELECT 'login', 'user', 0,
         jsonb_build_object('seed', TRUE, 'name', 'a.smirnov')::text,
         interval '2 days 4 hours'

  UNION ALL
  SELECT 'software_installed', 'computer', c.id,
         jsonb_build_object('seed', TRUE, 'name', 'Visual Studio Code', 'hostname', c.hostname)::text,
         interval '2 days'
  FROM computers c WHERE c.hostname = 'PC-IT-01'

  UNION ALL
  SELECT 'updated', 'employee', e.id,
         jsonb_build_object('seed', TRUE, 'name', e.last_name || ' ' || e.first_name, 'status', e.status)::text,
         interval '1 day'
  FROM employees e WHERE e.email = 'a.pavlov@company.ru'

  UNION ALL
  SELECT 'created', 'request', r.id,
         jsonb_build_object('seed', TRUE, 'name', r.title, 'priority', r.priority)::text,
         interval '3 days'
  FROM requests r WHERE r.title = 'Восстановить пароль от учётной записи'

  UNION ALL
  SELECT 'updated', 'request', r.id,
         jsonb_build_object('seed', TRUE, 'name', r.title, 'status', r.status)::text,
         interval '2 days 20 hours'
  FROM requests r WHERE r.title = 'Восстановить пароль от учётной записи'

  UNION ALL
  SELECT 'updated', 'consumable', c.id,
         jsonb_build_object('seed', TRUE, 'name', c.name, 'quantity', c.quantity)::text,
         interval '1 day'
  FROM consumables c WHERE c.name = 'Картридж HP 85A'

  UNION ALL
  SELECT 'created', 'request', r.id,
         jsonb_build_object('seed', TRUE, 'name', r.title, 'priority', r.priority)::text,
         interval '10 hours'
  FROM requests r WHERE r.title = 'Синий экран при загрузке ноутбука'

  UNION ALL
  SELECT 'created', 'request', r.id,
         jsonb_build_object('seed', TRUE, 'name', r.title, 'priority', r.priority)::text,
         interval '6 hours'
  FROM requests r WHERE r.title = 'Не включается компьютер в бухгалтерии'

  UNION ALL
  SELECT 'created', 'request', r.id,
         jsonb_build_object('seed', TRUE, 'name', r.title, 'priority', r.priority)::text,
         interval '4 hours'
  FROM requests r WHERE r.title = 'Заменить картридж в принтере HP'

  UNION ALL
  SELECT 'created', 'request', r.id,
         jsonb_build_object('seed', TRUE, 'name', r.title, 'priority', r.priority)::text,
         interval '3 hours'
  FROM requests r WHERE r.title = 'Настроить VPN для удалённой работы'

  UNION ALL
  SELECT 'login', 'user', 0,
         jsonb_build_object('seed', TRUE, 'name', 'n.grigoriev')::text,
         interval '5 hours'

  UNION ALL
  SELECT 'updated', 'computer', c.id,
         jsonb_build_object('seed', TRUE, 'name', c.hostname, 'status', c.status)::text,
         interval '2 hours'
  FROM computers c WHERE c.hostname = 'PC-ACC-03'

  UNION ALL
  SELECT 'updated', 'request', r.id,
         jsonb_build_object('seed', TRUE, 'name', r.title, 'status', r.status)::text,
         interval '1 hour'
  FROM requests r WHERE r.title = 'Не включается компьютер в бухгалтерии'
) AS v(action, entity_type, entity_id, details, age);

COMMIT;
