import { PrismaClient } from '../../../generated/prisma/client';

import type { ClientConfig } from './types';

export interface PrismaDbClientConfig extends ClientConfig {
  logQueries?: boolean;
}

const DEFAULT_CONFIG: PrismaDbClientConfig = {
  name: 'prisma',
  logQueries: false,
};

export class PrismaDbClient {
  readonly name: string;
  private readonly client: PrismaClient;

  constructor(config: PrismaDbClientConfig = DEFAULT_CONFIG) {
    this.name = config.name;
    this.client = new PrismaClient({
      log: config.logQueries ? ['query', 'info', 'warn', 'error'] : ['warn', 'error'],
    });
  }

  get raw(): PrismaClient {
    return this.client;
  }
}

const prismaDbClient = new PrismaDbClient();

export const prismaClient = prismaDbClient.raw;

export const getPrismaClient = (): PrismaClient => prismaDbClient.raw;
