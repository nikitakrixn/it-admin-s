/**
 * Базовый адрес API.
 *
 * В браузере используется публичный адрес (localhost или домен),
 * а на сервере — внутренний адрес контейнера, если он задан.
 * Это позволяет запускать приложение в Docker без правок клиентского кода.
 */
export const useApiBase = () => {
  const config = useRuntimeConfig()

  if (import.meta.server) {
    return (config.apiBaseServer as string) || (config.public.apiBase as string)
  }

  return config.public.apiBase as string
}
