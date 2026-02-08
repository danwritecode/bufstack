// https://nuxt.com/docs/api/configuration/nuxt-config
import tailwindcss from '@tailwindcss/vite'

export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: false },

  runtimeConfig: {
    backendUrl: "http://localhost:50051",
    public: {
      baseUrl: "",
    },
  },

  modules: [
    '@clerk/nuxt',
  ],

  vite: {
    plugins: [
      tailwindcss()
    ],
  },

  css: ['~/assets/css/tailwind.css'],
})
