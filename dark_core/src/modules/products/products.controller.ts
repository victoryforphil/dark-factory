import { Prisma, type Product, type Variant } from '../../../../generated/prisma/client';
import { dirname } from 'node:path';

import { getPrismaClient } from '../prisma/prisma.client';
import {
  buildGitLocator,
  buildDeterministicIdFromLocator,
  hostAbsolutePathToLocatorId,
  isGitLocator,
  isLocalLocator,
  locatorIdToHostPath,
  normalizeLocator,
  parseLocatorId,
} from '../../utils/locator';
import { buildRandomVariantId } from '../../utils/id';
import { scanProductGitInfo } from '../git/git.scan';
import Log, { formatLogMetadata } from '../../utils/logging';
import { IdCollisionDetectedError, NotFoundError } from '../common/controller.errors';
import type { CursorListQuery } from '../common/controller.types';

export type ProductIncludeOption = 'minimal' | 'full';

export interface ListProductsQuery extends CursorListQuery {
  include?: ProductIncludeOption;
}

export interface GetProductOptions {
  include?: ProductIncludeOption;
}

export type ProductWithVariants = Product & {
  variants: Variant[];
};

export type ProductRecord = Product | ProductWithVariants;

export interface CreateProductInput {
  locator: string;
  displayName?: string | null;
  workspaceLocator?: string | null;
}

const DEFAULT_LIST_LIMIT = 25;
const MAX_LIST_LIMIT = 100;
const DEFAULT_VARIANT_NAME = 'default';
const DEFAULT_PRODUCT_INCLUDE: ProductIncludeOption = 'minimal';

const normalizeLimit = (value?: number): number => {
  const fallback = DEFAULT_LIST_LIMIT;

  if (typeof value !== 'number' || Number.isNaN(value)) {
    return fallback;
  }

  return Math.max(1, Math.min(MAX_LIST_LIMIT, Math.floor(value)));
};

const resolveIncludeOption = (include?: ProductIncludeOption): ProductIncludeOption => {
  if (include === 'full') {
    return 'full';
  }

  return DEFAULT_PRODUCT_INCLUDE;
};

const buildProductInclude = (include: ProductIncludeOption): Prisma.ProductInclude | undefined => {
  if (include !== 'full') {
    return undefined;
  }

  return {
    variants: true,
  };
};

const ensureDefaultVariant = async (
  tx: Prisma.TransactionClient,
  productId: string,
  variantLocator: string | null,
): Promise<boolean> => {
  if (!variantLocator || !isLocalLocator(variantLocator)) {
    return false;
  }

  const existingDefaultVariant = await tx.variant.findUnique({
    where: {
        productId_name: {
          productId,
          name: DEFAULT_VARIANT_NAME,
        },
      },
  });

  if (existingDefaultVariant) {
    return false;
  }

  await tx.variant.create({
    data: {
      id: buildRandomVariantId(),
      productId,
      name: DEFAULT_VARIANT_NAME,
      locator: variantLocator,
    },
  });

  return true;
};

const resolveProductLocator = (canonicalLocator: string, gitInfo: Product['gitInfo']): string => {
  if (!isLocalLocator(canonicalLocator)) {
    return canonicalLocator;
  }

  if (!gitInfo || typeof gitInfo !== 'object' || Array.isArray(gitInfo)) {
    return canonicalLocator;
  }

  const gitSnapshot = gitInfo as Record<string, unknown>;

  const remoteUrl =
    typeof gitSnapshot.remoteUrl === 'string' ? gitSnapshot.remoteUrl.trim() : '';
  const branch = typeof gitSnapshot.branch === 'string' ? gitSnapshot.branch.trim() : '';

  if (!remoteUrl || !branch) {
    return canonicalLocator;
  }

  return buildGitLocator(remoteUrl, branch);
};

export interface UpdateProductInput {
  locator?: string;
  displayName?: string | null;
  workspaceLocator?: string | null;
}

const normalizeWorkspaceLocator = (value: string | null | undefined): string | null => {
  if (value === undefined) {
    return null;
  }

  if (value === null) {
    return null;
  }

  const normalized = normalizeLocator(value);
  const parsed = parseLocatorId(normalized);

  if (parsed.type !== 'local') {
    throw new Error(
      `Products // Workspace // Expected @local:// workspace locator ${formatLogMetadata({
        value,
      })}`,
    );
  }

  return parsed.locator;
};

const deriveWorkspaceLocator = (
  canonicalLocator: string,
  gitInfo: Product['gitInfo'],
): string | null => {
  if (!isLocalLocator(canonicalLocator)) {
    return null;
  }

  try {
    const localPath = locatorIdToHostPath(canonicalLocator);
    let workspaceBasePath = localPath;

    if (gitInfo && typeof gitInfo === 'object' && !Array.isArray(gitInfo)) {
      const gitSnapshot = gitInfo as Record<string, unknown>;
      if (typeof gitSnapshot.repoRoot === 'string' && gitSnapshot.repoRoot.trim().length > 0) {
        workspaceBasePath = gitSnapshot.repoRoot;
      }
    }

    return hostAbsolutePathToLocatorId(dirname(workspaceBasePath));
  } catch {
    return null;
  }
};

export const listProducts = async (query: ListProductsQuery = {}): Promise<ProductRecord[]> => {
  const prisma = getPrismaClient();
  const limit = normalizeLimit(query.limit);
  const include = resolveIncludeOption(query.include);
  const productInclude = buildProductInclude(include);

  Log.debug(
    `Core // Products Controller // Listing products ${formatLogMetadata({
      cursor: query.cursor ?? null,
      include,
      limit,
    })}`,
  );

  return prisma.product.findMany({
    take: limit,
    orderBy: { createdAt: 'desc' },
    ...(productInclude ? { include: productInclude } : {}),
    ...(query.cursor ? { cursor: { id: query.cursor }, skip: 1 } : {}),
  });
};

export const getProductById = async (
  id: string,
  options: GetProductOptions = {},
): Promise<ProductRecord> => {
  const prisma = getPrismaClient();
  const include = resolveIncludeOption(options.include);
  const productInclude = buildProductInclude(include);

  Log.debug(`Core // Products Controller // Getting product ${formatLogMetadata({ id, include })}`);

  const product = await prisma.product.findUnique({
    where: { id },
    ...(productInclude ? { include: productInclude } : {}),
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
  const gitInfo = await scanProductGitInfo(canonicalLocator);
  const productLocator = resolveProductLocator(canonicalLocator, gitInfo);
  const explicitWorkspaceLocator = normalizeWorkspaceLocator(input.workspaceLocator);
  const derivedWorkspaceLocator = deriveWorkspaceLocator(canonicalLocator, gitInfo);
  const workspaceLocator = explicitWorkspaceLocator ?? derivedWorkspaceLocator;
  const productId = buildDeterministicIdFromLocator(productLocator);
  const defaultVariantLocator = isLocalLocator(canonicalLocator) ? canonicalLocator : null;
  let defaultVariantCreated = false;

  const product = await prisma.$transaction(async (tx) => {
      const existingProductById = await tx.product.findUnique({
        where: { id: productId },
      });

      if (existingProductById) {
        if (existingProductById.locator !== productLocator) {
          throw new IdCollisionDetectedError(
            `Products // Create // ID collision detected ${formatLogMetadata({
              canonicalLocator: productLocator,
              existingLocator: existingProductById.locator,
              id: productId,
            })}`,
          );
        }

        defaultVariantCreated = await ensureDefaultVariant(
          tx,
          existingProductById.id,
          defaultVariantLocator,
        );

        return existingProductById;
      }

      const existingProductByLocator = await tx.product.findUnique({
        where: { locator: productLocator },
      });

      if (existingProductByLocator) {
        defaultVariantCreated = await ensureDefaultVariant(
          tx,
          existingProductByLocator.id,
          defaultVariantLocator,
        );
        return existingProductByLocator;
      }

      if (productLocator !== canonicalLocator && isGitLocator(productLocator)) {
        const existingProductByLegacyLocator = await tx.product.findUnique({
          where: { locator: canonicalLocator },
        });

        if (existingProductByLegacyLocator) {
          const updatedLegacyProduct = await tx.product.update({
            where: { id: existingProductByLegacyLocator.id },
            data: {
              locator: productLocator,
              workspaceLocator: existingProductByLegacyLocator.workspaceLocator ?? workspaceLocator,
              gitInfo: gitInfo ?? Prisma.DbNull,
            },
          });

          defaultVariantCreated = await ensureDefaultVariant(
            tx,
            updatedLegacyProduct.id,
            defaultVariantLocator,
          );

          return updatedLegacyProduct;
        }
      }

      const createdProduct = await tx.product.create({
        data: {
          id: productId,
          locator: productLocator,
          displayName: input.displayName ?? null,
          workspaceLocator,
          gitInfo: gitInfo ?? Prisma.DbNull,
        },
      });

      defaultVariantCreated = await ensureDefaultVariant(tx, createdProduct.id, defaultVariantLocator);

    return createdProduct;
  });

  Log.info(
    `Core // Products Controller // Product created ${formatLogMetadata({
      id: product.id,
      locator: product.locator,
      sourceLocator: canonicalLocator,
      workspaceLocator: product.workspaceLocator,
    })}`,
  );

  if (defaultVariantCreated) {
    Log.info(
      `Core // Products Controller // Local default variant ensured ${formatLogMetadata({
        locator: defaultVariantLocator,
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
      ...(input.workspaceLocator !== undefined
        ? { workspaceLocator: normalizeWorkspaceLocator(input.workspaceLocator) }
        : {}),
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
