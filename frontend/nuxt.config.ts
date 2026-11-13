// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',

  devtools: { enabled: true },

  modules: [
    '@nuxtjs/tailwindcss',
    '@vueuse/nuxt',
    '@nuxt/icon',
    '@nuxtjs/color-mode',
    '@nuxt/eslint',
  ],

  // Генерируется flat-конфиг ESLint (см. eslint.config.mjs в корне фронта)
  eslint: {
    config: {},
  },

  // App configuration
  app: {
    head: {
      title: 'IT-Admin',
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
        { name: 'description', content: 'IT Infrastructure Management System' },
      ],
      link: [
        { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' },
        { rel: 'preconnect', href: 'https://fonts.googleapis.com' },
        {
          rel: 'preconnect',
          href: 'https://fonts.gstatic.com',
          crossorigin: '',
        },
        {
          rel: 'stylesheet',
          href: 'https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap',
        },
      ],
    },
  },

  // Runtime config
  runtimeConfig: {
    // Private keys (only available server-side)
    apiSecret: '',
    apiBaseServer: 'http://localhost:8000/api',

    // Public keys (exposed to client)
    public: {
      apiBase: 'http://localhost:8000/api',
      appName: 'IT-Admin',
      appEnv: 'development',
      wgDefaultClientAddress: '',
      wgDefaultClientDns: '',
      wgDefaultAllowedIps: '',
      wgDefaultPersistentKeepalive: '',
      wgDefaultEndpointIp: '',
    },
  },

  // Tailwind CSS configuration
  tailwindcss: {
    cssPath: '~/assets/css/tailwind.css',
    configPath: 'tailwind.config.js',
  },

  // Color mode configuration
  colorMode: {
    classSuffix: '',
    preference: 'system',
    fallback: 'light',
  },

  // PostCSS configuration
  postcss: {
    plugins: {
      // @nuxtjs/tailwindcss добавляет плагин "tailwindcss/nesting", но Nuxt
      // грузит PostCSS-плагины через ESM-импорт, а у tailwindcss v3 нет
      // exports-карты на "./nesting" (ERR_UNSUPPORTED_DIR_IMPORT) — плагин и так
      // не загружался, просто ругался в сборке. В CSS-файлах вложенность через
      // "&" не используется, поэтому отключаем явно: при необходимости её
      // следует перейти на Tailwind 4, где для этого есть "@tailwindcss/postcss".
      'tailwindcss/nesting': false,
    },
  },

  // TypeScript configuration
  typescript: {
    strict: true,
    // Проверка типов выполняется отдельно: npm run typecheck (см. CI)
    typeCheck: false,
  },

  // Vite configuration
  vite: {
    server: {
      // Vite 8: опции HMR-сокета переехали из server.hmr.* в server.ws.*
      ws: {
        protocol: 'ws',
        host: 'localhost',
      },
    },
  },

  // Development server
  devServer: {
    port: 3000,
    host: '0.0.0.0',
  },
})
