export const config: CoreConfig = {
  env: process.env.NODE_ENV || "development",
  http: {
    address_listen: Bun.env.LISTEN_ADDRESS ?? "127.0.0.1",
    address_port: Number(Bun.env.PORT ?? 4150),
  },
};

export interface CoreConfig {
  env: string;
  http: HTTPConfig;
}

export interface HTTPConfig {
  address_listen: string;
  address_port: number;
}
