export default defineNuxtRouteMiddleware(async (to, from) => {
  const { isAuthenticated, fetchUser } = useAuth();

  // Если пользователь не авторизован, пытаемся загрузить данные
  if (!isAuthenticated.value) {
    await fetchUser();
  }

  // Если всё ещё не авторизован, редирект на логин
  if (!isAuthenticated.value) {
    return navigateTo("/login");
  }
});
