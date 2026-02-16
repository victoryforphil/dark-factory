import { mkdir } from 'node:fs/promises';
import { basename, dirname, isAbsolute, resolve } from 'node:path';

import { getConfig } from '../../config';
import { getPrismaClient } from '../prisma/prisma.client';
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
