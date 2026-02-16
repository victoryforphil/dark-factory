import { describe, expect, it } from 'bun:test';
import { Elysia } from 'elysia';

import { NotFoundError } from '../controllers';
import { createProductsRoutes } from './products.routes';

const unusedDependencies = {
  deleteProductById: async () => {
    throw new Error('not used in this test');
  },
  getProductById: async () => {
    throw new Error('not used in this test');
  },
  updateProductById: async () => {
    throw new Error('not used in this test');
  },
};

describe('products routes unit', () => {
  it('maps list controller errors into API failure response', async () => {
    const app = new Elysia().use(
      createProductsRoutes({
        ...unusedDependencies,
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

  it('maps missing product errors into 404 responses', async () => {
    const app = new Elysia().use(
      createProductsRoutes({
        ...unusedDependencies,
        createProduct: async () => {
          throw new Error('not used in this test');
        },
        listProducts: async () => {
          throw new Error('not used in this test');
        },
        getProductById: async () => {
          throw new NotFoundError('Product missing-product was not found');
        },
      }),
    );

    const response = await app.handle(new Request('http://localhost/products/missing-product'));

    expect(response.status).toBe(404);
    await expect(response.json()).resolves.toEqual({
      ok: false,
      error: {
        code: 'PRODUCTS_NOT_FOUND',
        message: 'Product missing-product was not found',
      },
    });
  });
});
