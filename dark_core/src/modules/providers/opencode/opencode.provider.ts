import { locatorIdToHostPath } from '../../../utils/locator';
import type {
  ActorProviderAdapter,
  ActorStatusLabel,
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
} from './opencode.controller';
import { mapOpenCodeMessages, mapOpenCodeSessionStatus } from './opencode.mapper';
import type { OpenCodeStatusLike } from './opencode.types';

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

export const opencodeActorProviderAdapter: ActorProviderAdapter = {
  provider: 'opencode',
  async spawn(input) {
    const directory = locatorIdToHostPath(input.workingLocator);
    const session = await createOpencodeSession({ directory, title: input.title });
    const attach = await buildOpencodeTuiAttachCommand({
      directory,
      sessionId: session.id,
      model: resolveOpencodeModel(),
    });

    return {
      actorLocator: buildActorLocator({
        provider: 'opencode',
        workingLocator: input.workingLocator,
        providerRef: session.id,
      }),
      providerSessionId: session.id,
      status: 'ready',
      title: session.title,
      description: input.description,
      connectionInfo: {
        provider: 'opencode',
        directory,
        projectId: attach.project.id,
      },
      attachCommand: attach.command,
    };
  },
  async poll(input) {
    const providerSessionId = requireSessionId(input.providerSessionId);
    const directory = locatorIdToHostPath(input.workingLocator);
    const statuses = await getOpencodeSessionStatuses({ directory });
    const sessionStatus = statuses[providerSessionId] as OpenCodeStatusLike | undefined;
    const status = mapOpenCodeSessionStatus(sessionStatus);

    if (status === 'stopped') {
      return {
        status,
        connectionInfo: {
          provider: 'opencode',
          directory,
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
      connectionInfo: {
        provider: 'opencode',
        directory,
        projectId: attach.project.id,
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
        provider: 'opencode',
        directory,
        projectId: attach.project.id,
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
    const messages = await listOpencodeSessionMessages({
      directory,
      id: providerSessionId,
      limit: input.nLastMessages,
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
    const [sessions, statuses] = await Promise.all([
      listOpencodeSessions({ directory }),
      getOpencodeSessionStatuses({ directory }),
    ]);
    const activeSessionIds = new Set(Object.keys(statuses));

    return Promise.all(
      sessions
        .filter((session) => activeSessionIds.has(session.id))
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
              provider: 'opencode',
              workingLocator: input.workingLocator,
              providerRef: session.id,
            }),
            providerSessionId: session.id,
            status: mapOpenCodeSessionStatus(statuses[session.id] as OpenCodeStatusLike | undefined),
            title: session.title,
            description,
            connectionInfo: {
              provider: 'opencode',
              directory,
            },
          };
        }),
    );
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
