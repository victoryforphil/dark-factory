import { mkdirSync, rmSync } from 'node:fs';
import { relative, resolve } from 'node:path';

import { resetPrismaClientForTests } from '../../modules/prisma/prisma.client';
import { resetConfigCache } from '../../config';

const REPO_ROOT = resolve(import.meta.dir, '../../../../');
const TEST_DB_DIR = resolve(REPO_ROOT, '.darkfactory/test');
const TEST_DB_ENV_KEY_RUNTIME = 'DARKFACTORY_PRISMA_DATABASE_URL';
const TEST_DB_ENV_KEY_PRISMA_CLI = 'DARKFACTORY_SQLITE_URL';
const PRISMA_SCHEMA_PATH = 'prisma/schema.prisma';

const toSafeSegment = (value: string): string => {
  return value
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '')
    .slice(0, 64);
};

const removeSqliteFiles = (dbFilePath: string): void => {
  for (const suffix of ['', '-shm', '-wal']) {
    rmSync(`${dbFilePath}${suffix}`, { force: true });
  }
};

const runPrismaDbPush = async (databaseUrl: string): Promise<void> => {
  const command = Bun.spawn(['bunx', 'prisma', 'db', 'push', '--schema', PRISMA_SCHEMA_PATH, '--url', databaseUrl], {
    cwd: REPO_ROOT,
    env: {
      ...process.env,
      RUST_LOG: 'info',
    },
    stdout: 'pipe',
    stderr: 'pipe',
  });

  const [stdout, stderr, exitCode] = await Promise.all([
    new Response(command.stdout).text(),
    new Response(command.stderr).text(),
    command.exited,
  ]);

  if (exitCode !== 0) {
    throw new Error(
      `Test // Prisma // db push failed (exit=${exitCode},stdout=${stdout.trim()},stderr=${stderr.trim()})`,
    );
  }
};

export interface SqliteTestDatabase {
  dbFilePath: string;
  databaseUrl: string;
  setup: () => Promise<void>;
  teardown: () => Promise<void>;
}

export const createSqliteTestDatabase = (label = 'integration'): SqliteTestDatabase => {
  mkdirSync(TEST_DB_DIR, { recursive: true });

  const safeLabel = toSafeSegment(label) || 'integration';
  const uniqueName = `${safeLabel}-${Date.now()}-${crypto.randomUUID()}`;
  const dbFilePath = resolve(TEST_DB_DIR, `${uniqueName}.db`);
  const dbRelativePath = relative(REPO_ROOT, dbFilePath).replace(/\\/g, '/');
  const runtimeDatabaseUrl = `file:${dbFilePath}`;
  const prismaCliDatabaseUrl = `file:./${dbRelativePath}`;

  return {
    dbFilePath,
    databaseUrl: runtimeDatabaseUrl,
    setup: async () => {
      await resetPrismaClientForTests();
      resetConfigCache();
      Bun.env[TEST_DB_ENV_KEY_RUNTIME] = runtimeDatabaseUrl;
      Bun.env[TEST_DB_ENV_KEY_PRISMA_CLI] = prismaCliDatabaseUrl;
      process.env[TEST_DB_ENV_KEY_RUNTIME] = runtimeDatabaseUrl;
      process.env[TEST_DB_ENV_KEY_PRISMA_CLI] = prismaCliDatabaseUrl;
      await runPrismaDbPush(prismaCliDatabaseUrl);
    },
    teardown: async () => {
      await resetPrismaClientForTests();
      resetConfigCache();
      delete Bun.env[TEST_DB_ENV_KEY_RUNTIME];
      delete Bun.env[TEST_DB_ENV_KEY_PRISMA_CLI];
      delete process.env[TEST_DB_ENV_KEY_RUNTIME];
      delete process.env[TEST_DB_ENV_KEY_PRISMA_CLI];
      removeSqliteFiles(dbFilePath);
    },
  };
};
