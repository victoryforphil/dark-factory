import { z } from 'zod';

import type { ConfigSubsystemDefinition } from '../../config/lib/types';

export const variantsConfigSection = {
  shape: {
    defaultWorkspaceLocator: z.string().min(1).nullable().default(null),
  },
  env: [
    {
      path: 'variants.defaultWorkspaceLocator',
      env: 'DARKFACTORY_VARIANTS_DEFAULT_WORKSPACE_LOCATOR',
    },
  ],
} satisfies ConfigSubsystemDefinition;
