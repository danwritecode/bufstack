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
    '@nuxtjs/color-mode',
    'shadcn-nuxt',
  ],

  shadcn: {
    prefix: '',
    componentDir: '@/components/ui',
  },

  colorMode: {
    classSuffix: '',
    preference: 'system',
    fallback: 'light',
  },

  vite: {
    plugins: [
      tailwindcss()
    ],
  },

  css: ['~/assets/css/tailwind.css'],
})
