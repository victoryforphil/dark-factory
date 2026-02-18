import { mkdir } from 'node:fs/promises';
import { basename, dirname, isAbsolute, resolve } from 'node:path';

import { getConfig } from '../../config';
import { getPrismaClient } from '../prisma/prisma.client';
import { listConfiguredProviders, getProvidersRuntimeConfig } from '../providers/providers.registry';
import {
  listActiveSshForwardSessions,
  listSshHosts,
  listSshPortForwardPresets,
  listTmuxSessions,
  startSshPortForward,
} from '../ssh/ssh.controller';
import Log, { formatLogMetadata } from '../../utils/logging';

export interface ServiceInfo {
  name: string;
  version: string;
  env: string;
}

export interface ResetLocalDatabaseResult {
  backupPath: string;
  databasePath: string;
  deletedRows: {
    products: number;
    variants: number;
  };
  resetAt: string;
}

export interface ProvidersInfo {
  defaultProvider: string;
  enabledProviders: string[];
  providers: Array<{
    key: string;
    configured: boolean;
    enabled: boolean;
    available: boolean;
  }>;
}

export interface SshInfo {
  hosts: Array<{
    key: string;
    host: string;
    source: 'config' | 'ssh_config';
    label: string;
    user?: string;
    port?: number;
    defaultPath?: string;
  }>;
  portForwards: Array<{
    name: string;
    host?: string;
    localPort: number;
    remotePort: number;
    remoteHost: string;
    description?: string;
  }>;
  activeForwards: Array<{
    name: string;
    attached: boolean;
    windows: number;
    currentCommand: string;
  }>;
  tmuxSessions: Array<{
    name: string;
    attached: boolean;
    windows: number;
    currentCommand: string;
  }>;
}

export interface StartSshPortForwardInput {
  presetName?: string;
  host?: string;
  localPort?: number;
  remotePort?: number;
  remoteHost?: string;
}

export interface StartSshPortForwardInfo {
  sessionName: string;
  host: string;
  command: string;
  forwardSpecs: string[];
  alreadyRunning: boolean;
}

const resolveFileDatabasePath = async (databaseUrl: string): Promise<string> => {
  if (!databaseUrl.startsWith('file:')) {
    throw new Error(`System // Reset DB // Only file: database URLs are supported (url=${databaseUrl})`);
  }

  const relativePath = databaseUrl.slice('file:'.length);

  if (!relativePath) {
    throw new Error('System // Reset DB // Missing database file path in prisma.databaseUrl');
  }

  if (databaseUrl.startsWith('file://')) {
    return decodeURIComponent(new URL(databaseUrl).pathname);
  }

  const possiblePaths = isAbsolute(relativePath)
    ? [relativePath]
    : [resolve(process.cwd(), relativePath), resolve(import.meta.dir, '..', '..', relativePath)];

  for (const possiblePath of possiblePaths) {
    if (await Bun.file(possiblePath).exists()) {
      return possiblePath;
    }
  }

  return possiblePaths[0] ?? relativePath;
};

const createBackupPath = (databasePath: string): string => {
  const timestamp = new Date().toISOString().replace(/[.:]/g, '-');
  const dbName = basename(databasePath);
  const dbNameWithoutExtension = dbName.endsWith('.db') ? dbName.slice(0, -3) : dbName;

  return resolve(dirname(databasePath), `${dbNameWithoutExtension}.${timestamp}.backup.db`);
};

export const getHealth = async (): Promise<{ status: 'ok'; timestamp: string }> => {
  return {
    status: 'ok',
    timestamp: new Date().toISOString(),
  };
};

export const getApiInfo = async (): Promise<ServiceInfo> => {
  return {
    name: 'dark_core',
    version: Bun.env.npm_package_version ?? '0.0.0',
    env: Bun.env.NODE_ENV ?? 'development',
  };
};

export const getMetrics = async (): Promise<Record<string, number>> => {
  return {
    uptimeSeconds: Math.floor(process.uptime()),
  };
};

export const getProvidersInfo = async (): Promise<ProvidersInfo> => {
  const runtime = getProvidersRuntimeConfig();

  return {
    defaultProvider: runtime.defaultProvider,
    enabledProviders: runtime.enabledProviders,
    providers: listConfiguredProviders(),
  };
};

export const getSshInfo = async (): Promise<SshInfo> => {
  return {
    hosts: listSshHosts(),
    portForwards: listSshPortForwardPresets(),
    activeForwards: listActiveSshForwardSessions(),
    tmuxSessions: listTmuxSessions(),
  };
};

export const startSshPortForwardByInput = async (
  input: StartSshPortForwardInput,
): Promise<StartSshPortForwardInfo> => {
  const presetName = input.presetName?.trim();
  const config = getConfig();

  if (presetName) {
    const preset = config.ssh.portForwards.find((candidate) => candidate.name === presetName);
    if (!preset) {
      throw new Error(
        `SSH // Port Forward // Preset not found ${formatLogMetadata({ presetName })}`,
      );
    }

    const host = input.host?.trim() || preset.host;
    if (!host) {
      throw new Error(
        `SSH // Port Forward // Preset host is required ${formatLogMetadata({ presetName })}`,
      );
    }

    return startSshPortForward({
      host,
      forwards: [
        {
          localPort: preset.localPort,
          remotePort: preset.remotePort,
          remoteHost: preset.remoteHost,
        },
      ],
      extraSshArgs: preset.sshArgs,
    });
  }

  const host = input.host?.trim();
  if (!host) {
    throw new Error('SSH // Port Forward // Host is required');
  }

  if (!input.localPort || !input.remotePort) {
    throw new Error('SSH // Port Forward // localPort and remotePort are required');
  }

  return startSshPortForward({
    host,
    forwards: [
      {
        localPort: input.localPort,
        remotePort: input.remotePort,
        remoteHost: input.remoteHost,
      },
    ],
  });
};

export const resetLocalDatabase = async (): Promise<ResetLocalDatabaseResult> => {
  const prisma = getPrismaClient();
  const databasePath = await resolveFileDatabasePath(getConfig().prisma.databaseUrl);

  if (!(await Bun.file(databasePath).exists())) {
    throw new Error(`System // Reset DB // Database file not found (path=${databasePath})`);
  }

  const backupPath = createBackupPath(databasePath);
  await mkdir(dirname(backupPath), { recursive: true });
  await Bun.write(backupPath, Bun.file(databasePath));

  const deletedRows = await prisma.$transaction(async (tx) => {
    const variants = await tx.variant.deleteMany({});
    const products = await tx.product.deleteMany({});

    return {
      products: products.count,
      variants: variants.count,
    };
  });

  const resetAt = new Date().toISOString();

  Log.info(
    `Core // System Controller // Local database reset ${formatLogMetadata({
      backupPath,
      databasePath,
      deletedRows,
      resetAt,
    })}`,
  );

  return {
    backupPath,
    databasePath,
    deletedRows,
    resetAt,
  };
};
