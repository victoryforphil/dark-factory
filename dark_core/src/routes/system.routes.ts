import { Elysia, t } from 'elysia';

import { getApiInfo, getHealth, getMetrics, resetLocalDatabase } from '../controllers';
import { failure, success, toErrorMessage } from '../utils/api-response';

export interface SystemRoutesDependencies {
  getApiInfo: typeof getApiInfo;
  getHealth: typeof getHealth;
  getMetrics: typeof getMetrics;
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

export const createSystemRoutes = (
  dependencies: SystemRoutesDependencies = {
    getApiInfo,
    getHealth,
    getMetrics,
    resetLocalDatabase,
  },
): Elysia => {
  return new Elysia({ prefix: '/system' })
    .get('/health', async ({ set }) => {
      try {
        return success(await dependencies.getHealth());
      } catch (error) {
        set.status = 500;
        return failure('SYSTEM_HEALTH_FAILED', toErrorMessage(error));
      }
    })
    .get('/info', async ({ set }) => {
      try {
        return success(await dependencies.getApiInfo());
      } catch (error) {
        set.status = 500;
        return failure('SYSTEM_INFO_FAILED', toErrorMessage(error));
      }
    })
    .get('/metrics', async ({ set }) => {
      try {
        return success(await dependencies.getMetrics());
      } catch (error) {
        set.status = 500;
        return failure('SYSTEM_METRICS_FAILED', toErrorMessage(error));
      }
    })
    .post(
      '/reset-db',
      async ({ set }) => {
        try {
          return success(await dependencies.resetLocalDatabase());
        } catch (error) {
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
