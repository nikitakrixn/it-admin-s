export interface ComponentDetail {
  label: string
  value: string
}

export interface ComputerComponent {
  id: number
  component_type: string
  name: string
  manufacturer: string | null
  model: string | null
  is_primary: boolean
  details: ComponentDetail[]
  notes: string | null
  created_at: string
  updated_at: string
}

export interface ComputerComponentListResponse {
  computer_id: number
  components: ComputerComponent[]
  total: number
}

export interface ComponentRequest {
  component_type: string
  name: string
  manufacturer?: string
  model?: string
  serial_number?: string
  notes?: string
  is_primary?: boolean

  cores?: number
  threads?: number
  base_frequency?: string
  max_frequency?: string
  cache_size?: string
  socket?: string
  tdp?: string

  capacity_gb?: number
  memory_type?: string
  frequency?: string
  slot_number?: number

  storage_type?: string
  interface?: string
  is_system_drive?: boolean
  mount_point?: string

  memory_gb?: number
  gpu_memory_type?: string
  driver_version?: string

  chipset?: string
  bios_version?: string
  form_factor?: string

  mac_address?: string
  ip_address?: string
  adapter_type?: string
  speed?: string
  network_status?: string

  size_inches?: number
  resolution?: string
  panel_type?: string
  refresh_rate?: number

  peripheral_type?: string
  connection_type?: string
  peripheral_status?: string
}

export interface InstalledSoftware {
  id: number
  software_id: number
  name: string
  manufacturer: string | null
  category: string | null
  version: string | null
  license_type: string | null
  license_end_date: string | null
  installation_date: string | null
  last_seen_date: string | null
  status: string | null
  notes: string | null
}

export interface ComputerSoftwareListResponse {
  computer_id: number
  software: InstalledSoftware[]
  total: number
}

export interface NewInstalledSoftwareRequest {
  software_catalog_id: number
  version?: string
  license_type?: string
  license_key?: string
  license_end_date?: string
  installation_date?: string
  status?: string
  notes?: string
}

export const useComputerComponents = () => {
  const { $api } = useNuxtApp()

  const fetchComponents = async (computerId: number) => {
    return await $api<ComputerComponentListResponse>(`/computers/${computerId}/components`)
  }

  const fetchComponent = async (computerId: number, componentType: string, componentId: number) => {
    return await $api<ComputerComponent>(
      `/computers/${computerId}/components/${componentType}/${componentId}`,
    )
  }

  const createComponent = async (computerId: number, data: ComponentRequest) => {
    return await $api<ComputerComponent>(`/computers/${computerId}/components`, {
      method: 'POST',
      body: data,
    })
  }

  const updateComponent = async (
    computerId: number,
    componentType: string,
    componentId: number,
    data: ComponentRequest,
  ) => {
    return await $api<ComputerComponent>(
      `/computers/${computerId}/components/${componentType}/${componentId}`,
      {
        method: 'PUT',
        body: data,
      },
    )
  }

  const deleteComponent = async (
    computerId: number,
    componentType: string,
    componentId: number,
  ) => {
    return await $api(`/computers/${computerId}/components/${componentType}/${componentId}`, {
      method: 'DELETE',
    })
  }

  const fetchInstalledSoftware = async (computerId: number) => {
    return await $api<ComputerSoftwareListResponse>(`/computers/${computerId}/software`)
  }

  const installSoftware = async (computerId: number, data: NewInstalledSoftwareRequest) => {
    return await $api<InstalledSoftware>(`/computers/${computerId}/software`, {
      method: 'POST',
      body: data,
    })
  }

  const uninstallSoftware = async (computerId: number, softwareId: number) => {
    return await $api(`/computers/${computerId}/software/${softwareId}`, {
      method: 'DELETE',
    })
  }

  return {
    fetchComponents,
    fetchComponent,
    createComponent,
    updateComponent,
    deleteComponent,
    fetchInstalledSoftware,
    installSoftware,
    uninstallSoftware,
  }
}
