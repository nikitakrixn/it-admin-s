-- Revert to original status constraint (remove 'terminated')
ALTER TABLE employees DROP CONSTRAINT check_employee_status;
ALTER TABLE employees ADD CONSTRAINT check_employee_status 
    CHECK (status IN ('active', 'inactive', 'vacation', 'sick_leave'));
