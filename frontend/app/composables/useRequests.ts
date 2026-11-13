export interface RequestItem {
  id: number
  title: string
  description: string
  requester_id: number
  assignee_id: number | null
  computer_id: number | null
  equipment_id: number | null
  priority: string
  status: string
  category: string | null
  created_at: string
  updated_at: string
  resolved_at: string | null
  closed_at: string | null
  first_response_at: string | null
  sla_deadline: string | null
  sla_breached: boolean | null
  estimated_time: number | null
  actual_time: number | null
  parent_request_id: number | null
  recurring_schedule: string | null
  knowledge_base_article_id: number | null
  satisfaction_rating: number | null
  satisfaction_comment: string | null
}

export interface NewRequestRequest {
  title: string
  description: string
  requester_id: number
  assignee_id?: number
  computer_id?: number
  equipment_id?: number
  priority?: string
  status?: string
  category?: string
  resolved_at?: string
  closed_at?: string
  first_response_at?: string
  sla_deadline?: string
  sla_breached?: boolean
  estimated_time?: number
  actual_time?: number
  parent_request_id?: number
  recurring_schedule?: string
  knowledge_base_article_id?: number
  satisfaction_rating?: number
  satisfaction_comment?: string
}

export interface RequestListResponse {
  requests: RequestItem[]
  total: number
  page: number
  per_page: number
}

export interface RequestComment {
  id: number
  request_id: number
  user_id: string
  user_email: string | null
  comment: string
  is_internal: boolean
  created_at: string
}

export interface RequestCommentListResponse {
  comments: RequestComment[]
  total: number
}

export const useRequests = () => {
  const { $api } = useNuxtApp()

  const fetchRequests = async (params?: {
    page?: number
    per_page?: number
    status?: string
    priority?: string
    assignee_id?: number
    requester_id?: number
  }) => {
    return await $api<RequestListResponse>('/requests', { params })
  }

  const fetchRequest = async (id: number) => {
    return await $api<RequestItem>(`/requests/${id}`)
  }

  const createRequest = async (data: NewRequestRequest) => {
    return await $api<RequestItem>('/requests', {
      method: 'POST',
      body: data,
    })
  }

  const updateRequest = async (id: number, data: Partial<NewRequestRequest>) => {
    return await $api<RequestItem>(`/requests/${id}`, {
      method: 'PUT',
      body: data,
    })
  }

  const deleteRequest = async (id: number) => {
    return await $api(`/requests/${id}`, {
      method: 'DELETE',
    })
  }

  const fetchRequestComments = async (requestId: number) => {
    return await $api<RequestCommentListResponse>(`/requests/${requestId}/comments`)
  }

  const createRequestComment = async (
    requestId: number,
    data: { comment: string; is_internal?: boolean },
  ) => {
    return await $api<RequestComment>(`/requests/${requestId}/comments`, {
      method: 'POST',
      body: data,
    })
  }

  const deleteRequestComment = async (commentId: number) => {
    return await $api(`/requests/comments/${commentId}`, {
      method: 'DELETE',
    })
  }

  return {
    fetchRequests,
    fetchRequest,
    createRequest,
    updateRequest,
    deleteRequest,
    fetchRequestComments,
    createRequestComment,
    deleteRequestComment,
  }
}
