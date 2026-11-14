import type { User } from '~/composables/useAuth'

/**
 * Клиент API. Ходит только в Nuxt-BFF (`/api/backend`), который сам
 * подставляет JWT из httpOnly-cookie.
 */
export default defineNuxtPlugin(() => {
  const user = useState<User | null>('user', () => null)
  const router = useRouter()

  // На сервере относительный baseURL не разрешается — используем origin запроса.
  const baseURL = import.meta.server ? `${useRequestURL().origin}/api/backend` : '/api/backend'
  const headers = import.meta.server ? useRequestHeaders(['cookie']) : undefined

  const api = $fetch.create({
    baseURL,
    headers,
    onResponseError({ response }) {
      if (response.status !== 401) {
        return
      }

      user.value = null

      if (router.currentRoute.value.path !== '/login') {
        navigateTo('/login')
      }
    },
  })

  return {
    provide: {
      api,
    },
  }
})
