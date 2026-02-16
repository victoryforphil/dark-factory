import { describe, expect, it } from 'bun:test';
import { Elysia } from 'elysia';

import { createSystemRoutes, type SystemRoutesDependencies } from './system.routes';

const createDependencies = (): SystemRoutesDependencies => {
  return {
    getApiInfo: async () => ({
      name: 'dark_core',
      version: '0.0.0',
      env: 'test',
    }),
    getHealth: async () => ({
      status: 'ok',
      timestamp: new Date().toISOString(),
    }),
    getMetrics: async () => ({
      uptimeSeconds: 42,
    }),
    resetLocalDatabase: async () => ({
      backupPath: '/tmp/darkfactory.dev.2026-02-16T00-00-00-000Z.backup.db',
      databasePath: '/tmp/darkfactory.dev.db',
      deletedRows: {
        products: 2,
        variants: 4,
      },
      resetAt: '2026-02-16T00:00:00.000Z',
    }),
  };
};

describe('system routes unit', () => {
  it('resets local database and returns backup path', async () => {
    const app = new Elysia().use(createSystemRoutes(createDependencies()));

    const response = await app.handle(
      new Request('http://localhost/system/reset-db', {
        method: 'POST',
      }),
    );

    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toEqual({
      ok: true,
      data: {
        backupPath: '/tmp/darkfactory.dev.2026-02-16T00-00-00-000Z.backup.db',
        databasePath: '/tmp/darkfactory.dev.db',
        deletedRows: {
          products: 2,
          variants: 4,
        },
        resetAt: '2026-02-16T00:00:00.000Z',
      },
    });
  });

  it('maps reset failures into API failure response', async () => {
    const dependencies = createDependencies();
    dependencies.resetLocalDatabase = async () => {
      throw new Error('reset failed');
    };

    const app = new Elysia().use(createSystemRoutes(dependencies));

    const response = await app.handle(
      new Request('http://localhost/system/reset-db', {
        method: 'POST',
      }),
    );

    expect(response.status).toBe(500);
    await expect(response.json()).resolves.toEqual({
      ok: false,
      error: {
        code: 'SYSTEM_RESET_DB_FAILED',
        message: 'reset failed',
      },
    });
  });
});
