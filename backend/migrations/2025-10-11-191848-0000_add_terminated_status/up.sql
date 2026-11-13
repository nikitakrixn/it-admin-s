-- Add 'terminated' status to employee status constraint
ALTER TABLE employees DROP CONSTRAINT check_employee_status;
ALTER TABLE employees ADD CONSTRAINT check_employee_status 
    CHECK (status IN ('active', 'inactive', 'vacation', 'sick_leave', 'terminated'));
