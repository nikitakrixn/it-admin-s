import type { FetchError } from 'ofetch'

export interface User {
  id: string
  email: string
  employee_id: number | null
  role: string
  is_active: boolean
  last_login_at: string | null
  created_at: string
}

export interface AuthResponse {
  token: string
  user: User
}

export interface LoginCredentials {
  email: string
  password: string
}

export interface RegisterCredentials {
  email: string
  password: string
  name: string
}

export interface ErrorResponse {
  error: string
  message: string
}

export const useAuth = () => {
  const config = useRuntimeConfig()
  const token = useCookie('auth_token', {
    maxAge: 60 * 60 * 24, // 24 hours
    sameSite: 'lax',
    secure: process.env.NODE_ENV === 'production'
  })
  const user = useState<User | null>('user', () => null)

  const login = async (credentials: LoginCredentials) => {
    try {
      const { data, error } = await useFetch<AuthResponse, FetchError<ErrorResponse>>(`${config.public.apiBase}/auth/login`, {
        method: 'POST',
        body: credentials
      })

      if (error.value) {
        throw new Error(error.value.data?.message || 'Login failed')
      }

      if (data.value) {
        token.value = data.value.token
        user.value = data.value.user
        return data.value
      }
    } catch (err: any) {
      throw new Error(err.message || 'Login failed')
    }
  }

  const register = async (credentials: RegisterCredentials) => {
    try {
      const { data, error } = await useFetch<AuthResponse, FetchError<ErrorResponse>>(`${config.public.apiBase}/auth/register`, {
        method: 'POST',
        body: credentials
      })

      if (error.value) {
        throw new Error(error.value.data?.message || 'Registration failed')
      }

      if (data.value) {
        token.value = data.value.token
        user.value = data.value.user
        return data.value
      }
    } catch (err: any) {
      throw new Error(err.message || 'Registration failed')
    }
  }

  const fetchUser = async () => {
    if (!token.value) {
      user.value = null
      return null
    }

    try {
      const { data, error } = await useFetch<User, FetchError<ErrorResponse>>(`${config.public.apiBase}/auth/me`, {
        headers: {
          Authorization: `Bearer ${token.value}`
        }
      })

      if (error.value) {
        token.value = null
        user.value = null
        return null
      }

      if (data.value) {
        user.value = data.value
        return data.value
      }
    } catch (err) {
      token.value = null
      user.value = null
      return null
    }
  }

  const logout = () => {
    token.value = null
    user.value = null
    navigateTo('/login')
  }

  const isAuthenticated = computed(() => !!token.value && !!user.value)

  return {
    user: readonly(user),
    token: readonly(token),
    isAuthenticated,
    login,
    register,
    fetchUser,
    logout
  }
}
