import { PrismaClient } from '../../../generated/prisma/client';
import { getConfig } from '../config';
import Log from '../utils/logging';

const prismaLogQueries = getConfig().prisma.logQueries;

export const prismaClient = new PrismaClient({
  log: prismaLogQueries ? ['query', 'info', 'warn', 'error'] : ['warn', 'error'],
});

Log.info(`Core // Client Prisma // Initialized (queryLogs=${prismaLogQueries})`);

export const getPrismaClient = (): PrismaClient => prismaClient;
