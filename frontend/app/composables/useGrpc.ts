import { createClient, type Client } from "@connectrpc/connect";
import type { DescService } from "@bufbuild/protobuf";

// Client-side auth, initialized by grpc-auth.client plugin
let cachedGetToken: ReturnType<typeof useAuth>['getToken'] | null = null;

export function initGrpcAuth(auth: ReturnType<typeof useAuth>) {
  cachedGetToken = auth.getToken
}

// Client-side transport is cached
let clientTransport: Awaited<ReturnType<typeof import("@connectrpc/connect-web").createGrpcWebTransport>> | null = null;

export async function useGrpcClient<T extends DescService>(
  service: T
): Promise<Client<T>> {
  if (import.meta.server) {
    // Server: create transport with forwarded headers
    const headers = useRequestHeaders(['cookie', 'authorization']);
    const { createGrpcTransport } = await import("@connectrpc/connect-node");
    const transport = createGrpcTransport({
      // @ts-ignore
      baseUrl: process.env.NUXT_BACKEND_URL,
      interceptors: [
        (next) => async (req) => {
          if (headers.cookie) req.header.set('cookie', headers.cookie);
          if (headers.authorization) req.header.set('authorization', headers.authorization);
          return next(req);
        }
      ],
    });
    return createClient(service, transport);
  } else {
    if (!clientTransport) {
      const { createGrpcWebTransport } = await import("@connectrpc/connect-web");
      clientTransport = createGrpcWebTransport({
        baseUrl: '/api/rpc',
        interceptors: cachedGetToken ? [
          (next) => async (req) => {
            await cachedGetToken!.value()
            return next(req);
          }
        ] : []
      });
    }
    return createClient(service, clientTransport);
  }
}
