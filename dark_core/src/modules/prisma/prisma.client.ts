import { PrismaClient } from '../../../../generated/prisma/client';
import { PrismaLibSql } from '@prisma/adapter-libsql';

import { getConfig } from '../../config';
import Log, { formatLogMetadata } from '../../utils/logging';

let prismaClient: PrismaClient | undefined;
let prismaSchemaEnsurePromise: Promise<void> | undefined;

interface SqliteTableInfoRow {
  name?: string;
}

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

const getTableColumnNames = async (tableName: string): Promise<Set<string>> => {
  const prisma = getPrismaClient();
  const rows = await prisma.$queryRawUnsafe<Array<SqliteTableInfoRow>>(`PRAGMA table_info("${tableName}")`);

  return new Set(rows.map((row) => row.name).filter((name): name is string => Boolean(name)));
};

const addColumnIfMissing = async (
  tableName: string,
  columnName: string,
  sqlDefinition: string,
): Promise<boolean> => {
  const prisma = getPrismaClient();
  const columns = await getTableColumnNames(tableName);

  if (columns.has(columnName)) {
    return false;
  }

  await prisma.$executeRawUnsafe(
    `ALTER TABLE "${tableName}" ADD COLUMN "${columnName}" ${sqlDefinition}`,
  );

  return true;
};

export const ensurePrismaSchema = async (): Promise<void> => {
  if (prismaSchemaEnsurePromise) {
    return prismaSchemaEnsurePromise;
  }

  prismaSchemaEnsurePromise = (async () => {
    const applied: string[] = [];

    if (await addColumnIfMissing('products', 'git_info', 'JSON')) {
      applied.push('products.git_info');
    }

    if (await addColumnIfMissing('variants', 'git_info', 'JSON')) {
      applied.push('variants.git_info');
    }

    if (await addColumnIfMissing('variants', 'git_info_updated_at', 'DATETIME')) {
      applied.push('variants.git_info_updated_at');
    }

    if (await addColumnIfMissing('variants', 'git_info_last_polled_at', 'DATETIME')) {
      applied.push('variants.git_info_last_polled_at');
    }

    if (applied.length > 0) {
      Log.info(
        `Core // Client Prisma // Applied schema compatibility updates ${formatLogMetadata({ applied })}`,
      );
    }
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
