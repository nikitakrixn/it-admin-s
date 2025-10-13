export default defineNuxtPlugin(() => {
  const config = useRuntimeConfig();
  const token = useCookie("auth_token");
  const user = useState<any>("user");

  const api = $fetch.create({
    baseURL: config.public.apiBase,
    onRequest({ options }) {
      // Добавляем токен к каждому запросу
      if (token.value) {
        options.headers = {
          ...options.headers,
          Authorization: `Bearer ${token.value}`,
        };
      }
    },
    onResponseError({ response, request }) {
      if (response.status === 401) {
        if (user.value && token.value) {
          console.warn("Access denied: insufficient permissions");
        } else {
          token.value = null;
          navigateTo("/login");
        }
      }
    },
  });

  return {
    provide: {
      api,
    },
  };
});
