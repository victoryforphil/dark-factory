import { Elysia } from 'elysia';

import {
  abortMockAgentSession,
  buildMockAgentAttachCommand,
  createMockAgentSession,
  deleteMockAgentSession,
  getMockAgentDirectoryState,
  getMockAgentSessionState,
  getMockAgentSessionStatuses,
  listMockAgentSessionMessages,
  listMockAgentSessions,
  sendMockAgentSessionCommand,
  sendMockAgentSessionPrompt,
} from './mockagent.controller';
import {
  mockAgentAbortBodySchema,
  mockAgentApiFailureResponseSchema,
  mockAgentApiSuccessResponseSchema,
  mockAgentAttachQuerySchema,
  mockAgentCommandBodySchema,
  mockAgentCreateSessionBodySchema,
  mockAgentDirectoryQuerySchema,
  mockAgentPromptBodySchema,
  mockAgentMessagesQuerySchema,
  mockAgentSessionParamsSchema,
  mockAgentSessionQuerySchema,
} from './mockagent.schemas';
import { failure, success, toErrorMessage } from '../../../utils/api-response';

export interface MockAgentRoutesDependencies {
  abortMockAgentSession: typeof abortMockAgentSession;
  buildMockAgentAttachCommand: typeof buildMockAgentAttachCommand;
  createMockAgentSession: typeof createMockAgentSession;
  deleteMockAgentSession: typeof deleteMockAgentSession;
  getMockAgentDirectoryState: typeof getMockAgentDirectoryState;
  getMockAgentSessionState: typeof getMockAgentSessionState;
  getMockAgentSessionStatuses: typeof getMockAgentSessionStatuses;
  listMockAgentSessionMessages: typeof listMockAgentSessionMessages;
  listMockAgentSessions: typeof listMockAgentSessions;
  sendMockAgentSessionCommand: typeof sendMockAgentSessionCommand;
  sendMockAgentSessionPrompt: typeof sendMockAgentSessionPrompt;
}

const toFailure = (errorCode: string, error: unknown) => {
  const message = toErrorMessage(error);
  const status = message.startsWith('MockAgent // Session //') ? 404 : 500;

  return {
    status,
    body: failure(errorCode, message),
  };
};

export const createMockAgentRoutes = (
  dependencies: MockAgentRoutesDependencies = {
    abortMockAgentSession,
    buildMockAgentAttachCommand,
    createMockAgentSession,
    deleteMockAgentSession,
    getMockAgentDirectoryState,
    getMockAgentSessionState,
    getMockAgentSessionStatuses,
    listMockAgentSessionMessages,
    listMockAgentSessions,
    sendMockAgentSessionCommand,
    sendMockAgentSessionPrompt,
  },
) => {
  return new Elysia({ prefix: '/mockagent' })
    .get(
      '/state',
      async ({ query, set }) => {
        try {
          return success(await dependencies.getMockAgentDirectoryState({ directory: query.directory }));
        } catch (error) {
          const response = toFailure('MOCKAGENT_STATE_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        query: mockAgentDirectoryQuerySchema,
        response: {
          200: mockAgentApiSuccessResponseSchema,
          404: mockAgentApiFailureResponseSchema,
          500: mockAgentApiFailureResponseSchema,
        },
      },
    )
    .get(
      '/sessions',
      async ({ query, set }) => {
        try {
          return success(await dependencies.listMockAgentSessions({ directory: query.directory }));
        } catch (error) {
          const response = toFailure('MOCKAGENT_SESSIONS_LIST_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        query: mockAgentDirectoryQuerySchema,
        response: {
          200: mockAgentApiSuccessResponseSchema,
          404: mockAgentApiFailureResponseSchema,
          500: mockAgentApiFailureResponseSchema,
        },
      },
    )
    .post(
      '/sessions',
      async ({ body, set }) => {
        try {
          const session = await dependencies.createMockAgentSession({
            directory: body.directory,
            title: body.title,
          });
          const attach = await dependencies.buildMockAgentAttachCommand({
            directory: body.directory,
            sessionId: session.id,
          });

          set.status = 201;
          return success({ session, attach });
        } catch (error) {
          const response = toFailure('MOCKAGENT_SESSION_CREATE_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        body: mockAgentCreateSessionBodySchema,
        response: {
          201: mockAgentApiSuccessResponseSchema,
          404: mockAgentApiFailureResponseSchema,
          500: mockAgentApiFailureResponseSchema,
        },
      },
    )
    .get(
      '/sessions/status',
      async ({ query, set }) => {
        try {
          return success(await dependencies.getMockAgentSessionStatuses({ directory: query.directory }));
        } catch (error) {
          const response = toFailure('MOCKAGENT_SESSION_STATUS_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        query: mockAgentDirectoryQuerySchema,
        response: {
          200: mockAgentApiSuccessResponseSchema,
          404: mockAgentApiFailureResponseSchema,
          500: mockAgentApiFailureResponseSchema,
        },
      },
    )
    .get(
      '/sessions/:id',
      async ({ params, query, set }) => {
        try {
          return success(
            await dependencies.getMockAgentSessionState({
              directory: query.directory,
              id: params.id,
              includeMessages: query.includeMessages,
            }),
          );
        } catch (error) {
          const response = toFailure('MOCKAGENT_SESSION_GET_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        params: mockAgentSessionParamsSchema,
        query: mockAgentSessionQuerySchema,
        response: {
          200: mockAgentApiSuccessResponseSchema,
          404: mockAgentApiFailureResponseSchema,
          500: mockAgentApiFailureResponseSchema,
        },
      },
    )
    .get(
      '/sessions/:id/messages',
      async ({ params, query, set }) => {
        try {
          const limitRaw = query.limit;
          const limit = limitRaw ? Number.parseInt(limitRaw, 10) : undefined;
          return success(
            await dependencies.listMockAgentSessionMessages({
              directory: query.directory,
              id: params.id,
              limit,
            }),
          );
        } catch (error) {
          const response = toFailure('MOCKAGENT_SESSION_MESSAGES_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        params: mockAgentSessionParamsSchema,
        query: mockAgentMessagesQuerySchema,
        response: {
          200: mockAgentApiSuccessResponseSchema,
          404: mockAgentApiFailureResponseSchema,
          500: mockAgentApiFailureResponseSchema,
        },
      },
    )
    .get(
      '/sessions/:id/attach',
      async ({ params, query, set }) => {
        try {
          return success(
            await dependencies.buildMockAgentAttachCommand({
              directory: query.directory,
              sessionId: params.id,
              model: query.model,
              agent: query.agent,
            }),
          );
        } catch (error) {
          const response = toFailure('MOCKAGENT_SESSION_ATTACH_COMMAND_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        params: mockAgentSessionParamsSchema,
        query: mockAgentAttachQuerySchema,
        response: {
          200: mockAgentApiSuccessResponseSchema,
          404: mockAgentApiFailureResponseSchema,
          500: mockAgentApiFailureResponseSchema,
        },
      },
    )
    .post(
      '/sessions/:id/command',
      async ({ params, body, set }) => {
        try {
          return success(
            await dependencies.sendMockAgentSessionCommand({
              directory: body.directory,
              id: params.id,
              command: body.command,
            }),
          );
        } catch (error) {
          const response = toFailure('MOCKAGENT_SESSION_COMMAND_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        params: mockAgentSessionParamsSchema,
        body: mockAgentCommandBodySchema,
        response: {
          200: mockAgentApiSuccessResponseSchema,
          404: mockAgentApiFailureResponseSchema,
          500: mockAgentApiFailureResponseSchema,
        },
      },
    )
    .post(
      '/sessions/:id/prompt',
      async ({ params, body, set }) => {
        try {
          return success(
            await dependencies.sendMockAgentSessionPrompt({
              directory: body.directory,
              id: params.id,
              prompt: body.prompt,
              noReply: body.noReply,
            }),
          );
        } catch (error) {
          const response = toFailure('MOCKAGENT_SESSION_PROMPT_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        params: mockAgentSessionParamsSchema,
        body: mockAgentPromptBodySchema,
        response: {
          200: mockAgentApiSuccessResponseSchema,
          404: mockAgentApiFailureResponseSchema,
          500: mockAgentApiFailureResponseSchema,
        },
      },
    )
    .post(
      '/sessions/:id/abort',
      async ({ params, body, set }) => {
        try {
          return success(
            await dependencies.abortMockAgentSession({
              directory: body.directory,
              id: params.id,
            }),
          );
        } catch (error) {
          const response = toFailure('MOCKAGENT_SESSION_ABORT_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        params: mockAgentSessionParamsSchema,
        body: mockAgentAbortBodySchema,
        response: {
          200: mockAgentApiSuccessResponseSchema,
          404: mockAgentApiFailureResponseSchema,
          500: mockAgentApiFailureResponseSchema,
        },
      },
    )
    .delete(
      '/sessions/:id',
      async ({ params, query, set }) => {
        try {
          return success(
            await dependencies.deleteMockAgentSession({
              directory: query.directory,
              id: params.id,
            }),
          );
        } catch (error) {
          const response = toFailure('MOCKAGENT_SESSION_DELETE_FAILED', error);
          set.status = response.status;
          return response.body;
        }
      },
      {
        params: mockAgentSessionParamsSchema,
        query: mockAgentDirectoryQuerySchema,
        response: {
          200: mockAgentApiSuccessResponseSchema,
          404: mockAgentApiFailureResponseSchema,
          500: mockAgentApiFailureResponseSchema,
        },
      },
    );
};

export const mockAgentRoutes = createMockAgentRoutes();
