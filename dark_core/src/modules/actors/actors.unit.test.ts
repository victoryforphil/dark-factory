import { describe, expect, it } from 'bun:test';
import { Elysia } from 'elysia';

import { createActorsRoutes, type ActorsRoutesDependencies } from './actors.routes';

const createDependencies = (): ActorsRoutesDependencies => {
  return {
    buildActorAttachById: async (id) => ({
      actor: {
        id,
      } as never,
      attachCommand: `mockagent --session='${id}'`,
      connectionInfo: { provider: 'mock' },
    }),
    createActor: async (input) => ({
      id: 'act_1',
      variantId: input.variantId,
      provider: input.provider,
      actorLocator: 'mock:///tmp/project#mock_session_0001',
      workingLocator: '@local:///tmp/project',
      providerSessionId: 'mock_session_0001',
      status: 'ready',
      title: input.title ?? null,
      description: input.description ?? null,
      connectionInfo: null,
      attachCommand: null,
      subAgents: null,
      metadata: input.metadata ?? null,
      createdAt: new Date(),
      updatedAt: new Date(),
    }),
    deleteActorById: async () => ({ id: 'act_1' } as never),
    getActorById: async (id) => ({ id } as never),
    listActorMessagesById: async () => [{ id: 'm1', role: 'assistant', createdAt: new Date().toISOString() }],
    listActors: async () => [{ id: 'act_1' } as never],
    pollActorById: async (id) => ({ id, status: 'ready' } as never),
    runActorCommandById: async () => ({ accepted: true }),
    sendActorMessageById: async () => ({ accepted: true }),
    updateActorById: async (id) => ({ id } as never),
  };
};

describe('actors routes unit', () => {
  it('creates actor and returns 201', async () => {
    const app = new Elysia().use(createActorsRoutes(createDependencies()));

    const response = await app.handle(
      new Request('http://localhost/actors/', {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify({
          variantId: 'var_1',
          provider: 'mock',
          title: 'actor one',
        }),
      }),
    );

    expect(response.status).toBe(201);
    await expect(response.json()).resolves.toMatchObject({
      ok: true,
      data: {
        id: 'act_1',
        provider: 'mock',
      },
    });
  });

  it('passes subAgents through actor create payload', async () => {
    let receivedSubAgents: unknown;
    const dependencies = createDependencies();
    dependencies.createActor = async (input) => {
      receivedSubAgents = input.subAgents;
      return {
        id: 'act_1',
        variantId: input.variantId,
        provider: input.provider,
        actorLocator: 'mock:///tmp/project#mock_session_0001',
        workingLocator: '@local:///tmp/project',
        providerSessionId: 'mock_session_0001',
        status: 'ready',
        title: input.title ?? null,
        description: input.description ?? null,
        connectionInfo: null,
        attachCommand: null,
        subAgents: input.subAgents ?? null,
        metadata: input.metadata ?? null,
        createdAt: new Date(),
        updatedAt: new Date(),
      };
    };

    const app = new Elysia().use(createActorsRoutes(dependencies));
    const response = await app.handle(
      new Request('http://localhost/actors/', {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify({
          variantId: 'var_1',
          provider: 'mock',
          subAgents: [
            {
              id: 'sub_1',
              status: 'running',
              children: [{ id: 'sub_1_1', status: 'ready' }],
            },
          ],
        }),
      }),
    );

    expect(response.status).toBe(201);
    expect(receivedSubAgents).toEqual([
      {
        id: 'sub_1',
        status: 'running',
        children: [{ id: 'sub_1_1', status: 'ready' }],
      },
    ]);
  });

  it('passes nLastMessages to dependencies', async () => {
    let received: { id: string; nLastMessages?: number } | undefined;
    const dependencies = createDependencies();
    dependencies.listActorMessagesById = async (id, input) => {
      received = { id, nLastMessages: input.nLastMessages };
      return [];
    };

    const app = new Elysia().use(createActorsRoutes(dependencies));
    const response = await app.handle(
      new Request('http://localhost/actors/act_1/messages?nLastMessages=4'),
    );

    expect(response.status).toBe(200);
    expect(received).toEqual({
      id: 'act_1',
      nLastMessages: 4,
    });
  });

  it('ignores invalid nLastMessages query values', async () => {
    let received: { id: string; nLastMessages?: number } | undefined;
    const dependencies = createDependencies();
    dependencies.listActorMessagesById = async (id, input) => {
      received = { id, nLastMessages: input.nLastMessages };
      return [];
    };

    const app = new Elysia().use(createActorsRoutes(dependencies));
    const response = await app.handle(
      new Request('http://localhost/actors/act_1/messages?nLastMessages=invalid'),
    );

    expect(response.status).toBe(200);
    expect(received).toEqual({
      id: 'act_1',
      nLastMessages: undefined,
    });
  });

  it('passes variantId through actor update payload', async () => {
    let receivedInput: unknown;
    const dependencies = createDependencies();
    dependencies.updateActorById = async (id, input) => {
      receivedInput = input;
      return {
        id,
      } as never;
    };

    const app = new Elysia().use(createActorsRoutes(dependencies));
    const response = await app.handle(
      new Request('http://localhost/actors/act_1', {
        method: 'PATCH',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify({
          variantId: 'var_2',
        }),
      }),
    );

    expect(response.status).toBe(200);
    expect(receivedInput).toEqual({
      variantId: 'var_2',
    });
  });
});
