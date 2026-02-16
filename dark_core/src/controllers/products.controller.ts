import type { Product } from '../../../generated/prisma/client';

import { getPrismaClient } from '../clients';
import type { CursorListQuery } from './controller.types';

export type ListProductsQuery = CursorListQuery;

export interface CreateProductInput {
  id?: string;
  locator: string;
  displayName?: string | null;
}

const DEFAULT_LIST_LIMIT = 25;
const MAX_LIST_LIMIT = 100;

const normalizeLimit = (value?: number): number => {
  const fallback = DEFAULT_LIST_LIMIT;

  if (typeof value !== 'number' || Number.isNaN(value)) {
    return fallback;
  }

  return Math.max(1, Math.min(MAX_LIST_LIMIT, Math.floor(value)));
};

export const listProducts = async (query: ListProductsQuery = {}): Promise<Product[]> => {
  const prisma = getPrismaClient();

  return prisma.product.findMany({
    take: normalizeLimit(query.limit),
    orderBy: { createdAt: 'desc' },
    ...(query.cursor ? { cursor: { id: query.cursor }, skip: 1 } : {}),
  });
};

export const createProduct = async (input: CreateProductInput): Promise<Product> => {
  const prisma = getPrismaClient();

  return prisma.product.create({
    data: {
      id: input.id ?? crypto.randomUUID(),
      locator: input.locator,
      displayName: input.displayName ?? null,
    },
  });
};
