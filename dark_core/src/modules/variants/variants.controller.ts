import { Prisma, type Variant } from '../../../../generated/prisma/client';

import { getPrismaClient } from '../prisma/prisma.client';
import Log, { formatLogMetadata } from '../../utils/logging';
import { NotFoundError } from '../common/controller.errors';
import type { CursorListQuery } from '../common/controller.types';
import { scanVariantGitInfo } from '../git/git.scan';

const DEFAULT_LIST_LIMIT = 25;
const MAX_LIST_LIMIT = 100;

export interface ListVariantsQuery extends CursorListQuery {
  productId?: string;
  locator?: string;
  name?: string;
}

export interface CreateVariantInput {
  product: {
    connect: {
      id: string;
    };
  };
  name?: string;
  locator: string;
}

export interface UpdateVariantInput {
  name?: string;
  locator?: string;
}

const normalizeLimit = (value?: number): number => {
  if (typeof value !== 'number' || Number.isNaN(value)) {
    return DEFAULT_LIST_LIMIT;
  }

  return Math.max(1, Math.min(MAX_LIST_LIMIT, Math.floor(value)));
};

export const listVariants = async (query: ListVariantsQuery = {}): Promise<Variant[]> => {
  const prisma = getPrismaClient();
  const limit = normalizeLimit(query.limit);

  Log.debug(
    `Core // Variants Controller // Listing variants ${formatLogMetadata({
      cursor: query.cursor ?? null,
      limit,
      locator: query.locator ?? null,
      name: query.name ?? null,
      productId: query.productId ?? null,
    })}`,
  );

  return prisma.variant.findMany({
    where: {
      ...(query.productId ? { productId: query.productId } : {}),
      ...(query.locator ? { locator: query.locator } : {}),
      ...(query.name ? { name: query.name } : {}),
    },
    take: limit,
    orderBy: { createdAt: 'desc' },
    ...(query.cursor ? { cursor: { id: query.cursor }, skip: 1 } : {}),
  });
};

export const getVariantById = async (id: string): Promise<Variant> => {
  const prisma = getPrismaClient();

  Log.debug(`Core // Variants Controller // Getting variant ${formatLogMetadata({ id })}`);

  const variant = await prisma.variant.findUnique({ where: { id } });

  if (!variant) {
    Log.warn(`Core // Variants Controller // Variant not found ${formatLogMetadata({ id })}`);
    throw new NotFoundError(`Variant ${id} was not found`);
  }

  return variant;
};

export const createVariant = async (input: CreateVariantInput): Promise<Variant> => {
  const prisma = getPrismaClient();

  const createdVariant = await prisma.variant.create({
    data: {
      id: crypto.randomUUID(),
      name: input.name,
      locator: input.locator,
      product: {
        connect: {
          id: input.product.connect.id,
        },
      },
    },
  });

  Log.info(
    `Core // Variants Controller // Variant created ${formatLogMetadata({
      id: createdVariant.id,
      locator: createdVariant.locator,
      name: createdVariant.name,
      productId: createdVariant.productId,
    })}`,
  );

  return syncVariantGitInfo(createdVariant.id);
};

export const syncVariantGitInfo = async (id: string): Promise<Variant> => {
  const prisma = getPrismaClient();

  const existingVariant = await prisma.variant.findUnique({ where: { id } });

  if (!existingVariant) {
    Log.warn(
      `Core // Variants Controller // Git sync skipped, variant not found ${formatLogMetadata({ id })}`,
    );
    throw new NotFoundError(`Variant ${id} was not found`);
  }

  const gitInfo = await scanVariantGitInfo(existingVariant.locator);
  const now = new Date();

  const updatedVariant = await prisma.variant.update({
    where: { id },
    data: {
      gitInfo: gitInfo ?? Prisma.DbNull,
      gitInfoLastPolledAt: now,
      gitInfoUpdatedAt: gitInfo ? now : null,
    },
  });

  Log.debug(
    `Core // Variants Controller // Git info synchronized ${formatLogMetadata({
      hasGitInfo: Boolean(gitInfo),
      id,
      locator: existingVariant.locator,
    })}`,
  );

  return updatedVariant;
};

export const updateVariantById = async (
  id: string,
  input: UpdateVariantInput,
): Promise<Variant> => {
  const prisma = getPrismaClient();

  const existingVariant = await prisma.variant.findUnique({ where: { id } });

  if (!existingVariant) {
    Log.warn(
      `Core // Variants Controller // Update skipped, variant not found ${formatLogMetadata({ id })}`,
    );
    throw new NotFoundError(`Variant ${id} was not found`);
  }

  const updatedVariant = await prisma.variant.update({
    where: { id },
    data: {
      ...(input.name !== undefined ? { name: input.name } : {}),
      ...(input.locator !== undefined ? { locator: input.locator } : {}),
    },
  });

  Log.info(`Core // Variants Controller // Variant updated ${formatLogMetadata({ id })}`);

  return updatedVariant;
};

export const deleteVariantById = async (id: string): Promise<Variant> => {
  const prisma = getPrismaClient();

  const existingVariant = await prisma.variant.findUnique({ where: { id } });

  if (!existingVariant) {
    Log.warn(
      `Core // Variants Controller // Delete skipped, variant not found ${formatLogMetadata({ id })}`,
    );
    throw new NotFoundError(`Variant ${id} was not found`);
  }

  const deletedVariant = await prisma.variant.delete({ where: { id } });
  Log.info(`Core // Variants Controller // Variant deleted ${formatLogMetadata({ id })}`);
  return deletedVariant;
};
