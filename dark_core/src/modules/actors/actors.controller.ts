import { Prisma, type Actor } from '../../../../generated/prisma/client';

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
  metadata?: Record<string, unknown>;
}

export interface UpdateActorInput {
  title?: string | null;
  description?: string | null;
  metadata?: Record<string, unknown> | null;
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

const normalizeLimit = (value?: number): number => {
  if (typeof value !== 'number' || Number.isNaN(value)) {
    return DEFAULT_LIST_LIMIT;
  }

  return Math.max(1, Math.min(MAX_LIST_LIMIT, Math.floor(value)));
};

const toJsonObject = (
  value: Record<string, unknown> | undefined,
): Prisma.InputJsonObject | undefined => {
  if (!value) {
    return undefined;
  }

  return value as Prisma.InputJsonObject;
};

const getActorOrThrow = async (id: string): Promise<Actor> => {
  const prisma = getPrismaClient();
  const actor = await prisma.actor.findUnique({ where: { id } });

  if (!actor) {
    throw new NotFoundError(`Actor ${id} was not found`);
  }

  return actor;
};

export const createActor = async (input: CreateActorInput): Promise<Actor> => {
  const prisma = getPrismaClient();
  const variant = await prisma.variant.findUnique({
    where: {
      id: input.variantId,
    },
  });

  if (!variant) {
    throw new NotFoundError(`Variant ${input.variantId} was not found`);
  }

  const runtimeProviders = getProvidersRuntimeConfig();
  const provider = input.provider?.trim().toLowerCase() || runtimeProviders.defaultProvider;
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
  await getActorOrThrow(id);

  return prisma.actor.update({
    where: { id },
    data: {
      ...(input.title !== undefined ? { title: input.title } : {}),
      ...(input.description !== undefined ? { description: input.description } : {}),
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

  return adapter.listMessages({
    actorLocator: actor.actorLocator,
    providerSessionId: actor.providerSessionId ?? undefined,
    workingLocator: actor.workingLocator,
    nLastMessages: input.nLastMessages,
  });
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
