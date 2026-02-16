import { buildMockAgentApp } from './mockagent.app';

export interface StartMockAgentServerInput {
  hostname?: string;
  port?: number;
}

export const startMockAgentServer = (input: StartMockAgentServerInput = {}) => {
  const hostname = input.hostname ?? '127.0.0.1';
  const port = input.port ?? 4199;
  const app = buildMockAgentApp();

  app.listen({
    hostname,
    port,
  });

  return app;
};
