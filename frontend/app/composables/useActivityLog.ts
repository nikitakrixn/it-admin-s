export interface ActivityLog {
  id: number
  user_id: string | null
  user_email: string | null
  action: string
  entity_type: string
  entity_id: number
  details: Record<string, any> | null
  ip_address: string | null
  user_agent: string | null
  created_at: string
}

export interface ActivityLogListResponse {
  logs: ActivityLog[]
  total: number
  page: number
  per_page: number
}

export const useActivityLog = () => {
  const { $api } = useNuxtApp()

  const fetchActivityLogs = async (params?: {
    page?: number
    per_page?: number
    action?: string
    entity_type?: string
    user_id?: string
  }) => {
    return await $api<ActivityLogListResponse>('/activity-log', {
      params
    })
  }

  return {
    fetchActivityLogs
  }
}
