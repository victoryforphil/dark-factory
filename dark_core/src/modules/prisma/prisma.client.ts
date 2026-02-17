import { PrismaClient } from '../../../../generated/prisma/client';
import { PrismaLibSql } from '@prisma/adapter-libsql';
import { existsSync } from 'node:fs';
import { mkdtemp, writeFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';
import { tmpdir } from 'node:os';

import { getConfig } from '../../config';
import Log, { formatLogMetadata } from '../../utils/logging';

let prismaClient: PrismaClient | undefined;
let prismaSchemaEnsurePromise: Promise<void> | undefined;
let prismaSchemaPathPromise: Promise<string> | undefined;

const REPO_ROOT = resolve(import.meta.dir, '../../../../');
const PRISMA_SCHEMA_CANDIDATES = [
  resolve(REPO_ROOT, 'prisma/schema.prisma'),
  resolve(REPO_ROOT, '../prisma/schema.prisma'),
  resolve(process.cwd(), 'prisma/schema.prisma'),
];

const resolveEmbeddedPrismaSchemaPath = async (): Promise<string | undefined> => {
  const embeddedSchema = Bun.embeddedFiles.find((file) => file.name.endsWith('.prisma') && file.name.includes('schema'));

  if (!embeddedSchema) {
    return undefined;
  }

  const schemaText = await embeddedSchema.text();
  const tempDir = await mkdtemp(join(tmpdir(), 'darkfactory-prisma-'));
  const schemaPath = join(tempDir, 'schema.prisma');

  await writeFile(schemaPath, schemaText, 'utf8');
  return schemaPath;
};

const resolvePrismaSchemaPath = async (): Promise<string> => {
  if (prismaSchemaPathPromise) {
    return prismaSchemaPathPromise;
  }

  prismaSchemaPathPromise = (async () => {
    for (const schemaPath of PRISMA_SCHEMA_CANDIDATES) {
      if (existsSync(schemaPath)) {
        return schemaPath;
      }
    }

    const embeddedSchemaPath = await resolveEmbeddedPrismaSchemaPath();
    if (embeddedSchemaPath) {
      return embeddedSchemaPath;
    }

    throw new Error(
      'Core // Client Prisma // Missing schema.prisma (include prisma/schema.prisma on disk or embed it in bun build --compile)',
    );
  })();

  return prismaSchemaPathPromise;
};

const normalizePrismaCliDatabaseUrl = (databaseUrl: string): string => {
  if (!databaseUrl.startsWith('file:')) {
    return databaseUrl;
  }

  const fileValue = databaseUrl.slice('file:'.length);

  if (fileValue === ':memory:') {
    return databaseUrl;
  }

  const [pathPart, queryPart] = fileValue.split('?');
  if (!pathPart || pathPart.startsWith('/')) {
    return databaseUrl;
  }

  const absolutePath = resolve(process.cwd(), pathPart);
  return queryPart ? `file:${absolutePath}?${queryPart}` : `file:${absolutePath}`;
};

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
  const schemaPath = await resolvePrismaSchemaPath();
  const prismaCliDatabaseUrl = normalizePrismaCliDatabaseUrl(databaseUrl);
  const command = Bun.spawn(
    ['bunx', 'prisma', 'db', 'push', '--schema', schemaPath, '--url', prismaCliDatabaseUrl],
    {
      cwd: existsSync(REPO_ROOT) ? REPO_ROOT : undefined,
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
  prismaSchemaPathPromise = undefined;
};
