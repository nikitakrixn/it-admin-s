export interface Computer {
  id: number
  inventory_number: string | null
  hostname: string | null
  computer_type: string
  manufacturer: string | null
  model: string | null
  serial_number: string | null
  os: string | null
  os_version: string | null
  os_build: string | null
  os_architecture: string | null
  domain_joined: boolean | null
  last_boot_time: string | null
  employee_id: number | null
  location: string | null
  purchase_date: string | null
  warranty_end_date: string | null
  status: string
  notes: string | null
  created_at: string
  updated_at: string
  last_seen_online: string | null
  is_online: boolean | null
  last_user_logged: string | null
  bitlocker_enabled: boolean | null
  antivirus_status: string | null
  windows_activation_status: string | null
  location_id: number | null
}

export interface NewComputerRequest {
  inventory_number?: string
  hostname?: string
  computer_type?: string
  manufacturer?: string
  model?: string
  serial_number?: string
  os?: string
  os_version?: string
  os_build?: string
  os_architecture?: string
  domain_joined?: boolean
  last_boot_time?: string
  employee_id?: number
  location?: string
  purchase_date?: string
  warranty_end_date?: string
  status?: string
  notes?: string
  last_seen_online?: string
  is_online?: boolean
  last_user_logged?: string
  bitlocker_enabled?: boolean
  antivirus_status?: string
  windows_activation_status?: string
  location_id?: number
}

export interface ComputerListResponse {
  computers: Computer[]
  total: number
  page: number
  per_page: number
}

export const useComputers = () => {
  const { $api } = useNuxtApp()

  const fetchComputers = async (params?: {
    page?: number
    per_page?: number
    status?: string
    computer_type?: string
  }) => {
    return await $api<ComputerListResponse>('/computers', { params })
  }

  const fetchComputer = async (id: number) => {
    return await $api<Computer>(`/computers/${id}`)
  }

  const createComputer = async (data: NewComputerRequest) => {
    return await $api<Computer>('/computers', {
      method: 'POST',
      body: data,
    })
  }

  const updateComputer = async (id: number, data: Partial<NewComputerRequest>) => {
    return await $api<Computer>(`/computers/${id}`, {
      method: 'PUT',
      body: data,
    })
  }

  const deleteComputer = async (id: number) => {
    return await $api(`/computers/${id}`, {
      method: 'DELETE',
    })
  }

  return {
    fetchComputers,
    fetchComputer,
    createComputer,
    updateComputer,
    deleteComputer,
  }
}
