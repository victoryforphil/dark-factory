import { t } from "elysia";

import { __transformDate__ } from "./__transformDate__";

import { __nullable__ } from "./__nullable__";

export const ActorPlain = t.Object(
  {
    id: t.String(),
    variantId: t.String(),
    provider: t.String(),
    actorLocator: t.String(),
    workingLocator: t.String(),
    providerSessionId: __nullable__(t.String()),
    status: t.String(),
    title: __nullable__(t.String()),
    description: __nullable__(t.String()),
    connectionInfo: __nullable__(t.Any()),
    attachCommand: __nullable__(t.String()),
    subAgents: __nullable__(t.Any()),
    metadata: __nullable__(t.Any()),
    createdAt: t.Date(),
    updatedAt: t.Date(),
  },
  { additionalProperties: false },
);

export const ActorRelations = t.Object(
  {
    variant: t.Object(
      {
        id: t.String(),
        productId: t.String(),
        name: t.String(),
        locator: t.String(),
        gitInfo: __nullable__(t.Any()),
        gitInfoUpdatedAt: __nullable__(t.Date()),
        gitInfoLastPolledAt: __nullable__(t.Date()),
        createdAt: t.Date(),
        updatedAt: t.Date(),
      },
      { additionalProperties: false },
    ),
  },
  { additionalProperties: false },
);

export const ActorPlainInputCreate = t.Object(
  {
    provider: t.String(),
    actorLocator: t.String(),
    workingLocator: t.String(),
    status: t.Optional(t.String()),
    title: t.Optional(__nullable__(t.String())),
    description: t.Optional(__nullable__(t.String())),
    connectionInfo: t.Optional(__nullable__(t.Any())),
    attachCommand: t.Optional(__nullable__(t.String())),
    subAgents: t.Optional(__nullable__(t.Any())),
    metadata: t.Optional(__nullable__(t.Any())),
  },
  { additionalProperties: false },
);

export const ActorPlainInputUpdate = t.Object(
  {
    provider: t.Optional(t.String()),
    actorLocator: t.Optional(t.String()),
    workingLocator: t.Optional(t.String()),
    status: t.Optional(t.String()),
    title: t.Optional(__nullable__(t.String())),
    description: t.Optional(__nullable__(t.String())),
    connectionInfo: t.Optional(__nullable__(t.Any())),
    attachCommand: t.Optional(__nullable__(t.String())),
    subAgents: t.Optional(__nullable__(t.Any())),
    metadata: t.Optional(__nullable__(t.Any())),
  },
  { additionalProperties: false },
);

export const ActorRelationsInputCreate = t.Object(
  {
    variant: t.Object(
      {
        connect: t.Object(
          {
            id: t.String({ additionalProperties: false }),
          },
          { additionalProperties: false },
        ),
      },
      { additionalProperties: false },
    ),
  },
  { additionalProperties: false },
);

export const ActorRelationsInputUpdate = t.Partial(
  t.Object(
    {
      variant: t.Object(
        {
          connect: t.Object(
            {
              id: t.String({ additionalProperties: false }),
            },
            { additionalProperties: false },
          ),
        },
        { additionalProperties: false },
      ),
    },
    { additionalProperties: false },
  ),
);

export const ActorWhere = t.Partial(
  t.Recursive(
    (Self) =>
      t.Object(
        {
          AND: t.Union([Self, t.Array(Self, { additionalProperties: false })]),
          NOT: t.Union([Self, t.Array(Self, { additionalProperties: false })]),
          OR: t.Array(Self, { additionalProperties: false }),
          id: t.String(),
          variantId: t.String(),
          provider: t.String(),
          actorLocator: t.String(),
          workingLocator: t.String(),
          providerSessionId: t.String(),
          status: t.String(),
          title: t.String(),
          description: t.String(),
          connectionInfo: t.Any(),
          attachCommand: t.String(),
          subAgents: t.Any(),
          metadata: t.Any(),
          createdAt: t.Date(),
          updatedAt: t.Date(),
        },
        { additionalProperties: false },
      ),
    { $id: "Actor" },
  ),
);

export const ActorWhereUnique = t.Recursive(
  (Self) =>
    t.Intersect(
      [
        t.Partial(
          t.Object(
            {
              id: t.String(),
              variantId_provider_providerSessionId: t.Object(
                {
                  variantId: t.String(),
                  provider: t.String(),
                  providerSessionId: t.String(),
                },
                { additionalProperties: false },
              ),
            },
            { additionalProperties: false },
          ),
          { additionalProperties: false },
        ),
        t.Union(
          [
            t.Object({ id: t.String() }),
            t.Object({
              variantId_provider_providerSessionId: t.Object(
                {
                  variantId: t.String(),
                  provider: t.String(),
                  providerSessionId: t.String(),
                },
                { additionalProperties: false },
              ),
            }),
          ],
          { additionalProperties: false },
        ),
        t.Partial(
          t.Object({
            AND: t.Union([
              Self,
              t.Array(Self, { additionalProperties: false }),
            ]),
            NOT: t.Union([
              Self,
              t.Array(Self, { additionalProperties: false }),
            ]),
            OR: t.Array(Self, { additionalProperties: false }),
          }),
          { additionalProperties: false },
        ),
        t.Partial(
          t.Object(
            {
              id: t.String(),
              variantId: t.String(),
              provider: t.String(),
              actorLocator: t.String(),
              workingLocator: t.String(),
              providerSessionId: t.String(),
              status: t.String(),
              title: t.String(),
              description: t.String(),
              connectionInfo: t.Any(),
              attachCommand: t.String(),
              subAgents: t.Any(),
              metadata: t.Any(),
              createdAt: t.Date(),
              updatedAt: t.Date(),
            },
            { additionalProperties: false },
          ),
        ),
      ],
      { additionalProperties: false },
    ),
  { $id: "Actor" },
);

export const ActorSelect = t.Partial(
  t.Object(
    {
      id: t.Boolean(),
      variantId: t.Boolean(),
      provider: t.Boolean(),
      actorLocator: t.Boolean(),
      workingLocator: t.Boolean(),
      providerSessionId: t.Boolean(),
      status: t.Boolean(),
      title: t.Boolean(),
      description: t.Boolean(),
      connectionInfo: t.Boolean(),
      attachCommand: t.Boolean(),
      subAgents: t.Boolean(),
      metadata: t.Boolean(),
      createdAt: t.Boolean(),
      updatedAt: t.Boolean(),
      variant: t.Boolean(),
      _count: t.Boolean(),
    },
    { additionalProperties: false },
  ),
);

export const ActorInclude = t.Partial(
  t.Object(
    { variant: t.Boolean(), _count: t.Boolean() },
    { additionalProperties: false },
  ),
);

export const ActorOrderBy = t.Partial(
  t.Object(
    {
      id: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      variantId: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      provider: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      actorLocator: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      workingLocator: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      providerSessionId: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      status: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      title: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      description: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      connectionInfo: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      attachCommand: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      subAgents: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      metadata: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      createdAt: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      updatedAt: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
    },
    { additionalProperties: false },
  ),
);

export const Actor = t.Composite([ActorPlain, ActorRelations], {
  additionalProperties: false,
});

export const ActorInputCreate = t.Composite(
  [ActorPlainInputCreate, ActorRelationsInputCreate],
  { additionalProperties: false },
);

export const ActorInputUpdate = t.Composite(
  [ActorPlainInputUpdate, ActorRelationsInputUpdate],
  { additionalProperties: false },
);
