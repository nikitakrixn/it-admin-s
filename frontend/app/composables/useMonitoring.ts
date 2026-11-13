export interface ComputerMonitoring {
  id: number
  computer_id: number
  cpu_usage: string | null
  ram_usage_percent: string | null
  ram_total_gb: number | null
  ram_used_gb: string | null
  disk_usage: Record<string, unknown> | null
  cpu_temperature: string | null
  gpu_temperature: string | null
  uptime_seconds: number | null
  network_stats: Record<string, unknown> | null
  processes_count: number | null
  logged_users: Record<string, unknown> | null
  collected_at: string
}

export interface NewComputerMonitoringRequest {
  computer_id: number
  cpu_usage?: number
  ram_usage_percent?: number
  ram_total_gb?: number
  ram_used_gb?: number
  disk_usage?: Record<string, unknown>
  cpu_temperature?: number
  gpu_temperature?: number
  uptime_seconds?: number
  network_stats?: Record<string, unknown>
  processes_count?: number
  logged_users?: Record<string, unknown>
  collected_at?: string
}

export interface ComputerMonitoringResponse {
  id: number
  computer_id: number
  cpu_usage: string | null
  ram_usage_percent: string | null
  ram_total_gb: number | null
  ram_used_gb: string | null
  disk_usage: Record<string, unknown> | null
  cpu_temperature: string | null
  gpu_temperature: string | null
  uptime_seconds: number | null
  network_stats: Record<string, unknown> | null
  processes_count: number | null
  logged_users: Record<string, unknown> | null
  collected_at: string
}

export interface ComputerMonitoringListResponse {
  monitoring: ComputerMonitoringResponse[]
  total: number
  page: number
  per_page: number
}

export interface ComputerMonitor {
  id: number
  computer_id: number
  name: string
  manufacturer: string | null
  model: string | null
  serial_number: string | null
  size_inches: string | null
  resolution: string | null
  panel_type: string | null
  refresh_rate: number | null
  is_primary: boolean | null
  notes: string | null
  created_at: string
  updated_at: string
}

export interface NewComputerMonitorRequest {
  computer_id: number
  name: string
  manufacturer?: string
  model?: string
  serial_number?: string
  size_inches?: number
  resolution?: string
  panel_type?: string
  refresh_rate?: number
  is_primary?: boolean
  notes?: string
}

export interface ComputerMonitorListResponse {
  monitors: ComputerMonitor[]
  total: number
  page: number
  per_page: number
}

export interface MonitoringDashboardSummary {
  total_computers_monitored: number
  total_monitoring_records: number
  avg_cpu_usage: number | null
  avg_ram_usage_percent: number | null
  computers_with_high_cpu: number
  computers_with_high_ram: number
  last_collected_at: string | null
}

export interface MonitoringDashboardListResponse {
  summary: MonitoringDashboardSummary
  recent_monitoring: ComputerMonitoringResponse[]
}

export const useMonitoring = () => {
  const { $api } = useNuxtApp()

  // ============================================================================
  // Computer monitoring metrics
  // ============================================================================
  const fetchLatestMonitoring = async (computerId: number) => {
    return await $api<ComputerMonitoringResponse>(`/monitoring/computers/${computerId}/latest`)
  }

  const fetchMonitoringHistory = async (
    computerId: number,
    params?: {
      page?: number
      per_page?: number
    },
  ) => {
    return await $api<ComputerMonitoringListResponse>(
      `/monitoring/computers/${computerId}/history`,
      {
        params,
      },
    )
  }

  const createMonitoring = async (data: NewComputerMonitoringRequest) => {
    return await $api<ComputerMonitoringResponse>(`/monitoring/computers/${data.computer_id}`, {
      method: 'POST',
      body: data,
    })
  }

  const deleteMonitoring = async (computerId: number, id: number) => {
    return await $api(`/monitoring/computers/${computerId}/history/${id}`, {
      method: 'DELETE',
    })
  }

  // ============================================================================
  // Computer monitors (physical displays)
  // ============================================================================
  const fetchComputerMonitors = async (params?: {
    page?: number
    per_page?: number
    computer_id?: number
  }) => {
    return await $api<ComputerMonitorListResponse>('/monitoring/monitors', {
      params,
    })
  }

  const getComputerMonitor = async (id: number) => {
    return await $api<ComputerMonitor>(`/monitoring/monitors/${id}`)
  }

  const createComputerMonitor = async (data: NewComputerMonitorRequest) => {
    return await $api<ComputerMonitor>('/monitoring/monitors', {
      method: 'POST',
      body: data,
    })
  }

  const updateComputerMonitor = async (id: number, data: NewComputerMonitorRequest) => {
    return await $api<ComputerMonitor>(`/monitoring/monitors/${id}`, {
      method: 'PUT',
      body: data,
    })
  }

  const deleteComputerMonitor = async (id: number) => {
    return await $api(`/monitoring/monitors/${id}`, {
      method: 'DELETE',
    })
  }

  // ============================================================================
  // Dashboard
  // ============================================================================
  const fetchDashboardSummary = async () => {
    return await $api<MonitoringDashboardListResponse>('/monitoring/dashboard')
  }

  return {
    fetchLatestMonitoring,
    fetchMonitoringHistory,
    createMonitoring,
    deleteMonitoring,
    fetchComputerMonitors,
    getComputerMonitor,
    createComputerMonitor,
    updateComputerMonitor,
    deleteComputerMonitor,
    fetchDashboardSummary,
  }
}
