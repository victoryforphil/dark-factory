export interface ClientConfig {
  name: string;
  timeoutMs?: number;
}

export interface HttpClientConfig extends ClientConfig {
  baseUrl: string;
  headers?: Record<string, string>;
}
