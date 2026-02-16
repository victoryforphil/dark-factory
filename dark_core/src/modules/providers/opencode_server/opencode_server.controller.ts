import { basename } from 'node:path';

import type { Message, Part, Session } from '@opencode-ai/sdk';

import { getConfig } from '../../../config';
import { normalizeProviderDirectory } from '../common/providers.directory';
import { toShellArgument } from '../common/providers.shell';
import { getOpencodeClient } from './opencode_server.client';

export interface OpencodeDirectoryInput {
  directory: string;
}

export interface CreateOpencodeSessionInput extends OpencodeDirectoryInput {
  title?: string;
}

export interface GetOpencodeSessionInput extends OpencodeDirectoryInput {
  id: string;
  includeMessages?: boolean;
}

export interface ListOpencodeSessionMessagesInput extends OpencodeDirectoryInput {
  id: string;
  limit?: number;
}

export interface SendOpencodeSessionCommandInput extends OpencodeDirectoryInput {
  id: string;
  command: string;
}

export interface SendOpencodeSessionPromptInput extends OpencodeDirectoryInput {
  id: string;
  prompt: string;
  noReply?: boolean;
}

export interface BuildTuiAttachCommandInput extends OpencodeDirectoryInput {
  sessionId: string;
  model?: string;
  agent?: string;
}

export interface TuiAttachCommandResult {
  command: string;
  project: {
    id: string;
    worktree: string;
  };
}

export interface OpencodeSessionState {
  session: Session;
  messages?: Array<{
    info: Message;
    parts: Part[];
  }>;
}

type OpencodeSessionStatusMap = Record<string, { type: 'idle' | 'busy' | 'retry' }>;

const getDirectoryClient = async (directory: string) => {
  const normalizedDirectory = await normalizeProviderDirectory(directory, 'OpenCode');
  const client = await getOpencodeClient(normalizedDirectory);

  return {
    client,
    directory: normalizedDirectory,
  };
};

export const createOpencodeSession = async (
  input: CreateOpencodeSessionInput,
): Promise<Session> => {
  const { client, directory } = await getDirectoryClient(input.directory);

  return client.session.create({
    body: {
      title: input.title ?? `Dark Factory // ${basename(directory)}`,
    },
  });
};

export const listOpencodeSessions = async (input: OpencodeDirectoryInput): Promise<Session[]> => {
  const { client } = await getDirectoryClient(input.directory);
  return client.session.list();
};

export const getOpencodeSessionState = async (
  input: GetOpencodeSessionInput,
): Promise<OpencodeSessionState> => {
  const { client } = await getDirectoryClient(input.directory);
  const session = await client.session.get({ path: { id: input.id } });

  if (!input.includeMessages) {
    return { session };
  }

  const messages = await client.session.messages({ path: { id: input.id } });

  return {
    session,
    messages,
  };
};

export const listOpencodeSessionMessages = async (
  input: ListOpencodeSessionMessagesInput,
): Promise<Array<{ info: Message; parts: Part[] }>> => {
  const { client } = await getDirectoryClient(input.directory);

  return client.session.messages({
    path: {
      id: input.id,
    },
    query: {
      ...(input.limit !== undefined ? { limit: input.limit } : {}),
    },
  });
};

export const getOpencodeSessionStatuses = async (
  input: OpencodeDirectoryInput,
): Promise<OpencodeSessionStatusMap> => {
  const { client } = await getDirectoryClient(input.directory);
  return client.session.status();
};

export const getOpencodeDirectoryState = async (input: OpencodeDirectoryInput) => {
  const { client } = await getDirectoryClient(input.directory);

  const [health, project, path, sessions] = await Promise.all([
    client.global.health(),
    client.project.current(),
    client.path.get(),
    client.session.list(),
  ]);

  return {
    health,
    project,
    path,
    sessions,
  };
};

export const sendOpencodeSessionCommand = async (input: SendOpencodeSessionCommandInput) => {
  const { client } = await getDirectoryClient(input.directory);

  return (client.session.command as (payload: unknown) => Promise<unknown>)({
    path: { id: input.id },
    body: { command: input.command },
  });
};

export const sendOpencodeSessionPrompt = async (input: SendOpencodeSessionPromptInput) => {
  const { client } = await getDirectoryClient(input.directory);

  return client.session.prompt({
    path: { id: input.id },
    body: {
      noReply: input.noReply,
      parts: [{ type: 'text', text: input.prompt }],
    },
  });
};

export const abortOpencodeSession = async (
  input: OpencodeDirectoryInput & { id: string },
): Promise<boolean> => {
  const { client } = await getDirectoryClient(input.directory);
  return client.session.abort({ path: { id: input.id } });
};

export const deleteOpencodeSession = async (
  input: OpencodeDirectoryInput & { id: string },
): Promise<boolean> => {
  const { client } = await getDirectoryClient(input.directory);
  return client.session.delete({ path: { id: input.id } });
};

export const buildOpencodeTuiAttachCommand = async (
  input: BuildTuiAttachCommandInput,
): Promise<TuiAttachCommandResult> => {
  const { client, directory } = await getDirectoryClient(input.directory);
  const project = await client.project.current();
  const config = getConfig();

  const commandParts = [
    config.opencode.tuiCommand,
    `--project=${toShellArgument(project.worktree || directory)}`,
    `--session=${toShellArgument(input.sessionId)}`,
  ];

  if (input.model) {
    commandParts.push(`--model=${toShellArgument(input.model)}`);
  }

  if (input.agent) {
    commandParts.push(`--agent=${toShellArgument(input.agent)}`);
  }

  return {
    command: commandParts.join(' '),
    project: {
      id: project.id,
      worktree: project.worktree,
    },
  };
};
