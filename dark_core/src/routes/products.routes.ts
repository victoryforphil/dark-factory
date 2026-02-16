import { Elysia, t } from 'elysia';
import {
  ProductPlain,
  ProductPlainInputCreate,
  ProductPlainInputUpdate,
} from '../../../generated/prismabox/Product';

import {
  createProduct,
  deleteProductById,
  getProductById,
  isIdCollisionDetectedError,
  isNotFoundError,
  listProducts,
  updateProductById,
} from '../controllers';
import { failure, success, toErrorMessage } from '../utils/api-response';
import Log, { formatLogMetadata } from '../utils/logging';

export interface ProductsRoutesDependencies {
  createProduct: typeof createProduct;
  deleteProductById: typeof deleteProductById;
  getProductById: typeof getProductById;
  listProducts: typeof listProducts;
  updateProductById: typeof updateProductById;
}

const productsListResponse = t.Object({
  ok: t.Literal(true),
  data: t.Array(ProductPlain),
});

const productCreateResponse = t.Object({
  ok: t.Literal(true),
  data: ProductPlain,
});

const productGetResponse = t.Object({
  ok: t.Literal(true),
  data: ProductPlain,
});

const productUpdateResponse = t.Object({
  ok: t.Literal(true),
  data: ProductPlain,
});

const productDeleteResponse = t.Object({
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

const notFoundResponse = t.Object({
  ok: t.Literal(false),
  error: t.Object({
    code: t.Literal('PRODUCTS_NOT_FOUND'),
    message: t.String(),
  }),
});

const idCollisionResponse = t.Object({
  ok: t.Literal(false),
  error: t.Object({
    code: t.Literal('ID_COLLISION_DETECTED'),
    message: t.String(),
  }),
});

export const createProductsRoutes = (
  dependencies: ProductsRoutesDependencies = {
    createProduct,
    deleteProductById,
    getProductById,
    listProducts,
    updateProductById,
  },
): Elysia => {
  return new Elysia({ prefix: '/products' })
    .get(
      '/',
      async ({ query, set }) => {
        try {
          const products = await dependencies.listProducts({
            cursor: query.cursor,
            limit: query.limit ? Number(query.limit) : undefined,
          });

          return success(products);
        } catch (error) {
          Log.error(
            `Core // Products Route // List failed ${formatLogMetadata({ error: toErrorMessage(error) })}`,
          );
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
          const createdProduct = await dependencies.createProduct(body);
          set.status = 201;
          return success(createdProduct);
        } catch (error) {
          if (isIdCollisionDetectedError(error)) {
            set.status = 500;
            Log.error(
              `Core // Products Route // ID collision detected ${formatLogMetadata({
                error: toErrorMessage(error),
                locator: body.locator,
              })}`,
            );
            return failure('ID_COLLISION_DETECTED', error.message);
          }

          Log.error(
            `Core // Products Route // Create failed ${formatLogMetadata({ error: toErrorMessage(error) })}`,
          );
          set.status = 500;
          return failure('PRODUCTS_CREATE_FAILED', toErrorMessage(error));
        }
      },
      {
        body: ProductPlainInputCreate,
        response: {
          201: productCreateResponse,
          500: t.Union([apiFailureResponse, idCollisionResponse]),
        },
      },
    )
    .get(
      '/:id',
      async ({ params, set }) => {
        try {
          const product = await dependencies.getProductById(params.id);
          return success(product);
        } catch (error) {
          if (isNotFoundError(error)) {
            set.status = 404;
            Log.warn(
              `Core // Products Route // Product not found ${formatLogMetadata({ id: params.id })}`,
            );
            return failure('PRODUCTS_NOT_FOUND', error.message);
          }

          Log.error(
            `Core // Products Route // Get failed ${formatLogMetadata({
              error: toErrorMessage(error),
              id: params.id,
            })}`,
          );
          set.status = 500;
          return failure('PRODUCTS_GET_FAILED', toErrorMessage(error));
        }
      },
      {
        params: t.Object({ id: t.String() }),
        response: {
          200: productGetResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    )
    .patch(
      '/:id',
      async ({ params, body, set }) => {
        try {
          const updatedProduct = await dependencies.updateProductById(params.id, body);
          return success(updatedProduct);
        } catch (error) {
          if (isNotFoundError(error)) {
            set.status = 404;
            Log.warn(
              `Core // Products Route // Update product not found ${formatLogMetadata({ id: params.id })}`,
            );
            return failure('PRODUCTS_NOT_FOUND', error.message);
          }

          Log.error(
            `Core // Products Route // Update failed ${formatLogMetadata({
              error: toErrorMessage(error),
              id: params.id,
            })}`,
          );
          set.status = 500;
          return failure('PRODUCTS_UPDATE_FAILED', toErrorMessage(error));
        }
      },
      {
        params: t.Object({ id: t.String() }),
        body: ProductPlainInputUpdate,
        response: {
          200: productUpdateResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    )
    .delete(
      '/:id',
      async ({ params, set }) => {
        try {
          const deletedProduct = await dependencies.deleteProductById(params.id);
          return success(deletedProduct);
        } catch (error) {
          if (isNotFoundError(error)) {
            set.status = 404;
            Log.warn(
              `Core // Products Route // Delete product not found ${formatLogMetadata({ id: params.id })}`,
            );
            return failure('PRODUCTS_NOT_FOUND', error.message);
          }

          Log.error(
            `Core // Products Route // Delete failed ${formatLogMetadata({
              error: toErrorMessage(error),
              id: params.id,
            })}`,
          );
          set.status = 500;
          return failure('PRODUCTS_DELETE_FAILED', toErrorMessage(error));
        }
      },
      {
        params: t.Object({ id: t.String() }),
        response: {
          200: productDeleteResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    );
};

export const productsRoutes = createProductsRoutes();
