import { joinURL } from 'ufo'

const config = useRuntimeConfig()

export default defineEventHandler(async (event) => {
  const proxyUrl = config.backendUrl
  const path = event.path
    .replace(/^\/api\//, '')
    .replace(/^rpc\//, '')

  const target = joinURL(proxyUrl, path)

  const opts = {
    headers: {
      'x-forwarded-for': '',
      'x-forwarded-port': '',
      'x-forwarded-proto': '',
    },
  }

  return await proxyRequest(event, target, opts)
})
