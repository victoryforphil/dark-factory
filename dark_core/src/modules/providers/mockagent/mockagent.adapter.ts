import { locatorIdToHostPath } from '../../../utils/locator';
import type {
  ActorProviderAdapter,
  ActorStatusLabel,
  ProviderMessage,
} from '../providers.common';
import { buildActorLocator } from '../providers.common';
import {
  buildMockAgentAttachCommand,
  createMockAgentSession,
  deleteMockAgentSession,
  getMockAgentSessionStatuses,
  listMockAgentSessionMessages,
  sendMockAgentSessionCommand,
  sendMockAgentSessionPrompt,
} from './mockagent.controller';

const requireSessionId = (providerSessionId: string | undefined): string => {
  if (!providerSessionId) {
    throw new Error('Providers // MockAgent // Missing provider session id');
  }

  return providerSessionId;
};

const mapMockStatus = (status: { type: string } | undefined): ActorStatusLabel => {
  if (!status) {
    return 'stopped';
  }

  if (status.type === 'idle') {
    return 'ready';
  }

  if (status.type === 'busy') {
    return 'busy';
  }

  if (status.type === 'retry') {
    return 'retrying';
  }

  return 'unknown';
};

const mapMockMessages = (
  messages: Array<{
    info: { id: string; role: string; time: { created: number } };
    parts: Array<{ type: string; text?: string }>;
  }>,
): ProviderMessage[] => {
  return messages.map((message) => {
    const text = message.parts
      .filter((part) => part.type === 'text' && typeof part.text === 'string')
      .map((part) => part.text)
      .join('\n')
      .trim();

    return {
      id: message.info.id,
      role: message.info.role,
      createdAt: new Date(message.info.time.created).toISOString(),
      ...(text.length > 0 ? { text } : {}),
      raw: {
        info: message.info,
        parts: message.parts,
      },
    };
  });
};

export const mockActorProviderAdapter: ActorProviderAdapter = {
  provider: 'mock',
  async spawn(input) {
    const directory = locatorIdToHostPath(input.workingLocator);
    const session = await createMockAgentSession({ directory, title: input.title });
    const attach = await buildMockAgentAttachCommand({
      directory,
      sessionId: session.id,
    });

    return {
      actorLocator: buildActorLocator({
        provider: 'mock',
        workingLocator: input.workingLocator,
        providerRef: session.id,
      }),
      providerSessionId: session.id,
      status: 'ready',
      title: session.title,
      description: input.description,
      connectionInfo: {
        provider: 'mock',
        directory,
        projectId: session.projectID,
      },
      attachCommand: attach.command,
    };
  },
  async poll(input) {
    const providerSessionId = requireSessionId(input.providerSessionId);
    const directory = locatorIdToHostPath(input.workingLocator);
    const statuses = await getMockAgentSessionStatuses({ directory });
    const mapped = mapMockStatus(statuses[providerSessionId]);

    if (mapped === 'stopped') {
      return {
        status: mapped,
        connectionInfo: {
          provider: 'mock',
          directory,
        },
      };
    }

    const attach = await buildMockAgentAttachCommand({
      directory,
      sessionId: providerSessionId,
    });

    return {
      status: mapped,
      connectionInfo: {
        provider: 'mock',
        directory,
        projectId: attach.project.id,
      },
      attachCommand: attach.command,
    };
  },
  async buildAttach(input) {
    const providerSessionId = requireSessionId(input.providerSessionId);
    const directory = locatorIdToHostPath(input.workingLocator);
    const attach = await buildMockAgentAttachCommand({
      directory,
      sessionId: providerSessionId,
      model: input.model,
      agent: input.agent,
    });

    return {
      attachCommand: attach.command,
      connectionInfo: {
        provider: 'mock',
        directory,
        projectId: attach.project.id,
      },
    };
  },
  async sendMessage(input) {
    const providerSessionId = requireSessionId(input.providerSessionId);
    const directory = locatorIdToHostPath(input.workingLocator);

    return (await sendMockAgentSessionPrompt({
      directory,
      id: providerSessionId,
      prompt: input.prompt,
      noReply: input.noReply,
    })) as Record<string, unknown>;
  },
  async listMessages(input) {
    const providerSessionId = requireSessionId(input.providerSessionId);
    const directory = locatorIdToHostPath(input.workingLocator);
    const messages = await listMockAgentSessionMessages({
      directory,
      id: providerSessionId,
      limit: input.nLastMessages,
    });

    return mapMockMessages(messages);
  },
  async runCommand(input) {
    const providerSessionId = requireSessionId(input.providerSessionId);
    const directory = locatorIdToHostPath(input.workingLocator);
    const command = [input.command, input.args].filter(Boolean).join(' ').trim();

    return (await sendMockAgentSessionCommand({
      directory,
      id: providerSessionId,
      command,
    })) as Record<string, unknown>;
  },
  async terminate(input) {
    const providerSessionId = input.providerSessionId;
    if (!providerSessionId) {
      return;
    }

    const directory = locatorIdToHostPath(input.workingLocator);
    await deleteMockAgentSession({ directory, id: providerSessionId });
  },
};
