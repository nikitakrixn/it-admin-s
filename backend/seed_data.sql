-- Seed data for testing employees module

-- Insert departments
INSERT INTO departments (name, description, created_at, updated_at) VALUES
('IT отдел', 'Отдел информационных технологий', NOW(), NOW()),
('Бухгалтерия', 'Финансовый отдел', NOW(), NOW()),
('HR отдел', 'Отдел кадров', NOW(), NOW()),
('Отдел продаж', 'Коммерческий отдел', NOW(), NOW()),
('Производство', 'Производственный отдел', NOW(), NOW())
ON CONFLICT DO NOTHING;

-- Insert positions
INSERT INTO positions (name, department_id, created_at, updated_at) VALUES
('Системный администратор', (SELECT id FROM departments WHERE name = 'IT отдел' LIMIT 1), NOW(), NOW()),
('Программист', (SELECT id FROM departments WHERE name = 'IT отдел' LIMIT 1), NOW(), NOW()),
('Главный бухгалтер', (SELECT id FROM departments WHERE name = 'Бухгалтерия' LIMIT 1), NOW(), NOW()),
('Бухгалтер', (SELECT id FROM departments WHERE name = 'Бухгалтерия' LIMIT 1), NOW(), NOW()),
('HR менеджер', (SELECT id FROM departments WHERE name = 'HR отдел' LIMIT 1), NOW(), NOW()),
('Менеджер по продажам', (SELECT id FROM departments WHERE name = 'Отдел продаж' LIMIT 1), NOW(), NOW()),
('Инженер', (SELECT id FROM departments WHERE name = 'Производство' LIMIT 1), NOW(), NOW())
ON CONFLICT DO NOTHING;

-- Insert sample employees
INSERT INTO employees (first_name, last_name, middle_name, position_id, department_id, email, phone, status, hire_date, created_at, updated_at) VALUES
('Иван', 'Иванов', 'Иванович', 
    (SELECT id FROM positions WHERE name = 'Системный администратор' LIMIT 1),
    (SELECT id FROM departments WHERE name = 'IT отдел' LIMIT 1),
    'ivanov@example.com', '+7 (999) 123-45-67', 'active', '2023-01-15', NOW(), NOW()),
    
('Петр', 'Петров', 'Петрович',
    (SELECT id FROM positions WHERE name = 'Программист' LIMIT 1),
    (SELECT id FROM departments WHERE name = 'IT отдел' LIMIT 1),
    'petrov@example.com', '+7 (999) 234-56-78', 'active', '2023-03-20', NOW(), NOW()),
    
('Мария', 'Сидорова', 'Александровна',
    (SELECT id FROM positions WHERE name = 'Главный бухгалтер' LIMIT 1),
    (SELECT id FROM departments WHERE name = 'Бухгалтерия' LIMIT 1),
    'sidorova@example.com', '+7 (999) 345-67-89', 'active', '2022-06-10', NOW(), NOW()),
    
('Анна', 'Козлова', 'Сергеевна',
    (SELECT id FROM positions WHERE name = 'HR менеджер' LIMIT 1),
    (SELECT id FROM departments WHERE name = 'HR отдел' LIMIT 1),
    'kozlova@example.com', '+7 (999) 456-78-90', 'active', '2023-02-01', NOW(), NOW()),
    
('Дмитрий', 'Смирнов', 'Викторович',
    (SELECT id FROM positions WHERE name = 'Менеджер по продажам' LIMIT 1),
    (SELECT id FROM departments WHERE name = 'Отдел продаж' LIMIT 1),
    'smirnov@example.com', '+7 (999) 567-89-01', 'active', '2023-04-15', NOW(), NOW())
ON CONFLICT DO NOTHING;
