import { Elysia, t } from 'elysia';

import {
  abortOpencodeSession,
  buildOpencodeTuiAttachCommand,
  createOpencodeSession,
  deleteOpencodeSession,
  getOpencodeDirectoryState,
  getOpencodeSessionState,
  listOpencodeSessions,
  sendOpencodeSessionCommand,
  sendOpencodeSessionPrompt,
} from './opencode.controller';
import { failure, success, toErrorMessage } from '../../utils/api-response';

export interface OpencodeRoutesDependencies {
  abortOpencodeSession: typeof abortOpencodeSession;
  buildOpencodeTuiAttachCommand: typeof buildOpencodeTuiAttachCommand;
  createOpencodeSession: typeof createOpencodeSession;
  deleteOpencodeSession: typeof deleteOpencodeSession;
  getOpencodeDirectoryState: typeof getOpencodeDirectoryState;
  getOpencodeSessionState: typeof getOpencodeSessionState;
  listOpencodeSessions: typeof listOpencodeSessions;
  sendOpencodeSessionCommand: typeof sendOpencodeSessionCommand;
  sendOpencodeSessionPrompt: typeof sendOpencodeSessionPrompt;
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

const toFailure = (errorCode: string, error: unknown) => {
  const message = toErrorMessage(error);
  const status = message.startsWith('OpenCode // Directory //') ? 400 : 500;

  return {
    status,
    body: failure(errorCode, message),
  };
};

export const createOpencodeRoutes = (
  dependencies: OpencodeRoutesDependencies = {
    abortOpencodeSession,
    buildOpencodeTuiAttachCommand,
    createOpencodeSession,
    deleteOpencodeSession,
    getOpencodeDirectoryState,
    getOpencodeSessionState,
    listOpencodeSessions,
    sendOpencodeSessionCommand,
    sendOpencodeSessionPrompt,
  },
): Elysia => {
  return new Elysia({ prefix: '/opencode' })
    .get(
      '/state',
      async ({ query, set }) => {
        try {
          return success(await dependencies.getOpencodeDirectoryState({ directory: query.directory }));
        } catch (error) {
          const response = toFailure('OPENCODE_STATE_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        query: t.Object({
          directory: t.String(),
        }),
        response: {
          200: apiSuccessResponse,
          400: apiFailureResponse,
          500: apiFailureResponse,
        },
      },
    )
    .get(
      '/sessions',
      async ({ query, set }) => {
        try {
          return success(await dependencies.listOpencodeSessions({ directory: query.directory }));
        } catch (error) {
          const response = toFailure('OPENCODE_SESSIONS_LIST_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        query: t.Object({
          directory: t.String(),
        }),
        response: {
          200: apiSuccessResponse,
          400: apiFailureResponse,
          500: apiFailureResponse,
        },
      },
    )
    .post(
      '/sessions',
      async ({ body, set }) => {
        try {
          const session = await dependencies.createOpencodeSession({
            directory: body.directory,
            title: body.title,
          });
          const attach = await dependencies.buildOpencodeTuiAttachCommand({
            directory: body.directory,
            sessionId: session.id,
          });

          set.status = 201;
          return success({ session, attach });
        } catch (error) {
          const response = toFailure('OPENCODE_SESSION_CREATE_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        body: t.Object({
          directory: t.String(),
          title: t.Optional(t.String()),
        }),
        response: {
          201: apiSuccessResponse,
          400: apiFailureResponse,
          500: apiFailureResponse,
        },
      },
    )
    .get(
      '/sessions/:id',
      async ({ params, query, set }) => {
        try {
          return success(
            await dependencies.getOpencodeSessionState({
              directory: query.directory,
              id: params.id,
              includeMessages: query.includeMessages,
            }),
          );
        } catch (error) {
          const response = toFailure('OPENCODE_SESSION_GET_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        params: t.Object({
          id: t.String(),
        }),
        query: t.Object({
          directory: t.String(),
          includeMessages: t.Optional(t.Boolean()),
        }),
        response: {
          200: apiSuccessResponse,
          400: apiFailureResponse,
          500: apiFailureResponse,
        },
      },
    )
    .get(
      '/sessions/:id/attach',
      async ({ params, query, set }) => {
        try {
          return success(
            await dependencies.buildOpencodeTuiAttachCommand({
              directory: query.directory,
              sessionId: params.id,
              model: query.model,
              agent: query.agent,
            }),
          );
        } catch (error) {
          const response = toFailure('OPENCODE_SESSION_ATTACH_COMMAND_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        params: t.Object({
          id: t.String(),
        }),
        query: t.Object({
          directory: t.String(),
          model: t.Optional(t.String()),
          agent: t.Optional(t.String()),
        }),
        response: {
          200: apiSuccessResponse,
          400: apiFailureResponse,
          500: apiFailureResponse,
        },
      },
    )
    .post(
      '/sessions/:id/command',
      async ({ params, body, set }) => {
        try {
          return success(
            await dependencies.sendOpencodeSessionCommand({
              directory: body.directory,
              id: params.id,
              command: body.command,
            }),
          );
        } catch (error) {
          const response = toFailure('OPENCODE_SESSION_COMMAND_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        params: t.Object({
          id: t.String(),
        }),
        body: t.Object({
          directory: t.String(),
          command: t.String(),
        }),
        response: {
          200: apiSuccessResponse,
          400: apiFailureResponse,
          500: apiFailureResponse,
        },
      },
    )
    .post(
      '/sessions/:id/prompt',
      async ({ params, body, set }) => {
        try {
          return success(
            await dependencies.sendOpencodeSessionPrompt({
              directory: body.directory,
              id: params.id,
              prompt: body.prompt,
              noReply: body.noReply,
            }),
          );
        } catch (error) {
          const response = toFailure('OPENCODE_SESSION_PROMPT_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        params: t.Object({
          id: t.String(),
        }),
        body: t.Object({
          directory: t.String(),
          prompt: t.String(),
          noReply: t.Optional(t.Boolean()),
        }),
        response: {
          200: apiSuccessResponse,
          400: apiFailureResponse,
          500: apiFailureResponse,
        },
      },
    )
    .post(
      '/sessions/:id/abort',
      async ({ params, body, set }) => {
        try {
          return success(
            await dependencies.abortOpencodeSession({
              directory: body.directory,
              id: params.id,
            }),
          );
        } catch (error) {
          const response = toFailure('OPENCODE_SESSION_ABORT_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        params: t.Object({
          id: t.String(),
        }),
        body: t.Object({
          directory: t.String(),
        }),
        response: {
          200: apiSuccessResponse,
          400: apiFailureResponse,
          500: apiFailureResponse,
        },
      },
    )
    .delete(
      '/sessions/:id',
      async ({ params, query, set }) => {
        try {
          return success(
            await dependencies.deleteOpencodeSession({
              directory: query.directory,
              id: params.id,
            }),
          );
        } catch (error) {
          const response = toFailure('OPENCODE_SESSION_DELETE_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        params: t.Object({
          id: t.String(),
        }),
        query: t.Object({
          directory: t.String(),
        }),
        response: {
          200: apiSuccessResponse,
          400: apiFailureResponse,
          500: apiFailureResponse,
        },
      },
    );
};

export const opencodeRoutes = createOpencodeRoutes();
