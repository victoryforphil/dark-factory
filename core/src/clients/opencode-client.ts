import {
  createOpencodeClient,
  createOpencodeServer,
  type Config as OpenCodeRuntimeConfig,
  type OpencodeClient,
  type OpencodeClientConfig,
  type ServerOptions,
} from "@opencode-ai/sdk";

import {
  ensureOpenCodeConfig,
  readOpenCodeConfig,
  resolveOpenCodeBaseUrl,
  type OpenCodeStoreConfig,
} from "../helpers/opencode-config";
import { logger } from "../logging";

const defaultClientConfig: Omit<OpencodeClientConfig, "baseUrl" | "directory"> = {
  throwOnError: true,
  responseStyle: "data",
};

const mergeOpenCodeConfig = (
  base: OpenCodeStoreConfig,
  overrides?: Partial<OpenCodeStoreConfig>,
): OpenCodeStoreConfig => {
  return {
    schemaVersion: base.schemaVersion,
    connectionMode: overrides?.connectionMode ?? base.connectionMode,
    hostname: overrides?.hostname ?? base.hostname,
    port: overrides?.port ?? base.port,
    timeoutMs: overrides?.timeoutMs ?? base.timeoutMs,
    baseUrl: overrides?.baseUrl === undefined ? base.baseUrl : overrides.baseUrl,
    directory: overrides?.directory === undefined ? base.directory : overrides.directory,
  };
};

type SessionListOptions = Parameters<OpencodeClient["session"]["list"]>[0];
type SessionCreateOptions = Parameters<OpencodeClient["session"]["create"]>[0];
type SessionPromptOptions = Parameters<OpencodeClient["session"]["prompt"]>[0];
type SessionAbortOptions = Parameters<OpencodeClient["session"]["abort"]>[0];
type EventSubscribeOptions = Parameters<OpencodeClient["event"]["subscribe"]>[0];

export interface OpenCodeClientOptions {
  config?: Partial<OpenCodeStoreConfig>;
  runtimeConfig?: OpenCodeRuntimeConfig;
  clientConfig?: Omit<OpencodeClientConfig, "baseUrl" | "directory">;
}

const toErrorMessage = (error: unknown): string => {
  return error instanceof Error ? error.message : String(error);
};

export class OpenCodeClient {
  private readonly config: OpenCodeStoreConfig;
  private readonly runtimeConfig: OpenCodeRuntimeConfig | undefined;
  private readonly clientConfig: Omit<OpencodeClientConfig, "baseUrl" | "directory">;

  private clientInstance: OpencodeClient | undefined;
  private closeServer: (() => void) | undefined;

  constructor(options?: OpenCodeClientOptions) {
    this.config = mergeOpenCodeConfig(readOpenCodeConfig(), options?.config);
    this.runtimeConfig = options?.runtimeConfig;
    this.clientConfig = {
      ...defaultClientConfig,
      ...options?.clientConfig,
    };
  }

  getConfig(): OpenCodeStoreConfig {
    return { ...this.config };
  }

  ensureConfig(): OpenCodeStoreConfig {
    return ensureOpenCodeConfig();
  }

  async connect(): Promise<OpencodeClient> {
    if (this.clientInstance) {
      return this.clientInstance;
    }

    try {
      if (this.config.connectionMode === "managed") {
        await this.connectManaged();
      } else {
        this.connectAttached();
      }
    } catch (error) {
      throw new Error(
        `Core // OpenCode // Connect failed (mode=${this.config.connectionMode},reason=${toErrorMessage(error)})`,
      );
    }

    return this.clientInstance!;
  }

  async use<T>(action: (client: OpencodeClient) => Promise<T>): Promise<T> {
    const client = await this.connect();
    return action(client);
  }

  async verifyConnection(): Promise<void> {
    const client = await this.connect();
    await client.path.get({
      throwOnError: true,
      responseStyle: "data",
    });
  }

  async listSessions(options?: SessionListOptions) {
    const client = await this.connect();
    return client.session.list(options);
  }

  async createSession(options?: SessionCreateOptions) {
    const client = await this.connect();
    return client.session.create(options);
  }

  async promptSession(options: SessionPromptOptions) {
    const client = await this.connect();
    return client.session.prompt(options);
  }

  async abortSession(options: SessionAbortOptions) {
    const client = await this.connect();
    return client.session.abort(options);
  }

  async subscribeEvents(options?: EventSubscribeOptions) {
    const client = await this.connect();
    return client.event.subscribe(options);
  }

  async close(): Promise<void> {
    this.closeServer?.();
    this.closeServer = undefined;
    this.clientInstance = undefined;
  }

  private connectAttached(): void {
    const baseUrl = resolveOpenCodeBaseUrl(this.config);
    this.clientInstance = this.createClient(baseUrl);

    logger.info(`Core // OpenCode // Client connected (mode=attach,base_url=${baseUrl})`);
  }

  private async connectManaged(): Promise<void> {
    const serverOptions: ServerOptions = {
      hostname: this.config.hostname,
      port: this.config.port,
      timeout: this.config.timeoutMs,
      config: this.runtimeConfig,
    };

    const server = await createOpencodeServer(serverOptions);

    this.closeServer = () => {
      server.close();
    };

    this.clientInstance = this.createClient(server.url);

    logger.info(`Core // OpenCode // Client connected (mode=managed,base_url=${server.url})`);
  }

  private createClient(baseUrl: string): OpencodeClient {
    return createOpencodeClient({
      ...this.clientConfig,
      baseUrl,
      directory: this.config.directory,
    });
  }
}

export const openCodeClient = new OpenCodeClient();
