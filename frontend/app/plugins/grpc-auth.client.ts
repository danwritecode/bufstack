import { initGrpcAuth } from '~/composables/useGrpc'

export default defineNuxtPlugin(() => {
  initGrpcAuth(useAuth())
})
