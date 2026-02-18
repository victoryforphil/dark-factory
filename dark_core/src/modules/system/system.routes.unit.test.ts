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
    getProvidersInfo: async () => ({
      defaultProvider: 'mock',
      enabledProviders: ['mock', 'opencode/server'],
      providers: [
        {
          key: 'mock',
          configured: true,
          enabled: true,
          available: true,
        },
        {
          key: 'opencode/server',
          configured: true,
          enabled: true,
          available: true,
        },
      ],
    }),
    getSshInfo: async () => ({
      hosts: [
        {
          key: 'devbox',
          host: 'devbox',
          source: 'config',
          label: 'Dev Box (devbox)',
          user: 'alice',
          port: 2222,
          defaultPath: '/srv/workspaces',
        },
      ],
      portForwards: [
        {
          name: 'grafana',
          host: 'devbox',
          localPort: 3300,
          remotePort: 3000,
          remoteHost: '127.0.0.1',
          description: 'Grafana dashboard',
        },
      ],
      activeForwards: [
        {
          name: 'dark-ssh-devbox-3300-3000',
          attached: false,
          windows: 1,
          currentCommand: 'ssh',
        },
      ],
      tmuxSessions: [
        {
          name: 'dark-ssh-devbox-3300-3000',
          attached: false,
          windows: 1,
          currentCommand: 'ssh',
        },
        {
          name: 'dark-opencode-server',
          attached: false,
          windows: 1,
          currentCommand: 'opencode',
        },
      ],
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
    startSshPortForwardByInput: async () => ({
      sessionName: 'dark-ssh-devbox-3300-3000',
      host: 'devbox',
      command: "ssh -N -L 3300:127.0.0.1:3000 devbox",
      forwardSpecs: ['3300:127.0.0.1:3000'],
      alreadyRunning: false,
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

  it('returns configured providers', async () => {
    const app = new Elysia().use(createSystemRoutes(createDependencies()));

    const response = await app.handle(new Request('http://localhost/system/providers'));

    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toMatchObject({
      ok: true,
      data: {
        defaultProvider: 'mock',
        enabledProviders: ['mock', 'opencode/server'],
      },
    });
  });

  it('returns merged ssh hosts and forward presets', async () => {
    const app = new Elysia().use(createSystemRoutes(createDependencies()));

    const response = await app.handle(new Request('http://localhost/system/ssh'));

    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toMatchObject({
      ok: true,
      data: {
        hosts: expect.arrayContaining([
          expect.objectContaining({
            key: 'devbox',
            source: 'config',
          }),
        ]),
        portForwards: expect.arrayContaining([
          expect.objectContaining({
            name: 'grafana',
            localPort: 3300,
          }),
        ]),
        activeForwards: expect.arrayContaining([
          expect.objectContaining({
            currentCommand: 'ssh',
          }),
        ]),
        tmuxSessions: expect.arrayContaining([
          expect.objectContaining({
            name: 'dark-opencode-server',
          }),
        ]),
      },
    });
  });

  it('starts ssh port-forward session from system route', async () => {
    const app = new Elysia().use(createSystemRoutes(createDependencies()));

    const response = await app.handle(
      new Request('http://localhost/system/ssh/port-forward', {
        method: 'POST',
        headers: {
          'content-type': 'application/json',
        },
        body: JSON.stringify({
          presetName: 'grafana',
        }),
      }),
    );

    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toMatchObject({
      ok: true,
      data: {
        sessionName: 'dark-ssh-devbox-3300-3000',
        host: 'devbox',
      },
    });
  });
});
