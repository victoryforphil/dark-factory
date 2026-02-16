import { DuckDBInstance } from "@duckdb/node-api";

import { storeManager } from "../helpers/store-manager";
import { logger } from "../logging";

type DuckDbBindings = Record<string, unknown>;

export interface DuckDbExecutor {
  run(sql: string, bindings?: DuckDbBindings): Promise<void>;
  queryRows<T>(sql: string, bindings?: DuckDbBindings): Promise<T[]>;
  queryFirst<T>(sql: string, bindings?: DuckDbBindings): Promise<T | undefined>;
}

const toRows = async <T>(reader: any): Promise<T[]> => {
  if (!reader) {
    return [];
  }

  if (typeof reader.getRowObjectsJson === "function") {
    const rows = reader.getRowObjectsJson();
    return rows instanceof Promise ? await rows : (rows as T[]);
  }

  if (typeof reader.getRowObjects === "function") {
    const rows = reader.getRowObjects();
    return rows instanceof Promise ? await rows : (rows as T[]);
  }

  return [];
};

export class DuckDbClient implements DuckDbExecutor {
  private instance: any | undefined;
  private connection: any | undefined;
  private initializePromise: Promise<void> | undefined;
  private operationQueue: Promise<void> = Promise.resolve();

  async initialize(): Promise<void> {
    if (this.initializePromise) {
      return this.initializePromise;
    }

    this.initializePromise = (async () => {
      const paths = storeManager.ensureReady();

      this.instance = await DuckDBInstance.fromCache(paths.databaseFilePath);
      this.connection = await this.instance.connect();

      await this.connection.run("SET autoload_known_extensions = false");
      await this.connection.run("SET autoinstall_known_extensions = false");

      logger.info(
        `Core // DuckDB // Connected (path=${paths.databaseFilePath},root=${paths.rootDir})`,
      );
    })();

    return this.initializePromise;
  }

  async run(sql: string, bindings?: DuckDbBindings): Promise<void> {
    await this.executeSerial(async () => {
      await this.runUnlocked(sql, bindings);
    });
  }

  async queryRows<T>(sql: string, bindings?: DuckDbBindings): Promise<T[]> {
    return this.executeSerial(async () => {
      return this.queryRowsUnlocked<T>(sql, bindings);
    });
  }

  async queryFirst<T>(sql: string, bindings?: DuckDbBindings): Promise<T | undefined> {
    const rows = await this.queryRows<T>(sql, bindings);
    return rows[0];
  }

  async transaction<T>(operation: (tx: DuckDbExecutor) => Promise<T>): Promise<T> {
    return this.executeSerial(async () => {
      await this.runUnlocked("BEGIN TRANSACTION");

      const tx: DuckDbExecutor = {
        run: async (sql: string, bindings?: DuckDbBindings) => {
          await this.runUnlocked(sql, bindings);
        },
        queryRows: async <TRow>(sql: string, bindings?: DuckDbBindings) => {
          return this.queryRowsUnlocked<TRow>(sql, bindings);
        },
        queryFirst: async <TRow>(sql: string, bindings?: DuckDbBindings) => {
          const rows = await this.queryRowsUnlocked<TRow>(sql, bindings);
          return rows[0];
        },
      };

      try {
        const result = await operation(tx);
        await this.runUnlocked("COMMIT");
        return result;
      } catch (error) {
        try {
          await this.runUnlocked("ROLLBACK");
        } catch {
          // Ignore rollback failures; preserve the original error.
        }

        throw error;
      }
    });
  }

  async close(): Promise<void> {
    if (!this.connection) {
      return;
    }

    if (typeof this.connection.disconnectSync === "function") {
      this.connection.disconnectSync();
    }

    if (typeof this.connection.closeSync === "function") {
      this.connection.closeSync();
    }

    this.connection = undefined;
    this.instance = undefined;
    this.initializePromise = undefined;
  }

  private async executeSerial<T>(operation: () => Promise<T>): Promise<T> {
    const execution = this.operationQueue.then(operation, operation);
    this.operationQueue = execution.then(
      () => undefined,
      () => undefined,
    );

    return execution;
  }

  private async ensureConnection(): Promise<any> {
    await this.initialize();

    if (!this.connection) {
      throw new Error("Core // DuckDB // Connection not initialized");
    }

    return this.connection;
  }

  private async runUnlocked(sql: string, bindings?: DuckDbBindings): Promise<void> {
    const connection = await this.ensureConnection();

    if (!bindings || Object.keys(bindings).length === 0) {
      await connection.run(sql);
      return;
    }

    await connection.run(sql, bindings);
  }

  private async queryRowsUnlocked<T>(sql: string, bindings?: DuckDbBindings): Promise<T[]> {
    const connection = await this.ensureConnection();

    const reader = !bindings || Object.keys(bindings).length === 0
      ? await connection.runAndReadAll(sql)
      : await connection.runAndReadAll(sql, bindings);

    return toRows<T>(reader);
  }
}

export const duckDbClient = new DuckDbClient();
