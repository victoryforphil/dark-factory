import { Elysia, t } from 'elysia';

import {
  getApiInfo,
  getHealth,
  getMetrics,
  getProvidersInfo,
  resetLocalDatabase,
} from './system.controller';
import { failure, success, toErrorMessage } from '../../utils/api-response';
import Log, { formatLogMetadata } from '../../utils/logging';

export interface SystemRoutesDependencies {
  getApiInfo: typeof getApiInfo;
  getHealth: typeof getHealth;
  getMetrics: typeof getMetrics;
  getProvidersInfo: typeof getProvidersInfo;
  resetLocalDatabase: typeof resetLocalDatabase;
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

export const createSystemRoutes = (
  dependencies: SystemRoutesDependencies = {
    getApiInfo,
    getHealth,
    getMetrics,
    getProvidersInfo,
    resetLocalDatabase,
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
