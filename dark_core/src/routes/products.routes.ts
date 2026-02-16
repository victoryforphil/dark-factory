import { Elysia, t } from 'elysia';

import { createProduct, listProducts } from '../controllers';
import { failure, success, toErrorMessage } from '../utils/api-response';

export const productsRoutes = new Elysia({ prefix: '/products' })
  .get(
    '/',
    async ({ query, set }) => {
      try {
        const products = await listProducts({
          cursor: query.cursor,
          limit: query.limit ? Number(query.limit) : undefined,
        });

        return success(products);
      } catch (error) {
        set.status = 500;
        return failure('PRODUCTS_LIST_FAILED', toErrorMessage(error));
      }
    },
    {
      query: t.Object({
        cursor: t.Optional(t.String()),
        limit: t.Optional(t.String()),
      }),
    },
  )
  .post(
    '/',
    async ({ body, set }) => {
      try {
        const createdProduct = await createProduct(body);
        set.status = 201;
        return success(createdProduct);
      } catch (error) {
        set.status = 500;
        return failure('PRODUCTS_CREATE_FAILED', toErrorMessage(error));
      }
    },
    {
      body: t.Object({
        id: t.Optional(t.String()),
        locator: t.String(),
        displayName: t.Optional(t.Union([t.String(), t.Null()])),
      }),
    },
  );
