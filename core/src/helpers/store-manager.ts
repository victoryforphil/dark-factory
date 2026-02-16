import { mkdirSync } from "node:fs";
import { join } from "node:path";

import { config } from "../config";
import {
  expectTomlTable,
  readTomlConfigIfExists,
  readTomlNumber,
  readTomlString,
  TomlSerde,
  updateTomlConfig,
  writeTomlConfig,
} from "./toml-serde";

const storeManifestFileName = "store.toml";

export interface StoreManifest {
  schemaVersion: number;
  databaseFileName: string;
  configsDirectoryName: string;
  createdAt: string;
  updatedAt: string;
}

export interface StorePaths {
  rootDir: string;
  configsDir: string;
  databaseFilePath: string;
  manifestFilePath: string;
}

const storeManifestSerde: TomlSerde<StoreManifest> = {
  decode(value: unknown): StoreManifest {
    const table = expectTomlTable(value, "store");

    const schemaVersion = readTomlNumber(table, "schema_version");
    const databaseFileName = readTomlString(table, "database_file_name");
    const configsDirectoryName = readTomlString(table, "configs_directory_name");
    const createdAt = readTomlString(table, "created_at");
    const updatedAt = readTomlString(table, "updated_at");

    return {
      schemaVersion,
      databaseFileName,
      configsDirectoryName,
      createdAt,
      updatedAt,
    };
  },
  encode(value: StoreManifest): Record<string, unknown> {
    return {
      schema_version: value.schemaVersion,
      database_file_name: value.databaseFileName,
      configs_directory_name: value.configsDirectoryName,
      created_at: value.createdAt,
      updated_at: value.updatedAt,
    };
  },
};

interface StoreManagerOptions {
  rootDir?: string;
  configsDirectoryName?: string;
  databaseFileName?: string;
}

const nowIsoString = () => new Date().toISOString();

const assertConfigName = (configName: string): void => {
  if (configName.trim().length === 0) {
    throw new Error("Core // Store // Config name must not be empty");
  }

  if (configName.includes("/") || configName.includes("\\") || configName.includes("..")) {
    throw new Error(`Core // Store // Invalid config name (name=${configName})`);
  }
};

export class StoreManager {
  private readonly rootDir: string;
  private readonly configsDirectoryName: string;
  private readonly databaseFileName: string;

  constructor(options?: StoreManagerOptions) {
    this.rootDir = options?.rootDir ?? config.store.root_dir;
    this.configsDirectoryName =
      options?.configsDirectoryName ?? config.store.configs_dir_name;
    this.databaseFileName = options?.databaseFileName ?? config.store.database_file_name;
  }

  locate(): StorePaths {
    const configsDir = join(this.rootDir, this.configsDirectoryName);

    return {
      rootDir: this.rootDir,
      configsDir,
      databaseFilePath: join(this.rootDir, this.databaseFileName),
      manifestFilePath: join(this.rootDir, storeManifestFileName),
    };
  }

  create(): StorePaths {
    const paths = this.locate();

    mkdirSync(paths.rootDir, { recursive: true });
    mkdirSync(paths.configsDir, { recursive: true });

    return paths;
  }

  ensure(): StorePaths {
    return this.create();
  }

  readManifest(): StoreManifest | undefined {
    const { manifestFilePath } = this.locate();
    return readTomlConfigIfExists(manifestFilePath, storeManifestSerde);
  }

  writeManifest(manifest: StoreManifest): StoreManifest {
    const { manifestFilePath } = this.locate();
    writeTomlConfig(manifestFilePath, storeManifestSerde, manifest);
    return manifest;
  }

  modifyManifest(mutator: (current: StoreManifest) => StoreManifest): StoreManifest {
    this.ensureReady();
    const { manifestFilePath } = this.locate();
    return updateTomlConfig(manifestFilePath, storeManifestSerde, (current) => {
      const next = mutator(current);
      return {
        ...next,
        updatedAt: nowIsoString(),
      };
    });
  }

  ensureManifest(): StoreManifest {
    const existing = this.readManifest();
    if (existing) {
      return existing;
    }

    const createdAt = nowIsoString();

    const manifest: StoreManifest = {
      schemaVersion: 1,
      databaseFileName: this.databaseFileName,
      configsDirectoryName: this.configsDirectoryName,
      createdAt,
      updatedAt: createdAt,
    };

    return this.writeManifest(manifest);
  }

  ensureReady(): StorePaths {
    const paths = this.ensure();
    this.ensureManifest();
    return paths;
  }

  resolveConfigPath(configName: string): string {
    assertConfigName(configName);

    const normalizedName = configName.endsWith(".toml") ? configName : `${configName}.toml`;
    return join(this.locate().configsDir, normalizedName);
  }

  readConfig<T>(configName: string, serde: TomlSerde<T>): T | undefined {
    this.ensureReady();
    const configPath = this.resolveConfigPath(configName);
    return readTomlConfigIfExists(configPath, serde);
  }

  writeConfig<T>(configName: string, serde: TomlSerde<T>, value: T): T {
    this.ensureReady();
    const configPath = this.resolveConfigPath(configName);
    writeTomlConfig(configPath, serde, value);
    this.modifyManifest((manifest) => manifest);
    return value;
  }

  modifyConfig<T>(
    configName: string,
    serde: TomlSerde<T>,
    mutator: (current: T) => T,
  ): T {
    this.ensureReady();
    const configPath = this.resolveConfigPath(configName);
    const next = updateTomlConfig(configPath, serde, mutator);
    this.modifyManifest((manifest) => manifest);
    return next;
  }
}

export const storeManager = new StoreManager();
