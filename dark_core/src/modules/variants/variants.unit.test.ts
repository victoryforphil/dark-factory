import { describe, expect, it } from 'bun:test';
import { Elysia } from 'elysia';

import { NotFoundError } from '../common/controller.errors';
import { createVariantsRoutes } from './variants.routes';

const dependenciesBase = {
  createVariant: async () => {
    throw new Error('not used in this test');
  },
  deleteVariantById: async () => {
    throw new Error('not used in this test');
  },
  getVariantById: async () => {
    throw new Error('not used in this test');
  },
  importVariantActors: async () => {
    throw new Error('not used in this test');
  },
  listVariants: async () => {
    throw new Error('not used in this test');
  },
  syncVariantGitInfo: async () => {
    throw new Error('not used in this test');
  },
  updateVariantById: async () => {
    throw new Error('not used in this test');
  },
};

describe('variants module unit', () => {
  it('maps list controller errors into API failure response', async () => {
    const app = new Elysia().use(
      createVariantsRoutes({
        ...dependenciesBase,
        listVariants: async () => {
          throw new Error('boom');
        },
      }),
    );

    const response = await app.handle(new Request('http://localhost/variants/'));

    expect(response.status).toBe(500);
    await expect(response.json()).resolves.toEqual({
      ok: false,
      error: {
        code: 'VARIANTS_LIST_FAILED',
        message: 'boom',
      },
    });
  });

  it('maps missing variant errors into 404 responses', async () => {
    const app = new Elysia().use(
      createVariantsRoutes({
        ...dependenciesBase,
        getVariantById: async () => {
          throw new NotFoundError('Variant missing-variant was not found');
        },
      }),
    );

    const response = await app.handle(new Request('http://localhost/variants/missing-variant'));

    expect(response.status).toBe(404);
    await expect(response.json()).resolves.toEqual({
      ok: false,
      error: {
        code: 'VARIANTS_NOT_FOUND',
        message: 'Variant missing-variant was not found',
      },
    });
  });

  it('maps poll requests to the git sync controller dependency', async () => {
    const app = new Elysia().use(
      createVariantsRoutes({
        ...dependenciesBase,
        syncVariantGitInfo: async () => {
          return {
            id: 'v_1',
            productId: 'p_1',
            name: 'default',
            locator: '@local:///tmp/demo',
            gitInfo: null,
            gitInfoUpdatedAt: null,
            gitInfoLastPolledAt: null,
            createdAt: new Date('2026-01-01T00:00:00.000Z'),
            updatedAt: new Date('2026-01-01T00:00:00.000Z'),
          };
        },
      }),
    );

    const response = await app.handle(
      new Request('http://localhost/variants/v_1/poll', {
        method: 'POST',
      }),
    );

    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toMatchObject({
      ok: true,
      data: {
        id: 'v_1',
      },
    });
  });

  it('supports poll=false on poll route by skipping sync', async () => {
    const app = new Elysia().use(
      createVariantsRoutes({
        ...dependenciesBase,
        getVariantById: async () => {
          return {
            id: 'v_1',
            productId: 'p_1',
            name: 'default',
            locator: '@local:///tmp/demo',
            gitInfo: null,
            gitInfoUpdatedAt: null,
            gitInfoLastPolledAt: null,
            createdAt: new Date('2026-01-01T00:00:00.000Z'),
            updatedAt: new Date('2026-01-01T00:00:00.000Z'),
          };
        },
        syncVariantGitInfo: async () => {
          throw new Error('sync should be skipped when poll=false');
        },
      }),
    );

    const response = await app.handle(
      new Request('http://localhost/variants/v_1/poll?poll=false', {
        method: 'POST',
      }),
    );

    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toMatchObject({
      ok: true,
      data: {
        id: 'v_1',
      },
    });
  });

  it('imports active provider actors for a variant', async () => {
    let received: { variantId: string; provider?: string } | undefined;

    const app = new Elysia().use(
      createVariantsRoutes({
        ...dependenciesBase,
        importVariantActors: async (input) => {
          received = input;
          return {
            variantId: input.variantId,
            provider: input.provider ?? 'opencode/server',
            discovered: 2,
            created: 1,
            updated: 1,
            actors: [],
          };
        },
      }),
    );

    const response = await app.handle(
      new Request('http://localhost/variants/v_1/actors/import', {
        method: 'POST',
        headers: {
          'content-type': 'application/json',
        },
        body: JSON.stringify({
          provider: 'opencode/server',
        }),
      }),
    );

    expect(response.status).toBe(200);
    expect(received).toEqual({
      variantId: 'v_1',
      provider: 'opencode/server',
    });
    await expect(response.json()).resolves.toMatchObject({
      ok: true,
      data: {
        variantId: 'v_1',
        provider: 'opencode/server',
        discovered: 2,
        created: 1,
        updated: 1,
      },
    });
  });
});
