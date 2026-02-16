import { describe, expect, it } from 'bun:test';
import { Elysia } from 'elysia';

import { createMockAgentRoutes, type MockAgentRoutesDependencies } from './mockagent.routes';

const createDependencies = (): MockAgentRoutesDependencies => {
  return {
    abortMockAgentSession: async () => true,
    buildMockAgentAttachCommand: async ({ sessionId }) => ({
      command: `mockagent --session='${sessionId}'`,
      project: {
        id: 'mock_project_0001',
        worktree: '/tmp/worktree',
      },
    }),
    createMockAgentSession: async ({ title }) => ({
      id: 'mock_session_0001',
      projectID: 'mock_project_0001',
      directory: '/tmp/worktree',
      title: title ?? 'created-title',
      version: '0.1.0-mock',
      time: {
        created: Date.now(),
        updated: Date.now(),
      },
    }),
    deleteMockAgentSession: async () => true,
    getMockAgentDirectoryState: async () => ({
      health: { healthy: true, version: '0.1.0-mock' },
      project: { id: 'mock_project_0001', worktree: '/tmp/worktree', time: { created: Date.now() } },
      path: {
        state: '/tmp/worktree/.mockagent/state',
        config: '/tmp/worktree/.mockagent/config',
        worktree: '/tmp/worktree',
        directory: '/tmp/worktree',
      },
      sessions: [],
    }),
    getMockAgentSessionState: async ({ id }) => ({
      session: {
        id,
        projectID: 'mock_project_0001',
        directory: '/tmp/worktree',
        title: 'mock-session',
        version: '0.1.0-mock',
        time: {
          created: Date.now(),
          updated: Date.now(),
        },
      },
    }),
    getMockAgentSessionStatuses: async () => ({
      mock_session_0001: {
        type: 'idle',
        updatedAt: Date.now(),
      },
    }),
    listMockAgentSessionMessages: async () => [
      {
        info: {
          id: 'mock_msg_0001',
          sessionID: 'mock_session_0001',
          role: 'assistant',
          time: {
            created: Date.now(),
          },
        },
        parts: [{ type: 'text', text: 'hello' }],
      },
    ],
    listMockAgentSessions: async () => [],
    sendMockAgentSessionCommand: async ({ command }) => ({
      accepted: true,
      command,
      sessionId: 'mock_session_0001',
      status: 'idle',
      output: `executed:${command}`,
    }),
    sendMockAgentSessionPrompt: async ({ noReply }) => ({
      accepted: true,
      sessionId: 'mock_session_0001',
      noReply: noReply ?? false,
      messageCount: 2,
    }),
  };
};

describe('mockagent routes unit', () => {
  it('creates a session and returns attach command', async () => {
    const app = new Elysia().use(createMockAgentRoutes(createDependencies()));

    const response = await app.handle(
      new Request('http://localhost/mockagent/sessions', {
        method: 'POST',
        headers: {
          'content-type': 'application/json',
        },
        body: JSON.stringify({
          directory: '/tmp/worktree',
          title: 'session from test',
        }),
      }),
    );

    expect(response.status).toBe(201);
    await expect(response.json()).resolves.toMatchObject({
      ok: true,
      data: {
        session: {
          id: 'mock_session_0001',
          title: 'session from test',
        },
        attach: {
          command: "mockagent --session='mock_session_0001'",
        },
      },
    });
  });

  it('returns session statuses', async () => {
    const app = new Elysia().use(createMockAgentRoutes(createDependencies()));

    const response = await app.handle(
      new Request('http://localhost/mockagent/sessions/status?directory=/tmp/worktree'),
    );

    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toMatchObject({
      ok: true,
      data: {
        mock_session_0001: {
          type: 'idle',
        },
      },
    });
  });

  it('passes through messages limit query', async () => {
    let received: { directory: string; id: string; limit?: number } | undefined;

    const dependencies = createDependencies();
    dependencies.listMockAgentSessionMessages = async (input) => {
      received = input;
      return [];
    };

    const app = new Elysia().use(createMockAgentRoutes(dependencies));

    const response = await app.handle(
      new Request('http://localhost/mockagent/sessions/mock_session_0001/messages?directory=/tmp/worktree&limit=5'),
    );

    expect(response.status).toBe(200);
    expect(received).toEqual({
      directory: '/tmp/worktree',
      id: 'mock_session_0001',
      limit: 5,
    });
  });

  it('maps session lookup failure to 404', async () => {
    const dependencies = createDependencies();
    dependencies.getMockAgentSessionState = async () => {
      throw new Error('MockAgent // Session // Session not found (directory=/tmp/worktree, sessionId=missing)');
    };

    const app = new Elysia().use(createMockAgentRoutes(dependencies));
    const response = await app.handle(
      new Request('http://localhost/mockagent/sessions/missing?directory=/tmp/worktree'),
    );

    expect(response.status).toBe(404);
    await expect(response.json()).resolves.toEqual({
      ok: false,
      error: {
        code: 'MOCKAGENT_SESSION_GET_FAILED',
        message: 'MockAgent // Session // Session not found (directory=/tmp/worktree, sessionId=missing)',
      },
    });
  });
});
