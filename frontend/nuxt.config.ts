// https://nuxt.com/docs/api/configuration/nuxt-config
import tailwindcss from '@tailwindcss/vite'

const clerkEnabled = process.env.NUXT_PUBLIC_CLERK_ENABLED === 'true'

export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: false },

  runtimeConfig: {
    backendUrl: "http://localhost:50051",
    public: {
      clerkEnabled,
    },
  },

  modules: [
    ...(clerkEnabled ? ['@clerk/nuxt'] : []),
    '@nuxtjs/color-mode',
    '@nuxt/eslint',
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
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      tailwindcss() as any
    ],
  },

  css: ['~/assets/css/tailwind.css'],
})
