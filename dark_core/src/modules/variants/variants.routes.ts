import { Elysia, t } from 'elysia';
import {
  VariantInputCreate,
  VariantPlain,
  VariantPlainInputUpdate,
} from '../../../../generated/prismabox/Variant';

import {
  createVariant,
  deleteVariantById,
  getVariantById,
  listVariants,
  syncVariantGitInfo,
  updateVariantById,
} from './variants.controller';
import { isNotFoundError } from '../common/controller.errors';
import { failure, success, toErrorMessage } from '../../utils/api-response';
import Log, { formatLogMetadata } from '../../utils/logging';

export interface VariantsRoutesDependencies {
  createVariant: typeof createVariant;
  deleteVariantById: typeof deleteVariantById;
  getVariantById: typeof getVariantById;
  listVariants: typeof listVariants;
  syncVariantGitInfo: typeof syncVariantGitInfo;
  updateVariantById: typeof updateVariantById;
}

const variantsListResponse = t.Object({
  ok: t.Literal(true),
  data: t.Array(VariantPlain),
});

const variantCreateResponse = t.Object({
  ok: t.Literal(true),
  data: VariantPlain,
});

const variantGetResponse = t.Object({
  ok: t.Literal(true),
  data: VariantPlain,
});

const variantUpdateResponse = t.Object({
  ok: t.Literal(true),
  data: VariantPlain,
});

const variantDeleteResponse = t.Object({
  ok: t.Literal(true),
  data: VariantPlain,
});

const variantPollResponse = t.Object({
  ok: t.Literal(true),
  data: VariantPlain,
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
    code: t.Literal('VARIANTS_NOT_FOUND'),
    message: t.String(),
  }),
});

const parsePollQuery = (poll?: string): boolean => {
  if (!poll) {
    return true;
  }

  return poll !== 'false' && poll !== '0';
};

export const createVariantsRoutes = (
  dependencies: VariantsRoutesDependencies = {
    createVariant,
    deleteVariantById,
    getVariantById,
    listVariants,
    syncVariantGitInfo,
    updateVariantById,
  },
): Elysia => {
  return new Elysia({ prefix: '/variants' })
    .get(
      '/',
      async ({ query, set }) => {
        try {
          const poll = parsePollQuery(query.poll);
          const variants = await dependencies.listVariants({
            cursor: query.cursor,
            limit: query.limit ? Number(query.limit) : undefined,
            productId: query.productId,
            locator: query.locator,
            name: query.name,
            poll,
          });

          return success(variants);
        } catch (error) {
          Log.error(
            `Core // Variants Route // List failed ${formatLogMetadata({ error: toErrorMessage(error) })}`,
          );
          set.status = 500;
          return failure('VARIANTS_LIST_FAILED', toErrorMessage(error));
        }
      },
      {
        query: t.Object({
          cursor: t.Optional(t.String()),
          limit: t.Optional(t.String()),
          productId: t.Optional(t.String()),
          locator: t.Optional(t.String()),
          name: t.Optional(t.String()),
          poll: t.Optional(t.String()),
        }),
        response: {
          200: variantsListResponse,
          500: apiFailureResponse,
        },
      },
    )
    .post(
      '/',
      async ({ body, set }) => {
        try {
          const createdVariant = await dependencies.createVariant(body);
          set.status = 201;
          return success(createdVariant);
        } catch (error) {
          Log.error(
            `Core // Variants Route // Create failed ${formatLogMetadata({ error: toErrorMessage(error) })}`,
          );
          set.status = 500;
          return failure('VARIANTS_CREATE_FAILED', toErrorMessage(error));
        }
      },
      {
        body: VariantInputCreate,
        response: {
          201: variantCreateResponse,
          500: apiFailureResponse,
        },
      },
    )
    .get(
      '/:id',
      async ({ params, query, set }) => {
        try {
          const variant = await dependencies.getVariantById(params.id, {
            poll: parsePollQuery(query.poll),
          });
          return success(variant);
        } catch (error) {
          if (isNotFoundError(error)) {
            set.status = 404;
            Log.warn(
              `Core // Variants Route // Variant not found ${formatLogMetadata({ id: params.id })}`,
            );
            return failure('VARIANTS_NOT_FOUND', error.message);
          }

          Log.error(
            `Core // Variants Route // Get failed ${formatLogMetadata({
              error: toErrorMessage(error),
              id: params.id,
            })}`,
          );
          set.status = 500;
          return failure('VARIANTS_GET_FAILED', toErrorMessage(error));
        }
      },
      {
        params: t.Object({ id: t.String() }),
        query: t.Object({
          poll: t.Optional(t.String()),
        }),
        response: {
          200: variantGetResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    )
    .post(
      '/:id/poll',
      async ({ params, query, set }) => {
        try {
          const poll = parsePollQuery(query.poll);
          const variant = poll
            ? await dependencies.syncVariantGitInfo(params.id)
            : await dependencies.getVariantById(params.id, { poll: false });
          return success(variant);
        } catch (error) {
          if (isNotFoundError(error)) {
            set.status = 404;
            Log.warn(
              `Core // Variants Route // Poll variant not found ${formatLogMetadata({ id: params.id })}`,
            );
            return failure('VARIANTS_NOT_FOUND', error.message);
          }

          Log.error(
            `Core // Variants Route // Poll failed ${formatLogMetadata({
              error: toErrorMessage(error),
              id: params.id,
            })}`,
          );
          set.status = 500;
          return failure('VARIANTS_POLL_FAILED', toErrorMessage(error));
        }
      },
      {
        params: t.Object({ id: t.String() }),
        query: t.Object({
          poll: t.Optional(t.String()),
        }),
        response: {
          200: variantPollResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    )
    .patch(
      '/:id',
      async ({ params, body, set }) => {
        try {
          const updatedVariant = await dependencies.updateVariantById(params.id, body);
          return success(updatedVariant);
        } catch (error) {
          if (isNotFoundError(error)) {
            set.status = 404;
            Log.warn(
              `Core // Variants Route // Update variant not found ${formatLogMetadata({ id: params.id })}`,
            );
            return failure('VARIANTS_NOT_FOUND', error.message);
          }

          Log.error(
            `Core // Variants Route // Update failed ${formatLogMetadata({
              error: toErrorMessage(error),
              id: params.id,
            })}`,
          );
          set.status = 500;
          return failure('VARIANTS_UPDATE_FAILED', toErrorMessage(error));
        }
      },
      {
        params: t.Object({ id: t.String() }),
        body: VariantPlainInputUpdate,
        response: {
          200: variantUpdateResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    )
    .delete(
      '/:id',
      async ({ params, set }) => {
        try {
          const deletedVariant = await dependencies.deleteVariantById(params.id);
          return success(deletedVariant);
        } catch (error) {
          if (isNotFoundError(error)) {
            set.status = 404;
            Log.warn(
              `Core // Variants Route // Delete variant not found ${formatLogMetadata({ id: params.id })}`,
            );
            return failure('VARIANTS_NOT_FOUND', error.message);
          }

          Log.error(
            `Core // Variants Route // Delete failed ${formatLogMetadata({
              error: toErrorMessage(error),
              id: params.id,
            })}`,
          );
          set.status = 500;
          return failure('VARIANTS_DELETE_FAILED', toErrorMessage(error));
        }
      },
      {
        params: t.Object({ id: t.String() }),
        response: {
          200: variantDeleteResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    );
};

export const variantsRoutes = createVariantsRoutes();
