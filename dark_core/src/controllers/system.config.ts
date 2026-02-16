import { z } from 'zod';

import type { ConfigSubsystemDefinition } from '../config/lib/types';

/** Server config owned by the system controller domain. */
export const serverConfigSection = {
  shape: {
    listenHost: z.string().min(1).default('127.0.0.1'),
    listenPort: z.number().int().min(1).max(65_535).default(4150),
  },
  env: [
    { path: 'server.listenHost', env: 'DARKFACTORY_SERVER_LISTEN_HOST' },
    { path: 'server.listenPort', env: 'DARKFACTORY_SERVER_PORT' },
  ],
} satisfies ConfigSubsystemDefinition;
