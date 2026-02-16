import { PrismaClient } from '../../../generated/prisma/client';
import { PrismaLibSql } from '@prisma/adapter-libsql';

import { getConfig } from '../config';
import Log from '../utils/logging';

const prismaDatabaseUrl = getConfig().prisma.databaseUrl;
const prismaLogQueries = getConfig().prisma.logQueries;
const prismaAdapter = new PrismaLibSql({ url: prismaDatabaseUrl });

export const prismaClient = new PrismaClient({
  adapter: prismaAdapter,
  log: prismaLogQueries ? ['query', 'info', 'warn', 'error'] : ['warn', 'error'],
});

let prismaSchemaReady: Promise<void> | undefined;

export const ensurePrismaSchema = async (): Promise<void> => {
  if (!prismaSchemaReady) {
    prismaSchemaReady = (async () => {
      await prismaClient.$executeRawUnsafe('PRAGMA foreign_keys = ON');

      await prismaClient.$executeRawUnsafe(`
        CREATE TABLE IF NOT EXISTS "products" (
          "id" TEXT NOT NULL PRIMARY KEY,
          "locator" TEXT NOT NULL,
          "display_name" TEXT,
          "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
          "updated_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
      `);

      await prismaClient.$executeRawUnsafe(
        'CREATE UNIQUE INDEX IF NOT EXISTS "products_locator_key" ON "products"("locator")',
      );

      await prismaClient.$executeRawUnsafe(
        'CREATE INDEX IF NOT EXISTS "products_locator_idx" ON "products"("locator")',
      );

      await prismaClient.$executeRawUnsafe(`
        CREATE TABLE IF NOT EXISTS "variants" (
          "id" TEXT NOT NULL PRIMARY KEY,
          "product_id" TEXT NOT NULL,
          "name" TEXT NOT NULL DEFAULT 'default',
          "locator" TEXT NOT NULL,
          "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
          "updated_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
          CONSTRAINT "variants_product_id_fkey"
            FOREIGN KEY ("product_id")
            REFERENCES "products"("id")
            ON DELETE CASCADE
            ON UPDATE CASCADE
        )
      `);

      await prismaClient.$executeRawUnsafe(
        'CREATE UNIQUE INDEX IF NOT EXISTS "variants_locator_key" ON "variants"("locator")',
      );

      await prismaClient.$executeRawUnsafe(
        'CREATE UNIQUE INDEX IF NOT EXISTS "variants_product_id_name_key" ON "variants"("product_id", "name")',
      );

      await prismaClient.$executeRawUnsafe(
        'CREATE INDEX IF NOT EXISTS "variants_product_id_idx" ON "variants"("product_id")',
      );

      await prismaClient.$executeRawUnsafe(
        'CREATE INDEX IF NOT EXISTS "variants_locator_idx" ON "variants"("locator")',
      );

      Log.info('Core // Client Prisma // Schema Ready (tables=products,variants)');
    })().catch((error) => {
      prismaSchemaReady = undefined;
      throw error;
    });
  }

  await prismaSchemaReady;
};

Log.info(`Core // Client Prisma // Initialized (db=sqlite,queryLogs=${prismaLogQueries})`);

export const getPrismaClient = (): PrismaClient => prismaClient;
