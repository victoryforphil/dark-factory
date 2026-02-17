import { parseLocatorId } from '../../utils/locator';

export type ActorProvider = string;

export type ActorStatusLabel =
  | 'spawning'
  | 'ready'
  | 'busy'
  | 'retrying'
  | 'stopped'
  | 'error'
  | 'unknown';

export interface ActorConnectionInfo {
  provider: ActorProvider;
  directory?: string;
  projectId?: string;
  serverUrl?: string;
  raw?: Record<string, unknown>;
  [key: string]: unknown;
}

export interface ProviderMessage {
  id: string;
  role: 'user' | 'assistant' | string;
  createdAt: string;
  text?: string;
  raw?: Record<string, unknown>;
}

export interface ProviderSubAgentSnapshot {
  id: string;
  parentId?: string;
  title?: string;
  status?: string;
  updatedAt?: string;
  depth?: number;
  summary?: string;
  children?: ProviderSubAgentSnapshot[];
  raw?: Record<string, unknown>;
}

export interface ProviderSessionSnapshot {
  actorLocator: string;
  providerSessionId: string;
  status: ActorStatusLabel;
  title?: string;
  description?: string;
  subAgents?: ProviderSubAgentSnapshot[];
  connectionInfo?: ActorConnectionInfo;
  attachCommand?: string;
}

export interface ActorProviderAdapter {
  provider: ActorProvider;
  spawn(input: {
    actorId: string;
    workingLocator: string;
    title?: string;
    description?: string;
    metadata?: Record<string, unknown>;
  }): Promise<{
    actorLocator: string;
    providerSessionId?: string;
    status: ActorStatusLabel;
    title?: string;
    description?: string;
    subAgents?: ProviderSubAgentSnapshot[];
    connectionInfo?: ActorConnectionInfo;
    attachCommand?: string;
  }>;
  poll(input: {
    actorLocator: string;
    providerSessionId?: string;
    workingLocator: string;
  }): Promise<{
    status: ActorStatusLabel;
    subAgents?: ProviderSubAgentSnapshot[];
    connectionInfo?: ActorConnectionInfo;
    attachCommand?: string;
  }>;
  buildAttach(input: {
    actorLocator: string;
    providerSessionId?: string;
    workingLocator: string;
    model?: string;
    agent?: string;
  }): Promise<{ attachCommand: string; connectionInfo?: ActorConnectionInfo }>;
  sendMessage(input: {
    actorLocator: string;
    providerSessionId?: string;
    workingLocator: string;
    prompt: string;
    noReply?: boolean;
    model?: string;
    agent?: string;
  }): Promise<Record<string, unknown>>;
  listMessages(input: {
    actorLocator: string;
    providerSessionId?: string;
    workingLocator: string;
    nLastMessages?: number;
  }): Promise<ProviderMessage[]>;
  runCommand(input: {
    actorLocator: string;
    providerSessionId?: string;
    workingLocator: string;
    command: string;
    args?: string;
    model?: string;
    agent?: string;
  }): Promise<Record<string, unknown>>;
  listSessions?(input: {
    workingLocator: string;
  }): Promise<ProviderSessionSnapshot[]>;
  terminate?(input: {
    actorLocator: string;
    providerSessionId?: string;
    workingLocator: string;
  }): Promise<void>;
}

export interface ParsedActorLocator {
  provider: string;
  canonicalPath: string;
  providerRef?: string;
}

export const buildActorLocator = (input: {
  provider: string;
  workingLocator: string;
  providerRef?: string;
}): string => {
  const parsed = parseLocatorId(input.workingLocator);
  if (parsed.type !== 'local') {
    throw new Error(
      `Providers // Locator // Unsupported working locator for actor locator (locator=${input.workingLocator})`,
    );
  }

  const base = `${input.provider.toLowerCase()}://${parsed.canonicalPath}`;
  if (!input.providerRef) {
    return base;
  }

  return `${base}#${input.providerRef}`;
};

export const parseActorLocator = (actorLocator: string): ParsedActorLocator => {
  const trimmed = actorLocator.trim();
  const providerDelimiterIndex = trimmed.indexOf(':///');

  if (providerDelimiterIndex <= 0) {
    throw new Error(`Providers // Locator // Invalid actor locator (actorLocator=${actorLocator})`);
  }

  const provider = trimmed.slice(0, providerDelimiterIndex).toLowerCase();
  const afterScheme = trimmed.slice(providerDelimiterIndex + 4);
  const [pathPart, providerRef] = afterScheme.split('#', 2);
  const canonicalPath = pathPart.startsWith('/') ? pathPart : `/${pathPart}`;

  return {
    provider,
    canonicalPath,
    providerRef: providerRef || undefined,
  };
};
