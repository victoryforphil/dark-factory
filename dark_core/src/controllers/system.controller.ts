export interface ServiceInfo {
  name: string;
  version: string;
  env: string;
}

export const getHealth = async (): Promise<{ status: 'ok'; timestamp: string }> => {
  return {
    status: 'ok',
    timestamp: new Date().toISOString(),
  };
};

export const getApiInfo = async (): Promise<ServiceInfo> => {
  return {
    name: 'dark_core',
    version: Bun.env.npm_package_version ?? '0.0.0',
    env: Bun.env.NODE_ENV ?? 'development',
  };
};

export const getMetrics = async (): Promise<Record<string, number>> => {
  return {
    uptimeSeconds: Math.floor(process.uptime()),
  };
};
