export interface Equipment {
  id: number
  inventory_number: string | null
  type_id: number | null
  name: string
  manufacturer: string | null
  model: string | null
  serial_number: string | null
  employee_id: number | null
  location: string | null
  ip_address: string | null
  mac_address: string | null
  purchase_date: string | null
  warranty_end_date: string | null
  status: string
  notes: string | null
  created_at: string
  updated_at: string
  location_id: number | null
  last_maintenance_date: string | null
  next_maintenance_date: string | null
  toner_level: number | null
  page_count: number | null
  paper_jam_count: number | null
  total_pages_printed: number | null
  color_pages_printed: number | null
  mono_pages_printed: number | null
  last_error_code: string | null
  last_error_time: string | null
  supplies_status: Record<string, unknown> | null
}

export interface NewEquipmentRequest {
  inventory_number?: string
  type_id?: number
  name?: string
  manufacturer?: string
  model?: string
  serial_number?: string
  employee_id?: number
  location?: string
  ip_address?: string
  mac_address?: string
  purchase_date?: string
  warranty_end_date?: string
  status?: string
  notes?: string
  location_id?: number
  last_maintenance_date?: string
  next_maintenance_date?: string
  toner_level?: number
  page_count?: number
  paper_jam_count?: number
  total_pages_printed?: number
  color_pages_printed?: number
  mono_pages_printed?: number
  last_error_code?: string
  last_error_time?: string
  supplies_status?: Record<string, unknown>
}

export interface EquipmentListResponse {
  equipment: Equipment[]
  total: number
  page: number
  per_page: number
}

export const useEquipment = () => {
  const { $api } = useNuxtApp()

  const fetchEquipmentList = async (params?: {
    page?: number
    per_page?: number
    status?: string
  }) => {
    return await $api<EquipmentListResponse>('/equipment', { params })
  }

  const fetchEquipment = async (id: number) => {
    return await $api<Equipment>(`/equipment/${id}`)
  }

  const createEquipment = async (data: NewEquipmentRequest) => {
    return await $api<Equipment>('/equipment', {
      method: 'POST',
      body: data,
    })
  }

  const updateEquipment = async (id: number, data: Partial<NewEquipmentRequest>) => {
    return await $api<Equipment>(`/equipment/${id}`, {
      method: 'PUT',
      body: data,
    })
  }

  const deleteEquipment = async (id: number) => {
    return await $api(`/equipment/${id}`, {
      method: 'DELETE',
    })
  }

  return {
    fetchEquipmentList,
    fetchEquipment,
    createEquipment,
    updateEquipment,
    deleteEquipment,
  }
}
