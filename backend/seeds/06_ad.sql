-- Демонстрационные группы и учётные записи Active Directory.

SET client_encoding = 'UTF8';

BEGIN;

INSERT INTO ad_groups (
  name, distinguished_name, description, group_type, group_scope, member_count, last_synced
)
VALUES
  ('IT-Administrators', 'CN=IT-Administrators,OU=Groups,DC=company,DC=local', 'Администраторы инфраструктуры',       'Security',     'Global',      0, now() - interval '15 minutes'),
  ('Domain Users',      'CN=Domain Users,OU=Groups,DC=company,DC=local',      'Все пользователи домена',             'Security',     'Global',      0, now() - interval '15 minutes'),
  ('Finance',           'CN=Finance,OU=Groups,DC=company,DC=local',           'Доступ к бухгалтерским системам',      'Security',     'Global',      0, now() - interval '15 minutes'),
  ('Sales',             'CN=Sales,OU=Groups,DC=company,DC=local',             'Доступ к CRM и коммерческим данным',   'Security',     'Global',      0, now() - interval '15 minutes'),
  ('HR',                'CN=HR,OU=Groups,DC=company,DC=local',                'Доступ к кадровым документам',         'Security',     'Global',      0, now() - interval '15 minutes'),
  ('VPN Users',         'CN=VPN Users,OU=Groups,DC=company,DC=local',         'Удалённый доступ через VPN',           'Security',     'Global',      0, now() - interval '15 minutes'),
  ('All Staff',         'CN=All Staff,OU=Groups,DC=company,DC=local',         'Список рассылки для всех сотрудников', 'Distribution', 'Universal',   0, now() - interval '15 minutes')
ON CONFLICT (name) DO NOTHING;

INSERT INTO domain_accounts (
  employee_id, username, display_name, email, account_type, distinguished_name,
  user_principal_name, sam_account_name, password_last_set, password_expires_at,
  password_never_expires, must_change_password, cannot_change_password,
  account_expires_at, is_enabled, is_locked, locked_at, last_logon, last_logon_ip,
  logon_count, bad_password_count, home_directory, profile_path, description, notes
)
SELECT e.id,
       v.username,
       v.display_name,
       v.email,
       v.account_type,
       v.distinguished_name,
       v.username || '@company.local',
       v.username,
       now() - (v.password_last_set_days || ' days')::interval,
       CASE WHEN v.password_expires_days IS NULL THEN NULL ELSE now() + (v.password_expires_days || ' days')::interval END,
       v.password_never_expires,
       v.must_change_password,
       FALSE,
       NULL,
       v.is_enabled,
       v.is_locked,
       CASE WHEN v.is_locked THEN now() - interval '2 days' ELSE NULL END,
       CASE WHEN v.last_logon_hours IS NULL THEN NULL ELSE now() - (v.last_logon_hours || ' hours')::interval END,
       v.last_logon_ip,
       v.logon_count,
       0,
       v.home_directory,
       v.profile_path,
       v.description,
       NULL
FROM (VALUES
  ('a.smirnov@company.ru',    'a.smirnov',    'Алексей Смирнов',    'a.smirnov@company.ru',    'admin',   'CN=Алексей Смирнов,OU=Users,DC=company,DC=local',    30, 60,   FALSE, FALSE, TRUE,  FALSE, 1,   '192.168.10.21', 342, '\\fs01\home\a.smirnov',    '\\fs01\profiles\a.smirnov',    'Системный администратор'),
  ('n.grigoriev@company.ru',  'n.grigoriev',  'Никита Григорьев',   'n.grigoriev@company.ru',  'user',    'CN=Никита Григорьев,OU=Users,DC=company,DC=local',   12, 78,   FALSE, FALSE, TRUE,  FALSE, 2,   '192.168.10.22', 187, '\\fs01\home\n.grigoriev',  '\\fs01\profiles\n.grigoriev',  'Техник IT'),
  ('m.kuznetsova@company.ru', 'm.kuznetsova', 'Мария Кузнецова',    'm.kuznetsova@company.ru', 'user',    'CN=Мария Кузнецова,OU=Users,DC=company,DC=local',    45, 45,   FALSE, FALSE, TRUE,  FALSE, 1,   '192.168.10.31', 512, '\\fs01\home\m.kuznetsova', '\\fs01\profiles\m.kuznetsova', 'Главный бухгалтер'),
  ('d.orlov@company.ru',      'd.orlov',      'Дмитрий Орлов',      'd.orlov@company.ru',      'user',    'CN=Дмитрий Орлов,OU=Users,DC=company,DC=local',      20, 70,   FALSE, FALSE, TRUE,  FALSE, 3,   '192.168.10.32', 264, '\\fs01\home\d.orlov',      '\\fs01\profiles\d.orlov',      'Бухгалтер'),
  ('e.volkova@company.ru',    'e.volkova',    'Екатерина Волкова',  'e.volkova@company.ru',    'user',    'CN=Екатерина Волкова,OU=Users,DC=company,DC=local',  8,  82,   FALSE, FALSE, TRUE,  FALSE, 30,  '192.168.10.33', 143, '\\fs01\home\e.volkova',    '\\fs01\profiles\e.volkova',    'Бухгалтер'),
  ('p.sokolov@company.ru',    'p.sokolov',    'Павел Соколов',      'p.sokolov@company.ru',    'user',    'CN=Павел Соколов,OU=Users,DC=company,DC=local',      60, 30,   FALSE, FALSE, TRUE,  FALSE, 1,   '192.168.10.41', 398, '\\fs01\home\p.sokolov',    '\\fs01\profiles\p.sokolov',    'Менеджер по продажам'),
  ('a.lebedeva@company.ru',   'a.lebedeva',   'Анна Лебедева',      'a.lebedeva@company.ru',   'user',    'CN=Анна Лебедева,OU=Users,DC=company,DC=local',      15, 75,   FALSE, FALSE, TRUE,  FALSE, 2,   '192.168.10.42', 221, '\\fs01\home\a.lebedeva',   '\\fs01\profiles\a.lebedeva',   'Менеджер по продажам'),
  ('s.morozov@company.ru',    's.morozov',    'Сергей Морозов',     's.morozov@company.ru',    'user',    'CN=Сергей Морозов,OU=Users,DC=company,DC=local',     70, 20,   FALSE, FALSE, TRUE,  FALSE, 4,   '192.168.10.43', 176, '\\fs01\home\s.morozov',    '\\fs01\profiles\s.morozov',    'Менеджер по продажам'),
  ('i.novikov@company.ru',    'i.novikov',    'Игорь Новиков',      'i.novikov@company.ru',    'user',    'CN=Игорь Новиков,OU=Users,DC=company,DC=local',      25, 65,   FALSE, FALSE, TRUE,  FALSE, 1,   '192.168.10.11', 730, '\\fs01\home\i.novikov',    '\\fs01\profiles\i.novikov',    'Генеральный директор'),
  ('o.fedorova@company.ru',   'o.fedorova',   'Ольга Фёдорова',     'o.fedorova@company.ru',   'user',    'CN=Ольга Фёдорова,OU=Users,DC=company,DC=local',     10, 80,   FALSE, FALSE, TRUE,  FALSE, 2,   '192.168.10.12', 289, '\\fs01\home\o.fedorova',   '\\fs01\profiles\o.fedorova',   'HR менеджер'),
  ('a.pavlov@company.ru',     'a.pavlov',     'Артём Павлов',       'a.pavlov@company.ru',     'user',    'CN=Артём Павлов,OU=Users,DC=company,DC=local',       90, 0,    FALSE, FALSE, TRUE,  FALSE, 120, '192.168.10.44', 64,  '\\fs01\home\a.pavlov',     '\\fs01\profiles\a.pavlov',     'Менеджер по продажам, в отпуске'),
  ('v.zakharova@company.ru',  'v.zakharova',  'Виктория Захарова',  'v.zakharova@company.ru',  'user',    'CN=Виктория Захарова,OU=Users,DC=company,DC=local',  35, 55,   FALSE, FALSE, TRUE,  FALSE, 48,  '192.168.10.34', 198, '\\fs01\home\v.zakharova',  '\\fs01\profiles\v.zakharova',  'Бухгалтер, на больничном'),
  (NULL,                      'svc-backup',   'Служба резервного копирования', NULL,            'service', 'CN=svc-backup,OU=Service Accounts,DC=company,DC=local', 180, NULL, TRUE,  FALSE, TRUE,  FALSE, 1, '192.168.20.11', 8760, NULL, NULL, 'Служебная учётная запись для бэкапов'),
  (NULL,                      'svc-sql',      'Служба SQL Server',  NULL,                      'service', 'CN=svc-sql,OU=Service Accounts,DC=company,DC=local',    180, NULL, TRUE,  FALSE, TRUE,  FALSE, 1, '192.168.20.10', 8760, NULL, NULL, 'Служебная учётная запись SQL Server'),
  (NULL,                      'guest',        'Гостевая учётная запись', NULL,                 'guest',   'CN=guest,OU=Users,DC=company,DC=local',               400, NULL, TRUE,  FALSE, FALSE, TRUE,  NULL, NULL, 0, NULL, NULL, 'Отключена, используется только для гостевого Wi-Fi')
) AS v(employee_email, username, display_name, email, account_type, distinguished_name,
       password_last_set_days, password_expires_days, password_never_expires,
       must_change_password, is_enabled, is_locked, last_logon_hours, last_logon_ip,
       logon_count, home_directory, profile_path, description)
LEFT JOIN employees e ON e.email = v.employee_email
ON CONFLICT (username) DO NOTHING;

INSERT INTO ad_group_memberships (employee_id, ad_group_id, added_at)
SELECT e.id, g.id, now() - interval '90 days'
FROM (VALUES
  ('IT-Administrators', 'a.smirnov@company.ru'),
  ('IT-Administrators', 'n.grigoriev@company.ru'),
  ('Finance',           'm.kuznetsova@company.ru'),
  ('Finance',           'd.orlov@company.ru'),
  ('Finance',           'e.volkova@company.ru'),
  ('Finance',           'v.zakharova@company.ru'),
  ('Sales',             'p.sokolov@company.ru'),
  ('Sales',             'a.lebedeva@company.ru'),
  ('Sales',             's.morozov@company.ru'),
  ('Sales',             'a.pavlov@company.ru'),
  ('HR',                'o.fedorova@company.ru'),
  ('VPN Users',         'a.smirnov@company.ru'),
  ('VPN Users',         'a.lebedeva@company.ru'),
  ('VPN Users',         's.morozov@company.ru'),
  ('VPN Users',         'i.novikov@company.ru')
) AS v(group_name, employee_email)
JOIN ad_groups g ON g.name = v.group_name
JOIN employees e ON e.email = v.employee_email
ON CONFLICT (employee_id, ad_group_id) DO NOTHING;

-- Все сотрудники входят в Domain Users и в список рассылки All Staff.
INSERT INTO ad_group_memberships (employee_id, ad_group_id, added_at)
SELECT e.id, g.id, now() - interval '90 days'
FROM employees e
CROSS JOIN ad_groups g
WHERE g.name IN ('Domain Users', 'All Staff')
ON CONFLICT (employee_id, ad_group_id) DO NOTHING;

UPDATE ad_groups g
SET member_count = (SELECT count(*) FROM ad_group_memberships m WHERE m.ad_group_id = g.id),
    last_synced  = now() - interval '15 minutes';

COMMIT;
