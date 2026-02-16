import { locatorIdToHostPath } from '../../../utils/locator';
import type { ActorProviderAdapter, ActorStatusLabel } from '../providers.common';
import { buildActorLocator } from '../providers.common';
import {
  buildOpencodeTuiAttachCommand,
  createOpencodeSession,
  deleteOpencodeSession,
  getOpencodeSessionStatuses,
  listOpencodeSessionMessages,
  sendOpencodeSessionCommand,
  sendOpencodeSessionPrompt,
} from './opencode.controller';
import { mapOpenCodeMessages, mapOpenCodeSessionStatus } from './opencode.mapper';
import type { OpenCodeStatusLike } from './opencode.types';

const requireSessionId = (providerSessionId: string | undefined): string => {
  if (!providerSessionId) {
    throw new Error('Providers // OpenCode // Missing provider session id');
  }

  return providerSessionId;
};

export const opencodeActorProviderAdapter: ActorProviderAdapter = {
  provider: 'opencode',
  async spawn(input) {
    const directory = locatorIdToHostPath(input.workingLocator);
    const session = await createOpencodeSession({ directory, title: input.title });
    const attach = await buildOpencodeTuiAttachCommand({
      directory,
      sessionId: session.id,
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
      model: input.model,
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
