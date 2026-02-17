import { t } from "elysia";

import { __transformDate__ } from "./__transformDate__";

import { __nullable__ } from "./__nullable__";

export const VariantPlain = t.Object(
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
);

export const VariantRelations = t.Object(
  {
    product: t.Object(
      {
        id: t.String(),
        locator: t.String(),
        displayName: __nullable__(t.String()),
        workspaceLocator: __nullable__(t.String()),
        gitInfo: __nullable__(t.Any()),
        createdAt: t.Date(),
        updatedAt: t.Date(),
      },
      { additionalProperties: false },
    ),
    actors: t.Array(
      t.Object(
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
      ),
      { additionalProperties: false },
    ),
  },
  { additionalProperties: false },
);

export const VariantPlainInputCreate = t.Object(
  {
    name: t.Optional(t.String()),
    locator: t.String(),
    gitInfo: t.Optional(__nullable__(t.Any())),
    gitInfoUpdatedAt: t.Optional(__nullable__(t.Date())),
    gitInfoLastPolledAt: t.Optional(__nullable__(t.Date())),
  },
  { additionalProperties: false },
);

export const VariantPlainInputUpdate = t.Object(
  {
    name: t.Optional(t.String()),
    locator: t.Optional(t.String()),
    gitInfo: t.Optional(__nullable__(t.Any())),
    gitInfoUpdatedAt: t.Optional(__nullable__(t.Date())),
    gitInfoLastPolledAt: t.Optional(__nullable__(t.Date())),
  },
  { additionalProperties: false },
);

export const VariantRelationsInputCreate = t.Object(
  {
    product: t.Object(
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
    actors: t.Optional(
      t.Object(
        {
          connect: t.Array(
            t.Object(
              {
                id: t.String({ additionalProperties: false }),
              },
              { additionalProperties: false },
            ),
            { additionalProperties: false },
          ),
        },
        { additionalProperties: false },
      ),
    ),
  },
  { additionalProperties: false },
);

export const VariantRelationsInputUpdate = t.Partial(
  t.Object(
    {
      product: t.Object(
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
      actors: t.Partial(
        t.Object(
          {
            connect: t.Array(
              t.Object(
                {
                  id: t.String({ additionalProperties: false }),
                },
                { additionalProperties: false },
              ),
              { additionalProperties: false },
            ),
            disconnect: t.Array(
              t.Object(
                {
                  id: t.String({ additionalProperties: false }),
                },
                { additionalProperties: false },
              ),
              { additionalProperties: false },
            ),
          },
          { additionalProperties: false },
        ),
      ),
    },
    { additionalProperties: false },
  ),
);

export const VariantWhere = t.Partial(
  t.Recursive(
    (Self) =>
      t.Object(
        {
          AND: t.Union([Self, t.Array(Self, { additionalProperties: false })]),
          NOT: t.Union([Self, t.Array(Self, { additionalProperties: false })]),
          OR: t.Array(Self, { additionalProperties: false }),
          id: t.String(),
          productId: t.String(),
          name: t.String(),
          locator: t.String(),
          gitInfo: t.Any(),
          gitInfoUpdatedAt: t.Date(),
          gitInfoLastPolledAt: t.Date(),
          createdAt: t.Date(),
          updatedAt: t.Date(),
        },
        { additionalProperties: false },
      ),
    { $id: "Variant" },
  ),
);

export const VariantWhereUnique = t.Recursive(
  (Self) =>
    t.Intersect(
      [
        t.Partial(
          t.Object(
            {
              id: t.String(),
              productId_name: t.Object(
                { productId: t.String(), name: t.String() },
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
              productId_name: t.Object(
                { productId: t.String(), name: t.String() },
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
              productId: t.String(),
              name: t.String(),
              locator: t.String(),
              gitInfo: t.Any(),
              gitInfoUpdatedAt: t.Date(),
              gitInfoLastPolledAt: t.Date(),
              createdAt: t.Date(),
              updatedAt: t.Date(),
            },
            { additionalProperties: false },
          ),
        ),
      ],
      { additionalProperties: false },
    ),
  { $id: "Variant" },
);

export const VariantSelect = t.Partial(
  t.Object(
    {
      id: t.Boolean(),
      productId: t.Boolean(),
      name: t.Boolean(),
      locator: t.Boolean(),
      gitInfo: t.Boolean(),
      gitInfoUpdatedAt: t.Boolean(),
      gitInfoLastPolledAt: t.Boolean(),
      product: t.Boolean(),
      actors: t.Boolean(),
      createdAt: t.Boolean(),
      updatedAt: t.Boolean(),
      _count: t.Boolean(),
    },
    { additionalProperties: false },
  ),
);

export const VariantInclude = t.Partial(
  t.Object(
    { product: t.Boolean(), actors: t.Boolean(), _count: t.Boolean() },
    { additionalProperties: false },
  ),
);

export const VariantOrderBy = t.Partial(
  t.Object(
    {
      id: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      productId: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      name: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      locator: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      gitInfo: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      gitInfoUpdatedAt: t.Union([t.Literal("asc"), t.Literal("desc")], {
        additionalProperties: false,
      }),
      gitInfoLastPolledAt: t.Union([t.Literal("asc"), t.Literal("desc")], {
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

export const Variant = t.Composite([VariantPlain, VariantRelations], {
  additionalProperties: false,
});

export const VariantInputCreate = t.Composite(
  [VariantPlainInputCreate, VariantRelationsInputCreate],
  { additionalProperties: false },
);

export const VariantInputUpdate = t.Composite(
  [VariantPlainInputUpdate, VariantRelationsInputUpdate],
  { additionalProperties: false },
);
