// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2025-07-15",

  devtools: { enabled: true },

  modules: [
    "@nuxtjs/tailwindcss",
    "@vueuse/nuxt",
    "@nuxt/icon",
    "@nuxtjs/color-mode",
  ],

  // App configuration
  app: {
    head: {
      title: "IT-Admin",
      meta: [
        { charset: "utf-8" },
        { name: "viewport", content: "width=device-width, initial-scale=1" },
        { name: "description", content: "IT Infrastructure Management System" },
      ],
      link: [
        { rel: "icon", type: "image/x-icon", href: "/favicon.ico" },
        { rel: "preconnect", href: "https://fonts.googleapis.com" },
        {
          rel: "preconnect",
          href: "https://fonts.gstatic.com",
          crossorigin: "",
        },
        {
          rel: "stylesheet",
          href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap",
        },
      ],
    },
  },

  // Runtime config
  runtimeConfig: {
    // Private keys (only available server-side)
    apiSecret: "",

    // Public keys (exposed to client)
    public: {
      apiBase: "http://localhost:8000/api",
      appName: "IT-Admin",
      appEnv: "development",
      wgDefaultClientAddress: "",
      wgDefaultClientDns: "",
      wgDefaultAllowedIps: "",
      wgDefaultPersistentKeepalive: "",
      wgDefaultEndpointIp: "",
    },
  },

  // Tailwind CSS configuration
  tailwindcss: {
    cssPath: "~/assets/css/tailwind.css",
    configPath: "tailwind.config.js",
  },

  // Color mode configuration
  colorMode: {
    classSuffix: "",
    preference: "system",
    fallback: "light",
  },

  // TypeScript configuration
  typescript: {
    strict: true,
    typeCheck: false, // Отключаем для dev режима, можно включить для production
  },

  // Vite configuration
  vite: {
    server: {
      hmr: {
        protocol: "ws",
        host: "localhost",
      },
    },
  },

  // Development server
  devServer: {
    port: 3000,
    host: "0.0.0.0",
  },
});
