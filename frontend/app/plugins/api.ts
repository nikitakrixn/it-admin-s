import type { User } from '~/composables/useAuth'

export default defineNuxtPlugin(() => {
  const token = useCookie('auth_token')
  const user = useState<User | null>('user', () => null)

  const api = $fetch.create({
    baseURL: useApiBase(),
    onRequest({ options }) {
      // Добавляем токен к каждому запросу
      if (token.value) {
        options.headers = new Headers(options.headers || {})
        options.headers.set('Authorization', `Bearer ${token.value}`)
      }
    },
    onResponseError({ response }) {
      if (response.status === 401) {
        if (user.value && token.value) {
          console.warn('Access denied: insufficient permissions')
        } else {
          token.value = null
          navigateTo('/login')
        }
      }
    },
  })

  return {
    provide: {
      api,
    },
  }
})
