import { Elysia, t } from 'elysia';

import {
  buildActorAttachById,
  createActor,
  deleteActorById,
  getActorById,
  listActorMessagesById,
  listActors,
  pollActorById,
  runActorCommandById,
  sendActorMessageById,
  updateActorById,
} from './actors.controller';
import { isNotFoundError } from '../common/controller.errors';
import { failure, success, toErrorMessage } from '../../utils/api-response';
import Log, { formatLogMetadata } from '../../utils/logging';

export interface ActorsRoutesDependencies {
  buildActorAttachById: typeof buildActorAttachById;
  createActor: typeof createActor;
  deleteActorById: typeof deleteActorById;
  getActorById: typeof getActorById;
  listActorMessagesById: typeof listActorMessagesById;
  listActors: typeof listActors;
  pollActorById: typeof pollActorById;
  runActorCommandById: typeof runActorCommandById;
  sendActorMessageById: typeof sendActorMessageById;
  updateActorById: typeof updateActorById;
}

const apiSuccessResponse = t.Object({
  ok: t.Literal(true),
  data: t.Any(),
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
    code: t.Literal('ACTORS_NOT_FOUND'),
    message: t.String(),
  }),
});

const isUnsupportedProviderError = (message: string): boolean => {
  return message.startsWith('Providers // Registry // Unsupported provider');
};

const isDisabledProviderError = (message: string): boolean => {
  return message.startsWith('Providers // Registry // Provider disabled');
};

const parseBooleanQuery = (value?: string): boolean => {
  if (!value) {
    return false;
  }

  return value !== 'false' && value !== '0';
};

const parseOptionalPositiveInt = (value?: string): number | undefined => {
  if (!value) {
    return undefined;
  }

  const parsed = Number(value);
  if (!Number.isFinite(parsed)) {
    return undefined;
  }

  const normalized = Math.floor(parsed);
  if (normalized <= 0) {
    return undefined;
  }

  return normalized;
};

const subAgentSchema: ReturnType<typeof t.Object> = t.Object({
  id: t.String(),
  parentId: t.Optional(t.Union([t.String(), t.Null()])),
  title: t.Optional(t.Union([t.String(), t.Null()])),
  status: t.Optional(t.Union([t.String(), t.Null()])),
  updatedAt: t.Optional(t.Union([t.String(), t.Null()])),
  depth: t.Optional(t.Union([t.Number(), t.Null()])),
  summary: t.Optional(t.Union([t.String(), t.Null()])),
  children: t.Optional(t.Array(t.Any())),
});

const logActorRouteError = (event: string, metadata: Record<string, string | number | boolean>) => {
  Log.error(`Core // Actors Route // ${event} ${formatLogMetadata(metadata)}`);
};

const logActorRouteWarn = (event: string, metadata: Record<string, string | number | boolean>) => {
  Log.warn(`Core // Actors Route // ${event} ${formatLogMetadata(metadata)}`);
};

export const createActorsRoutes = (
  dependencies: ActorsRoutesDependencies = {
    buildActorAttachById,
    createActor,
    deleteActorById,
    getActorById,
    listActorMessagesById,
    listActors,
    pollActorById,
    runActorCommandById,
    sendActorMessageById,
    updateActorById,
  },
) => {
  return new Elysia({ prefix: '/actors' })
    .get(
      '/',
      async ({ query, set }) => {
        try {
          const actors = await dependencies.listActors({
            cursor: query.cursor,
            limit: query.limit ? Number(query.limit) : undefined,
            productId: query.productId,
            provider: query.provider,
            status: query.status,
            variantId: query.variantId,
          });
          return success(actors);
        } catch (error) {
          logActorRouteError('List failed', {
            error: toErrorMessage(error),
            productId: query.productId ?? '-',
            provider: query.provider ?? '-',
            status: query.status ?? '-',
            variantId: query.variantId ?? '-',
          });
          set.status = 500;
          return failure('ACTORS_LIST_FAILED', toErrorMessage(error));
        }
      },
      {
        query: t.Object({
          cursor: t.Optional(t.String()),
          limit: t.Optional(t.String()),
          productId: t.Optional(t.String()),
          provider: t.Optional(t.String()),
          status: t.Optional(t.String()),
          variantId: t.Optional(t.String()),
        }),
        response: {
          200: apiSuccessResponse,
          500: apiFailureResponse,
        },
      },
    )
    .post(
      '/',
      async ({ body, set }) => {
        try {
          const actor = await dependencies.createActor({
            variantId: body.variantId,
            provider: body.provider,
            title: body.title,
            description: body.description,
            subAgents: body.subAgents,
            metadata: body.metadata,
          });
          set.status = 201;
          return success(actor);
        } catch (error) {
          const message = toErrorMessage(error);

          if (isNotFoundError(error)) {
            logActorRouteWarn('Create failed (variant not found)', {
              error: message,
              variantId: body.variantId,
              provider: body.provider ?? '-',
            });
            set.status = 404;
            return failure('ACTORS_NOT_FOUND', message);
          }

          if (isUnsupportedProviderError(message)) {
            logActorRouteWarn('Create failed (provider unsupported)', {
              error: message,
              provider: body.provider ?? '-',
              variantId: body.variantId,
            });
            set.status = 400;
            return failure('ACTORS_PROVIDER_UNSUPPORTED', message);
          }

          if (isDisabledProviderError(message)) {
            logActorRouteWarn('Create failed (provider disabled)', {
              error: message,
              provider: body.provider ?? '-',
              variantId: body.variantId,
            });
            set.status = 400;
            return failure('ACTORS_PROVIDER_DISABLED', message);
          }

          logActorRouteError('Create failed', {
            error: message,
            provider: body.provider ?? '-',
            variantId: body.variantId,
          });
          set.status = 500;
          return failure('ACTORS_CREATE_FAILED', message);
        }
      },
      {
        body: t.Object({
          variantId: t.String(),
          provider: t.Optional(t.String()),
          title: t.Optional(t.String()),
          description: t.Optional(t.String()),
          subAgents: t.Optional(t.Array(subAgentSchema)),
          metadata: t.Optional(t.Record(t.String(), t.Any())),
        }),
        response: {
          201: apiSuccessResponse,
          400: apiFailureResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    )
    .get(
      '/:id',
      async ({ params, set }) => {
        try {
          return success(await dependencies.getActorById(params.id));
        } catch (error) {
          if (isNotFoundError(error)) {
            logActorRouteWarn('Get failed (not found)', {
              actorId: params.id,
              error: error.message,
            });
            set.status = 404;
            return failure('ACTORS_NOT_FOUND', error.message);
          }

          logActorRouteError('Get failed', {
            actorId: params.id,
            error: toErrorMessage(error),
          });
          set.status = 500;
          return failure('ACTORS_GET_FAILED', toErrorMessage(error));
        }
      },
      {
        params: t.Object({ id: t.String() }),
        response: {
          200: apiSuccessResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    )
    .patch(
      '/:id',
      async ({ params, body, set }) => {
        try {
          return success(await dependencies.updateActorById(params.id, body));
        } catch (error) {
          if (isNotFoundError(error)) {
            logActorRouteWarn('Update failed (not found)', {
              actorId: params.id,
              error: error.message,
            });
            set.status = 404;
            return failure('ACTORS_NOT_FOUND', error.message);
          }

          logActorRouteError('Update failed', {
            actorId: params.id,
            error: toErrorMessage(error),
          });
          set.status = 500;
          return failure('ACTORS_UPDATE_FAILED', toErrorMessage(error));
        }
      },
      {
        params: t.Object({ id: t.String() }),
        body: t.Object({
          variantId: t.Optional(t.String()),
          title: t.Optional(t.Union([t.String(), t.Null()])),
          description: t.Optional(t.Union([t.String(), t.Null()])),
          subAgents: t.Optional(t.Union([t.Array(subAgentSchema), t.Null()])),
          metadata: t.Optional(t.Union([t.Record(t.String(), t.Any()), t.Null()])),
        }),
        response: {
          200: apiSuccessResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    )
    .delete(
      '/:id',
      async ({ params, query, set }) => {
        try {
          const terminate = parseBooleanQuery(query.terminate);
          return success(await dependencies.deleteActorById(params.id, { terminate }));
        } catch (error) {
          if (isNotFoundError(error)) {
            logActorRouteWarn('Delete failed (not found)', {
              actorId: params.id,
              error: error.message,
              terminate: parseBooleanQuery(query.terminate),
            });
            set.status = 404;
            return failure('ACTORS_NOT_FOUND', error.message);
          }

          logActorRouteError('Delete failed', {
            actorId: params.id,
            error: toErrorMessage(error),
            terminate: parseBooleanQuery(query.terminate),
          });
          set.status = 500;
          return failure('ACTORS_DELETE_FAILED', toErrorMessage(error));
        }
      },
      {
        params: t.Object({ id: t.String() }),
        query: t.Object({
          terminate: t.Optional(t.String()),
        }),
        response: {
          200: apiSuccessResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    )
    .post(
      '/:id/poll',
      async ({ params, set }) => {
        try {
          return success(await dependencies.pollActorById(params.id));
        } catch (error) {
          if (isNotFoundError(error)) {
            logActorRouteWarn('Poll failed (not found)', {
              actorId: params.id,
              error: error.message,
            });
            set.status = 404;
            return failure('ACTORS_NOT_FOUND', error.message);
          }

          logActorRouteError('Poll failed', {
            actorId: params.id,
            error: toErrorMessage(error),
          });
          set.status = 500;
          return failure('ACTORS_POLL_FAILED', toErrorMessage(error));
        }
      },
      {
        params: t.Object({ id: t.String() }),
        response: {
          200: apiSuccessResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    )
    .get(
      '/:id/attach',
      async ({ params, query, set }) => {
        try {
          return success(
            await dependencies.buildActorAttachById(params.id, {
              model: query.model,
              agent: query.agent,
            }),
          );
        } catch (error) {
          if (isNotFoundError(error)) {
            logActorRouteWarn('Attach failed (not found)', {
              actorId: params.id,
              error: error.message,
              model: query.model ?? '-',
              agent: query.agent ?? '-',
            });
            set.status = 404;
            return failure('ACTORS_NOT_FOUND', error.message);
          }

          logActorRouteError('Attach failed', {
            actorId: params.id,
            error: toErrorMessage(error),
            model: query.model ?? '-',
            agent: query.agent ?? '-',
          });
          set.status = 500;
          return failure('ACTORS_ATTACH_FAILED', toErrorMessage(error));
        }
      },
      {
        params: t.Object({ id: t.String() }),
        query: t.Object({
          model: t.Optional(t.String()),
          agent: t.Optional(t.String()),
        }),
        response: {
          200: apiSuccessResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    )
    .post(
      '/:id/messages',
      async ({ params, body, set }) => {
        try {
          return success(
            await dependencies.sendActorMessageById(params.id, {
              prompt: body.prompt,
              noReply: body.noReply,
              model: body.model,
              agent: body.agent,
            }),
          );
        } catch (error) {
          if (isNotFoundError(error)) {
            logActorRouteWarn('Message send failed (not found)', {
              actorId: params.id,
              error: error.message,
            });
            set.status = 404;
            return failure('ACTORS_NOT_FOUND', error.message);
          }

          logActorRouteError('Message send failed', {
            actorId: params.id,
            error: toErrorMessage(error),
            model: body.model ?? '-',
            agent: body.agent ?? '-',
          });
          set.status = 500;
          return failure('ACTORS_MESSAGE_SEND_FAILED', toErrorMessage(error));
        }
      },
      {
        params: t.Object({ id: t.String() }),
        body: t.Object({
          prompt: t.String(),
          noReply: t.Optional(t.Boolean()),
          model: t.Optional(t.String()),
          agent: t.Optional(t.String()),
        }),
        response: {
          200: apiSuccessResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    )
    .get(
      '/:id/messages',
      async ({ params, query, set }) => {
        try {
          return success(
            await dependencies.listActorMessagesById(params.id, {
              nLastMessages: parseOptionalPositiveInt(query.nLastMessages),
            }),
          );
        } catch (error) {
          if (isNotFoundError(error)) {
            logActorRouteWarn('Messages list failed (not found)', {
              actorId: params.id,
              error: error.message,
            });
            set.status = 404;
            return failure('ACTORS_NOT_FOUND', error.message);
          }

          logActorRouteError('Messages list failed', {
            actorId: params.id,
            error: toErrorMessage(error),
            nLastMessages: query.nLastMessages ?? '-',
          });
          set.status = 500;
          return failure('ACTORS_MESSAGES_LIST_FAILED', toErrorMessage(error));
        }
      },
      {
        params: t.Object({ id: t.String() }),
        query: t.Object({
          nLastMessages: t.Optional(t.String()),
        }),
        response: {
          200: apiSuccessResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    )
    .post(
      '/:id/commands',
      async ({ params, body, set }) => {
        try {
          return success(
            await dependencies.runActorCommandById(params.id, {
              command: body.command,
              args: body.args,
              model: body.model,
              agent: body.agent,
            }),
          );
        } catch (error) {
          if (isNotFoundError(error)) {
            logActorRouteWarn('Command failed (not found)', {
              actorId: params.id,
              error: error.message,
              command: body.command,
            });
            set.status = 404;
            return failure('ACTORS_NOT_FOUND', error.message);
          }

          set.status = 500;
          logActorRouteError('Command failed', {
            actorId: params.id,
            error: toErrorMessage(error),
            command: body.command,
            model: body.model ?? '-',
            agent: body.agent ?? '-',
          });
          return failure('ACTORS_COMMAND_FAILED', toErrorMessage(error));
        }
      },
      {
        params: t.Object({ id: t.String() }),
        body: t.Object({
          command: t.String(),
          args: t.Optional(t.String()),
          model: t.Optional(t.String()),
          agent: t.Optional(t.String()),
        }),
        response: {
          200: apiSuccessResponse,
          404: notFoundResponse,
          500: apiFailureResponse,
        },
      },
    );
};

export const actorsRoutes = createActorsRoutes();
