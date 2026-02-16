import { t } from 'elysia';

export const mockAgentApiSuccessResponseSchema = t.Object({
  ok: t.Literal(true),
  data: t.Any(),
});

export const mockAgentApiFailureResponseSchema = t.Object({
  ok: t.Literal(false),
  error: t.Object({
    code: t.String(),
    message: t.String(),
  }),
});

export const mockAgentDirectoryQuerySchema = t.Object({
  directory: t.String(),
});

export const mockAgentSessionParamsSchema = t.Object({
  id: t.String(),
});

export const mockAgentCreateSessionBodySchema = t.Object({
  directory: t.String(),
  title: t.Optional(t.String()),
});

export const mockAgentCommandBodySchema = t.Object({
  directory: t.String(),
  command: t.String(),
});

export const mockAgentPromptBodySchema = t.Object({
  directory: t.String(),
  prompt: t.String(),
  noReply: t.Optional(t.Boolean()),
});

export const mockAgentAbortBodySchema = t.Object({
  directory: t.String(),
});

export const mockAgentSessionQuerySchema = t.Object({
  directory: t.String(),
  includeMessages: t.Optional(t.Boolean()),
});

export const mockAgentAttachQuerySchema = t.Object({
  directory: t.String(),
  model: t.Optional(t.String()),
  agent: t.Optional(t.String()),
});

export const mockAgentMessagesQuerySchema = t.Object({
  directory: t.String(),
  limit: t.Optional(t.String()),
});
