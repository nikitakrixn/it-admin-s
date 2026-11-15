export type InventoryAuditType = 'full' | 'partial' | 'spot_check' | 'annual'

export type InventoryAuditStatus = 'in_progress' | 'completed' | 'cancelled'

export type InventoryItemStatus =
  'pending' | 'found' | 'missing' | 'damaged' | 'relocated' | 'surplus' | 'disposed'

export type InventoryCondition = 'new' | 'good' | 'fair' | 'poor' | 'broken'

export interface InventoryCounts {
  total: number
  checked: number
  found: number
  missing: number
  damaged: number
}

export interface InventoryAudit {
  id: number
  audit_name: string
  audit_date: string
  audit_type: InventoryAuditType
  location: string | null
  department_id: number | null
  audited_by: string
  audited_by_email: string | null
  status: InventoryAuditStatus
  notes: string | null
  started_at: string | null
  completed_at: string | null
  created_at: string
  counts: InventoryCounts
}

export interface InventoryItem {
  id: number
  audit_id: number
  entity_type: string
  entity_id: number
  entity_name: string | null
  entity_inventory_number: string | null
  expected_location: string | null
  actual_location: string | null
  expected_condition: string | null
  actual_condition: string | null
  status: InventoryItemStatus
  discrepancy_notes: string | null
  checked_at: string | null
  checked_by: string | null
}

export interface InventoryAuditListResponse {
  audits: InventoryAudit[]
  total: number
  page: number
  per_page: number
}

export interface InventoryAuditDetailResponse {
  audit: InventoryAudit
  items: InventoryItem[]
}

export interface CreateInventoryAuditPayload {
  audit_name: string
  audit_type?: InventoryAuditType
  location?: string
  department_id?: number
  notes?: string
}

export interface UpdateInventoryItemPayload {
  status: InventoryItemStatus
  actual_location?: string
  actual_condition?: InventoryCondition | ''
  discrepancy_notes?: string
}

export const AUDIT_TYPE_LABELS: Record<InventoryAuditType, string> = {
  full: 'Полная',
  partial: 'Частичная',
  spot_check: 'Выборочная',
  annual: 'Годовая',
}

export const AUDIT_STATUS_LABELS: Record<InventoryAuditStatus, string> = {
  in_progress: 'В работе',
  completed: 'Завершена',
  cancelled: 'Отменена',
}

export const AUDIT_STATUS_COLORS: Record<InventoryAuditStatus, string> = {
  in_progress: 'bg-blue-100 text-blue-800',
  completed: 'bg-green-100 text-green-800',
  cancelled: 'bg-gray-100 text-gray-700',
}

export const ITEM_STATUS_LABELS: Record<InventoryItemStatus, string> = {
  pending: 'Не проверено',
  found: 'Найдено',
  missing: 'Отсутствует',
  damaged: 'Повреждено',
  relocated: 'Перемещено',
  surplus: 'Излишек',
  disposed: 'Списано',
}

export const ITEM_STATUS_COLORS: Record<InventoryItemStatus, string> = {
  pending: 'bg-gray-100 text-gray-700',
  found: 'bg-green-100 text-green-800',
  missing: 'bg-red-100 text-red-800',
  damaged: 'bg-amber-100 text-amber-800',
  relocated: 'bg-blue-100 text-blue-800',
  surplus: 'bg-purple-100 text-purple-800',
  disposed: 'bg-gray-200 text-gray-700',
}

export const CONDITION_LABELS: Record<InventoryCondition, string> = {
  new: 'Новое',
  good: 'Хорошее',
  fair: 'Удовлетворительное',
  poor: 'Плохое',
  broken: 'Неисправное',
}

export const useInventory = () => {
  const { $api } = useNuxtApp()

  const fetchAudits = (params?: { page?: number; per_page?: number; status?: string }) =>
    $api<InventoryAuditListResponse>('/inventory/audits', { params })

  const fetchAudit = (id: number) => $api<InventoryAuditDetailResponse>(`/inventory/audits/${id}`)

  const createAudit = (payload: CreateInventoryAuditPayload) =>
    $api<InventoryAuditDetailResponse>('/inventory/audits', {
      method: 'POST',
      body: payload,
    })

  const updateItem = (auditId: number, itemId: number, payload: UpdateInventoryItemPayload) =>
    $api<InventoryItem>(`/inventory/audits/${auditId}/items/${itemId}`, {
      method: 'PATCH',
      body: payload,
    })

  const completeAudit = (id: number, markPendingMissing = false) =>
    $api<InventoryAuditDetailResponse>(`/inventory/audits/${id}/complete`, {
      method: 'POST',
      body: { mark_pending_missing: markPendingMissing },
    })

  const cancelAudit = (id: number) =>
    $api<InventoryAuditDetailResponse>(`/inventory/audits/${id}/cancel`, { method: 'POST' })

  return { fetchAudits, fetchAudit, createAudit, updateItem, completeAudit, cancelAudit }
}
