import { Prisma, type Actor, type Variant } from '../../../../generated/prisma/client';

import type { CursorListQuery } from '../common/controller.types';
import { NotFoundError } from '../common/controller.errors';
import { getPrismaClient } from '../prisma/prisma.client';
import { getProviderAdapter, getProvidersRuntimeConfig } from '../providers/providers.registry';
import { buildRandomActorId } from '../../utils/id';
import Log, { formatLogMetadata } from '../../utils/logging';

const DEFAULT_LIST_LIMIT = 25;
const MAX_LIST_LIMIT = 100;

export interface CreateActorInput {
  variantId: string;
  provider?: string;
  title?: string;
  description?: string;
  subAgents?: ActorSubAgent[];
  metadata?: Record<string, unknown>;
}

export interface UpdateActorInput {
  variantId?: string;
  title?: string | null;
  description?: string | null;
  subAgents?: ActorSubAgent[] | null;
  metadata?: Record<string, unknown> | null;
}

export interface ActorSubAgent {
  id: string;
  parentId?: string | null;
  title?: string | null;
  status?: string | null;
  updatedAt?: string | null;
  depth?: number | null;
  summary?: string | null;
  children?: ActorSubAgent[];
}

export interface ListActorsQuery extends CursorListQuery {
  variantId?: string;
  productId?: string;
  provider?: string;
  status?: string;
}

export interface PollActorOptions {
  model?: string;
  agent?: string;
}

export interface ImportVariantActorsInput {
  variantId: string;
  provider?: string;
}

export interface ImportVariantActorsResult {
  variantId: string;
  provider: string;
  discovered: number;
  created: number;
  updated: number;
  actors: Actor[];
}

const normalizeLimit = (value?: number): number => {
  if (typeof value !== 'number' || Number.isNaN(value)) {
    return DEFAULT_LIST_LIMIT;
  }

  return Math.max(1, Math.min(MAX_LIST_LIMIT, Math.floor(value)));
};

const normalizeMessageLimit = (value?: number): number | undefined => {
  if (typeof value !== 'number' || Number.isNaN(value) || !Number.isFinite(value)) {
    return undefined;
  }

  const normalized = Math.floor(value);
  if (normalized <= 0) {
    return undefined;
  }

  return normalized;
};

const toJsonObject = (
  value: Record<string, unknown> | undefined,
): Prisma.InputJsonObject | undefined => {
  if (!value) {
    return undefined;
  }

  return value as Prisma.InputJsonObject;
};

const toJsonArray = (
  value: ActorSubAgent[] | undefined,
): Prisma.InputJsonArray | undefined => {
  if (!value) {
    return undefined;
  }

  return value as unknown as Prisma.InputJsonArray;
};

const getActorOrThrow = async (id: string): Promise<Actor> => {
  const prisma = getPrismaClient();
  const actor = await prisma.actor.findUnique({ where: { id } });

  if (!actor) {
    throw new NotFoundError(`Actor ${id} was not found`);
  }

  return actor;
};

const getVariantOrThrow = async (id: string): Promise<Variant> => {
  const prisma = getPrismaClient();
  const variant = await prisma.variant.findUnique({ where: { id } });

  if (!variant) {
    throw new NotFoundError(`Variant ${id} was not found`);
  }

  return variant;
};

const resolveProvider = (provider?: string): string => {
  const runtimeProviders = getProvidersRuntimeConfig();
  return provider?.trim().toLowerCase() || runtimeProviders.defaultProvider;
};

export const createActor = async (input: CreateActorInput): Promise<Actor> => {
  const prisma = getPrismaClient();
  const variant = await getVariantOrThrow(input.variantId);

  const provider = resolveProvider(input.provider);
  const adapter = getProviderAdapter(provider);
  const actorId = buildRandomActorId();
  const spawned = await adapter.spawn({
    actorId,
    workingLocator: variant.locator,
    title: input.title,
    description: input.description,
    metadata: input.metadata,
  });

  const actor = await prisma.actor.create({
    data: {
      id: actorId,
      variantId: variant.id,
      provider,
      actorLocator: spawned.actorLocator,
      workingLocator: variant.locator,
      providerSessionId: spawned.providerSessionId,
      status: spawned.status,
      title: spawned.title ?? input.title,
      description: spawned.description ?? input.description,
      connectionInfo: spawned.connectionInfo
        ? (spawned.connectionInfo as Prisma.InputJsonObject)
        : Prisma.DbNull,
      attachCommand: spawned.attachCommand,
      subAgents: spawned.subAgents
        ? (spawned.subAgents as unknown as Prisma.InputJsonArray)
        : input.subAgents
          ? toJsonArray(input.subAgents)
          : Prisma.DbNull,
      metadata: input.metadata ? (input.metadata as Prisma.InputJsonObject) : Prisma.DbNull,
    },
  });

  Log.info(
    `Core // Actors Controller // Actor created ${formatLogMetadata({
      actorId: actor.id,
      provider,
      variantId: actor.variantId,
    })}`,
  );

  return actor;
};

export const importVariantActors = async (
  input: ImportVariantActorsInput,
): Promise<ImportVariantActorsResult> => {
  const prisma = getPrismaClient();
  const variant = await getVariantOrThrow(input.variantId);
  const provider = resolveProvider(input.provider);
  const adapter = getProviderAdapter(provider);

  if (!adapter.listSessions) {
    throw new Error(
      `Providers // Registry // Provider import unsupported ${JSON.stringify({ provider })}`,
    );
  }

  const discoveredSessions = await adapter.listSessions({
    workingLocator: variant.locator,
  });

  const dedupedSessions = new Map<string, (typeof discoveredSessions)[number]>();
  for (const session of discoveredSessions) {
    if (!session.providerSessionId) {
      continue;
    }

    dedupedSessions.set(session.providerSessionId, session);
  }

  const sessionList = Array.from(dedupedSessions.values());
  if (sessionList.length === 0) {
    return {
      variantId: variant.id,
      provider,
      discovered: 0,
      created: 0,
      updated: 0,
      actors: [],
    };
  }

  const existingActors = await prisma.actor.findMany({
    where: {
      variantId: variant.id,
      provider,
      providerSessionId: {
        in: sessionList.map((session) => session.providerSessionId),
      },
    },
  });

  const existingActorBySessionId = new Map<string, Actor>();
  for (const actor of existingActors) {
    if (!actor.providerSessionId) {
      continue;
    }

    existingActorBySessionId.set(actor.providerSessionId, actor);
  }

  const importedActors: Actor[] = [];
  let created = 0;
  let updated = 0;

  for (const session of sessionList) {
    const existing = existingActorBySessionId.get(session.providerSessionId);

    if (existing) {
      const updatedActor = await prisma.actor.update({
        where: {
          id: existing.id,
        },
        data: {
          actorLocator: session.actorLocator,
          workingLocator: variant.locator,
          status: session.status,
          ...(session.title !== undefined ? { title: session.title } : {}),
          ...(session.description !== undefined ? { description: session.description } : {}),
          ...(session.connectionInfo !== undefined
            ? {
                connectionInfo: session.connectionInfo
                  ? (session.connectionInfo as Prisma.InputJsonObject)
                  : Prisma.DbNull,
              }
            : {}),
          ...(session.subAgents !== undefined
            ? {
                subAgents: session.subAgents
                  ? (session.subAgents as unknown as Prisma.InputJsonArray)
                  : Prisma.DbNull,
              }
            : {}),
          ...(session.attachCommand !== undefined ? { attachCommand: session.attachCommand } : {}),
        },
      });

      importedActors.push(updatedActor);
      updated += 1;
      continue;
    }

    const createdActor = await prisma.actor.create({
      data: {
        id: buildRandomActorId(),
        variantId: variant.id,
        provider,
        actorLocator: session.actorLocator,
        workingLocator: variant.locator,
        providerSessionId: session.providerSessionId,
        status: session.status,
        title: session.title,
        description: session.description,
        connectionInfo: session.connectionInfo
          ? (session.connectionInfo as Prisma.InputJsonObject)
          : Prisma.DbNull,
        attachCommand: session.attachCommand,
        subAgents: session.subAgents
          ? (session.subAgents as unknown as Prisma.InputJsonArray)
          : Prisma.DbNull,
      },
    });

    importedActors.push(createdActor);
    created += 1;
  }

  Log.info(
    `Core // Actors Controller // Variant actors imported ${formatLogMetadata({
      created,
      discovered: sessionList.length,
      provider,
      updated,
      variantId: variant.id,
    })}`,
  );

  return {
    variantId: variant.id,
    provider,
    discovered: sessionList.length,
    created,
    updated,
    actors: importedActors,
  };
};

export const listActors = async (query: ListActorsQuery = {}): Promise<Actor[]> => {
  const prisma = getPrismaClient();
  const limit = normalizeLimit(query.limit);

  return prisma.actor.findMany({
    where: {
      ...(query.variantId ? { variantId: query.variantId } : {}),
      ...(query.productId ? { variant: { productId: query.productId } } : {}),
      ...(query.provider ? { provider: query.provider.trim().toLowerCase() } : {}),
      ...(query.status ? { status: query.status } : {}),
    },
    orderBy: {
      createdAt: 'desc',
    },
    take: limit,
    ...(query.cursor ? { cursor: { id: query.cursor }, skip: 1 } : {}),
  });
};

export const getActorById = async (id: string): Promise<Actor> => {
  return getActorOrThrow(id);
};

export const updateActorById = async (id: string, input: UpdateActorInput): Promise<Actor> => {
  const prisma = getPrismaClient();
  const actor = await getActorOrThrow(id);
  const destinationVariant =
    input.variantId !== undefined && input.variantId !== actor.variantId
      ? await getVariantOrThrow(input.variantId)
      : null;

  return prisma.actor.update({
    where: { id },
    data: {
      ...(destinationVariant
        ? {
            variantId: destinationVariant.id,
            workingLocator: destinationVariant.locator,
          }
        : {}),
      ...(input.title !== undefined ? { title: input.title } : {}),
      ...(input.description !== undefined ? { description: input.description } : {}),
      ...(input.subAgents !== undefined
        ? {
            subAgents: input.subAgents === null ? Prisma.DbNull : toJsonArray(input.subAgents),
          }
        : {}),
      ...(input.metadata !== undefined
        ? { metadata: input.metadata === null ? Prisma.DbNull : toJsonObject(input.metadata) }
        : {}),
    },
  });
};

export const deleteActorById = async (
  id: string,
  options: { terminate?: boolean } = {},
): Promise<Actor> => {
  const prisma = getPrismaClient();
  const actor = await getActorOrThrow(id);
  const adapter = getProviderAdapter(actor.provider);

  if (options.terminate && adapter.terminate) {
    await adapter.terminate({
      actorLocator: actor.actorLocator,
      providerSessionId: actor.providerSessionId ?? undefined,
      workingLocator: actor.workingLocator,
    });
  }

  return prisma.actor.delete({ where: { id } });
};

export const pollActorById = async (id: string): Promise<Actor> => {
  const prisma = getPrismaClient();
  const actor = await getActorOrThrow(id);
  const adapter = getProviderAdapter(actor.provider);

  const polled = await adapter.poll({
    actorLocator: actor.actorLocator,
    providerSessionId: actor.providerSessionId ?? undefined,
    workingLocator: actor.workingLocator,
  });

  return prisma.actor.update({
    where: { id },
    data: {
      status: polled.status,
      connectionInfo: polled.connectionInfo
        ? (polled.connectionInfo as Prisma.InputJsonObject)
        : Prisma.DbNull,
      ...(polled.subAgents !== undefined
        ? {
            subAgents: polled.subAgents
              ? (polled.subAgents as unknown as Prisma.InputJsonArray)
              : Prisma.DbNull,
          }
        : {}),
      ...(polled.attachCommand !== undefined ? { attachCommand: polled.attachCommand } : {}),
    },
  });
};

export const buildActorAttachById = async (
  id: string,
  options: PollActorOptions = {},
): Promise<{ actor: Actor; attachCommand: string; connectionInfo?: Record<string, unknown> }> => {
  const prisma = getPrismaClient();
  const actor = await getActorOrThrow(id);
  const adapter = getProviderAdapter(actor.provider);
  const attach = await adapter.buildAttach({
    actorLocator: actor.actorLocator,
    providerSessionId: actor.providerSessionId ?? undefined,
    workingLocator: actor.workingLocator,
    model: options.model,
    agent: options.agent,
  });

  const updatedActor = await prisma.actor.update({
    where: { id },
    data: {
      attachCommand: attach.attachCommand,
      ...(attach.connectionInfo
        ? { connectionInfo: attach.connectionInfo as Prisma.InputJsonObject }
        : {}),
    },
  });

  return {
    actor: updatedActor,
    attachCommand: attach.attachCommand,
    connectionInfo: attach.connectionInfo?.raw,
  };
};

export const sendActorMessageById = async (
  id: string,
  input: { prompt: string; noReply?: boolean; model?: string; agent?: string },
): Promise<Record<string, unknown>> => {
  const actor = await getActorOrThrow(id);
  const adapter = getProviderAdapter(actor.provider);

  return adapter.sendMessage({
    actorLocator: actor.actorLocator,
    providerSessionId: actor.providerSessionId ?? undefined,
    workingLocator: actor.workingLocator,
    prompt: input.prompt,
    noReply: input.noReply,
    model: input.model,
    agent: input.agent,
  });
};

export const listActorMessagesById = async (
  id: string,
  input: { nLastMessages?: number } = {},
) => {
  const actor = await getActorOrThrow(id);
  const adapter = getProviderAdapter(actor.provider);
  const nLastMessages = normalizeMessageLimit(input.nLastMessages);

  const messages = await adapter.listMessages({
    actorLocator: actor.actorLocator,
    providerSessionId: actor.providerSessionId ?? undefined,
    workingLocator: actor.workingLocator,
    nLastMessages,
  });

  return [...messages].sort((left, right) => left.createdAt.localeCompare(right.createdAt));
};

export const runActorCommandById = async (
  id: string,
  input: { command: string; args?: string; model?: string; agent?: string },
): Promise<Record<string, unknown>> => {
  const actor = await getActorOrThrow(id);
  const adapter = getProviderAdapter(actor.provider);

  return adapter.runCommand({
    actorLocator: actor.actorLocator,
    providerSessionId: actor.providerSessionId ?? undefined,
    workingLocator: actor.workingLocator,
    command: input.command,
    args: input.args,
    model: input.model,
    agent: input.agent,
  });
};
