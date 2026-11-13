export interface NetworkSegment {
  id: number
  name: string
  network_address: string
  subnet_mask: string
  gateway: string | null
  vlan_id: number | null
  dns_servers: string | null
  dhcp_enabled: boolean | null
  dhcp_range_start: string | null
  dhcp_range_end: string | null
  description: string | null
  location: string | null
  created_at: string
  updated_at: string
}

export interface NewNetworkSegmentRequest {
  name: string
  network_address: string
  subnet_mask: string
  gateway?: string
  vlan_id?: number
  dns_servers?: string
  dhcp_enabled?: boolean
  dhcp_range_start?: string
  dhcp_range_end?: string
  description?: string
  location?: string
}

export interface NetworkSegmentListResponse {
  segments: NetworkSegment[]
  total: number
  page: number
  per_page: number
}

export interface NetworkDevice {
  id: number
  device_type: string
  name: string
  manufacturer: string | null
  model: string | null
  serial_number: string | null
  ip_address: string | null
  mac_address: string | null
  management_url: string | null
  location: string | null
  firmware_version: string | null
  port_count: number | null
  status: string | null
  notes: string | null
  created_at: string
  updated_at: string
}

export interface NewNetworkDeviceRequest {
  device_type: string
  name: string
  manufacturer?: string
  model?: string
  serial_number?: string
  ip_address?: string
  mac_address?: string
  management_url?: string
  location?: string
  firmware_version?: string
  port_count?: number
  status?: string
  notes?: string
}

export interface NetworkDeviceListResponse {
  devices: NetworkDevice[]
  total: number
  page: number
  per_page: number
}

export interface IpAddress {
  id: number
  network_segment_id: number | null
  ip_address: string
  status: string | null
  assigned_to_type: string | null
  assigned_to_id: number | null
  hostname: string | null
  mac_address: string | null
  is_reserved: boolean | null
  reservation_reason: string | null
  last_seen: string | null
  notes: string | null
  created_at: string
  updated_at: string
}

export interface NewIpAddressRequest {
  network_segment_id?: number
  ip_address: string
  status?: string
  assigned_to_type?: string
  assigned_to_id?: number
  hostname?: string
  mac_address?: string
  is_reserved?: boolean
  reservation_reason?: string
  last_seen?: string
  notes?: string
}

export interface IpAddressListResponse {
  addresses: IpAddress[]
  total: number
  page: number
  per_page: number
}

export interface NetworkScan {
  id: number
  scan_type: string
  network_segment_id: number | null
  ip_range: string | null
  devices_found: number | null
  scan_duration: number | null
  scan_results: Record<string, unknown> | null
  initiated_by: string | null
  started_at: string
  completed_at: string | null
  status: string | null
  error_message: string | null
  created_at: string
}

export interface NewNetworkScanRequest {
  scan_type: string
  network_segment_id?: number
  ip_range?: string
  devices_found?: number
  scan_duration?: number
  scan_results?: Record<string, unknown>
  initiated_by?: string
  started_at?: string
  completed_at?: string
  status?: string
  error_message?: string
}

export interface NetworkScanListResponse {
  scans: NetworkScan[]
  total: number
  page: number
  per_page: number
}

export interface RunNetworkScanRequest {
  start_ip: string
  end_ip: string
  ports?: number[]
  scan_type?: string
}

export interface ScannedHost {
  ip: string
  open_ports: number[]
}

export const useNetwork = () => {
  const { $api } = useNuxtApp()

  // ============================================================================
  // Network segments
  // ============================================================================
  const fetchNetworkSegments = async (params?: { page?: number; per_page?: number }) => {
    return await $api<NetworkSegmentListResponse>('/network/segments', {
      params,
    })
  }

  const getNetworkSegment = async (id: number) => {
    return await $api<NetworkSegment>(`/network/segments/${id}`)
  }

  const createNetworkSegment = async (data: NewNetworkSegmentRequest) => {
    return await $api<NetworkSegment>('/network/segments', {
      method: 'POST',
      body: data,
    })
  }

  const updateNetworkSegment = async (id: number, data: NewNetworkSegmentRequest) => {
    return await $api<NetworkSegment>(`/network/segments/${id}`, {
      method: 'PUT',
      body: data,
    })
  }

  const deleteNetworkSegment = async (id: number) => {
    return await $api(`/network/segments/${id}`, {
      method: 'DELETE',
    })
  }

  // ============================================================================
  // Network devices
  // ============================================================================
  const fetchNetworkDevices = async (params?: { page?: number; per_page?: number }) => {
    return await $api<NetworkDeviceListResponse>('/network/devices', {
      params,
    })
  }

  const getNetworkDevice = async (id: number) => {
    return await $api<NetworkDevice>(`/network/devices/${id}`)
  }

  const createNetworkDevice = async (data: NewNetworkDeviceRequest) => {
    return await $api<NetworkDevice>('/network/devices', {
      method: 'POST',
      body: data,
    })
  }

  const updateNetworkDevice = async (id: number, data: NewNetworkDeviceRequest) => {
    return await $api<NetworkDevice>(`/network/devices/${id}`, {
      method: 'PUT',
      body: data,
    })
  }

  const deleteNetworkDevice = async (id: number) => {
    return await $api(`/network/devices/${id}`, {
      method: 'DELETE',
    })
  }

  // ============================================================================
  // IP addresses
  // ============================================================================
  const fetchIpAddresses = async (params?: {
    page?: number
    per_page?: number
    segment_id?: number
  }) => {
    return await $api<IpAddressListResponse>('/network/ip-addresses', {
      params,
    })
  }

  const getIpAddress = async (id: number) => {
    return await $api<IpAddress>(`/network/ip-addresses/${id}`)
  }

  const createIpAddress = async (data: NewIpAddressRequest) => {
    return await $api<IpAddress>('/network/ip-addresses', {
      method: 'POST',
      body: data,
    })
  }

  const updateIpAddress = async (id: number, data: NewIpAddressRequest) => {
    return await $api<IpAddress>(`/network/ip-addresses/${id}`, {
      method: 'PUT',
      body: data,
    })
  }

  const deleteIpAddress = async (id: number) => {
    return await $api(`/network/ip-addresses/${id}`, {
      method: 'DELETE',
    })
  }

  // ============================================================================
  // Network scans
  // ============================================================================
  const fetchNetworkScans = async (params?: { page?: number; per_page?: number }) => {
    return await $api<NetworkScanListResponse>('/network/scans', {
      params,
    })
  }

  const getNetworkScan = async (id: number) => {
    return await $api<NetworkScan>(`/network/scans/${id}`)
  }

  const createNetworkScan = async (data: NewNetworkScanRequest) => {
    return await $api<NetworkScan>('/network/scans', {
      method: 'POST',
      body: data,
    })
  }

  const runNetworkScan = async (data: RunNetworkScanRequest) => {
    return await $api<NetworkScan>('/network/scans/run', {
      method: 'POST',
      body: data,
    })
  }

  const updateNetworkScan = async (id: number, data: NewNetworkScanRequest) => {
    return await $api<NetworkScan>(`/network/scans/${id}`, {
      method: 'PUT',
      body: data,
    })
  }

  const deleteNetworkScan = async (id: number) => {
    return await $api(`/network/scans/${id}`, {
      method: 'DELETE',
    })
  }

  return {
    // Network segments
    fetchNetworkSegments,
    getNetworkSegment,
    createNetworkSegment,
    updateNetworkSegment,
    deleteNetworkSegment,
    // Network devices
    fetchNetworkDevices,
    getNetworkDevice,
    createNetworkDevice,
    updateNetworkDevice,
    deleteNetworkDevice,
    // IP addresses
    fetchIpAddresses,
    getIpAddress,
    createIpAddress,
    updateIpAddress,
    deleteIpAddress,
    // Network scans
    fetchNetworkScans,
    getNetworkScan,
    createNetworkScan,
    runNetworkScan,
    updateNetworkScan,
    deleteNetworkScan,
  }
}
