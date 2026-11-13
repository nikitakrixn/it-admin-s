-- ============================================================================
-- Представления для отделов и должностей
-- ============================================================================

-- Отделы с подсчетами
CREATE OR REPLACE VIEW v_departments_with_counts AS
SELECT
    d.id,
    d.name,
    d.description,
    d.created_at,
    d.updated_at,
    COUNT(DISTINCT CASE WHEN e.status = 'active' THEN e.id END) as employee_count,
    COUNT(DISTINCT p.id) as position_count,
    COUNT(DISTINCT c.id) as computer_count
FROM departments d
LEFT JOIN employees e ON d.id = e.department_id
LEFT JOIN positions p ON d.id = p.department_id
LEFT JOIN computers c ON e.id = c.employee_id AND c.status = 'active'
GROUP BY d.id, d.name, d.description, d.created_at, d.updated_at;

-- Должности с подсчетами и названием отдела
CREATE OR REPLACE VIEW v_positions_with_details AS
SELECT
    p.id,
    p.name,
    p.department_id,
    d.name as department_name,
    p.created_at,
    p.updated_at,
    COUNT(CASE WHEN e.status = 'active' THEN e.id END) as employee_count
FROM positions p
LEFT JOIN departments d ON p.department_id = d.id
LEFT JOIN employees e ON p.id = e.position_id
GROUP BY p.id, p.name, p.department_id, d.name, p.created_at, p.updated_at;

COMMENT ON VIEW v_departments_with_counts IS 'Отделы с количеством сотрудников, должностей и компьютеров';
COMMENT ON VIEW v_positions_with_details IS 'Должности с названием отдела и количеством сотрудников';
