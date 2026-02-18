import { Elysia, t } from 'elysia';
import {
  Product,
  ProductPlain,
  ProductPlainInputCreate,
  ProductPlainInputUpdate,
} from '../../../../generated/prismabox/Product';

import {
  VariantPlain,
} from '../../../../generated/prismabox/Variant';
import {
  createProduct,
  deleteProductById,
  getProductById,
  listProducts,
  updateProductById,
} from './products.controller';
import { createVariant, listVariants } from '../variants/variants.controller';
import { cloneVariantForProduct } from '../variant_clones/variant_clones.controller';
import { isIdCollisionDetectedError, isNotFoundError } from '../common/controller.errors';
import { failure, success, toErrorMessage } from '../../utils/api-response';
import Log, {
  formatLogMetadata,
  logInfoDuration,
  logRouteStart,
  logRouteSuccess,
  startLogTimer,
} from '../../utils/logging';

export interface ProductsRoutesDependencies {
  cloneVariantForProduct: typeof cloneVariantForProduct;
  createProduct: typeof createProduct;
  createVariant: typeof createVariant;
  deleteProductById: typeof deleteProductById;
  getProductById: typeof getProductById;
  listVariants: typeof listVariants;
  listProducts: typeof listProducts;
  updateProductById: typeof updateProductById;
}

const productsListResponse = t.Object({
  ok: t.Literal(true),
  data: t.Array(t.Union([ProductPlain, Product])),
});

const productCreateResponse = t.Object({
  ok: t.Literal(true),
  data: ProductPlain,
});

const productGetResponse = t.Object({
  ok: t.Literal(true),
  data: t.Union([ProductPlain, Product]),
});

const productUpdateResponse = t.Object({
  ok: t.Literal(true),
  data: ProductPlain,
});

const productDeleteResponse = t.Object({
  ok: t.Literal(true),
  data: ProductPlain,
});

const productVariantsListResponse = t.Object({
  ok: t.Literal(true),
  data: t.Array(VariantPlain),
});

const productVariantCreateResponse = t.Object({
  ok: t.Literal(true),
  data: VariantPlain,
});

const productVariantCloneResponse = t.Object({
  ok: t.Literal(true),
  data: t.Object({
    variant: VariantPlain,
    clone: t.Object({
      cloneType: t.String(),
      sourceLocator: t.String(),
      sourceLocatorKind: t.Union([t.Literal('local'), t.Literal('git')]),
      targetPath: t.String(),
      targetLocator: t.String(),
      branchName: t.Nullable(t.String()),
      generatedTargetPath: t.Boolean(),
      generatedBranchName: t.Boolean(),
      attemptedCommand: t.Nullable(t.String()),
      usedNoLocalRetry: t.Boolean(),
      isAsync: t.Boolean(),
    }),
  }),
});

const productVariantCloneInput = t.Object({
  name: t.Optional(t.String()),
  targetPath: t.Optional(t.String()),
  cloneType: t.Optional(t.String()),
  branchName: t.Optional(t.String()),
  sourceVariantId: t.Optional(t.String()),
  runAsync: t.Optional(t.Boolean()),
});

const productVariantCreateInput = t.Object({
  locator: t.String(),
  name: t.Optional(t.String()),
});

const productIncludeQuerySchema = t.Optional(t.Union([t.Literal('minimal'), t.Literal('full')]));

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
    cloneVariantForProduct,
    createProduct,
    createVariant,
    deleteProductById,
    getProductById,
    listVariants,
    listProducts,
    updateProductById,
  },
): Elysia => {
  return new Elysia({ prefix: '/products' })
    .get(
      '/',
      async ({ query, set }) => {
        const startedAt = logRouteStart('Products // List', {
          cursor: query.cursor ?? null,
          include: query.include ?? null,
          limit: query.limit ?? null,
        });
        try {
          const products = await dependencies.listProducts({
            cursor: query.cursor,
            include: query.include,
            limit: query.limit ? Number(query.limit) : undefined,
          });

          logRouteSuccess('Products // List', startedAt, {
            count: products.length,
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
          include: productIncludeQuerySchema,
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
        const startedAt = logRouteStart('Products // Create', {
          displayName: body.displayName ?? null,
          locator: body.locator,
        });
        try {
          const createdProduct = await dependencies.createProduct(body);
          set.status = 201;
          logRouteSuccess('Products // Create', startedAt, {
            id: createdProduct.id,
            workspaceLocator: createdProduct.workspaceLocator,
          });
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
      async ({ params, query, set }) => {
        const startedAt = startLogTimer();
        try {
          const product = await dependencies.getProductById(params.id, {
            include: query.include,
          });
          logInfoDuration('Core // Products Route // Get succeeded', startedAt, {
            id: params.id,
            include: query.include ?? 'minimal',
          });
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
        query: t.Object({
          include: productIncludeQuerySchema,
        }),
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
        const startedAt = startLogTimer();
        try {
          const updatedProduct = await dependencies.updateProductById(params.id, body);
          logInfoDuration('Core // Products Route // Update succeeded', startedAt, {
            id: params.id,
          });
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
        const startedAt = startLogTimer();
        try {
          const deletedProduct = await dependencies.deleteProductById(params.id);
          logInfoDuration('Core // Products Route // Delete succeeded', startedAt, {
            id: params.id,
          });
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
    )
    .get(
      '/:id/variants',
      async ({ params, query, set }) => {
        const startedAt = startLogTimer();
        try {
          await dependencies.getProductById(params.id, { include: 'minimal' });

          const variants = await dependencies.listVariants({
            productId: params.id,
            cursor: query.cursor,
            limit: query.limit ? Number(query.limit) : undefined,
            poll: query.poll !== 'false' && query.poll !== '0',
          });

          logInfoDuration('Core // Products Route // List variants succeeded', startedAt, {
            count: variants.length,
            id: params.id,
            poll: query.poll ?? 'default',
          });
          return success(variants);
        } catch (error) {
          if (isNotFoundError(error)) {
            set.status = 404;
            return failure('PRODUCTS_NOT_FOUND', error.message);
          }

          set.status = 500;
          Log.error(
            `Core // Products Route // List variants failed ${formatLogMetadata({
              error: toErrorMessage(error),
              id: params.id,
            })}`,
          );
          return failure('PRODUCT_VARIANTS_LIST_FAILED', toErrorMessage(error));
        }
      },
      {
        params: t.Object({ id: t.String() }),
        query: t.Object({
          cursor: t.Optional(t.String()),
          limit: t.Optional(t.String()),
          poll: t.Optional(t.String()),
        }),
        response: {
          200: productVariantsListResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    )
    .post(
      '/:id/variants',
      async ({ params, body, set }) => {
        const startedAt = startLogTimer();
        try {
          await dependencies.getProductById(params.id, { include: 'minimal' });
          const variant = await dependencies.createVariant({
            product: {
              connect: {
                id: params.id,
              },
            },
            locator: body.locator,
            name: body.name,
          });

          set.status = 201;
          logInfoDuration('Core // Products Route // Create variant succeeded', startedAt, {
            id: params.id,
            variantId: variant.id,
            variantName: variant.name,
          });
          return success(variant);
        } catch (error) {
          if (isNotFoundError(error)) {
            set.status = 404;
            return failure('PRODUCTS_NOT_FOUND', error.message);
          }

          set.status = 500;
          Log.error(
            `Core // Products Route // Create variant failed ${formatLogMetadata({
              error: toErrorMessage(error),
              id: params.id,
            })}`,
          );
          return failure('PRODUCT_VARIANTS_CREATE_FAILED', toErrorMessage(error));
        }
      },
      {
        params: t.Object({ id: t.String() }),
        body: productVariantCreateInput,
        response: {
          201: productVariantCreateResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    )
    .post(
      '/:id/variants/clone',
      async ({ params, body, set }) => {
        const startedAt = startLogTimer();
        try {
          const cloned = await dependencies.cloneVariantForProduct({
            productId: params.id,
            branchName: body.branchName,
            cloneType:
              body.cloneType === undefined
                ? undefined
                : (body.cloneType as 'auto' | 'local.copy' | 'git.clone_branch'),
            name: body.name,
            sourceVariantId: body.sourceVariantId,
            targetPath: body.targetPath,
            runAsync: body.runAsync,
          });

          set.status = cloned.clone.isAsync ? 202 : 201;
          logInfoDuration('Core // Products Route // Clone variant succeeded', startedAt, {
            cloneType: cloned.clone.cloneType,
            generatedBranchName: cloned.clone.generatedBranchName,
            generatedTargetPath: cloned.clone.generatedTargetPath,
            id: params.id,
            isAsync: cloned.clone.isAsync,
            variantId: cloned.variant.id,
          });
          return success(cloned);
        } catch (error) {
          if (isNotFoundError(error)) {
            set.status = 404;
            return failure('PRODUCTS_NOT_FOUND', error.message);
          }

          const message = toErrorMessage(error);
          if (message.includes('Workspace unresolved')) {
            set.status = 400;
            return failure('VARIANTS_CLONE_WORKSPACE_UNRESOLVED', message);
          }

          if (message.includes('Target path already exists')) {
            set.status = 400;
            return failure('VARIANTS_CLONE_TARGET_INVALID', message);
          }

          if (message.includes('Unsupported product locator')) {
            set.status = 400;
            return failure('VARIANTS_CLONE_UNSUPPORTED', message);
          }

          set.status = 500;
          Log.error(
            `Core // Products Route // Clone variant failed ${formatLogMetadata({
              error: message,
              id: params.id,
            })}`,
          );
          return failure('VARIANTS_CLONE_FAILED', message);
        }
      },
      {
        params: t.Object({ id: t.String() }),
        body: productVariantCloneInput,
        response: {
          201: productVariantCloneResponse,
          202: productVariantCloneResponse,
          400: apiFailureResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    );
};

export const productsRoutes = createProductsRoutes();
