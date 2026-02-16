import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";
import TOML from "@iarna/toml";

export interface TomlSerde<T> {
  decode(value: unknown): T;
  encode(value: T): Record<string, unknown>;
}

const ensureParentDirectory = (filePath: string) => {
  mkdirSync(dirname(filePath), { recursive: true });
};

const parseTomlDocument = (filePath: string, text: string): unknown => {
  try {
    return TOML.parse(text);
  } catch (error) {
    const message = error instanceof Error ? error.message : "Unable to parse TOML";
    throw new Error(`Core // TOML // Parse failed (path=${filePath},reason=${message})`);
  }
};

const serializeTomlDocument = (filePath: string, document: Record<string, unknown>): string => {
  try {
    const serialized = TOML.stringify(document as any);
    return serialized.endsWith("\n") ? serialized : `${serialized}\n`;
  } catch (error) {
    const message = error instanceof Error ? error.message : "Unable to serialize TOML";
    throw new Error(`Core // TOML // Serialize failed (path=${filePath},reason=${message})`);
  }
};

export const readTomlConfig = <T>(filePath: string, serde: TomlSerde<T>): T => {
  if (!existsSync(filePath)) {
    throw new Error(`Core // TOML // Config not found (path=${filePath})`);
  }

  const text = readFileSync(filePath, "utf8");
  const document = parseTomlDocument(filePath, text);

  try {
    return serde.decode(document);
  } catch (error) {
    const message = error instanceof Error ? error.message : "Decode failed";
    throw new Error(`Core // TOML // Decode failed (path=${filePath},reason=${message})`);
  }
};

export const readTomlConfigIfExists = <T>(
  filePath: string,
  serde: TomlSerde<T>,
): T | undefined => {
  if (!existsSync(filePath)) {
    return undefined;
  }

  return readTomlConfig(filePath, serde);
};

export const writeTomlConfig = <T>(
  filePath: string,
  serde: TomlSerde<T>,
  value: T,
): void => {
  ensureParentDirectory(filePath);

  const document = serde.encode(value);
  const serialized = serializeTomlDocument(filePath, document);

  writeFileSync(filePath, serialized, "utf8");
};

export const updateTomlConfig = <T>(
  filePath: string,
  serde: TomlSerde<T>,
  mutator: (current: T) => T,
): T => {
  const current = readTomlConfig(filePath, serde);
  const next = mutator(current);
  writeTomlConfig(filePath, serde, next);
  return next;
};

export const expectTomlTable = (
  value: unknown,
  path: string,
): Record<string, unknown> => {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new Error(`Expected TOML table at ${path}`);
  }

  return value as Record<string, unknown>;
};

export function readTomlString(
  table: Record<string, unknown>,
  key: string,
): string;
export function readTomlString(
  table: Record<string, unknown>,
  key: string,
  options: { optional: true },
): string | undefined;
export function readTomlString(
  table: Record<string, unknown>,
  key: string,
  options?: { optional?: boolean },
): string | undefined {
  const value = table[key];

  if (value === undefined || value === null) {
    if (options?.optional) {
      return undefined;
    }

    throw new Error(`Expected TOML string at ${key}`);
  }

  if (typeof value !== "string") {
    throw new Error(`Expected TOML string at ${key}`);
  }

  return value;
}

export function readTomlNumber(
  table: Record<string, unknown>,
  key: string,
): number;
export function readTomlNumber(
  table: Record<string, unknown>,
  key: string,
  options: { optional: true },
): number | undefined;
export function readTomlNumber(
  table: Record<string, unknown>,
  key: string,
  options?: { optional?: boolean },
): number | undefined {
  const value = table[key];

  if (value === undefined || value === null) {
    if (options?.optional) {
      return undefined;
    }

    throw new Error(`Expected TOML number at ${key}`);
  }

  if (typeof value !== "number" || Number.isNaN(value)) {
    throw new Error(`Expected TOML number at ${key}`);
  }

  return value;
}

export function readTomlBoolean(
  table: Record<string, unknown>,
  key: string,
): boolean;
export function readTomlBoolean(
  table: Record<string, unknown>,
  key: string,
  options: { optional: true },
): boolean | undefined;
export function readTomlBoolean(
  table: Record<string, unknown>,
  key: string,
  options?: { optional?: boolean },
): boolean | undefined {
  const value = table[key];

  if (value === undefined || value === null) {
    if (options?.optional) {
      return undefined;
    }

    throw new Error(`Expected TOML boolean at ${key}`);
  }

  if (typeof value !== "boolean") {
    throw new Error(`Expected TOML boolean at ${key}`);
  }

  return value;
}
