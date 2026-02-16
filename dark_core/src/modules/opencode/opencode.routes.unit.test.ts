import { describe, expect, it } from 'bun:test';
import { Elysia } from 'elysia';

import { createOpencodeRoutes, type OpencodeRoutesDependencies } from './opencode.routes';

const createDependencies = (): OpencodeRoutesDependencies => {
  return {
    abortOpencodeSession: async () => true,
    buildOpencodeTuiAttachCommand: async ({ sessionId }) => ({
      command: `opencode --session='${sessionId}'`,
      project: {
        id: 'project-1',
        worktree: '/tmp/worktree',
      },
    }),
    createOpencodeSession: async ({ title }) => ({
      id: 'session-1',
      projectID: 'project-1',
      directory: '/tmp/worktree',
      title: title ?? 'created-title',
      version: '0.0.0',
      time: {
        created: Date.now(),
        updated: Date.now(),
      },
    }),
    deleteOpencodeSession: async () => true,
    getOpencodeDirectoryState: async () => ({
      health: { healthy: true, version: '1.0.0' },
      project: { id: 'project-1', worktree: '/tmp/worktree', time: { created: Date.now() } },
      path: {
        state: '/tmp/state',
        config: '/tmp/config',
        worktree: '/tmp/worktree',
        directory: '/tmp/worktree',
      },
      sessions: [],
    }),
    getOpencodeSessionState: async ({ id }) => ({
      session: {
        id,
        projectID: 'project-1',
        directory: '/tmp/worktree',
        title: 'session-title',
        version: '0.0.0',
        time: {
          created: Date.now(),
          updated: Date.now(),
        },
      },
    }),
    listOpencodeSessions: async () => [],
    sendOpencodeSessionCommand: async ({ command }) => ({
      result: `executed:${command}`,
    }),
    sendOpencodeSessionPrompt: async ({ prompt }) => ({
      result: `prompted:${prompt}`,
    }),
  };
};

describe('opencode routes unit', () => {
  it('creates a session and returns attach command', async () => {
    const app = new Elysia().use(createOpencodeRoutes(createDependencies()));

    const response = await app.handle(
      new Request('http://localhost/opencode/sessions', {
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
          id: 'session-1',
          title: 'session from test',
        },
        attach: {
          command: "opencode --session='session-1'",
        },
      },
    });
  });

  it('passes query params through for session state lookup', async () => {
    let received: { directory: string; id: string; includeMessages?: boolean } | undefined;

    const dependencies = createDependencies();
    dependencies.getOpencodeSessionState = async (input) => {
      received = input;
      return {
        session: {
          id: input.id,
          projectID: 'project-1',
          directory: '/tmp/worktree',
          title: 'session-title',
          version: '0.0.0',
          time: {
            created: Date.now(),
            updated: Date.now(),
          },
        },
      };
    };

    const app = new Elysia().use(createOpencodeRoutes(dependencies));

    const response = await app.handle(
      new Request('http://localhost/opencode/sessions/session-abc?directory=/tmp/worktree&includeMessages=true'),
    );

    expect(response.status).toBe(200);
    expect(received).toEqual({
      directory: '/tmp/worktree',
      id: 'session-abc',
      includeMessages: true,
    });
  });

  it('maps directory validation failures to 400', async () => {
    const dependencies = createDependencies();
    dependencies.listOpencodeSessions = async () => {
      throw new Error('OpenCode // Directory // Expected existing directory (directory=/missing)');
    };

    const app = new Elysia().use(createOpencodeRoutes(dependencies));

    const response = await app.handle(
      new Request('http://localhost/opencode/sessions?directory=/missing'),
    );

    expect(response.status).toBe(400);
    await expect(response.json()).resolves.toEqual({
      ok: false,
      error: {
        code: 'OPENCODE_SESSIONS_LIST_FAILED',
        message: 'OpenCode // Directory // Expected existing directory (directory=/missing)',
      },
    });
  });

  it('maps unexpected command errors to 500', async () => {
    const dependencies = createDependencies();
    dependencies.sendOpencodeSessionCommand = async () => {
      throw new Error('command failed');
    };

    const app = new Elysia().use(createOpencodeRoutes(dependencies));

    const response = await app.handle(
      new Request('http://localhost/opencode/sessions/session-1/command', {
        method: 'POST',
        headers: {
          'content-type': 'application/json',
        },
        body: JSON.stringify({
          directory: '/tmp/worktree',
          command: '/help',
        }),
      }),
    );

    expect(response.status).toBe(500);
    await expect(response.json()).resolves.toEqual({
      ok: false,
      error: {
        code: 'OPENCODE_SESSION_COMMAND_FAILED',
        message: 'command failed',
      },
    });
  });

  it('builds attach command with model and agent query options', async () => {
    let received: {
      directory: string;
      sessionId: string;
      model?: string;
      agent?: string;
    } | undefined;

    const dependencies = createDependencies();
    dependencies.buildOpencodeTuiAttachCommand = async (input) => {
      received = input;
      return {
        command: `opencode --session='${input.sessionId}'`,
        project: {
          id: 'project-1',
          worktree: '/tmp/worktree',
        },
      };
    };

    const app = new Elysia().use(createOpencodeRoutes(dependencies));

    const response = await app.handle(
      new Request(
        'http://localhost/opencode/sessions/session-77/attach?directory=/tmp/worktree&model=openai/gpt-5&agent=general',
      ),
    );

    expect(response.status).toBe(200);
    expect(received).toEqual({
      directory: '/tmp/worktree',
      sessionId: 'session-77',
      model: 'openai/gpt-5',
      agent: 'general',
    });
  });
});
