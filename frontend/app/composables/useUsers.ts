import type { User } from '~/composables/useAuth'

export type UserRole = 'admin' | 'user'

export interface CreateUserPayload {
  email: string
  password: string
  role: UserRole
}

export interface UpdateUserPayload {
  role?: UserRole
  is_active?: boolean
}

export const useUsers = () => {
  const { $api } = useNuxtApp()

  const fetchUsers = () => $api<User[]>('/auth/users')

  const createUser = (payload: CreateUserPayload) =>
    $api<User>('/auth/register', { method: 'POST', body: payload })

  const updateUser = (id: string, payload: UpdateUserPayload) =>
    $api<User>(`/auth/users/${id}`, { method: 'PATCH', body: payload })

  return { fetchUsers, createUser, updateUser }
}
