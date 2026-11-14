export default defineNuxtRouteMiddleware(async () => {
  const { isAuthenticated, isAdmin, fetchUser } = useAuth()

  // После перезагрузки состояние пользователя пустое — восстанавливаем его по cookie.
  if (!isAuthenticated.value) {
    await fetchUser()
  }

  if (!isAuthenticated.value) {
    return navigateTo('/login')
  }

  // Раздел доступен только администраторам: бэкенд тоже проверяет роль,
  // здесь мы просто не показываем страницу остальным.
  if (!isAdmin.value) {
    return navigateTo('/')
  }
})
