-- Демонстрационные расходные материалы и история их движения.

SET client_encoding = 'UTF8';

BEGIN;

UPDATE consumables
SET compatible_with = (SELECT id FROM hardware_types WHERE name = 'Лазерный принтер')
WHERE name = 'Картридж HP 85A'
  AND compatible_with IS NULL;

INSERT INTO consumables (
  name, manufacturer, model, category, compatible_with, quantity, min_quantity,
  unit, price_per_unit, notes
)
SELECT v.name,
       v.manufacturer,
       v.model,
       v.category,
       ht.id,
       v.quantity,
       v.min_quantity,
       v.unit,
       v.price_per_unit,
       v.notes
FROM (VALUES
  ('Картридж HP 85A',              'HP',       'CE285A',     'Картриджи',  'Лазерный принтер', 12, 2,  'шт',   2890.00, 'Оригинальный картридж для офисных принтеров'),
  ('Картридж Kyocera TK-3160',     'Kyocera',  'TK-3160',    'Картриджи',  'МФУ',              6,  2,  'шт',   4200.00, 'Для МФУ Kyocera ECOSYS M3145dn'),
  ('Тонер Xerox 106R03512',        'Xerox',    '106R03512',  'Тонеры',     'МФУ',              3,  1,  'шт',   3800.00, 'Чёрный тонер-картридж для Xerox C405'),
  ('Бумага А4, 500 листов',        'SvetoCopy', 'A4',        'Бумага',     NULL,               48, 10, 'уп.',  320.00,  'Офисная бумага 80 г/м²'),
  ('Мышь Logitech M90',            'Logitech', 'M90',        'Периферия',  NULL,               14, 5,  'шт',   690.00,  'Проводная оптическая мышь'),
  ('Клавиатура Logitech K120',     'Logitech', 'K120',       'Периферия',  NULL,               9,  5,  'шт',   850.00,  'Проводная клавиатура USB'),
  ('Патч-корд Cat6, 2 м',          'Hyperline', 'PC-6-2M',   'Кабели',     NULL,               35, 10, 'шт',   150.00,  'Сетевой кабель для коммутатора'),
  ('USB-флешка Kingston 32GB',     'Kingston', 'DT100G3/32', 'Накопители', NULL,               7,  3,  'шт',   550.00,  'Флешка для служебных задач'),
  ('Картридж Canon 725',           'Canon',    'Cartridge 725','Картриджи','Лазерный принтер', 2,  2,  'шт',   2900.00, 'Для Canon i-SENSYS LBP6030, остаток на минимуме')
) AS v(name, manufacturer, model, category, type_name, quantity, min_quantity,
       unit, price_per_unit, notes)
LEFT JOIN hardware_types ht ON ht.name = v.type_name
WHERE NOT EXISTS (SELECT 1 FROM consumables c WHERE c.name = v.name);

-- Начальный остаток: одно движение на позицию, если истории ещё нет.
INSERT INTO consumable_movements (
  consumable_id, quantity, movement_type, performed_by, notes, created_at
)
SELECT c.id,
       c.quantity,
       'purchase',
       '549b358e-b0bf-42a0-961b-4b58d956aa99'::uuid,
       'Начальный остаток (демо-данные)',
       now() - interval '30 days'
FROM consumables c
WHERE c.quantity > 0
  AND NOT EXISTS (
    SELECT 1 FROM consumable_movements m WHERE m.consumable_id = c.id
  );

-- Несколько операций для наглядной истории.
INSERT INTO consumable_movements (
  consumable_id, quantity, movement_type, performed_by, notes, created_at
)
SELECT c.id,
       v.quantity,
       v.movement_type,
       '549b358e-b0bf-42a0-961b-4b58d956aa99'::uuid,
       v.notes,
       now() - (v.days_ago || ' days')::interval
FROM (VALUES
  ('Картридж HP 85A',          -2, 'usage',    'Замена картриджа в принтере отдела продаж', 20),
  ('Картридж HP 85A',           3, 'purchase', 'Закупка картриджей у поставщика',           9),
  ('Бумага А4, 500 листов',    -5, 'usage',    'Выдано в бухгалтерию',                      15),
  ('Бумага А4, 500 листов',    10, 'purchase', 'Плановая закупка бумаги',                   5),
  ('Мышь Logitech M90',        -1, 'usage',    'Замена мыши на рабочем месте',              12),
  ('Патч-корд Cat6, 2 м',      -4, 'usage',    'Подключение новых рабочих мест',            8),
  ('Картридж Kyocera TK-3160', -1, 'usage',    'Замена картриджа в МФУ бухгалтерии',        18)
) AS v(name, quantity, movement_type, notes, days_ago)
JOIN consumables c ON c.name = v.name
WHERE (
  SELECT count(*) FROM consumable_movements m WHERE m.consumable_id = c.id
) = 1;

-- Приводим остаток к сумме движений (триггер уже применил каждое движение).
UPDATE consumables c
SET quantity = COALESCE(
  (SELECT sum(m.quantity) FROM consumable_movements m WHERE m.consumable_id = c.id),
  c.quantity
);

COMMIT;
