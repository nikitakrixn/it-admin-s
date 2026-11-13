export interface AdGroup {
  id: number
  name: string
  distinguished_name: string | null
  description: string | null
  group_type: string | null
  group_scope: string | null
  member_count: number | null
  last_synced: string | null
  created_at: string
}

export interface NewAdGroupRequest {
  name: string
  distinguished_name?: string
  description?: string
  group_type?: string
  group_scope?: string
}

export interface AdGroupListResponse {
  groups: AdGroup[]
  total: number
  page: number
  per_page: number
}

export interface AdGroupMembership {
  id: number
  employee_id: number
  ad_group_id: number
  added_at: string | null
}

export interface NewAdGroupMembershipRequest {
  employee_id: number
  ad_group_id: number
}

export interface AdGroupMembershipListResponse {
  memberships: AdGroupMembership[]
  total: number
  page: number
  per_page: number
}

export interface DomainAccount {
  id: number
  employee_id: number | null
  username: string
  display_name: string | null
  email: string | null
  account_type: string
  distinguished_name: string | null
  user_principal_name: string | null
  sam_account_name: string | null
  password_last_set: string | null
  password_expires_at: string | null
  password_never_expires: boolean | null
  must_change_password: boolean | null
  cannot_change_password: boolean | null
  account_expires_at: string | null
  is_enabled: boolean | null
  is_locked: boolean | null
  locked_at: string | null
  created_at: string
  updated_at: string
}

export interface NewDomainAccountRequest {
  employee_id?: number
  username: string
  display_name?: string
  email?: string
  account_type: string
  distinguished_name?: string
  user_principal_name?: string
  sam_account_name?: string
  password_last_set?: string
  password_expires_at?: string
  password_never_expires?: boolean
  must_change_password?: boolean
  cannot_change_password?: boolean
  account_expires_at?: string
  is_enabled?: boolean
  is_locked?: boolean
  locked_at?: string
}

export interface DomainAccountListResponse {
  accounts: DomainAccount[]
  total: number
  page: number
  per_page: number
}

export interface AdSyncHistory {
  id: number
  sync_type: string | null
  status: string | null
  records_added: number | null
  records_updated: number | null
  records_removed: number | null
  error_message: string | null
  sync_started_at: string
  sync_completed_at: string | null
  created_at: string
}

export interface NewAdSyncHistoryRequest {
  sync_type?: string
  status?: string
  records_added?: number
  records_updated?: number
  records_removed?: number
  error_message?: string
  sync_started_at?: string
  sync_completed_at?: string
}

export interface AdSyncHistoryListResponse {
  syncs: AdSyncHistory[]
  total: number
  page: number
  per_page: number
}

export const useAd = () => {
  const { $api } = useNuxtApp()

  // ============================================================================
  // AD Groups
  // ============================================================================
  const fetchAdGroups = async (params?: { page?: number; per_page?: number }) => {
    return await $api<AdGroupListResponse>('/ad/groups', {
      params,
    })
  }

  const getAdGroup = async (id: number) => {
    return await $api<AdGroup>(`/ad/groups/${id}`)
  }

  const createAdGroup = async (data: NewAdGroupRequest) => {
    return await $api<AdGroup>('/ad/groups', {
      method: 'POST',
      body: data,
    })
  }

  const updateAdGroup = async (id: number, data: NewAdGroupRequest) => {
    return await $api<AdGroup>(`/ad/groups/${id}`, {
      method: 'PUT',
      body: data,
    })
  }

  const deleteAdGroup = async (id: number) => {
    return await $api(`/ad/groups/${id}`, {
      method: 'DELETE',
    })
  }

  // ============================================================================
  // AD Group Memberships
  // ============================================================================
  const fetchAdGroupMemberships = async (params?: {
    page?: number
    per_page?: number
    ad_group_id?: number
  }) => {
    return await $api<AdGroupMembershipListResponse>('/ad/group-memberships', {
      params,
    })
  }

  const getAdGroupMembership = async (id: number) => {
    return await $api<AdGroupMembership>(`/ad/group-memberships/${id}`)
  }

  const createAdGroupMembership = async (data: NewAdGroupMembershipRequest) => {
    return await $api<AdGroupMembership>('/ad/group-memberships', {
      method: 'POST',
      body: data,
    })
  }

  const updateAdGroupMembership = async (id: number, data: NewAdGroupMembershipRequest) => {
    return await $api<AdGroupMembership>(`/ad/group-memberships/${id}`, {
      method: 'PUT',
      body: data,
    })
  }

  const deleteAdGroupMembership = async (id: number) => {
    return await $api(`/ad/group-memberships/${id}`, {
      method: 'DELETE',
    })
  }

  // ============================================================================
  // Domain Accounts
  // ============================================================================
  const fetchDomainAccounts = async (params?: {
    page?: number
    per_page?: number
    account_status?: string
  }) => {
    return await $api<DomainAccountListResponse>('/ad/accounts', {
      params,
    })
  }

  const getDomainAccount = async (id: number) => {
    return await $api<DomainAccount>(`/ad/accounts/${id}`)
  }

  const createDomainAccount = async (data: NewDomainAccountRequest) => {
    return await $api<DomainAccount>('/ad/accounts', {
      method: 'POST',
      body: data,
    })
  }

  const updateDomainAccount = async (id: number, data: Partial<NewDomainAccountRequest>) => {
    return await $api<DomainAccount>(`/ad/accounts/${id}`, {
      method: 'PUT',
      body: data,
    })
  }

  const deleteDomainAccount = async (id: number) => {
    return await $api(`/ad/accounts/${id}`, {
      method: 'DELETE',
    })
  }

  // ============================================================================
  // AD Sync History
  // ============================================================================
  const fetchAdSyncHistory = async (params?: { page?: number; per_page?: number }) => {
    return await $api<AdSyncHistoryListResponse>('/ad/sync-history', {
      params,
    })
  }

  const getAdSyncHistory = async (id: number) => {
    return await $api<AdSyncHistory>(`/ad/sync-history/${id}`)
  }

  const createAdSyncHistory = async (data: NewAdSyncHistoryRequest) => {
    return await $api<AdSyncHistory>('/ad/sync-history', {
      method: 'POST',
      body: data,
    })
  }

  const updateAdSyncHistory = async (id: number, data: NewAdSyncHistoryRequest) => {
    return await $api<AdSyncHistory>(`/ad/sync-history/${id}`, {
      method: 'PUT',
      body: data,
    })
  }

  const deleteAdSyncHistory = async (id: number) => {
    return await $api(`/ad/sync-history/${id}`, {
      method: 'DELETE',
    })
  }

  return {
    // AD groups
    fetchAdGroups,
    getAdGroup,
    createAdGroup,
    updateAdGroup,
    deleteAdGroup,
    // AD group memberships
    fetchAdGroupMemberships,
    getAdGroupMembership,
    createAdGroupMembership,
    updateAdGroupMembership,
    deleteAdGroupMembership,
    // Domain accounts
    fetchDomainAccounts,
    getDomainAccount,
    createDomainAccount,
    updateDomainAccount,
    deleteDomainAccount,
    // AD sync history
    fetchAdSyncHistory,
    getAdSyncHistory,
    createAdSyncHistory,
    updateAdSyncHistory,
    deleteAdSyncHistory,
  }
}
