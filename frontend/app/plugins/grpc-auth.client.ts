import { initGrpcAuth } from '~/composables/useGrpc'

export default defineNuxtPlugin(() => {
  const { clerkEnabled } = useRuntimeConfig().public
  if (!clerkEnabled) return

  initGrpcAuth(useAuth())
})
