import { z } from 'zod';

import { opencodeServerConfigSection } from '../modules/providers/opencode_server/opencode_server.config';
import { providersConfigSection } from '../modules/providers/providers.config';
import { prismaConfigSection } from '../modules/prisma/prisma.config';
import { sshConfigSection } from '../modules/ssh/ssh.config';
import { serverConfigSection } from '../modules/system/system.config';
import { variantsConfigSection } from '../modules/variants/variants.config';
import { coreEnvironmentConfigValue, coreEnvironmentEnvBinding } from './env.config';
import { createSubsystemSchema } from './lib/subsystem';
import type { EnvBinding } from './lib/types';

/**
 * Human-owned root config definition.
 *
 * Keep this file small and obvious: each key is a top-level config section,
 * and each section points to a local config definition owned by that domain.
 */
export const coreConfigDefinition = {
  /** Runtime environment (see env.config.ts). */
  env: coreEnvironmentConfigValue,

  /** HTTP server runtime settings. */
  server: serverConfigSection,

  /** Prisma runtime settings. */
  prisma: prismaConfigSection,

  /** OpenCode SDK/server runtime settings. */
  opencode: opencodeServerConfigSection,

  /** Provider enablement/default selection for actors. */
  providers: providersConfigSection,

  /** Variant runtime defaults. */
  variants: variantsConfigSection,

  /** SSH host + tunnel presets. */
  ssh: sshConfigSection,
} as const;

/**
 * Global env bindings. Each binding points directly at the final config path.
 */
export const coreConfigEnvBindings: ReadonlyArray<EnvBinding> = [
  coreEnvironmentEnvBinding,
  ...coreConfigDefinition.server.env,
  ...coreConfigDefinition.prisma.env,
  ...coreConfigDefinition.opencode.env,
  ...coreConfigDefinition.providers.env,
  ...coreConfigDefinition.variants.env,
  ...coreConfigDefinition.ssh.env,
];

/**
 * Creates the runtime Zod schema from the root definition.
 * strict=true rejects unknown keys; strict=false allows passthrough.
 */
export const createCoreConfigSchema = (strict: boolean) => {
  const rootShape = {
    env: coreConfigDefinition.env.schema,
    server: createSubsystemSchema(coreConfigDefinition.server, strict),
    prisma: createSubsystemSchema(coreConfigDefinition.prisma, strict),
    opencode: createSubsystemSchema(coreConfigDefinition.opencode, strict),
    providers: createSubsystemSchema(coreConfigDefinition.providers, strict),
    variants: createSubsystemSchema(coreConfigDefinition.variants, strict),
    ssh: createSubsystemSchema(coreConfigDefinition.ssh, strict),
  };

  return strict ? z.object(rootShape).strict() : z.object(rootShape).passthrough();
};

const coreConfigTypeSchema = createCoreConfigSchema(true);

/** Fully resolved config type after defaults and validation. */
export type AppConfig = z.infer<typeof coreConfigTypeSchema>;
