import { PrismaClient } from '../../../../generated/prisma/client';
import { PrismaLibSql } from '@prisma/adapter-libsql';
import { resolve } from 'node:path';

import { getConfig } from '../../config';
import Log, { formatLogMetadata } from '../../utils/logging';

let prismaClient: PrismaClient | undefined;
let prismaSchemaEnsurePromise: Promise<void> | undefined;

const REPO_ROOT = resolve(import.meta.dir, '../../../../');
const PRISMA_SCHEMA_PATH = 'prisma/schema.prisma';

const createPrismaClient = (): PrismaClient => {
  const prismaDatabaseUrl = getConfig().prisma.databaseUrl;
  const prismaLogQueries = getConfig().prisma.logQueries;
  const prismaAdapter = new PrismaLibSql({ url: prismaDatabaseUrl });

  const client = new PrismaClient({
    adapter: prismaAdapter,
    log: prismaLogQueries ? ['query', 'info', 'warn', 'error'] : ['warn', 'error'],
  });

  Log.info(
    `Core // Client Prisma // Initialized ${formatLogMetadata({
      database: 'sqlite',
      queryLogs: prismaLogQueries,
      url: prismaDatabaseUrl,
    })}`,
  );

  return client;
};

export const getPrismaClient = (): PrismaClient => {
  if (!prismaClient) {
    prismaClient = createPrismaClient();
  }

  return prismaClient;
};

const runPrismaDbPush = async (databaseUrl: string): Promise<void> => {
  const command = Bun.spawn(
    ['bunx', 'prisma', 'db', 'push', '--schema', PRISMA_SCHEMA_PATH, '--url', databaseUrl],
    {
      cwd: REPO_ROOT,
      env: {
        ...process.env,
        RUST_LOG: 'info',
      },
      stdout: 'pipe',
      stderr: 'pipe',
    },
  );

  const [stdout, stderr, exitCode] = await Promise.all([
    new Response(command.stdout).text(),
    new Response(command.stderr).text(),
    command.exited,
  ]);

  if (exitCode !== 0) {
    throw new Error(
      `Core // Client Prisma // db push failed ${formatLogMetadata({
        exitCode,
        stderr: stderr.trim() || null,
        stdout: stdout.trim() || null,
      })}`,
    );
  }
};

export const ensurePrismaSchema = async (): Promise<void> => {
  if (prismaSchemaEnsurePromise) {
    return prismaSchemaEnsurePromise;
  }

  prismaSchemaEnsurePromise = (async () => {
    const databaseUrl = getConfig().prisma.databaseUrl;
    await runPrismaDbPush(databaseUrl);
    Log.info('Core // Client Prisma // Ensured schema with Prisma db push');
  })();

  return prismaSchemaEnsurePromise;
};

export const resetPrismaClientForTests = async (): Promise<void> => {
  if (prismaClient) {
    await prismaClient.$disconnect();
  }

  prismaClient = undefined;
  prismaSchemaEnsurePromise = undefined;
};
