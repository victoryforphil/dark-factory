import { homedir } from "node:os";
import { join } from "node:path";

const readNumberEnv = (value: string | undefined, fallback: number): number => {
  if (!value) {
    return fallback;
  }

  const parsed = Number(value);
  return Number.isFinite(parsed) ? parsed : fallback;
};

const readOptionalStringEnv = (value: string | undefined): string | undefined => {
  const normalized = value?.trim();
  return normalized && normalized.length > 0 ? normalized : undefined;
};

const readOpenCodeConnectionModeEnv = (
  value: string | undefined,
): OpenCodeConnectionMode => {
  if (value === "managed") {
    return "managed";
  }

  return "attach";
};

export const config: CoreConfig = {
  env: process.env.NODE_ENV || "development",
  http: {
    address_listen: Bun.env.LISTEN_ADDRESS ?? "127.0.0.1",
    address_port: readNumberEnv(Bun.env.PORT, 4150),
  },
  store: {
    root_dir: Bun.env.DARKFACTORY_STORE_DIR ?? join(homedir(), ".darkfactory"),
    configs_dir_name: Bun.env.DARKFACTORY_STORE_CONFIGS_DIR ?? "configs",
    database_file_name: Bun.env.DARKFACTORY_STORE_DB_FILE ?? "core.duckdb",
  },
  opencode: {
    connection_mode: readOpenCodeConnectionModeEnv(
      Bun.env.DARKFACTORY_OPENCODE_CONNECTION_MODE,
    ),
    hostname: Bun.env.DARKFACTORY_OPENCODE_HOSTNAME ?? "127.0.0.1",
    port: readNumberEnv(Bun.env.DARKFACTORY_OPENCODE_PORT, 4096),
    timeout_ms: readNumberEnv(Bun.env.DARKFACTORY_OPENCODE_TIMEOUT_MS, 5000),
    base_url: readOptionalStringEnv(Bun.env.DARKFACTORY_OPENCODE_BASE_URL),
    directory: readOptionalStringEnv(Bun.env.DARKFACTORY_OPENCODE_DIRECTORY),
  },
};

export interface CoreConfig {
  env: string;
  http: HTTPConfig;
  store: StoreConfig;
  opencode: OpenCodeConfig;
}

export interface HTTPConfig {
  address_listen: string;
  address_port: number;
}

export interface StoreConfig {
  root_dir: string;
  configs_dir_name: string;
  database_file_name: string;
}

export type OpenCodeConnectionMode = "attach" | "managed";

export interface OpenCodeConfig {
  connection_mode: OpenCodeConnectionMode;
  hostname: string;
  port: number;
  timeout_ms: number;
  base_url?: string;
  directory?: string;
}
