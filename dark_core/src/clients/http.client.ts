import type { HttpClientConfig } from './types';

export class HttpClient {
  private readonly config: HttpClientConfig;

  constructor(config: HttpClientConfig) {
    this.config = config;
  }

  async request<TResponse>(path: string, init: RequestInit = {}): Promise<TResponse> {
    const timeoutMs = this.config.timeoutMs ?? 5000;
    const controller = new AbortController();
    const timeout = setTimeout(() => controller.abort(), timeoutMs);

    try {
      const response = await fetch(`${this.config.baseUrl}${path}`, {
        ...init,
        headers: {
          ...this.config.headers,
          ...(init.headers ?? {}),
        },
        signal: controller.signal,
      });

      if (!response.ok) {
        throw new Error(`HTTP request failed (status=${response.status},path=${path})`);
      }

      return (await response.json()) as TResponse;
    } finally {
      clearTimeout(timeout);
    }
  }
}
