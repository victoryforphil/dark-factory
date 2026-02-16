import { createMockAgentEngine, type MockAgentEngineOptions } from './mockagent.engine';

export interface MockAgentDirectoryInput {
  directory: string;
}

export interface CreateMockAgentSessionInput extends MockAgentDirectoryInput {
  title?: string;
}

export interface GetMockAgentSessionInput extends MockAgentDirectoryInput {
  id: string;
  includeMessages?: boolean;
}

export interface SendMockAgentSessionCommandInput extends MockAgentDirectoryInput {
  id: string;
  command: string;
}

export interface SendMockAgentSessionPromptInput extends MockAgentDirectoryInput {
  id: string;
  prompt: string;
  noReply?: boolean;
}

export interface BuildMockAgentAttachCommandInput extends MockAgentDirectoryInput {
  sessionId: string;
  model?: string;
  agent?: string;
}

let defaultMockAgentEngine = createMockAgentEngine();

export const resetMockAgentEngineForTests = (options?: MockAgentEngineOptions): void => {
  defaultMockAgentEngine = createMockAgentEngine(options);
};

export const createMockAgentSession = async (input: CreateMockAgentSessionInput) => {
  return defaultMockAgentEngine.createSession({ directory: input.directory, title: input.title });
};

export const listMockAgentSessions = async (input: MockAgentDirectoryInput) => {
  return defaultMockAgentEngine.listSessions(input.directory);
};

export const getMockAgentSessionState = async (input: GetMockAgentSessionInput) => {
  return defaultMockAgentEngine.getSession({
    directory: input.directory,
    id: input.id,
    includeMessages: input.includeMessages,
  });
};

export const getMockAgentSessionStatuses = async (input: MockAgentDirectoryInput) => {
  return defaultMockAgentEngine.getSessionStatuses(input.directory);
};

export const getMockAgentDirectoryState = async (input: MockAgentDirectoryInput) => {
  return defaultMockAgentEngine.getDirectoryState(input.directory);
};

export const sendMockAgentSessionCommand = async (input: SendMockAgentSessionCommandInput) => {
  return defaultMockAgentEngine.sendCommand({
    directory: input.directory,
    id: input.id,
    command: input.command,
  });
};

export const sendMockAgentSessionPrompt = async (input: SendMockAgentSessionPromptInput) => {
  return defaultMockAgentEngine.sendPrompt({
    directory: input.directory,
    id: input.id,
    prompt: input.prompt,
    noReply: input.noReply,
  });
};

export const listMockAgentSessionMessages = async (
  input: MockAgentDirectoryInput & { id: string; limit?: number },
) => {
  return defaultMockAgentEngine.listMessages({
    directory: input.directory,
    id: input.id,
    limit: input.limit,
  });
};

export const abortMockAgentSession = async (
  input: MockAgentDirectoryInput & { id: string },
): Promise<boolean> => {
  return defaultMockAgentEngine.abortSession({ directory: input.directory, id: input.id });
};

export const deleteMockAgentSession = async (
  input: MockAgentDirectoryInput & { id: string },
): Promise<boolean> => {
  return defaultMockAgentEngine.deleteSession({ directory: input.directory, id: input.id });
};

export const buildMockAgentAttachCommand = async (input: BuildMockAgentAttachCommandInput) => {
  return defaultMockAgentEngine.buildAttachCommand({
    directory: input.directory,
    sessionId: input.sessionId,
    model: input.model,
    agent: input.agent,
  });
};
