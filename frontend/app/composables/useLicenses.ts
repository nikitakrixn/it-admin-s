export interface LicensePool {
  id: number
  software_catalog_id: number
  license_type: string
  total_licenses: number
  used_licenses: number | null
  available_licenses: number | null
  license_key: string | null
  purchase_date: string | null
  expiration_date: string | null
  vendor_id: number | null
  contract_id: number | null
  auto_renewal: boolean | null
  notes: string | null
  created_at: string
  updated_at: string
}

export interface NewLicensePoolRequest {
  software_catalog_id: number
  license_type?: string
  total_licenses?: number
  used_licenses?: number
  available_licenses?: number
  license_key?: string
  purchase_date?: string
  expiration_date?: string
  vendor_id?: number
  contract_id?: number
  auto_renewal?: boolean
  notes?: string
}

export interface LicensePoolListResponse {
  licenses: LicensePool[]
  total: number
  page: number
  per_page: number
}

export const useLicenses = () => {
  const { $api } = useNuxtApp()

  const fetchLicenses = async (params?: {
    page?: number
    per_page?: number
    license_type?: string
    software_catalog_id?: number
  }) => {
    return await $api<LicensePoolListResponse>('/licenses', { params })
  }

  const fetchLicense = async (id: number) => {
    return await $api<LicensePool>(`/licenses/${id}`)
  }

  const createLicense = async (data: NewLicensePoolRequest) => {
    return await $api<LicensePool>('/licenses', {
      method: 'POST',
      body: data,
    })
  }

  const updateLicense = async (id: number, data: Partial<NewLicensePoolRequest>) => {
    return await $api<LicensePool>(`/licenses/${id}`, {
      method: 'PUT',
      body: data,
    })
  }

  const deleteLicense = async (id: number) => {
    return await $api(`/licenses/${id}`, {
      method: 'DELETE',
    })
  }

  return {
    fetchLicenses,
    fetchLicense,
    createLicense,
    updateLicense,
    deleteLicense,
  }
}
