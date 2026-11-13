export interface CountItem {
  key: string
  count: number
}

export interface PriorityResolution {
  priority: string
  count: number
  avg_hours: number | null
}

export interface TopIssue {
  title: string
  count: number
  avg_hours: number | null
}

export interface WeekdayActivity {
  weekday: number
  count: number
}

export interface AnalyticsOverview {
  period_days: number
  total_employees: number
  active_employees: number
  total_computers: number
  active_computers: number
  total_requests: number
  open_requests: number
  requests_created_in_period: number
  requests_resolved_in_period: number
  avg_resolution_hours: number | null
  total_licenses: number
  used_licenses: number
  expiring_licenses: number
  expired_licenses: number
  total_equipment: number
  equipment_needs_attention: number
  low_stock_consumables: number
  requests_by_status: CountItem[]
  requests_by_priority: CountItem[]
  requests_by_category: CountItem[]
  resolution_by_priority: PriorityResolution[]
  top_issues: TopIssue[]
  activity_by_weekday: WeekdayActivity[]
}

export const useAnalytics = () => {
  const { $api } = useNuxtApp()

  const fetchAnalyticsOverview = async (days = 30) => {
    return await $api<AnalyticsOverview>('/dashboard/analytics', {
      params: { days },
    })
  }

  return {
    fetchAnalyticsOverview,
  }
}
