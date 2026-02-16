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
            metadata: body.metadata,
          });
          set.status = 201;
          return success(actor);
        } catch (error) {
          const message = toErrorMessage(error);

          if (isNotFoundError(error)) {
            set.status = 404;
            return failure('ACTORS_NOT_FOUND', message);
          }

          if (isUnsupportedProviderError(message)) {
            set.status = 400;
            return failure('ACTORS_PROVIDER_UNSUPPORTED', message);
          }

          if (isDisabledProviderError(message)) {
            set.status = 400;
            return failure('ACTORS_PROVIDER_DISABLED', message);
          }

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
            set.status = 404;
            return failure('ACTORS_NOT_FOUND', error.message);
          }

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
            set.status = 404;
            return failure('ACTORS_NOT_FOUND', error.message);
          }

          set.status = 500;
          return failure('ACTORS_UPDATE_FAILED', toErrorMessage(error));
        }
      },
      {
        params: t.Object({ id: t.String() }),
        body: t.Object({
          title: t.Optional(t.Union([t.String(), t.Null()])),
          description: t.Optional(t.Union([t.String(), t.Null()])),
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
            set.status = 404;
            return failure('ACTORS_NOT_FOUND', error.message);
          }

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
            set.status = 404;
            return failure('ACTORS_NOT_FOUND', error.message);
          }

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
            set.status = 404;
            return failure('ACTORS_NOT_FOUND', error.message);
          }

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
            set.status = 404;
            return failure('ACTORS_NOT_FOUND', error.message);
          }

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
              nLastMessages: query.nLastMessages ? Number(query.nLastMessages) : undefined,
            }),
          );
        } catch (error) {
          if (isNotFoundError(error)) {
            set.status = 404;
            return failure('ACTORS_NOT_FOUND', error.message);
          }

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
            set.status = 404;
            return failure('ACTORS_NOT_FOUND', error.message);
          }

          set.status = 500;
          Log.error(
            `Core // Actors Route // Command failed ${formatLogMetadata({
              actorId: params.id,
              error: toErrorMessage(error),
            })}`,
          );
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
