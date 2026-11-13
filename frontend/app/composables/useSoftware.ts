export interface Software {
  id: number
  name: string
  manufacturer: string | null
  category: string | null
  description: string | null
  website: string | null
  is_system: boolean
  requires_license: boolean
  created_at: string
  updated_at: string
}

export interface SoftwareListResponse {
  software: Software[]
  total: number
  page: number
  per_page: number
}

export interface SoftwareInstallation {
  id: number
  computer_id: number
  computer_name: string
  inventory_number: string | null
  employee_id: number | null
  employee_name: string | null
  version: string | null
  license_type: string | null
  license_end_date: string | null
  installation_date: string | null
  status: string | null
}

export interface SoftwareInstallationListResponse {
  software_id: number
  installations: SoftwareInstallation[]
  total: number
}

export interface CreateSoftwareData {
  name: string
  manufacturer?: string
  category?: string
  description?: string
  website?: string
  is_system?: boolean
  requires_license?: boolean
}

export interface UpdateSoftwareData {
  name?: string
  manufacturer?: string
  category?: string
  description?: string
  website?: string
  is_system?: boolean
  requires_license?: boolean
}

export const useSoftware = () => {
  const { $api } = useNuxtApp()

  const fetchSoftware = async (params?: {
    page?: number
    per_page?: number
    category?: string
    search?: string
  }) => {
    return await $api<SoftwareListResponse>('/software', {
      params,
    })
  }

  const fetchSoftwareById = async (id: number) => {
    return await $api<Software>(`/software/${id}`)
  }

  const fetchSoftwareInstallations = async (id: number) => {
    return await $api<SoftwareInstallationListResponse>(`/software/${id}/installations`)
  }

  const createSoftware = async (data: CreateSoftwareData) => {
    return await $api<Software>('/software', {
      method: 'POST',
      body: data,
    })
  }

  const updateSoftware = async (id: number, data: UpdateSoftwareData) => {
    return await $api<Software>(`/software/${id}`, {
      method: 'PUT',
      body: data,
    })
  }

  const deleteSoftware = async (id: number) => {
    return await $api(`/software/${id}`, {
      method: 'DELETE',
    })
  }

  return {
    fetchSoftware,
    fetchSoftwareById,
    fetchSoftwareInstallations,
    createSoftware,
    updateSoftware,
    deleteSoftware,
  }
}
