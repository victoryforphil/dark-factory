import { createOpencodeClient, type OpencodeClient } from '@opencode-ai/sdk';

import { getConfig } from '../../../config';
import Log, { formatLogMetadata } from '../../../utils/logging';

interface OpencodeServerRuntime {
  url: string;
  close: () => void;
  source: 'managed-tmux' | 'external';
  tmuxSessionName?: string;
}

export interface OpencodeRuntimeInfo {
  source: 'managed-tmux' | 'external' | 'uninitialized';
  tmuxSessionName?: string;
}

let managedOpencodeRuntime: OpencodeServerRuntime | undefined;

const stripAnsi = (value: string): string => {
  return value.replace(/\u001b\[[0-9;]*m/g, '');
};

const extractLogFilePath = (value: string): string | undefined => {
  const match = value.match(/check log file at (\S+)/i);
  return match?.[1];
};

const shellEscapeSingleQuotes = (value: string): string => {
  return `'${value.replace(/'/g, `'\\''`)}'`;
};

const normalizeConnectHost = (host: string): string => {
  const trimmed = host.trim();
  if (!trimmed || trimmed === '0.0.0.0') {
    return '127.0.0.1';
  }

  if (trimmed === '::') {
    return '::1';
  }

  return trimmed;
};

const decodeBytes = (value: Uint8Array | undefined): string => {
  if (!value) {
    return '';
  }

  return new TextDecoder().decode(value).trim();
};

const runTmux = (args: string[]): {
  success: boolean;
  stdout: string;
  stderr: string;
  exitCode: number | null;
} => {
  const result = Bun.spawnSync({
    cmd: ['tmux', ...args],
    stdout: 'pipe',
    stderr: 'pipe',
  });

  return {
    success: result.success,
    stdout: decodeBytes(result.stdout),
    stderr: decodeBytes(result.stderr),
    exitCode: result.exitCode,
  };
};

const ensureTmuxAvailable = (): void => {
  const version = runTmux(['-V']);
  if (version.success) {
    return;
  }

  throw new Error(
    `Providers // OpenCode // tmux unavailable (stderr=${version.stderr || '<empty>'},exit=${version.exitCode ?? 'null'})`,
  );
};

const tmuxSessionExists = (sessionName: string): boolean => {
  const result = runTmux(['has-session', '-t', sessionName]);
  return result.success;
};

const killTmuxSession = (sessionName: string): void => {
  const result = runTmux(['kill-session', '-t', sessionName]);
  if (result.success) {
    return;
  }

  if (/can't find session/i.test(result.stderr)) {
    return;
  }

  throw new Error(
    `Providers // OpenCode // Failed to kill tmux session (session=${sessionName},stderr=${result.stderr || '<empty>'},exit=${result.exitCode ?? 'null'})`,
  );
};

const captureTmuxTail = (sessionName: string, lines: number): string => {
  const start = `-${Math.max(1, lines)}`;
  const result = runTmux(['capture-pane', '-pt', sessionName, '-S', start]);
  if (!result.success) {
    return '<unable to capture tmux output>';
  }

  return result.stdout || '<empty tmux output>';
};

const launchOpencodeInTmux = (input: {
  mode: 'serve' | 'web';
  bindHost: string;
  port: number;
  tmuxSessionName: string;
}): void => {
  const launchCommand = [
    'opencode',
    input.mode,
    '--hostname',
    shellEscapeSingleQuotes(input.bindHost),
    '--port',
    String(input.port),
  ].join(' ');

  const result = runTmux([
    'new-session',
    '-d',
    '-s',
    input.tmuxSessionName,
    '/bin/sh',
    '-lc',
    launchCommand,
  ]);

  if (result.success) {
    return;
  }

  throw new Error(
    `Providers // OpenCode // Failed to launch tmux opencode server (session=${input.tmuxSessionName},stderr=${result.stderr || '<empty>'},exit=${result.exitCode ?? 'null'})`,
  );
};

const waitForConfiguredOpencodeReachable = async (timeoutMs: number): Promise<boolean> => {
  const startedAt = Date.now();
  while (Date.now() - startedAt < timeoutMs) {
    if (await isConfiguredOpencodeReachable()) {
      return true;
    }

    await Bun.sleep(250);
  }

  return false;
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
  return `http://${normalizeConnectHost(config.opencode.hostname)}:${config.opencode.port}`;
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

  if (await isConfiguredOpencodeReachable()) {
    managedOpencodeRuntime = {
      url: buildConfiguredBaseUrl(),
      close: () => {},
      source: 'external',
    };
    Log.info(
      `Core // Providers OpenCode // Auto-start skipped, reusing existing server ${formatLogMetadata({
        host: config.opencode.hostname,
        port: config.opencode.port,
        source: 'external',
      })}`,
    );
    return;
  }

  ensureTmuxAvailable();

  const sessionName = config.opencode.tmuxSessionName;
  const sessionExists = tmuxSessionExists(sessionName);
  if (sessionExists) {
    killTmuxSession(sessionName);
  }

  try {
    launchOpencodeInTmux({
      mode: config.opencode.serverMode,
      bindHost: config.opencode.hostname,
      port: config.opencode.port,
      tmuxSessionName: sessionName,
    });
  } catch (error) {
    const rawMessage = error instanceof Error ? error.message : String(error);
    const cleanMessage = stripAnsi(rawMessage).replace(/\s+/g, ' ').trim();
    const logFile = extractLogFilePath(cleanMessage);
    Log.error(
      `Core // Providers OpenCode // tmux startup failed ${formatLogMetadata({
        error: cleanMessage,
        host: config.opencode.hostname,
        logFile: logFile ?? '-',
        mode: config.opencode.serverMode,
        port: config.opencode.port,
        tmuxSessionName: sessionName,
      })}`,
    );
    throw error;
  }

  const becameReachable = await waitForConfiguredOpencodeReachable(config.opencode.startupTimeoutMs);
  if (!becameReachable) {
    const tail = captureTmuxTail(sessionName, 40);
    throw new Error(
      `Providers // OpenCode // Server startup timed out (session=${sessionName}, mode=${config.opencode.serverMode}, host=${config.opencode.hostname}, port=${config.opencode.port}, tail=${tail})`,
    );
  }

  managedOpencodeRuntime = {
    url: buildConfiguredBaseUrl(),
    close: () => {
      try {
        killTmuxSession(sessionName);
      } catch {
        // noop: process shutdown path should not throw.
      }
    },
    source: 'managed-tmux',
    tmuxSessionName: sessionName,
  };
  Log.info(
    `Core // Providers OpenCode // Managed server ready ${formatLogMetadata({
      mode: config.opencode.serverMode,
      source: 'managed-tmux',
      tmuxSessionName: sessionName,
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

export const getOpencodeRuntimeInfo = (): OpencodeRuntimeInfo => {
  if (!managedOpencodeRuntime) {
    return { source: 'uninitialized' };
  }

  return {
    source: managedOpencodeRuntime.source,
    tmuxSessionName: managedOpencodeRuntime.tmuxSessionName,
  };
};

export const closeManagedOpencodeServer = (): void => {
  if (!managedOpencodeRuntime) {
    return;
  }

  if (managedOpencodeRuntime.source === 'managed-tmux') {
    managedOpencodeRuntime.close();
  }
  managedOpencodeRuntime = undefined;
};
