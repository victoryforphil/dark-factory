import { config, type OpenCodeConnectionMode } from "../config";
import { storeManager } from "./store-manager";
import {
  expectTomlTable,
  readTomlNumber,
  readTomlString,
  type TomlSerde,
} from "./toml-serde";

const opencodeConfigName = "opencode";
const opencodeConfigSchemaVersion = 1;

const opencodeConnectionModes = new Set<OpenCodeConnectionMode>([
  "attach",
  "managed",
]);

export interface OpenCodeStoreConfig {
  schemaVersion: number;
  connectionMode: OpenCodeConnectionMode;
  hostname: string;
  port: number;
  timeoutMs: number;
  baseUrl?: string;
  directory?: string;
}

const normalizeOptionalString = (value: string | undefined): string | undefined => {
  const normalized = value?.trim();
  return normalized && normalized.length > 0 ? normalized : undefined;
};

const assertNonEmptyString = (value: string, field: string): string => {
  const normalized = value.trim();
  if (normalized.length === 0) {
    throw new Error(`Core // OpenCode Config // Invalid value (field=${field},reason=empty)`);
  }

  return normalized;
};

const assertPositiveInteger = (value: number, field: string): number => {
  if (!Number.isInteger(value) || value <= 0) {
    throw new Error(
      `Core // OpenCode Config // Invalid value (field=${field},value=${String(value)})`,
    );
  }

  return value;
};

const assertSchemaVersion = (value: number): number => {
  const schemaVersion = assertPositiveInteger(value, "schemaVersion");

  if (schemaVersion !== opencodeConfigSchemaVersion) {
    throw new Error(
      `Core // OpenCode Config // Invalid value (field=schemaVersion,value=${schemaVersion})`,
    );
  }

  return schemaVersion;
};

const assertConnectionMode = (
  value: string,
  field: string,
): OpenCodeConnectionMode => {
  if (!opencodeConnectionModes.has(value as OpenCodeConnectionMode)) {
    throw new Error(`Core // OpenCode Config // Invalid value (field=${field},value=${value})`);
  }

  return value as OpenCodeConnectionMode;
};

const assertBaseUrl = (value: string | undefined): string | undefined => {
  const normalized = normalizeOptionalString(value);
  if (!normalized) {
    return undefined;
  }

  try {
    const parsed = new URL(normalized);
    return parsed.toString().replace(/\/$/, "");
  } catch {
    throw new Error(
      `Core // OpenCode Config // Invalid value (field=baseUrl,value=${normalized})`,
    );
  }
};

const normalizeOpenCodeConfig = (
  value: OpenCodeStoreConfig,
): OpenCodeStoreConfig => {
  return {
    schemaVersion: assertSchemaVersion(value.schemaVersion),
    connectionMode: assertConnectionMode(value.connectionMode, "connectionMode"),
    hostname: assertNonEmptyString(value.hostname, "hostname"),
    port: assertPositiveInteger(value.port, "port"),
    timeoutMs: assertPositiveInteger(value.timeoutMs, "timeoutMs"),
    baseUrl: assertBaseUrl(value.baseUrl),
    directory: normalizeOptionalString(value.directory),
  };
};

const openCodeConfigSerde: TomlSerde<OpenCodeStoreConfig> = {
  decode(value: unknown): OpenCodeStoreConfig {
    const table = expectTomlTable(value, "opencode");

    const schemaVersion =
      readTomlNumber(table, "schema_version", { optional: true }) ??
      opencodeConfigSchemaVersion;
    const connectionMode =
      readTomlString(table, "connection_mode", { optional: true }) ??
      config.opencode.connection_mode;
    const hostname =
      readTomlString(table, "hostname", { optional: true }) ?? config.opencode.hostname;
    const port =
      readTomlNumber(table, "port", { optional: true }) ?? config.opencode.port;
    const timeoutMs =
      readTomlNumber(table, "timeout_ms", { optional: true }) ??
      config.opencode.timeout_ms;
    const baseUrl = readTomlString(table, "base_url", { optional: true });
    const directory = readTomlString(table, "directory", { optional: true });

    return normalizeOpenCodeConfig({
      schemaVersion,
      connectionMode: assertConnectionMode(connectionMode, "connection_mode"),
      hostname,
      port,
      timeoutMs,
      baseUrl,
      directory,
    });
  },
  encode(value: OpenCodeStoreConfig): Record<string, unknown> {
    const normalized = normalizeOpenCodeConfig(value);

    const document: Record<string, unknown> = {
      schema_version: normalized.schemaVersion,
      connection_mode: normalized.connectionMode,
      hostname: normalized.hostname,
      port: normalized.port,
      timeout_ms: normalized.timeoutMs,
    };

    if (normalized.baseUrl) {
      document.base_url = normalized.baseUrl;
    }

    if (normalized.directory) {
      document.directory = normalized.directory;
    }

    return document;
  },
};

export const createDefaultOpenCodeConfig = (): OpenCodeStoreConfig => {
  return normalizeOpenCodeConfig({
    schemaVersion: opencodeConfigSchemaVersion,
    connectionMode: config.opencode.connection_mode,
    hostname: config.opencode.hostname,
    port: config.opencode.port,
    timeoutMs: config.opencode.timeout_ms,
    baseUrl: config.opencode.base_url,
    directory: config.opencode.directory,
  });
};

export const readOpenCodeConfig = (): OpenCodeStoreConfig => {
  const persisted = storeManager.readConfig(opencodeConfigName, openCodeConfigSerde);
  if (persisted) {
    return persisted;
  }

  return createDefaultOpenCodeConfig();
};

export const ensureOpenCodeConfig = (): OpenCodeStoreConfig => {
  storeManager.ensureReady();

  const persisted = storeManager.readConfig(opencodeConfigName, openCodeConfigSerde);
  if (persisted) {
    return persisted;
  }

  return storeManager.writeConfig(
    opencodeConfigName,
    openCodeConfigSerde,
    createDefaultOpenCodeConfig(),
  );
};

export const writeOpenCodeConfig = (
  value: OpenCodeStoreConfig,
): OpenCodeStoreConfig => {
  storeManager.ensureReady();

  const normalized = normalizeOpenCodeConfig(value);
  return storeManager.writeConfig(opencodeConfigName, openCodeConfigSerde, normalized);
};

export const updateOpenCodeConfig = (
  mutator: (current: OpenCodeStoreConfig) => OpenCodeStoreConfig,
): OpenCodeStoreConfig => {
  const current = ensureOpenCodeConfig();
  const next = normalizeOpenCodeConfig(mutator(current));
  return writeOpenCodeConfig(next);
};

export const resolveOpenCodeBaseUrl = (
  value: Pick<OpenCodeStoreConfig, "baseUrl" | "hostname" | "port">,
): string => {
  if (value.baseUrl) {
    return assertBaseUrl(value.baseUrl)!;
  }

  return `http://${value.hostname}:${value.port}`;
};
