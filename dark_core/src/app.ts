import { Elysia } from 'elysia';
import { openapi } from '@elysiajs/openapi';
import { logger } from '@grotto/logysia';
import { llms } from '@opuu/elysia-llms-txt';

import {
  actorsRoutes,
  createRealtimeEventHub,
  createRealtimeRoutes,
  productsRoutes,
  REALTIME_INTERNAL_ORIGIN_HEADER,
  REALTIME_INTERNAL_ORIGIN_WS_RPC,
  shouldBroadcastRouteMutation,
  systemRoutes,
  variantsRoutes,
} from './modules';
import { ensurePrismaSchema } from './modules/prisma/prisma.client';
import Log from './utils/logging';

const resolveSetStatusCode = (setStatus: unknown): number => {
  if (typeof setStatus === 'number') {
    return setStatus;
  }

  if (typeof setStatus === 'string') {
    const parsed = Number(setStatus);
    if (Number.isFinite(parsed)) {
      return parsed;
    }
  }

  return 200;
};

export const buildApp = (): Elysia => {
  const realtimeEventHub = createRealtimeEventHub();
  const app = new Elysia();

  const dispatchHttpRequest = (request: Request): Promise<Response> => {
    return app.handle(request);
  };

  app
    .use(openapi())
    .use(
      llms({
        source: {
          type: 'url',
          url: '/openapi/json',
        },
      }),
    )
    .use(
      logger({
        logIP: false,
        writer: {
          write(message: string) {
            Log.info(`Core // HTTP // ${message.trim()}`);
          },
        },
      }),
    )
    .onAfterHandle(({ request, response, set }) => {
      if (request.headers.get(REALTIME_INTERNAL_ORIGIN_HEADER) === REALTIME_INTERNAL_ORIGIN_WS_RPC) {
        return;
      }

      const method = request.method.toUpperCase();
      const path = new URL(request.url).pathname;
      const status = response instanceof Response ? response.status : resolveSetStatusCode(set.status);

      if (!shouldBroadcastRouteMutation({ method, path, status })) {
        return;
      }

      realtimeEventHub.publishRouteMutation({
        method,
        path,
        status,
        source: 'http',
      });
    })
    .onStart(async () => {
      await ensurePrismaSchema();
    })
    .get('/', () => ({ service: 'dark_core', status: 'ok' }))
    .use(
      createRealtimeRoutes({
        dispatchHttpRequest,
        eventHub: realtimeEventHub,
      }),
    )
    .use(systemRoutes)
    .use(actorsRoutes)
    .use(productsRoutes)
    .use(variantsRoutes);

  return app;
};
