import { describe, expect, it } from 'bun:test';
import { Elysia } from 'elysia';

import { NotFoundError } from '../controllers';
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
  listVariants: async () => {
    throw new Error('not used in this test');
  },
  updateVariantById: async () => {
    throw new Error('not used in this test');
  },
};

describe('variants routes unit', () => {
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
});
