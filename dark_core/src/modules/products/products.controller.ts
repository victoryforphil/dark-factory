import { Prisma, type Product } from '../../../../generated/prisma/client';

import { getPrismaClient } from '../prisma/prisma.client';
import {
  buildDeterministicIdFromLocator,
  isLocalLocator,
  normalizeLocator,
} from '../../utils/locator';
import { scanProductGitInfo } from '../git/git.scan';
import Log, { formatLogMetadata } from '../../utils/logging';
import { IdCollisionDetectedError, NotFoundError } from '../common/controller.errors';
import type { CursorListQuery } from '../common/controller.types';

export type ListProductsQuery = CursorListQuery;

export interface CreateProductInput {
  locator: string;
  displayName?: string | null;
}

const DEFAULT_LIST_LIMIT = 25;
const MAX_LIST_LIMIT = 100;
const DEFAULT_VARIANT_NAME = 'default';

const normalizeLimit = (value?: number): number => {
  const fallback = DEFAULT_LIST_LIMIT;

  if (typeof value !== 'number' || Number.isNaN(value)) {
    return fallback;
  }

  return Math.max(1, Math.min(MAX_LIST_LIMIT, Math.floor(value)));
};

export interface UpdateProductInput {
  locator?: string;
  displayName?: string | null;
}

export const listProducts = async (query: ListProductsQuery = {}): Promise<Product[]> => {
  const prisma = getPrismaClient();
  const limit = normalizeLimit(query.limit);

  Log.debug(
    `Core // Products Controller // Listing products ${formatLogMetadata({ cursor: query.cursor ?? null, limit })}`,
  );

  return prisma.product.findMany({
    take: limit,
    orderBy: { createdAt: 'desc' },
    ...(query.cursor ? { cursor: { id: query.cursor }, skip: 1 } : {}),
  });
};

export const getProductById = async (id: string): Promise<Product> => {
  const prisma = getPrismaClient();

  Log.debug(`Core // Products Controller // Getting product ${formatLogMetadata({ id })}`);

  const product = await prisma.product.findUnique({
    where: { id },
  });

  if (!product) {
    Log.warn(`Core // Products Controller // Product not found ${formatLogMetadata({ id })}`);
    throw new NotFoundError(`Product ${id} was not found`);
  }

  return product;
};

export const createProduct = async (input: CreateProductInput): Promise<Product> => {
  const prisma = getPrismaClient();
  const canonicalLocator = normalizeLocator(input.locator);
  const productId = buildDeterministicIdFromLocator(canonicalLocator);
  const gitInfo = await scanProductGitInfo(canonicalLocator);

  const product = await prisma.$transaction(async (tx) => {
    const existingProductById = await tx.product.findUnique({
      where: { id: productId },
    });

    if (existingProductById) {
      if (existingProductById.locator !== canonicalLocator) {
        throw new IdCollisionDetectedError(
          `Products // Create // ID collision detected ${formatLogMetadata({
            canonicalLocator,
            existingLocator: existingProductById.locator,
            id: productId,
          })}`,
        );
      }

      return existingProductById;
    }

    const existingProductByLocator = await tx.product.findUnique({
      where: { locator: canonicalLocator },
    });

    if (existingProductByLocator) {
      return existingProductByLocator;
    }

    const createdProduct = await tx.product.create({
      data: {
        id: productId,
        locator: canonicalLocator,
        displayName: input.displayName ?? null,
        gitInfo: gitInfo ?? Prisma.DbNull,
      },
    });

    if (isLocalLocator(createdProduct.locator)) {
      await tx.variant.create({
        data: {
          id: crypto.randomUUID(),
          productId: createdProduct.id,
          name: DEFAULT_VARIANT_NAME,
          locator: createdProduct.locator,
        },
      });
    }

    return createdProduct;
  });

  Log.info(
    `Core // Products Controller // Product created ${formatLogMetadata({
      id: product.id,
      locator: product.locator,
    })}`,
  );

  if (isLocalLocator(product.locator)) {
    Log.debug(
      `Core // Products Controller // Default variant created ${formatLogMetadata({
        locator: product.locator,
        name: DEFAULT_VARIANT_NAME,
        productId: product.id,
      })}`,
    );
  }

  return product;
};

export const updateProductById = async (
  id: string,
  input: UpdateProductInput,
): Promise<Product> => {
  const prisma = getPrismaClient();

  const existingProduct = await prisma.product.findUnique({ where: { id } });

  if (!existingProduct) {
    Log.warn(
      `Core // Products Controller // Update skipped, product not found ${formatLogMetadata({ id })}`,
    );
    throw new NotFoundError(`Product ${id} was not found`);
  }

  const updatedProduct = await prisma.product.update({
    where: { id },
    data: {
      ...(input.locator !== undefined ? { locator: normalizeLocator(input.locator) } : {}),
      ...(input.displayName !== undefined ? { displayName: input.displayName } : {}),
    },
  });

  Log.info(`Core // Products Controller // Product updated ${formatLogMetadata({ id })}`);
  return updatedProduct;
};

export const deleteProductById = async (id: string): Promise<Product> => {
  const prisma = getPrismaClient();

  const existingProduct = await prisma.product.findUnique({ where: { id } });

  if (!existingProduct) {
    Log.warn(
      `Core // Products Controller // Delete skipped, product not found ${formatLogMetadata({ id })}`,
    );
    throw new NotFoundError(`Product ${id} was not found`);
  }

  const deletedProduct = await prisma.product.delete({ where: { id } });
  Log.info(`Core // Products Controller // Product deleted ${formatLogMetadata({ id })}`);
  return deletedProduct;
};
