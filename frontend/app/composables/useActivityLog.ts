/**
 * Дополнительные данные записи журнала.
 *
 * Состав свободный (каждый сервис пишет своё), поэтому индексная подпись
 * обязательна — но типичные поля описаны явно, чтобы читать их без приведений.
 */
export interface ActivityLogDetails {
  /** Что именно изменилось: поле -> старое/новое значение */
  changes?: Record<string, unknown>
  synced_count?: number
  imported_count?: number
  [key: string]: unknown
}

export interface ActivityLog {
  id: number
  user_id: string | null
  user_email: string | null
  action: string
  entity_type: string
  entity_id: number
  details: ActivityLogDetails | null
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
      params,
    })
  }

  const fetchEntityHistory = async (entityType: string, entityId: number) => {
    return await $api<ActivityLogListResponse>('/activity-log', {
      params: {
        entity_type: entityType,
        entity_id: entityId,
        per_page: 100,
      },
    })
  }

  return {
    fetchActivityLogs,
    fetchEntityHistory,
  }
}
