export default defineNuxtRouteMiddleware(async (to, from) => {
  const { isAuthenticated, fetchUser } = useAuth()

  // Проверяем авторизацию
  if (!isAuthenticated.value) {
    await fetchUser()
  }

  // Если авторизован, редирект на главную
  if (isAuthenticated.value) {
    return navigateTo('/')
  }
})
