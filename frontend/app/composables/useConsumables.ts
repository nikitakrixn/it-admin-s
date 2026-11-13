export type ConsumableMovementType = 'purchase' | 'usage' | 'return' | 'write_off' | 'transfer'

export interface Consumable {
  id: number
  name: string
  manufacturer: string | null
  model: string | null
  category: string | null
  compatible_with: number | null
  compatible_with_name: string | null
  quantity: number
  min_quantity: number
  unit: string | null
  notes: string | null
  created_at: string
  updated_at: string
}

export interface ConsumableListResponse {
  consumables: Consumable[]
  total: number
  page: number
  per_page: number
}

export interface ConsumableMovement {
  id: number
  consumable_id: number
  equipment_id: number | null
  computer_id: number | null
  quantity: number
  movement_type: ConsumableMovementType
  employee_id: number | null
  performed_by: string | null
  performed_by_email: string | null
  notes: string | null
  created_at: string
}

export interface ConsumableMovementListResponse {
  movements: ConsumableMovement[]
  total: number
  total_in: number
  total_out: number
}

export interface NewConsumableRequest {
  name: string
  manufacturer?: string
  model?: string
  category?: string
  compatible_with?: number
  quantity?: number
  min_quantity?: number
  unit?: string
  notes?: string
}

export interface NewConsumableMovementRequest {
  quantity: number
  movement_type: ConsumableMovementType
  employee_id?: number
  equipment_id?: number
  computer_id?: number
  notes?: string
}

export interface ConsumableMovementCreatedResponse {
  movement: ConsumableMovement
  consumable: Consumable
}

export const useConsumables = () => {
  const { $api } = useNuxtApp()

  const fetchConsumables = async (params?: {
    page?: number
    per_page?: number
    category?: string
  }) => {
    return await $api<ConsumableListResponse>('/consumables', { params })
  }

  const fetchConsumable = async (id: number) => {
    return await $api<Consumable>(`/consumables/${id}`)
  }

  const createConsumable = async (data: NewConsumableRequest) => {
    return await $api<Consumable>('/consumables', {
      method: 'POST',
      body: data,
    })
  }

  const updateConsumable = async (id: number, data: Partial<NewConsumableRequest>) => {
    return await $api<Consumable>(`/consumables/${id}`, {
      method: 'PUT',
      body: data,
    })
  }

  const deleteConsumable = async (id: number) => {
    return await $api(`/consumables/${id}`, {
      method: 'DELETE',
    })
  }

  const fetchMovements = async (id: number, params?: { page?: number; per_page?: number }) => {
    return await $api<ConsumableMovementListResponse>(`/consumables/${id}/movements`, { params })
  }

  const createMovement = async (id: number, data: NewConsumableMovementRequest) => {
    return await $api<ConsumableMovementCreatedResponse>(`/consumables/${id}/movements`, {
      method: 'POST',
      body: data,
    })
  }

  return {
    fetchConsumables,
    fetchConsumable,
    createConsumable,
    updateConsumable,
    deleteConsumable,
    fetchMovements,
    createMovement,
  }
}
