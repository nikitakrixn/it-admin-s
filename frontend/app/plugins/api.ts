export default defineNuxtPlugin(() => {
  const config = useRuntimeConfig()
  const token = useCookie('auth_token')

  const api = $fetch.create({
    baseURL: config.public.apiBase,
    onRequest({ options }) {
      // Добавляем токен к каждому запросу
      if (token.value) {
        options.headers = {
          ...options.headers,
          Authorization: `Bearer ${token.value}`
        }
      }
    },
    onResponseError({ response }) {
      // Обработка ошибок авторизации
      if (response.status === 401) {
        token.value = null
        navigateTo('/login')
      }
    }
  })

  return {
    provide: {
      api
    }
  }
})
