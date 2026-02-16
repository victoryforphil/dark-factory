import { mkdirSync, existsSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname } from 'node:path';

import TOML from '@iarna/toml';

const asObject = (value: unknown): Record<string, unknown> => {
  if (typeof value !== 'object' || value === null || Array.isArray(value)) {
    throw new Error('TOML content must be an object at root');
  }

  return value as Record<string, unknown>;
};

/** Reads a TOML file. Missing files return an empty object. */
export const readTomlFile = (filePath: string): Record<string, unknown> => {
  if (!existsSync(filePath)) {
    return {};
  }

  try {
    const raw = readFileSync(filePath, 'utf8');
    const parsed = TOML.parse(raw);
    return asObject(parsed);
  } catch (error) {
    const reason = error instanceof Error ? error.message : String(error);
    throw new Error(`Config // TOML // Failed to read file (path=${filePath},reason=${reason})`);
  }
};

/** Converts an object to TOML text. */
export const toTomlString = (value: unknown): string => {
  return TOML.stringify(asObject(value));
};

/** Writes a TOML file, creating parent directories when needed. */
export const writeTomlFile = (filePath: string, value: unknown): void => {
  const document = asObject(value);

  try {
    mkdirSync(dirname(filePath), { recursive: true });
    writeFileSync(filePath, toTomlString(document), 'utf8');
  } catch (error) {
    const reason = error instanceof Error ? error.message : String(error);
    throw new Error(`Config // TOML // Failed to write file (path=${filePath},reason=${reason})`);
  }
};
