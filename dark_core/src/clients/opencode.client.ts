import { HttpClient } from './http.client';
import type { HttpClientConfig } from './types';

export interface OpenCodeClientConfig extends HttpClientConfig {
  token?: string;
  transport?: 'http' | 'sdk';
}

export class OpenCodeClient {
  readonly name: string;
  private readonly config: OpenCodeClientConfig;
  private readonly httpClient: HttpClient;

  constructor(config: OpenCodeClientConfig) {
    this.config = {
      ...config,
      transport: config.transport ?? 'http',
    };

    this.name = config.name;

    this.httpClient = new HttpClient({
      ...this.config,
      headers: {
        ...(this.config.headers ?? {}),
        ...(this.config.token ? { Authorization: `Bearer ${this.config.token}` } : {}),
      },
    });
  }

  async ping(): Promise<{ ok: boolean }> {
    return this.httpClient.request<{ ok: boolean }>('/health');
  }
}
