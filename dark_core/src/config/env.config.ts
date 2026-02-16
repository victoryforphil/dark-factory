import { z } from 'zod';

import type { ConfigEnvironment, ConfigValueDefinition, EnvBinding } from './lib/types';

/** Root `env` field definition and its canonical env var name. */
export const coreEnvironmentConfigValue = {
  schema: z.string().min(1).default(Bun.env.NODE_ENV ?? 'development'),
  env: 'DARKFACTORY_ENV',
} satisfies ConfigValueDefinition;

/** Explicit env binding for the root `env` field. */
export const coreEnvironmentEnvBinding: EnvBinding = {
  path: 'env',
  env: coreEnvironmentConfigValue.env ?? 'DARKFACTORY_ENV',
};

/** Resolves runtime environment from explicit config env first, then NODE_ENV. */
export const resolveRuntimeEnvironment = (env: ConfigEnvironment): string => {
  return env.DARKFACTORY_ENV ?? env.NODE_ENV ?? Bun.env.NODE_ENV ?? 'development';
};

/** True when runtime environment should be treated as production. */
export const isProductionEnvironment = (env: ConfigEnvironment): boolean => {
  return resolveRuntimeEnvironment(env) === 'production';
};
