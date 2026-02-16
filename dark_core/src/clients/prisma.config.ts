import { z } from 'zod';

import type { ConfigSubsystemDefinition } from '../config/lib/types';

/** Prisma config owned by the prisma client domain. */
export const prismaConfigSection = {
  shape: {
    logQueries: z.boolean().default(false),
  },
  env: [{ path: 'prisma.logQueries', env: 'DARKFACTORY_PRISMA_LOG_QUERIES' }],
} satisfies ConfigSubsystemDefinition;
