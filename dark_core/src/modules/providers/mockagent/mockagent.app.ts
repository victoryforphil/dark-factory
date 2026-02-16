import { Elysia } from 'elysia';

import { createMockAgentRoutes, type MockAgentRoutesDependencies } from './mockagent.routes';

export const buildMockAgentApp = (
  dependencies?: MockAgentRoutesDependencies,
) => {
  return new Elysia().get('/', () => ({ service: 'mockagent', status: 'ok' })).use(createMockAgentRoutes(dependencies));
};
