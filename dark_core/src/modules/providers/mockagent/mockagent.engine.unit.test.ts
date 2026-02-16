import { describe, expect, it } from 'bun:test';

import { createMockAgentEngine } from './mockagent.engine';

describe('mockagent engine', () => {
  it('creates deterministic sessions and attach commands', () => {
    const engine = createMockAgentEngine({
      startTimeMs: 1_700_000_000_000,
      timeStepMs: 10,
      tuiCommand: 'mockagent-cli',
    });

    const session = engine.createSession({ directory: '/tmp/worktree-a' });
    expect(session.id).toBe('mock_session_0001');
    expect(session.projectID).toBe('mock_project_0001');
    expect(session.time.created).toBe(1_700_000_000_010);

    const attach = engine.buildAttachCommand({
      directory: '/tmp/worktree-a',
      sessionId: session.id,
      model: 'openai/gpt-5',
      agent: 'general',
    });

    expect(attach.command).toContain('mockagent-cli');
    expect(attach.command).toContain("--session='mock_session_0001'");
    expect(attach.command).toContain("--model='openai/gpt-5'");
    expect(attach.command).toContain("--agent='general'");
  });

  it('supports prompt and message history with limit', () => {
    const engine = createMockAgentEngine({ startTimeMs: 100, timeStepMs: 1 });
    const session = engine.createSession({ directory: '/tmp/worktree-b' });

    engine.sendPrompt({ directory: '/tmp/worktree-b', id: session.id, prompt: 'hello' });
    engine.sendPrompt({ directory: '/tmp/worktree-b', id: session.id, prompt: 'second', noReply: true });

    const allMessages = engine.listMessages({ directory: '/tmp/worktree-b', id: session.id });
    expect(allMessages.length).toBe(3);
    expect(allMessages[0]?.parts[0]?.text).toBe('hello');
    expect(allMessages[1]?.parts[0]?.text).toBe('MockAgent reply // hello');

    const tailMessages = engine.listMessages({ directory: '/tmp/worktree-b', id: session.id, limit: 1 });
    expect(tailMessages.length).toBe(1);
    expect(tailMessages[0]?.parts[0]?.text).toBe('second');
  });

  it('supports deterministic status commands and forced failure', () => {
    const engine = createMockAgentEngine();
    const session = engine.createSession({ directory: '/tmp/worktree-c' });

    const busyResult = engine.sendCommand({
      directory: '/tmp/worktree-c',
      id: session.id,
      command: '/busy',
    });
    expect(busyResult.status).toBe('busy');

    const statuses = engine.getSessionStatuses('/tmp/worktree-c');
    expect(statuses[session.id]?.type).toBe('busy');

    expect(() =>
      engine.sendCommand({ directory: '/tmp/worktree-c', id: session.id, command: '/fail' }),
    ).toThrow('MockAgent // Command // Forced failure');
  });
});
