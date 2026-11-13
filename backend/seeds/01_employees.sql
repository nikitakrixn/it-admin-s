-- Демонстрационные сотрудники.
-- Скрипт идемпотентный: повторный запуск не создаёт дубликатов.

SET client_encoding = 'UTF8';

BEGIN;

-- Тестовая запись с пустыми полями превращается в системного администратора.
UPDATE employees
SET first_name        = 'Алексей',
    last_name         = 'Смирнов',
    middle_name       = 'Викторович',
    position_id       = (SELECT id FROM positions WHERE name = 'Системный администратор'),
    department_id     = (SELECT id FROM departments WHERE name = 'IT отдел'),
    email             = 'a.smirnov@company.ru',
    phone             = '+7 (495) 120-45-01',
    ad_username       = 'a.smirnov',
    hire_date         = DATE '2019-03-04',
    status            = 'active',
    notes             = 'Отвечает за инфраструктуру и поддержку пользователей',
    location_id       = (SELECT id FROM locations WHERE name = 'IT отдел'),
    telegram_username = 'a_smirnov',
    is_vip            = FALSE,
    updated_at        = now()
WHERE id = 1
  AND (email IS NULL OR email = '');

INSERT INTO employees (
  first_name, last_name, middle_name, position_id, department_id, email, phone,
  ad_username, hire_date, status, notes, location_id, telegram_username, is_vip
)
SELECT v.first_name,
       v.last_name,
       v.middle_name,
       p.id,
       d.id,
       v.email,
       v.phone,
       v.ad_username,
       v.hire_date::date,
       v.status,
       v.notes,
       l.id,
       v.telegram_username,
       v.is_vip
FROM (VALUES
  ('Алексей',  'Смирнов',   'Викторович',   'Системный администратор', 'IT отдел',     'a.smirnov@company.ru',     '+7 (495) 120-45-01', 'a.smirnov',     '2019-03-04', 'active',     'Отвечает за инфраструктуру и поддержку пользователей', 'IT отдел',      'a_smirnov',   FALSE),
  ('Мария',    'Кузнецова', 'Сергеевна',    'Главный бухгалтер',       'Бухгалтерия',  'm.kuznetsova@company.ru',  '+7 (495) 120-45-10', 'm.kuznetsova',  '2017-08-21', 'active',     'Руководитель бухгалтерской службы',                    'Бухгалтерия',   NULL,          FALSE),
  ('Дмитрий',  'Орлов',     'Андреевич',    'Бухгалтер',               'Бухгалтерия',  'd.orlov@company.ru',       '+7 (495) 120-45-11', 'd.orlov',       '2021-02-15', 'active',     NULL,                                                   'Бухгалтерия',   NULL,          FALSE),
  ('Екатерина','Волкова',   'Игоревна',     'Бухгалтер',               'Бухгалтерия',  'e.volkova@company.ru',     '+7 (495) 120-45-12', 'e.volkova',     '2022-09-05', 'active',     NULL,                                                   'Бухгалтерия',   NULL,          FALSE),
  ('Павел',    'Соколов',   'Николаевич',   'Менеджер по продажам',    'Отдел продаж', 'p.sokolov@company.ru',     '+7 (495) 120-45-20', 'p.sokolov',     '2020-06-01', 'active',     'Ключевые клиенты, направление B2B',                    'Отдел продаж',  NULL,          FALSE),
  ('Анна',     'Лебедева',  'Дмитриевна',   'Менеджер по продажам',    'Отдел продаж', 'a.lebedeva@company.ru',    '+7 (495) 120-45-21', 'a.lebedeva',    '2023-01-16', 'active',     NULL,                                                   'Отдел продаж',  NULL,          FALSE),
  ('Сергей',   'Морозов',   'Владимирович', 'Менеджер по продажам',    'Отдел продаж', 's.morozov@company.ru',     '+7 (495) 120-45-22', 's.morozov',     '2024-04-08', 'active',     NULL,                                                   'Отдел продаж',  NULL,          FALSE),
  ('Игорь',    'Новиков',   'Петрович',     'Директор',                'Администрация','i.novikov@company.ru',     '+7 (495) 120-45-00', 'i.novikov',     '2015-11-02', 'active',     'Генеральный директор',                                 'Главный офис',  NULL,          TRUE),
  ('Ольга',    'Фёдорова',  'Александровна','HR менеджер',             'HR отдел',     'o.fedorova@company.ru',    '+7 (495) 120-45-30', 'o.fedorova',    '2018-05-14', 'active',     NULL,                                                   'Главный офис',  NULL,          FALSE),
  ('Никита',   'Григорьев', 'Олегович',     'Техник IT',               'IT отдел',     'n.grigoriev@company.ru',   '+7 (495) 120-45-02', 'n.grigoriev',   '2023-10-02', 'active',     'Поддержка пользователей и обслуживание техники',       'IT отдел',      NULL,          FALSE),
  ('Артём',    'Павлов',    'Ильич',        'Менеджер по продажам',    'Отдел продаж', 'a.pavlov@company.ru',      '+7 (495) 120-45-23', 'a.pavlov',      '2025-02-10', 'vacation',   'В отпуске до конца месяца',                            'Отдел продаж',  NULL,          FALSE),
  ('Виктория', 'Захарова',  'Максимовна',   'Бухгалтер',               'Бухгалтерия',  'v.zakharova@company.ru',   '+7 (495) 120-45-13', 'v.zakharova',   '2021-11-29', 'sick_leave', 'Больничный лист',                                      'Бухгалтерия',   NULL,          FALSE)
) AS v(first_name, last_name, middle_name, position_name, department_name, email, phone,
       ad_username, hire_date, status, notes, location_name, telegram_username, is_vip)
JOIN positions p ON p.name = v.position_name
JOIN departments d ON d.name = v.department_name
JOIN locations l ON l.name = v.location_name
ON CONFLICT (email) DO NOTHING;

COMMIT;
