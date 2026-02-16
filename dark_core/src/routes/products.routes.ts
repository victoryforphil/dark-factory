import { Elysia, t } from 'elysia';
import { ProductPlain, ProductPlainInputCreate } from '../../../generated/prismabox/Product';

import { createProduct, listProducts } from '../controllers';
import { failure, success, toErrorMessage } from '../utils/api-response';

const productsListResponse = t.Object({
  ok: t.Literal(true),
  data: t.Array(ProductPlain),
});

const productCreateResponse = t.Object({
  ok: t.Literal(true),
  data: ProductPlain,
});

const apiFailureResponse = t.Object({
  ok: t.Literal(false),
  error: t.Object({
    code: t.String(),
    message: t.String(),
  }),
});

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
      response: {
        200: productsListResponse,
        500: apiFailureResponse,
      },
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
      body: ProductPlainInputCreate,
      response: {
        201: productCreateResponse,
        500: apiFailureResponse,
      },
    },
  );
