import { resolve } from 'node:path';

import { toShellArgument } from '../common/providers.shell';
import type {
  MockAgentAttachResult,
  MockAgentCommandResult,
  MockAgentDirectoryState,
  MockAgentMessage,
  MockAgentPromptResult,
  MockAgentSession,
  MockAgentSessionStatus,
  MockAgentSessionStatusState,
} from './mockagent.types';

interface MockAgentProject {
  id: string;
  worktree: string;
  createdAt: number;
}

interface StoredSession {
  session: MockAgentSession;
  status: MockAgentSessionStatusState;
  messages: MockAgentMessage[];
}

export interface MockAgentEngineOptions {
  startTimeMs?: number;
  timeStepMs?: number;
  version?: string;
  tuiCommand?: string;
}

export interface CreateMockAgentSessionInput {
  directory: string;
  title?: string;
}

export interface GetMockAgentSessionInput {
  directory: string;
  id: string;
  includeMessages?: boolean;
}

export interface SendMockAgentCommandInput {
  directory: string;
  id: string;
  command: string;
}

export interface SendMockAgentPromptInput {
  directory: string;
  id: string;
  prompt: string;
  noReply?: boolean;
}

export interface BuildMockAgentAttachInput {
  directory: string;
  sessionId: string;
  model?: string;
  agent?: string;
}

export class MockAgentEngine {
  private readonly timeStepMs: number;
  private readonly version: string;
  private readonly tuiCommand: string;
  private currentTimeMs: number;
  private nextProjectNumber = 1;
  private nextSessionNumber = 1;
  private nextMessageNumber = 1;
  private readonly projectsByDirectory = new Map<string, MockAgentProject>();
  private readonly sessionsById = new Map<string, StoredSession>();

  constructor(options: MockAgentEngineOptions = {}) {
    this.currentTimeMs = options.startTimeMs ?? Date.now();
    this.timeStepMs = options.timeStepMs ?? 1;
    this.version = options.version ?? '0.1.0-mock';
    this.tuiCommand = options.tuiCommand ?? 'mockagent';
  }

  createSession(input: CreateMockAgentSessionInput): MockAgentSession {
    const directory = normalizeDirectory(input.directory);
    const project = this.getOrCreateProject(directory);
    const now = this.nextTimestamp();
    const sessionId = formatPrefixedId('mock_session', this.nextSessionNumber++);

    const session: MockAgentSession = {
      id: sessionId,
      projectID: project.id,
      directory,
      title: input.title ?? `Mock Agent // ${project.id}`,
      version: this.version,
      time: {
        created: now,
        updated: now,
      },
    };

    this.sessionsById.set(session.id, {
      session,
      status: {
        type: 'idle',
        updatedAt: now,
      },
      messages: [],
    });

    return session;
  }

  listSessions(directory: string): MockAgentSession[] {
    const normalizedDirectory = normalizeDirectory(directory);

    return Array.from(this.sessionsById.values())
      .map((stored) => stored.session)
      .filter((session) => session.directory === normalizedDirectory)
      .sort((left, right) => left.time.created - right.time.created);
  }

  getSession(input: GetMockAgentSessionInput): {
    session: MockAgentSession;
    messages?: MockAgentMessage[];
  } {
    const stored = this.requireSession(input.id, input.directory);

    return {
      session: stored.session,
      messages: input.includeMessages ? [...stored.messages] : undefined,
    };
  }

  getDirectoryState(directory: string): MockAgentDirectoryState {
    const normalizedDirectory = normalizeDirectory(directory);
    const project = this.getOrCreateProject(normalizedDirectory);

    return {
      health: {
        healthy: true,
        version: this.version,
      },
      project: {
        id: project.id,
        worktree: project.worktree,
        time: {
          created: project.createdAt,
        },
      },
      path: {
        state: `${normalizedDirectory}/.mockagent/state`,
        config: `${normalizedDirectory}/.mockagent/config`,
        worktree: normalizedDirectory,
        directory: normalizedDirectory,
      },
      sessions: this.listSessions(normalizedDirectory),
    };
  }

  getSessionStatuses(directory: string): Record<string, MockAgentSessionStatusState> {
    const normalizedDirectory = normalizeDirectory(directory);
    const statusEntries = Array.from(this.sessionsById.values())
      .filter((stored) => stored.session.directory === normalizedDirectory)
      .map((stored) => [stored.session.id, stored.status] as const);

    return Object.fromEntries(statusEntries);
  }

  sendCommand(input: SendMockAgentCommandInput): MockAgentCommandResult {
    const stored = this.requireSession(input.id, input.directory);

    if (input.command === '/fail') {
      throw new Error(`MockAgent // Command // Forced failure (sessionId=${input.id})`);
    }

    if (input.command === '/busy') {
      this.setSessionStatus(stored, 'busy');
      return {
        accepted: true,
        command: input.command,
        sessionId: input.id,
        status: stored.status.type,
        output: 'status:busy',
      };
    }

    if (input.command === '/retry') {
      this.setSessionStatus(stored, 'retry');
      return {
        accepted: true,
        command: input.command,
        sessionId: input.id,
        status: stored.status.type,
        output: 'status:retry',
      };
    }

    if (input.command === '/idle') {
      this.setSessionStatus(stored, 'idle');
      return {
        accepted: true,
        command: input.command,
        sessionId: input.id,
        status: stored.status.type,
        output: 'status:idle',
      };
    }

    this.touchSession(stored);

    return {
      accepted: true,
      command: input.command,
      sessionId: input.id,
      status: stored.status.type,
      output: `executed:${input.command}`,
    };
  }

  sendPrompt(input: SendMockAgentPromptInput): MockAgentPromptResult {
    const stored = this.requireSession(input.id, input.directory);
    this.appendMessage(stored, 'user', input.prompt);

    if (input.noReply) {
      this.setSessionStatus(stored, 'idle');
      return {
        accepted: true,
        sessionId: input.id,
        noReply: true,
        messageCount: stored.messages.length,
      };
    }

    const assistantMessage = this.appendMessage(
      stored,
      'assistant',
      `MockAgent reply // ${input.prompt}`,
    );
    this.setSessionStatus(stored, 'idle');

    return {
      accepted: true,
      sessionId: input.id,
      noReply: false,
      messageCount: stored.messages.length,
      reply: assistantMessage,
    };
  }

  listMessages(input: { directory: string; id: string; limit?: number }): MockAgentMessage[] {
    const stored = this.requireSession(input.id, input.directory);
    const messages = stored.messages;

    if (!input.limit || input.limit >= messages.length) {
      return [...messages];
    }

    return messages.slice(messages.length - input.limit);
  }

  abortSession(input: { directory: string; id: string }): boolean {
    const stored = this.requireSession(input.id, input.directory);
    this.setSessionStatus(stored, 'idle');
    return true;
  }

  deleteSession(input: { directory: string; id: string }): boolean {
    const stored = this.requireSession(input.id, input.directory);
    return this.sessionsById.delete(stored.session.id);
  }

  buildAttachCommand(input: BuildMockAgentAttachInput): MockAgentAttachResult {
    const stored = this.requireSession(input.sessionId, input.directory);
    const project = this.getOrCreateProject(stored.session.directory);

    const commandParts = [
      this.tuiCommand,
      `--directory=${toShellArgument(stored.session.directory)}`,
      `--session=${toShellArgument(stored.session.id)}`,
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
  }

  private getOrCreateProject(directory: string): MockAgentProject {
    const existing = this.projectsByDirectory.get(directory);
    if (existing) {
      return existing;
    }

    const project: MockAgentProject = {
      id: formatPrefixedId('mock_project', this.nextProjectNumber++),
      worktree: directory,
      createdAt: this.nextTimestamp(),
    };
    this.projectsByDirectory.set(directory, project);
    return project;
  }

  private requireSession(id: string, directory: string): StoredSession {
    const normalizedDirectory = normalizeDirectory(directory);
    const stored = this.sessionsById.get(id);
    if (!stored || stored.session.directory !== normalizedDirectory) {
      throw new Error(
        `MockAgent // Session // Session not found (directory=${normalizedDirectory}, sessionId=${id})`,
      );
    }
    return stored;
  }

  private appendMessage(
    stored: StoredSession,
    role: 'user' | 'assistant' | 'system',
    text: string,
  ): MockAgentMessage {
    const createdAt = this.nextTimestamp();
    const message: MockAgentMessage = {
      info: {
        id: formatPrefixedId('mock_msg', this.nextMessageNumber++),
        sessionID: stored.session.id,
        role,
        time: {
          created: createdAt,
        },
      },
      parts: [
        {
          type: 'text',
          text,
        },
      ],
    };

    stored.messages.push(message);
    this.touchSession(stored);
    return message;
  }

  private touchSession(stored: StoredSession): void {
    stored.session.time.updated = this.nextTimestamp();
  }

  private setSessionStatus(stored: StoredSession, status: MockAgentSessionStatus): void {
    stored.status = {
      type: status,
      updatedAt: this.nextTimestamp(),
    };
    this.touchSession(stored);
  }

  private nextTimestamp(): number {
    const next = this.currentTimeMs;
    this.currentTimeMs += this.timeStepMs;
    return next;
  }
}

export const createMockAgentEngine = (options?: MockAgentEngineOptions): MockAgentEngine => {
  return new MockAgentEngine(options);
};

const formatPrefixedId = (prefix: string, value: number): string => {
  return `${prefix}_${String(value).padStart(4, '0')}`;
};

const normalizeDirectory = (directory: string): string => {
  return resolve(directory);
};
