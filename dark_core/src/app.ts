import { Elysia } from 'elysia';
import { openapi } from '@elysiajs/openapi';
import { logger } from '@grotto/logysia';
import { llms } from '@opuu/elysia-llms-txt';

import { opencodeRoutes, productsRoutes, systemRoutes, variantsRoutes } from './modules';
import { ensurePrismaSchema } from './modules/prisma/prisma.client';
import Log from './utils/logging';

export const buildApp = (): Elysia => {
  return new Elysia()
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
    .onStart(async () => {
      await ensurePrismaSchema();
    })
    .get('/', () => ({ service: 'dark_core', status: 'ok' }))
    .use(systemRoutes)
    .use(opencodeRoutes)
    .use(productsRoutes)
    .use(variantsRoutes);
};
