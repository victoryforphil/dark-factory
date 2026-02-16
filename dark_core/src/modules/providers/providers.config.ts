import { z } from 'zod';

import type { ConfigSubsystemDefinition } from '../../config/lib/types';

/** Provider selection/runtime defaults for actor spawning. */
export const providersConfigSection = {
  shape: {
    defaultProvider: z.string().min(1).default('mock'),
    enabledProviders: z.array(z.string().min(1)).default(['mock', 'opencode/server']),
  },
  env: [{ path: 'providers.defaultProvider', env: 'DARKFACTORY_DEFAULT_PROVIDER' }],
} satisfies ConfigSubsystemDefinition;
