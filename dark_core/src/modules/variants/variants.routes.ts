import { Elysia, t } from 'elysia';
import {
  ActorPlain,
} from '../../../../generated/prismabox/Actor';
import {
  VariantInputCreate,
  VariantPlain,
  VariantPlainInputUpdate,
} from '../../../../generated/prismabox/Variant';
import { importVariantActors } from '../actors/actors.controller';

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
import Log, { formatLogMetadata, logRouteStart, logRouteSuccess } from '../../utils/logging';

export interface VariantsRoutesDependencies {
  createVariant: typeof createVariant;
  deleteVariantById: typeof deleteVariantById;
  getVariantById: typeof getVariantById;
  importVariantActors: typeof importVariantActors;
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

const variantImportActorsResponse = t.Object({
  ok: t.Literal(true),
  data: t.Object({
    variantId: t.String(),
    provider: t.String(),
    discovered: t.Number(),
    created: t.Number(),
    updated: t.Number(),
    actors: t.Array(ActorPlain),
  }),
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

const isUnsupportedProviderError = (message: string): boolean => {
  return message.startsWith('Providers // Registry // Unsupported provider');
};

const isDisabledProviderError = (message: string): boolean => {
  return message.startsWith('Providers // Registry // Provider disabled');
};

const isProviderImportUnsupportedError = (message: string): boolean => {
  return message.startsWith('Providers // Registry // Provider import unsupported');
};

const parsePollQuery = (poll?: string): boolean => {
  if (!poll) {
    return true;
  }

  return poll !== 'false' && poll !== '0';
};

const parseDryQuery = (dry?: string): boolean => {
  if (!dry) {
    return false;
  }

  return dry === 'true' || dry === '1';
};

export const createVariantsRoutes = (
  dependencies: VariantsRoutesDependencies = {
    createVariant,
    deleteVariantById,
    getVariantById,
    importVariantActors,
    listVariants,
    syncVariantGitInfo,
    updateVariantById,
  },
): Elysia => {
  return new Elysia({ prefix: '/variants' })
    .get(
      '/',
      async ({ query, set }) => {
        const startedAt = logRouteStart('Variants // List', {
          cursor: query.cursor ?? null,
          limit: query.limit ?? null,
          locator: query.locator ?? null,
          name: query.name ?? null,
          poll: query.poll ?? null,
          productId: query.productId ?? null,
        });
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

          logRouteSuccess('Variants // List', startedAt, {
            count: variants.length,
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
        const startedAt = logRouteStart('Variants // Create', {
          locator: body.locator,
          name: body.name ?? null,
          productId: body.product?.connect?.id ?? null,
        });
        try {
          const createdVariant = await dependencies.createVariant(body);
          set.status = 201;
          logRouteSuccess('Variants // Create', startedAt, {
            variantId: createdVariant.id,
          });
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
        const startedAt = logRouteStart('Variants // Get', {
          id: params.id,
          poll: query.poll ?? null,
        });
        try {
          const variant = await dependencies.getVariantById(params.id, {
            poll: parsePollQuery(query.poll),
          });
          logRouteSuccess('Variants // Get', startedAt, {
            id: params.id,
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
        const startedAt = logRouteStart('Variants // Poll', {
          id: params.id,
          poll: query.poll ?? null,
        });
        try {
          const poll = parsePollQuery(query.poll);
          const variant = poll
            ? await dependencies.syncVariantGitInfo(params.id)
            : await dependencies.getVariantById(params.id, { poll: false });
          logRouteSuccess('Variants // Poll', startedAt, {
            id: params.id,
          });
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
    .post(
      '/:id/actors/import',
      async ({ params, body, set }) => {
        try {
          return success(
            await dependencies.importVariantActors({
              variantId: params.id,
              provider: body.provider,
            }),
          );
        } catch (error) {
          if (isNotFoundError(error)) {
            set.status = 404;
            Log.warn(
              `Core // Variants Route // Import actors variant not found ${formatLogMetadata({ id: params.id })}`,
            );
            return failure('VARIANTS_NOT_FOUND', error.message);
          }

          const message = toErrorMessage(error);

          if (isUnsupportedProviderError(message)) {
            set.status = 400;
            Log.warn(
              `Core // Variants Route // Import actors provider unsupported ${formatLogMetadata({
                error: message,
                id: params.id,
                provider: body.provider ?? '-',
              })}`,
            );
            return failure('VARIANTS_PROVIDER_UNSUPPORTED', message);
          }

          if (isDisabledProviderError(message)) {
            set.status = 400;
            Log.warn(
              `Core // Variants Route // Import actors provider disabled ${formatLogMetadata({
                error: message,
                id: params.id,
                provider: body.provider ?? '-',
              })}`,
            );
            return failure('VARIANTS_PROVIDER_DISABLED', message);
          }

          if (isProviderImportUnsupportedError(message)) {
            set.status = 400;
            Log.warn(
              `Core // Variants Route // Import actors provider unsupported for import ${formatLogMetadata({
                error: message,
                id: params.id,
                provider: body.provider ?? '-',
              })}`,
            );
            return failure('VARIANTS_PROVIDER_IMPORT_UNSUPPORTED', message);
          }

          Log.error(
            `Core // Variants Route // Import actors failed ${formatLogMetadata({
              error: message,
              id: params.id,
              provider: body.provider ?? '-',
            })}`,
          );
          set.status = 500;
          return failure('VARIANTS_IMPORT_ACTORS_FAILED', message);
        }
      },
      {
        params: t.Object({ id: t.String() }),
        body: t.Object({
          provider: t.Optional(t.String()),
        }),
        response: {
          200: variantImportActorsResponse,
          400: apiFailureResponse,
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
      async ({ params, query, set }) => {
        const startedAt = logRouteStart('Variants // Delete', {
          dry: query.dry ?? null,
          id: params.id,
        });
        try {
          const deletedVariant = await dependencies.deleteVariantById(params.id, {
            dry: parseDryQuery(query.dry),
          });
          logRouteSuccess('Variants // Delete', startedAt, {
            id: params.id,
          });
          return success(deletedVariant);
        } catch (error) {
          if (isNotFoundError(error)) {
            set.status = 404;
            Log.warn(
              `Core // Variants Route // Delete variant not found ${formatLogMetadata({ id: params.id })}`,
            );
            return failure('VARIANTS_NOT_FOUND', error.message);
          }

          const message = toErrorMessage(error);
          if (message.includes('Undo blocked')) {
            set.status = 400;
            Log.warn(
              `Core // Variants Route // Delete undo blocked ${formatLogMetadata({
                dry: query.dry ?? null,
                error: message,
                id: params.id,
              })}`,
            );
            return failure('VARIANTS_DELETE_UNDO_BLOCKED', message);
          }

          Log.error(
            `Core // Variants Route // Delete failed ${formatLogMetadata({
              dry: query.dry ?? null,
              error: message,
              id: params.id,
            })}`,
          );
          set.status = 500;
          return failure('VARIANTS_DELETE_FAILED', message);
        }
      },
      {
        params: t.Object({ id: t.String() }),
        query: t.Object({
          dry: t.Optional(t.String()),
        }),
        response: {
          200: variantDeleteResponse,
          400: apiFailureResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    );
};

export const variantsRoutes = createVariantsRoutes();
