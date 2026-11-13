-- Демонстрационные заявки и комментарии к ним.

SET client_encoding = 'UTF8';

BEGIN;

-- Убираем тестовую заявку, созданную вручную.
DELETE FROM request_comments
WHERE request_id IN (SELECT id FROM requests WHERE title = 'ссс');

DELETE FROM activity_log
WHERE entity_type = 'request'
  AND entity_id IN (SELECT id FROM requests WHERE title = 'ссс');

DELETE FROM requests WHERE title = 'ссс';

-- Демонстрационные заявки пересоздаются, чтобы данные оставались предсказуемыми.
DELETE FROM request_comments
WHERE request_id IN (
  SELECT id FROM requests WHERE title IN (
    'Не включается компьютер в бухгалтерии',
    'Нужен доступ к папке «Договоры»',
    'Не печатает МФУ Kyocera',
    'Установить Microsoft 365 на новый ноутбук',
    'Медленно работает интернет в отделе продаж',
    'Восстановить пароль от учётной записи',
    'Настроить корпоративную почту на телефоне',
    'Заменить картридж в принтере HP',
    'Синий экран при загрузке ноутбука',
    'Предоставить доступ к 1С',
    'Обновить антивирус на сервере',
    'Не работает IP-телефон в переговорной',
    'Подготовить ноутбук для нового сотрудника',
    'Настроить VPN для удалённой работы',
    'Заявка на закупку расходных материалов'
  )
);

DELETE FROM activity_log
WHERE entity_type = 'request'
  AND entity_id IN (
    SELECT id FROM requests WHERE title IN (
      'Не включается компьютер в бухгалтерии',
      'Нужен доступ к папке «Договоры»',
      'Не печатает МФУ Kyocera',
      'Установить Microsoft 365 на новый ноутбук',
      'Медленно работает интернет в отделе продаж',
      'Восстановить пароль от учётной записи',
      'Настроить корпоративную почту на телефоне',
      'Заменить картридж в принтере HP',
      'Синий экран при загрузке ноутбука',
      'Предоставить доступ к 1С',
      'Обновить антивирус на сервере',
      'Не работает IP-телефон в переговорной',
      'Подготовить ноутбук для нового сотрудника',
      'Настроить VPN для удалённой работы',
      'Заявка на закупку расходных материалов'
    )
  );

DELETE FROM requests
WHERE title IN (
  'Не включается компьютер в бухгалтерии',
  'Нужен доступ к папке «Договоры»',
  'Не печатает МФУ Kyocera',
  'Установить Microsoft 365 на новый ноутбук',
  'Медленно работает интернет в отделе продаж',
  'Восстановить пароль от учётной записи',
  'Настроить корпоративную почту на телефоне',
  'Заменить картридж в принтере HP',
  'Синий экран при загрузке ноутбука',
  'Предоставить доступ к 1С',
  'Обновить антивирус на сервере',
  'Не работает IP-телефон в переговорной',
  'Подготовить ноутбук для нового сотрудника',
  'Настроить VPN для удалённой работы',
  'Заявка на закупку расходных материалов'
);

INSERT INTO requests (
  title, description, requester_id, assignee_id, computer_id, equipment_id,
  priority, status, category, created_at, updated_at, resolved_at, closed_at,
  first_response_at, sla_deadline, sla_breached, satisfaction_rating,
  satisfaction_comment
)
SELECT v.title,
       v.description,
       requester.id,
       assignee.id,
       c.id,
       eq.id,
       v.priority,
       v.status,
       v.category,
       now() - (v.created_ago || ' hours')::interval,
       now() - (v.updated_ago || ' hours')::interval,
       CASE WHEN v.resolved_ago IS NULL THEN NULL ELSE now() - (v.resolved_ago || ' hours')::interval END,
       CASE WHEN v.closed_ago IS NULL THEN NULL ELSE now() - (v.closed_ago || ' hours')::interval END,
       CASE WHEN v.first_response_ago IS NULL THEN NULL ELSE now() - (v.first_response_ago || ' hours')::interval END,
       now() - (v.created_ago || ' hours')::interval + (v.sla_hours || ' hours')::interval,
       v.sla_breached,
       v.satisfaction_rating,
       v.satisfaction_comment
FROM (VALUES
  ('Не включается компьютер в бухгалтерии', 'При включении горит оранжевый индикатор, изображения нет. Работа приостановлена.', 'd.orlov@company.ru', 'a.smirnov@company.ru', 'PC-ACC-02', NULL, 'high', 'in_progress', 'hardware', 6, 1, NULL, NULL, 5, 8, FALSE, NULL, NULL),
  ('Нужен доступ к папке «Договоры»', 'Прошу предоставить доступ на чтение к сетевой папке с договорами.', 'e.volkova@company.ru', 'a.smirnov@company.ru', NULL, NULL, 'medium', 'new', 'access', 2, 2, NULL, NULL, NULL, 24, FALSE, NULL, NULL),
  ('Не печатает МФУ Kyocera', 'Документы уходят в очередь, но не печатаются. Ошибка на панели МФУ.', 'm.kuznetsova@company.ru', 'n.grigoriev@company.ru', NULL, 'EQ-0001', 'high', 'resolved', 'hardware', 30, 4, 5, NULL, 1, 8, FALSE, 5, 'Быстро помогли, спасибо!'),
  ('Установить Microsoft 365 на новый ноутбук', 'Новый ноутбук, нужен офисный пакет и настройка почты.', 'a.lebedeva@company.ru', 'n.grigoriev@company.ru', 'PC-SALES-02', NULL, 'medium', 'in_progress', 'software', 20, 3, NULL, NULL, 2, 16, FALSE, NULL, NULL),
  ('Медленно работает интернет в отделе продаж', 'Страницы грузятся по несколько секунд, видеозвонки прерываются.', 'p.sokolov@company.ru', 'a.smirnov@company.ru', NULL, NULL, 'high', 'waiting', 'network', 48, 6, NULL, NULL, 3, 8, TRUE, NULL, NULL),
  ('Восстановить пароль от учётной записи', 'Забыл пароль, не могу войти в систему и почту.', 's.morozov@company.ru', 'a.smirnov@company.ru', NULL, NULL, 'critical', 'closed', 'access', 72, 68, 70, 68, 1, 4, FALSE, 5, 'Пароль восстановили за 10 минут.'),
  ('Настроить корпоративную почту на телефоне', 'Нужно подключить рабочую почту на iPhone.', 'i.novikov@company.ru', 'n.grigoriev@company.ru', NULL, NULL, 'low', 'resolved', 'consultation', 96, 80, 82, NULL, 12, 48, FALSE, 5, NULL),
  ('Заменить картридж в принтере HP', 'Принтер в отделе продаж печатает с полосами, тонер заканчивается.', 'o.fedorova@company.ru', NULL, NULL, 'EQ-0002', 'medium', 'new', 'hardware', 4, 4, NULL, NULL, NULL, 24, FALSE, NULL, NULL),
  ('Синий экран при загрузке ноутбука', 'После обновления Windows ноутбук падает в BSOD при входе в систему.', 'e.volkova@company.ru', 'a.smirnov@company.ru', 'PC-ACC-03', NULL, 'critical', 'in_progress', 'hardware', 10, 2, NULL, NULL, 2, 4, TRUE, NULL, NULL),
  ('Предоставить доступ к 1С', 'Нужен доступ к бухгалтерской базе 1С для нового сотрудника.', 'v.zakharova@company.ru', 'a.smirnov@company.ru', NULL, NULL, 'high', 'waiting', 'access', 26, 8, NULL, NULL, 4, 8, TRUE, NULL, NULL),
  ('Обновить антивирус на сервере', 'На SRV-01 устарели базы Kaspersky, требуется обновление и проверка.', 'i.novikov@company.ru', 'n.grigoriev@company.ru', 'SRV-01', NULL, 'medium', 'closed', 'software', 120, 110, 112, 110, 5, 24, FALSE, 5, NULL),
  ('Не работает IP-телефон в переговорной', 'На телефоне нет гудка, индикатор питания не горит.', 'o.fedorova@company.ru', 'n.grigoriev@company.ru', NULL, 'EQ-0009', 'medium', 'resolved', 'network', 54, 50, 51, NULL, 2, 8, FALSE, 4, NULL),
  ('Подготовить ноутбук для нового сотрудника', 'Нужен ноутбук с корпоративным ПО к выходу нового менеджера.', 'o.fedorova@company.ru', 'a.smirnov@company.ru', NULL, NULL, 'medium', 'in_progress', 'hardware', 36, 12, NULL, NULL, 6, 24, FALSE, NULL, NULL),
  ('Настроить VPN для удалённой работы', 'Нужен доступ к внутренним ресурсам из дома.', 'a.pavlov@company.ru', 'a.smirnov@company.ru', NULL, NULL, 'high', 'new', 'network', 3, 3, NULL, NULL, NULL, 8, FALSE, NULL, NULL),
  ('Заявка на закупку расходных материалов', 'Закончилась бумага А4 и картриджи для офисных принтеров.', 'm.kuznetsova@company.ru', NULL, NULL, NULL, 'low', 'cancelled', 'other', 200, 190, NULL, 190, 24, 48, FALSE, NULL, 'Закупка перенесена на следующий квартал')
) AS v(title, description, requester_email, assignee_email, computer_hostname,
       equipment_inventory, priority, status, category, created_ago, updated_ago,
       resolved_ago, closed_ago, first_response_ago, sla_hours, sla_breached,
       satisfaction_rating, satisfaction_comment)
JOIN employees requester ON requester.email = v.requester_email
LEFT JOIN employees assignee ON assignee.email = v.assignee_email
LEFT JOIN computers c ON c.hostname = v.computer_hostname
LEFT JOIN equipment eq ON eq.inventory_number = v.equipment_inventory;

INSERT INTO request_comments (request_id, user_id, comment, is_internal, created_at)
SELECT r.id,
       '549b358e-b0bf-42a0-961b-4b58d956aa99'::uuid,
       v.comment,
       v.is_internal,
       now() - (v.hours_ago || ' hours')::interval
FROM (VALUES
  ('Не включается компьютер в бухгалтерии', 'Проверил блок питания — требуется замена. Заказал новый.', TRUE, 4),
  ('Не включается компьютер в бухгалтерии', 'Замена согласована, деталь привезут завтра.', FALSE, 1),
  ('Не печатает МФУ Kyocera', 'Перезагрузил МФУ и очистил очередь печати — печать восстановлена.', FALSE, 5),
  ('Медленно работает интернет в отделе продаж', 'Проверяю коммутатор в отделе продаж, возможен сбой порта.', TRUE, 6),
  ('Синий экран при загрузке ноутбука', 'Извлёк дамп памяти, диагностирую драйвер видеокарты.', TRUE, 2),
  ('Восстановить пароль от учётной записи', 'Пароль сброшен, данные для входа отправлены на личную почту.', FALSE, 70)
) AS v(request_title, comment, is_internal, hours_ago)
JOIN requests r ON r.title = v.request_title
WHERE NOT EXISTS (
  SELECT 1 FROM request_comments c WHERE c.request_id = r.id
);

COMMIT;
