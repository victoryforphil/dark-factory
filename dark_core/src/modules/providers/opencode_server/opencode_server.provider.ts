import { locatorIdToHostPath } from '../../../utils/locator';
import { logInfoDuration, startLogTimer } from '../../../utils/logging';
import { getConfig } from '../../../config';
import type {
  ActorConnectionInfo,
  ActorProviderAdapter,
  ActorStatusLabel,
  ProviderSubAgentSnapshot,
  ProviderSessionSnapshot,
} from '../providers.common';
import { buildActorLocator } from '../providers.common';
import {
  buildOpencodeTuiAttachCommand,
  createOpencodeSession,
  deleteOpencodeSession,
  getOpencodeSessionStatuses,
  listOpencodeSessions,
  listOpencodeSessionMessages,
  sendOpencodeSessionCommand,
  sendOpencodeSessionPrompt,
} from './opencode_server.controller';
import { getOpencodeBaseUrl, getOpencodeRuntimeInfo } from './opencode_server.client';
import { mapOpenCodeMessages, mapOpenCodeSessionStatus } from './opencode_server.mapper';
import type { OpenCodeStatusLike } from './opencode_server.types';

export const OPENCODE_SERVER_PROVIDER_KEY = 'opencode/server';

const DEFAULT_OPENCODE_MODEL = 'openai/gpt-5.3-codex';

const requireSessionId = (providerSessionId: string | undefined): string => {
  if (!providerSessionId) {
    throw new Error('Providers // OpenCode // Missing provider session id');
  }

  return providerSessionId;
};

const resolveOpencodeModel = (model?: string): string => {
  const normalized = model?.trim();
  if (normalized) {
    return normalized;
  }

  return DEFAULT_OPENCODE_MODEL;
};

type OpenCodeSessionLike = {
  id: string;
  title?: string;
} & Record<string, unknown>;

const toIsoString = (value: unknown): string | undefined => {
  if (typeof value === 'string') {
    const trimmed = value.trim();
    return trimmed.length > 0 ? trimmed : undefined;
  }

  if (typeof value === 'number' && Number.isFinite(value)) {
    const normalized = value < 1_000_000_000_000 ? value * 1000 : value;
    return new Date(normalized).toISOString();
  }

  return undefined;
};

const readSessionParentId = (session: OpenCodeSessionLike): string | undefined => {
  const candidate =
    session.parentId ??
    session.parentID ??
    session.parent_id ??
    (typeof session.parent === 'object' && session.parent !== null
      ? (session.parent as Record<string, unknown>).id
      : undefined);

  if (typeof candidate !== 'string') {
    return undefined;
  }

  const trimmed = candidate.trim();
  return trimmed.length > 0 ? trimmed : undefined;
};

const readSessionUpdatedAt = (session: OpenCodeSessionLike): string | undefined => {
  const explicit = toIsoString(session.updatedAt ?? session.updated_at ?? session.lastUpdatedAt);
  if (explicit) {
    return explicit;
  }

  const time = session.time as Record<string, unknown> | undefined;
  return toIsoString(time?.updated ?? time?.updatedAt);
};

const toMillis = (value: unknown): number | undefined => {
  if (typeof value === 'number' && Number.isFinite(value)) {
    return value < 1_000_000_000_000 ? value * 1000 : value;
  }

  if (typeof value === 'string' && value.trim().length > 0) {
    const parsed = Date.parse(value);
    return Number.isNaN(parsed) ? undefined : parsed;
  }

  return undefined;
};

const sessionUpdatedAtMs = (session: OpenCodeSessionLike): number | undefined => {
  const explicit = toMillis(session.updatedAt ?? session.updated_at ?? session.lastUpdatedAt);
  if (explicit !== undefined) {
    return explicit;
  }

  const time = session.time as Record<string, unknown> | undefined;
  return toMillis(time?.updated ?? time?.updatedAt ?? time?.created ?? time?.createdAt);
};

const selectSessionsForImport = (input: {
  sessions: OpenCodeSessionLike[];
  statuses: Record<string, OpenCodeStatusLike | undefined>;
}): OpenCodeSessionLike[] => {
  const activeSessionIds = new Set(Object.keys(input.statuses));
  const activeSessions = input.sessions.filter((session) => activeSessionIds.has(session.id));
  if (activeSessions.length > 0) {
    return activeSessions;
  }

  const config = getConfig();
  if (!config.opencode.includeRecentSessionsWhenStatusEmpty) {
    return [];
  }

  const nowMs = Date.now();
  const windowMs = config.opencode.recentSessionWindowHours * 60 * 60 * 1000;
  const minUpdatedAtMs = nowMs - windowMs;

  return input.sessions
    .map((session) => ({
      session,
      updatedAtMs: sessionUpdatedAtMs(session),
    }))
    .filter((entry) => entry.updatedAtMs !== undefined && entry.updatedAtMs >= minUpdatedAtMs)
    .sort((left, right) => (right.updatedAtMs ?? 0) - (left.updatedAtMs ?? 0))
    .slice(0, config.opencode.recentSessionLimit)
    .map((entry) => entry.session);
};

const buildSubAgentTree = (input: {
  rootSessionId: string;
  sessions: OpenCodeSessionLike[];
  statuses: Record<string, OpenCodeStatusLike | undefined>;
}): ProviderSubAgentSnapshot[] => {
  const byId = new Map<string, OpenCodeSessionLike>();
  const childMap = new Map<string, OpenCodeSessionLike[]>();

  for (const session of input.sessions) {
    byId.set(session.id, session);
  }

  for (const session of input.sessions) {
    const parentId = readSessionParentId(session);
    if (!parentId || !byId.has(parentId)) {
      continue;
    }

    const list = childMap.get(parentId) ?? [];
    list.push(session);
    childMap.set(parentId, list);
  }

  const toNode = (session: OpenCodeSessionLike, depth: number): ProviderSubAgentSnapshot => {
    const children = (childMap.get(session.id) ?? []).map((child) => toNode(child, depth + 1));

    return {
      id: session.id,
      parentId: readSessionParentId(session),
      title: session.title,
      status: mapOpenCodeSessionStatus(input.statuses[session.id]),
      updatedAt: readSessionUpdatedAt(session),
      depth,
      children,
      raw: session,
    };
  };

  return (childMap.get(input.rootSessionId) ?? []).map((child) => toNode(child, 0));
};

const toSessionDescription = (
  messages: ReturnType<typeof mapOpenCodeMessages>,
): string | undefined => {
  const ordered = [...messages].sort((left, right) => right.createdAt.localeCompare(left.createdAt));
  const preferred = ordered.find((message) => {
    const role = message.role.trim().toLowerCase();
    return (
      ['assistant', 'agent', 'model'].includes(role) &&
      typeof message.text === 'string' &&
      message.text.trim().length > 0
    );
  });
  const fallback = ordered.find((message) => {
    const role = message.role.trim().toLowerCase();
    return role !== 'user' && typeof message.text === 'string' && message.text.trim().length > 0;
  });
  const finalMessage = preferred ?? fallback;
  const text = finalMessage?.text?.trim();
  if (!text) {
    return undefined;
  }

  const compact = text.replace(/\s+/g, ' ');
  return compact.length > 280 ? `${compact.slice(0, 277)}...` : compact;
};

const buildConnectionInfo = (input: {
  directory: string;
  projectId?: string;
}): ActorConnectionInfo => {
  const runtimeInfo = getOpencodeRuntimeInfo();

  return {
    provider: OPENCODE_SERVER_PROVIDER_KEY,
    directory: input.directory,
    serverUrl: getOpencodeBaseUrl(),
    ...(input.projectId ? { projectId: input.projectId } : {}),
    ...(runtimeInfo.source !== 'uninitialized' ? { serverSource: runtimeInfo.source } : {}),
    ...(runtimeInfo.tmuxSessionName
      ? {
          serverTmuxSessionName: runtimeInfo.tmuxSessionName,
          serverTmuxAttachCommand: `tmux attach -t ${runtimeInfo.tmuxSessionName}`,
        }
      : {}),
  };
};

export const opencodeServerActorProviderAdapter: ActorProviderAdapter = {
  provider: OPENCODE_SERVER_PROVIDER_KEY,
  async spawn(input) {
    const directory = locatorIdToHostPath(input.workingLocator);
    const session = await createOpencodeSession({ directory, title: input.title });
    const [sessions, statuses] = await Promise.all([
      listOpencodeSessions({ directory }),
      getOpencodeSessionStatuses({ directory }),
    ]);
    const sessionById = new Map(sessions.map((entry) => [entry.id, entry as OpenCodeSessionLike]));
    const root = sessionById.get(session.id);
    const attach = await buildOpencodeTuiAttachCommand({
      directory,
      sessionId: session.id,
      model: resolveOpencodeModel(),
    });

    return {
      actorLocator: buildActorLocator({
        provider: OPENCODE_SERVER_PROVIDER_KEY,
        workingLocator: input.workingLocator,
        providerRef: session.id,
      }),
      providerSessionId: session.id,
      status: 'ready',
      title: session.title,
      description: input.description,
      ...(root
        ? {
            subAgents: buildSubAgentTree({
              rootSessionId: session.id,
              sessions: sessions as OpenCodeSessionLike[],
              statuses,
            }),
          }
        : {}),
      connectionInfo: {
        ...buildConnectionInfo({
          directory,
          projectId: attach.project.id,
        }),
      },
      attachCommand: attach.command,
    };
  },
  async poll(input) {
    const providerSessionId = requireSessionId(input.providerSessionId);
    const directory = locatorIdToHostPath(input.workingLocator);
    const [sessions, statuses] = await Promise.all([
      listOpencodeSessions({ directory }),
      getOpencodeSessionStatuses({ directory }),
    ]);
    const sessionStatus = statuses[providerSessionId] as OpenCodeStatusLike | undefined;
    const status = mapOpenCodeSessionStatus(sessionStatus);
    const subAgents = buildSubAgentTree({
      rootSessionId: providerSessionId,
      sessions: sessions as OpenCodeSessionLike[],
      statuses,
    });

    if (status === 'stopped') {
      return {
        status,
        subAgents,
        connectionInfo: {
          ...buildConnectionInfo({
            directory,
          }),
        },
      };
    }

    const attach = await buildOpencodeTuiAttachCommand({
      directory,
      sessionId: providerSessionId,
      model: resolveOpencodeModel(),
    });

    return {
      status,
      subAgents,
      connectionInfo: {
        ...buildConnectionInfo({
          directory,
          projectId: attach.project.id,
        }),
      },
      attachCommand: attach.command,
    };
  },
  async buildAttach(input) {
    const providerSessionId = requireSessionId(input.providerSessionId);
    const directory = locatorIdToHostPath(input.workingLocator);
    const attach = await buildOpencodeTuiAttachCommand({
      directory,
      sessionId: providerSessionId,
      model: resolveOpencodeModel(input.model),
      agent: input.agent,
    });

    return {
      attachCommand: attach.command,
      connectionInfo: {
        ...buildConnectionInfo({
          directory,
          projectId: attach.project.id,
        }),
      },
    };
  },
  async sendMessage(input) {
    const providerSessionId = requireSessionId(input.providerSessionId);
    const directory = locatorIdToHostPath(input.workingLocator);

    return (await sendOpencodeSessionPrompt({
      directory,
      id: providerSessionId,
      prompt: input.prompt,
      noReply: input.noReply,
    })) as Record<string, unknown>;
  },
  async listMessages(input) {
    const providerSessionId = requireSessionId(input.providerSessionId);
    const directory = locatorIdToHostPath(input.workingLocator);
    const startedAtMs = startLogTimer();
    const messages = await listOpencodeSessionMessages({
      directory,
      id: providerSessionId,
      limit: input.nLastMessages,
    });

    logInfoDuration('Core // Providers OpenCode // Messages fetched', startedAtMs, {
      providerSessionId,
      requestedLimit: input.nLastMessages ?? null,
      resultCount: messages.length,
    });

    return mapOpenCodeMessages(messages);
  },
  async runCommand(input) {
    const providerSessionId = requireSessionId(input.providerSessionId);
    const directory = locatorIdToHostPath(input.workingLocator);
    const command = [input.command, input.args].filter(Boolean).join(' ').trim();

    return (await sendOpencodeSessionCommand({
      directory,
      id: providerSessionId,
      command,
    })) as Record<string, unknown>;
  },
  async listSessions(input) {
    const directory = locatorIdToHostPath(input.workingLocator);
    const startedAtMs = startLogTimer();
    const [sessions, statuses] = await Promise.all([
      listOpencodeSessions({ directory }),
      getOpencodeSessionStatuses({ directory }),
    ]);
    const sessionsForImport = selectSessionsForImport({
      sessions: sessions as OpenCodeSessionLike[],
      statuses,
    });
    const sessionById = new Map(sessionsForImport.map((session) => [session.id, session]));
    const isRootSession = (session: OpenCodeSessionLike): boolean => {
      const parentId = readSessionParentId(session);
      return !parentId || !sessionById.has(parentId);
    };

    const importedSessions = await Promise.all(
      sessionsForImport
        .filter((session) => isRootSession(session as OpenCodeSessionLike))
        .map(async (session): Promise<ProviderSessionSnapshot> => {
          const rawMessages = await listOpencodeSessionMessages({
            directory,
            id: session.id,
            limit: 40,
          });
          const mappedMessages = mapOpenCodeMessages(rawMessages);
          const description = toSessionDescription(mappedMessages);

          return {
            actorLocator: buildActorLocator({
              provider: OPENCODE_SERVER_PROVIDER_KEY,
              workingLocator: input.workingLocator,
              providerRef: session.id,
            }),
            providerSessionId: session.id,
            status: mapOpenCodeSessionStatus(statuses[session.id] as OpenCodeStatusLike | undefined),
            title: session.title,
            description,
            subAgents: buildSubAgentTree({
              rootSessionId: session.id,
              sessions: sessionsForImport as OpenCodeSessionLike[],
              statuses,
            }),
            connectionInfo: {
              ...buildConnectionInfo({
                directory,
              }),
            },
          };
        }),
    );

    logInfoDuration('Core // Providers OpenCode // Sessions listed', startedAtMs, {
      activeCount: importedSessions.length,
      directory,
      sessionCount: sessions.length,
      statusCount: Object.keys(statuses).length,
      importPoolCount: sessionsForImport.length,
    });

    return importedSessions;
  },
  async terminate(input) {
    const providerSessionId = input.providerSessionId;
    if (!providerSessionId) {
      return;
    }

    const directory = locatorIdToHostPath(input.workingLocator);
    await deleteOpencodeSession({ directory, id: providerSessionId });
  },
};

export const mapOpenCodeStatusToActorStatus = (status: OpenCodeStatusLike | undefined): ActorStatusLabel => {
  return mapOpenCodeSessionStatus(status);
};

export const __opencodeProviderInternals = {
  buildSubAgentTree,
  readSessionParentId,
  readSessionUpdatedAt,
  selectSessionsForImport,
  sessionUpdatedAtMs,
};
