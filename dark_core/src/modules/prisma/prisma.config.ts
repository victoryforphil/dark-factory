import { z } from 'zod';

import type { ConfigSubsystemDefinition } from '../../config/lib/types';

/** Prisma config owned by the prisma client domain. */
export const prismaConfigSection = {
  shape: {
    databaseUrl: z.string().min(1).default('file:../.darkfactory/darkfactory.dev.db'),
    logQueries: z.boolean().default(false),
  },
  env: [
    { path: 'prisma.databaseUrl', env: 'DARKFACTORY_PRISMA_DATABASE_URL' },
    { path: 'prisma.logQueries', env: 'DARKFACTORY_PRISMA_LOG_QUERIES' },
  ],
} satisfies ConfigSubsystemDefinition;
