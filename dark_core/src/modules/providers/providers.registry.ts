import type { ActorProviderAdapter } from './providers.common';
import { mockActorProviderAdapter } from './mockagent/mockagent.adapter';
import { opencodeActorProviderAdapter } from './opencode/opencode.provider';
import { getConfig } from '../../config';

const providerRegistry: Record<string, ActorProviderAdapter> = {
  mock: mockActorProviderAdapter,
  opencode: opencodeActorProviderAdapter,
};

export const listProviderAdapters = (): ActorProviderAdapter[] => {
  return Object.values(providerRegistry);
};

export interface ProvidersRuntimeConfig {
  defaultProvider: string;
  enabledProviders: string[];
}

export const getProvidersRuntimeConfig = (): ProvidersRuntimeConfig => {
  const config = getConfig();
  const enabledProviders = Array.from(
    new Set(config.providers.enabledProviders.map((provider) => provider.trim().toLowerCase())),
  );

  return {
    defaultProvider: config.providers.defaultProvider.trim().toLowerCase(),
    enabledProviders,
  };
};

export const listConfiguredProviders = (): Array<{
  key: string;
  configured: boolean;
  enabled: boolean;
  available: boolean;
}> => {
  const { enabledProviders } = getProvidersRuntimeConfig();
  const knownKeys = new Set(Object.keys(providerRegistry));
  const configuredKeys = new Set(enabledProviders);
  const allKeys = Array.from(new Set([...knownKeys, ...configuredKeys])).sort();

  return allKeys.map((key) => ({
    key,
    configured: configuredKeys.has(key),
    enabled: configuredKeys.has(key),
    available: knownKeys.has(key),
  }));
};

export const getProviderAdapter = (provider: string): ActorProviderAdapter => {
  const normalizedProvider = provider.trim().toLowerCase();
  const { enabledProviders } = getProvidersRuntimeConfig();

  if (!enabledProviders.includes(normalizedProvider)) {
    throw new Error(
      `Providers // Registry // Provider disabled ${JSON.stringify({ provider: normalizedProvider })}`,
    );
  }

  const adapter = providerRegistry[normalizedProvider];

  if (!adapter) {
    throw new Error(
      `Providers // Registry // Unsupported provider ${JSON.stringify({ provider: normalizedProvider })}`,
    );
  }

  return adapter;
};
