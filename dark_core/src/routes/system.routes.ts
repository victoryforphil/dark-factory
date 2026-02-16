import { Elysia } from 'elysia';

import { getApiInfo, getHealth, getMetrics } from '../controllers';
import { failure, success, toErrorMessage } from '../utils/api-response';

export const systemRoutes = new Elysia({ prefix: '/system' })
  .get('/health', async ({ set }) => {
    try {
      return success(await getHealth());
    } catch (error) {
      set.status = 500;
      return failure('SYSTEM_HEALTH_FAILED', toErrorMessage(error));
    }
  })
  .get('/info', async ({ set }) => {
    try {
      return success(await getApiInfo());
    } catch (error) {
      set.status = 500;
      return failure('SYSTEM_INFO_FAILED', toErrorMessage(error));
    }
  })
  .get('/metrics', async ({ set }) => {
    try {
      return success(await getMetrics());
    } catch (error) {
      set.status = 500;
      return failure('SYSTEM_METRICS_FAILED', toErrorMessage(error));
    }
  });
