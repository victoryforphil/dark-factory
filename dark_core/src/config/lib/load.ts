import { resolve } from 'node:path';

import { coreConfigEnvBindings, createCoreConfigSchema } from '../core.config';
import { isProductionEnvironment } from '../env.config';
import { envOverlayFromBindings } from './env';
import { deepMerge } from './merge';
import { readTomlFile, writeTomlFile } from './toml';
import type { AppConfig } from '../core.config';
import type { ConfigEnvironment } from './types';

export interface LoadConfigOptions {
  /** Path to TOML config file. Defaults to ./config.toml */
  path?: string;

  /** Enforce strict schema. Defaults to true in production. */
  strict?: boolean;

  /** Convenience inverse of strict. If set, it takes priority over strict. */
  allowUnknown?: boolean;

  /** Injected env map for tests or custom loaders. */
  env?: ConfigEnvironment;

  /** Base directory used to resolve the config path. */
  cwd?: string;
}

const DEFAULT_CONFIG_ROOT = resolve(import.meta.dir, '../../..');
export const DEFAULT_CONFIG_PATH = resolve(DEFAULT_CONFIG_ROOT, 'config.toml');

let configSingleton: AppConfig | undefined;

const resolveStrictMode = (options: LoadConfigOptions, env: ConfigEnvironment): boolean => {
  if (typeof options.allowUnknown === 'boolean') {
    return !options.allowUnknown;
  }

  if (typeof options.strict === 'boolean') {
    return options.strict;
  }

  return isProductionEnvironment(env);
};

const secretKeyPattern = /(password|token|secret|api[-_]?key|private[-_]?key)/i;

const redactValue = (value: unknown, key?: string): unknown => {
  if (key && secretKeyPattern.test(key)) {
    return '[REDACTED]';
  }

  if (Array.isArray(value)) {
    return value.map((item) => redactValue(item));
  }

  if (typeof value !== 'object' || value === null) {
    return value;
  }

  const objectValue = value as Record<string, unknown>;
  const result: Record<string, unknown> = {};

  for (const [childKey, childValue] of Object.entries(objectValue)) {
    result[childKey] = redactValue(childValue, childKey);
  }

  return result;
};

export const redactConfig = (config: AppConfig): Record<string, unknown> => {
  return redactValue(config) as Record<string, unknown>;
};

/** Build config from schema defaults only. */
export const createDefaultConfig = (): AppConfig => {
  return createCoreConfigSchema(true).parse({});
};

/**
 * Loads effective config using this precedence:
 * defaults -> TOML file -> explicit global env bindings.
 */
export const loadConfig = (options: LoadConfigOptions = {}): AppConfig => {
  const env = options.env ?? Bun.env;
  const strict = resolveStrictMode(options, env);
  const schema = createCoreConfigSchema(strict);

  const defaults = schema.parse({});
  const configPath = options.path
    ? resolve(options.cwd ?? process.cwd(), options.path)
    : DEFAULT_CONFIG_PATH;
  const fileConfig = readTomlFile(configPath);
  const explicitEnvOverlay = envOverlayFromBindings(env, coreConfigEnvBindings);

  const mergedConfig = deepMerge(defaults, fileConfig, explicitEnvOverlay);
  return schema.parse(mergedConfig);
};

export const getConfig = (options: LoadConfigOptions = {}): AppConfig => {
  if (configSingleton) {
    return configSingleton;
  }

  configSingleton = loadConfig(options);
  return configSingleton;
};

export const resetConfigCache = (): void => {
  configSingleton = undefined;
};

export const writeConfig = (path: string, config: AppConfig): void => {
  const validated = createCoreConfigSchema(true).parse(config);
  writeTomlFile(path, validated);
};

export const printConfig = (config: AppConfig): void => {
  console.info(JSON.stringify(redactConfig(config), null, 2));
};
