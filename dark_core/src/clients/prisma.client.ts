import { PrismaClient } from '../../../generated/prisma/client';
import { PrismaLibSql } from '@prisma/adapter-libsql';

import { getConfig } from '../config';
import Log from '../utils/logging';

let prismaClient: PrismaClient | undefined;

const createPrismaClient = (): PrismaClient => {
  const prismaDatabaseUrl = getConfig().prisma.databaseUrl;
  const prismaLogQueries = getConfig().prisma.logQueries;
  const prismaAdapter = new PrismaLibSql({ url: prismaDatabaseUrl });

  const client = new PrismaClient({
    adapter: prismaAdapter,
    log: prismaLogQueries ? ['query', 'info', 'warn', 'error'] : ['warn', 'error'],
  });

  Log.info(
    `Core // Client Prisma // Initialized (db=sqlite,url=${prismaDatabaseUrl},queryLogs=${prismaLogQueries})`,
  );

  return client;
};

export const getPrismaClient = (): PrismaClient => {
  if (!prismaClient) {
    prismaClient = createPrismaClient();
  }

  return prismaClient;
};

export const resetPrismaClientForTests = async (): Promise<void> => {
  if (prismaClient) {
    await prismaClient.$disconnect();
  }

  prismaClient = undefined;
};
