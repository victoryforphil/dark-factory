import { Elysia, t } from 'elysia';

import {
  getApiInfo,
  getHealth,
  getMetrics,
  getProvidersInfo,
  getSshInfo,
  resetLocalDatabase,
  startSshPortForwardByInput,
} from './system.controller';
import { failure, success, toErrorMessage } from '../../utils/api-response';
import Log, { formatLogMetadata } from '../../utils/logging';

export interface SystemRoutesDependencies {
  getApiInfo: typeof getApiInfo;
  getHealth: typeof getHealth;
  getMetrics: typeof getMetrics;
  getProvidersInfo: typeof getProvidersInfo;
  getSshInfo: typeof getSshInfo;
  resetLocalDatabase: typeof resetLocalDatabase;
  startSshPortForwardByInput: typeof startSshPortForwardByInput;
}

const apiFailureResponse = t.Object({
  ok: t.Literal(false),
  error: t.Object({
    code: t.String(),
    message: t.String(),
  }),
});

const resetDatabaseResponse = t.Object({
  ok: t.Literal(true),
  data: t.Object({
    backupPath: t.String(),
    databasePath: t.String(),
    deletedRows: t.Object({
      products: t.Number(),
      variants: t.Number(),
    }),
    resetAt: t.String(),
  }),
});

const providersInfoResponse = t.Object({
  ok: t.Literal(true),
  data: t.Object({
    defaultProvider: t.String(),
    enabledProviders: t.Array(t.String()),
    providers: t.Array(
      t.Object({
        key: t.String(),
        configured: t.Boolean(),
        enabled: t.Boolean(),
        available: t.Boolean(),
      }),
    ),
  }),
});

const sshInfoResponse = t.Object({
  ok: t.Literal(true),
  data: t.Object({
    hosts: t.Array(
      t.Object({
        key: t.String(),
        host: t.String(),
        source: t.Union([t.Literal('config'), t.Literal('ssh_config')]),
        label: t.String(),
        user: t.Optional(t.String()),
        port: t.Optional(t.Number()),
        defaultPath: t.Optional(t.String()),
      }),
    ),
    portForwards: t.Array(
      t.Object({
        name: t.String(),
        host: t.Optional(t.String()),
        localPort: t.Number(),
        remotePort: t.Number(),
        remoteHost: t.String(),
        description: t.Optional(t.String()),
      }),
    ),
    activeForwards: t.Array(
      t.Object({
        name: t.String(),
        attached: t.Boolean(),
        windows: t.Number(),
        currentCommand: t.String(),
      }),
    ),
    tmuxSessions: t.Array(
      t.Object({
        name: t.String(),
        attached: t.Boolean(),
        windows: t.Number(),
        currentCommand: t.String(),
      }),
    ),
  }),
});

const sshStartBody = t.Object({
  presetName: t.Optional(t.String()),
  host: t.Optional(t.String()),
  localPort: t.Optional(t.Number()),
  remotePort: t.Optional(t.Number()),
  remoteHost: t.Optional(t.String()),
});

const sshStartResponse = t.Object({
  ok: t.Literal(true),
  data: t.Object({
    sessionName: t.String(),
    host: t.String(),
    command: t.String(),
    forwardSpecs: t.Array(t.String()),
    alreadyRunning: t.Boolean(),
  }),
});

export const createSystemRoutes = (
  dependencies: SystemRoutesDependencies = {
    getApiInfo,
    getHealth,
    getMetrics,
    getProvidersInfo,
    getSshInfo,
    resetLocalDatabase,
    startSshPortForwardByInput,
  },
): Elysia => {
  return new Elysia({ prefix: '/system' })
    .get('/health', async ({ set }) => {
      try {
        return success(await dependencies.getHealth());
      } catch (error) {
        Log.error(
          `Core // System Route // Health failed ${formatLogMetadata({ error: toErrorMessage(error) })}`,
        );
        set.status = 500;
        return failure('SYSTEM_HEALTH_FAILED', toErrorMessage(error));
      }
    })
    .get('/info', async ({ set }) => {
      try {
        return success(await dependencies.getApiInfo());
      } catch (error) {
        Log.error(
          `Core // System Route // Info failed ${formatLogMetadata({ error: toErrorMessage(error) })}`,
        );
        set.status = 500;
        return failure('SYSTEM_INFO_FAILED', toErrorMessage(error));
      }
    })
    .get('/metrics', async ({ set }) => {
      try {
        return success(await dependencies.getMetrics());
      } catch (error) {
        Log.error(
          `Core // System Route // Metrics failed ${formatLogMetadata({ error: toErrorMessage(error) })}`,
        );
        set.status = 500;
        return failure('SYSTEM_METRICS_FAILED', toErrorMessage(error));
      }
    })
    .get(
      '/providers',
      async ({ set }) => {
        try {
          return success(await dependencies.getProvidersInfo());
        } catch (error) {
          Log.error(
            `Core // System Route // Providers failed ${formatLogMetadata({ error: toErrorMessage(error) })}`,
          );
          set.status = 500;
          return failure('SYSTEM_PROVIDERS_FAILED', toErrorMessage(error));
        }
      },
      {
        response: {
          200: providersInfoResponse,
          500: apiFailureResponse,
        },
      },
    )
    .get(
      '/ssh',
      async ({ set }) => {
        try {
          return success(await dependencies.getSshInfo());
        } catch (error) {
          Log.error(
            `Core // System Route // SSH info failed ${formatLogMetadata({ error: toErrorMessage(error) })}`,
          );
          set.status = 500;
          return failure('SYSTEM_SSH_INFO_FAILED', toErrorMessage(error));
        }
      },
      {
        response: {
          200: sshInfoResponse,
          500: apiFailureResponse,
        },
      },
    )
    .post(
      '/ssh/port-forward',
      async ({ body, set }) => {
        try {
          return success(await dependencies.startSshPortForwardByInput(body));
        } catch (error) {
          const message = toErrorMessage(error);
          if (
            message.includes('Preset not found') ||
            message.includes('required') ||
            message.includes('tmux unavailable')
          ) {
            set.status = 400;
            return failure('SYSTEM_SSH_PORT_FORWARD_INVALID', message);
          }

          Log.error(
            `Core // System Route // SSH port-forward failed ${formatLogMetadata({ error: message })}`,
          );
          set.status = 500;
          return failure('SYSTEM_SSH_PORT_FORWARD_FAILED', message);
        }
      },
      {
        body: sshStartBody,
        response: {
          200: sshStartResponse,
          400: apiFailureResponse,
          500: apiFailureResponse,
        },
      },
    )
    .post(
      '/reset-db',
      async ({ set }) => {
        try {
          return success(await dependencies.resetLocalDatabase());
        } catch (error) {
          Log.error(
            `Core // System Route // Reset db failed ${formatLogMetadata({ error: toErrorMessage(error) })}`,
          );
          set.status = 500;
          return failure('SYSTEM_RESET_DB_FAILED', toErrorMessage(error));
        }
      },
      {
        response: {
          200: resetDatabaseResponse,
          500: apiFailureResponse,
        },
      },
    );
};

export const systemRoutes = createSystemRoutes();
