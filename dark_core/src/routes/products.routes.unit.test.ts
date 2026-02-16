import { describe, expect, it } from 'bun:test';
import { Elysia } from 'elysia';

import { createProductsRoutes } from './products.routes';

describe('products routes unit', () => {
  it('maps list controller errors into API failure response', async () => {
    const app = new Elysia().use(
      createProductsRoutes({
        createProduct: async () => {
          throw new Error('not used in this test');
        },
        listProducts: async () => {
          throw new Error('boom');
        },
      }),
    );

    const response = await app.handle(new Request('http://localhost/products/'));

    expect(response.status).toBe(500);
    await expect(response.json()).resolves.toEqual({
      ok: false,
      error: {
        code: 'PRODUCTS_LIST_FAILED',
        message: 'boom',
      },
    });
  });
});
