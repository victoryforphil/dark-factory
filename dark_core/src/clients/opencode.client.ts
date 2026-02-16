import { createOpencode, createOpencodeClient, type OpencodeClient } from '@opencode-ai/sdk';

import { getConfig } from '../config';

interface OpencodeServerRuntime {
  url: string;
  close: () => void;
}

let managedOpencodeRuntime: OpencodeServerRuntime | undefined;

const buildConfiguredBaseUrl = (): string => {
  const config = getConfig();
  return `http://${config.opencode.hostname}:${config.opencode.port}`;
};

const getClientBaseUrl = (): string => {
  if (managedOpencodeRuntime) {
    return managedOpencodeRuntime.url;
  }

  return buildConfiguredBaseUrl();
};

const ensureManagedOpencodeServer = async (): Promise<void> => {
  if (managedOpencodeRuntime) {
    return;
  }

  const config = getConfig();
  if (!config.opencode.autoStartServer) {
    return;
  }

  const { server } = await createOpencode({
    hostname: config.opencode.hostname,
    port: config.opencode.port,
    timeout: config.opencode.startupTimeoutMs,
    config: {
      logLevel: config.opencode.logLevel,
    },
  });

  managedOpencodeRuntime = {
    url: server.url,
    close: server.close,
  };
};

export const getOpencodeClient = async (directory?: string): Promise<OpencodeClient> => {
  await ensureManagedOpencodeServer();

  return createOpencodeClient({
    baseUrl: getClientBaseUrl(),
    directory,
    responseStyle: 'data',
    throwOnError: true,
  });
};

export const getOpencodeBaseUrl = (): string => {
  return getClientBaseUrl();
};

export const closeManagedOpencodeServer = (): void => {
  if (!managedOpencodeRuntime) {
    return;
  }

  managedOpencodeRuntime.close();
  managedOpencodeRuntime = undefined;
};
