export interface Employee {
  id: number
  first_name: string
  last_name: string
  middle_name: string | null
  full_name: string
  position_id: number | null
  position_name: string | null
  department_id: number | null
  department_name: string | null
  email: string | null
  phone: string | null
  ad_username: string | null
  hire_date: string | null
  termination_date: string | null
  status: string
  notes: string | null
  created_at: string
  updated_at: string
}

export interface EmployeeListResponse {
  employees: Employee[]
  total: number
  page: number
  per_page: number
}

export interface Department {
  id: number
  name: string
  description: string | null
  employee_count: number
  created_at: string
  updated_at: string
}

export interface Position {
  id: number
  name: string
  department_id: number | null
  department_name: string | null
  created_at: string
  updated_at: string
}

export interface CreateEmployeeData {
  first_name: string
  last_name: string
  middle_name?: string
  position_id?: number
  department_id?: number
  email?: string
  phone?: string
  ad_username?: string
  hire_date?: string
  status: string
  notes?: string
}

export interface UpdateEmployeeData {
  first_name?: string
  last_name?: string
  middle_name?: string
  position_id?: number
  department_id?: number
  email?: string
  phone?: string
  ad_username?: string
  hire_date?: string
  termination_date?: string
  status?: string
  notes?: string
}

export interface BulkDeleteResponse {
  deleted_count: number
  failed_ids: number[]
}

export const useEmployees = () => {
  const { $api } = useNuxtApp()

  const fetchEmployees = async (params?: {
    page?: number
    per_page?: number
    status?: string
    department_id?: number
  }) => {
    return await $api<EmployeeListResponse>('/employees', {
      params
    })
  }

  const fetchEmployee = async (id: number) => {
    return await $api<Employee>(`/employees/${id}`)
  }

  const createEmployee = async (data: CreateEmployeeData) => {
    return await $api<Employee>('/employees', {
      method: 'POST',
      body: data
    })
  }

  const updateEmployee = async (id: number, data: UpdateEmployeeData) => {
    return await $api<Employee>(`/employees/${id}`, {
      method: 'PUT',
      body: data
    })
  }

  const deleteEmployee = async (id: number) => {
    return await $api(`/employees/${id}`, {
      method: 'DELETE'
    })
  }

  const bulkDeleteEmployees = async (ids: number[]) => {
    return await $api<BulkDeleteResponse>('/employees/bulk-delete', {
      method: 'POST',
      body: { ids }
    })
  }

  const fetchDepartments = async () => {
    return await $api<Department[]>('/employees/departments')
  }

  const fetchPositions = async () => {
    return await $api<Position[]>('/employees/positions')
  }

  const createDepartment = async (data: { name: string; description?: string }) => {
    return await $api<Department[]>('/employees/departments', {
      method: 'POST',
      body: data
    })
  }

  const createPosition = async (data: { name: string; department_id?: number }) => {
    return await $api<Position[]>('/employees/positions', {
      method: 'POST',
      body: data
    })
  }

  return {
    fetchEmployees,
    fetchEmployee,
    createEmployee,
    updateEmployee,
    deleteEmployee,
    bulkDeleteEmployees,
    fetchDepartments,
    fetchPositions,
    createDepartment,
    createPosition
  }
}
