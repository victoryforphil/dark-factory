import { z } from 'zod';

import { prismaConfigSection } from '../clients/prisma.config';
import { serverConfigSection } from '../controllers/system.config';
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
} as const;

/**
 * Global env bindings. Each binding points directly at the final config path.
 */
export const coreConfigEnvBindings: ReadonlyArray<EnvBinding> = [
  coreEnvironmentEnvBinding,
  ...coreConfigDefinition.server.env,
  ...coreConfigDefinition.prisma.env,
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
  };

  return strict ? z.object(rootShape).strict() : z.object(rootShape).passthrough();
};

const coreConfigTypeSchema = createCoreConfigSchema(true);

/** Fully resolved config type after defaults and validation. */
export type AppConfig = z.infer<typeof coreConfigTypeSchema>;
