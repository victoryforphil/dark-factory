import { createOpencode, createOpencodeClient, type OpencodeClient } from '@opencode-ai/sdk';

import { getConfig } from '../../../config';
import Log, { formatLogMetadata } from '../../../utils/logging';

interface OpencodeServerRuntime {
  url: string;
  close: () => void;
  source: 'managed' | 'external';
}

let managedOpencodeRuntime: OpencodeServerRuntime | undefined;

const stripAnsi = (value: string): string => {
  return value.replace(/\u001b\[[0-9;]*m/g, '');
};

const extractLogFilePath = (value: string): string | undefined => {
  const match = value.match(/check log file at (\S+)/i);
  return match?.[1];
};

const isPortBindFailure = (value: string): boolean => {
  return /failed to start server on port/i.test(value);
};

const isConfiguredOpencodeReachable = async (): Promise<boolean> => {
  const baseUrl = buildConfiguredBaseUrl();
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), 1500);

  try {
    const response = await fetch(baseUrl, {
      method: 'GET',
      signal: controller.signal,
    });

    return response.ok || response.status === 404;
  } catch {
    return false;
  } finally {
    clearTimeout(timeoutId);
  }
};

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

  let server: { url: string; close: () => void };
  try {
    const created = await createOpencode({
      hostname: config.opencode.hostname,
      port: config.opencode.port,
      timeout: config.opencode.startupTimeoutMs,
      config: {
        logLevel: config.opencode.logLevel,
      },
    });

    server = created.server;
  } catch (error) {
    const rawMessage = error instanceof Error ? error.message : String(error);
    const cleanMessage = stripAnsi(rawMessage).replace(/\s+/g, ' ').trim();
    const logFile = extractLogFilePath(cleanMessage);

    if (isPortBindFailure(cleanMessage) && (await isConfiguredOpencodeReachable())) {
      managedOpencodeRuntime = {
        url: buildConfiguredBaseUrl(),
        close: () => {},
        source: 'external',
      };
      Log.info(
        `Core // Providers OpenCode // Auto-start skipped, reusing existing server ${formatLogMetadata({
          host: config.opencode.hostname,
          logFile: logFile ?? '-',
          port: config.opencode.port,
          source: 'external',
        })}`,
      );
      return;
    }

    Log.error(
      `Core // Providers OpenCode // Server startup failed ${formatLogMetadata({
        error: cleanMessage,
        host: config.opencode.hostname,
        logFile: logFile ?? '-',
        port: config.opencode.port,
      })}`,
    );

    const reason = logFile
      ? `Providers // OpenCode // Server startup failed (logFile=${logFile})`
      : 'Providers // OpenCode // Server startup failed';
    throw new Error(reason);
  }

  managedOpencodeRuntime = {
    url: server.url,
    close: server.close,
    source: 'managed',
  };
  Log.info(
    `Core // Providers OpenCode // Managed server ready ${formatLogMetadata({
      source: 'managed',
      url: managedOpencodeRuntime.url,
    })}`,
  );
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

  if (managedOpencodeRuntime.source === 'managed') {
    managedOpencodeRuntime.close();
  }
  managedOpencodeRuntime = undefined;
};
