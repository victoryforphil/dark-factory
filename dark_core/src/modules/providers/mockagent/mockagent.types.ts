export type MockAgentSessionStatus = 'idle' | 'busy' | 'retry';

export interface MockAgentSession {
  id: string;
  projectID: string;
  directory: string;
  title: string;
  version: string;
  time: {
    created: number;
    updated: number;
  };
}

export interface MockAgentMessageInfo {
  id: string;
  sessionID: string;
  role: 'user' | 'assistant' | 'system';
  time: {
    created: number;
  };
}

export interface MockAgentMessagePart {
  type: 'text';
  text: string;
}

export interface MockAgentMessage {
  info: MockAgentMessageInfo;
  parts: MockAgentMessagePart[];
}

export interface MockAgentSessionStatusState {
  type: MockAgentSessionStatus;
  updatedAt: number;
}

export interface MockAgentDirectoryState {
  health: {
    healthy: boolean;
    version: string;
  };
  project: {
    id: string;
    worktree: string;
    time: {
      created: number;
    };
  };
  path: {
    state: string;
    config: string;
    worktree: string;
    directory: string;
  };
  sessions: MockAgentSession[];
}

export interface MockAgentAttachResult {
  command: string;
  project: {
    id: string;
    worktree: string;
  };
}

export interface MockAgentCommandResult {
  accepted: true;
  command: string;
  sessionId: string;
  status: MockAgentSessionStatus;
  output: string;
}

export interface MockAgentPromptResult {
  accepted: true;
  sessionId: string;
  noReply: boolean;
  messageCount: number;
  reply?: MockAgentMessage;
}
