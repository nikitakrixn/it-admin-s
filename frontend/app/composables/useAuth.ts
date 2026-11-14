export interface User {
  id: string
  email: string
  employee_id: number | null
  role: string
  is_active: boolean
  last_login_at: string | null
  created_at: string
}

export interface LoginCredentials {
  email: string
  password: string
}

export interface CreateUserCredentials {
  email: string
  password: string
  role: 'admin' | 'user'
}

export interface ChangePasswordCredentials {
  currentPassword: string
  newPassword: string
}

export const useAuth = () => {
  const user = useState<User | null>('user', () => null)

  /**
   * Вход выполняется через Nuxt-BFF: сервер получает JWT, кладёт его
   * в httpOnly-cookie и возвращает только профиль пользователя.
   */
  const login = async (credentials: LoginCredentials) => {
    try {
      const response = await $fetch<{ user: User }>('/api/auth/login', {
        method: 'POST',
        body: credentials,
      })

      user.value = response.user
      return response.user
    } catch (error) {
      throw new Error(getErrorMessage(error, 'Не удалось войти'), { cause: error })
    }
  }

  /** Загружает профиль по cookie. Возвращает null, если сессии нет. */
  const fetchUser = async () => {
    const requestFetch = useRequestFetch()

    try {
      const current = await requestFetch<User>('/api/auth/me')
      user.value = current
      return current
    } catch {
      user.value = null
      return null
    }
  }

  const logout = async () => {
    try {
      await $fetch('/api/auth/logout', { method: 'POST' })
    } catch {
      // Даже если сервер недоступен, локальное состояние нужно очистить.
    }

    user.value = null
    await navigateTo('/login')
  }

  const changePassword = async (credentials: ChangePasswordCredentials) => {
    const { $api } = useNuxtApp()

    await $api('/auth/change-password', {
      method: 'POST',
      body: {
        current_password: credentials.currentPassword,
        new_password: credentials.newPassword,
      },
    })
  }

  const isAuthenticated = computed(() => !!user.value)
  const isAdmin = computed(() => user.value?.role === 'admin')

  return {
    user: readonly(user),
    isAuthenticated,
    isAdmin,
    login,
    fetchUser,
    logout,
    changePassword,
  }
}
