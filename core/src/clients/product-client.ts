import type { Product } from "../gen/proto/core/v1/types/product";
import type { Variant } from "../gen/proto/core/v1/types/variant";
import {
  deriveStageZeroProductIdentity,
} from "../helpers/locator-id";
import { DuckDbExecutor, duckDbClient } from "./duckdb-client";

interface ProductRecord {
  product: Product;
  defaultVariant: Variant;
}

interface DeleteProductResult {
  productId: string;
  defaultVariantId: string;
}

interface ProductListResult {
  records: ProductRecord[];
  totalCount: number;
}

interface ProductRecordRow {
  product_id: unknown;
  product_locator: unknown;
  product_display_name: unknown;
  product_default_variant_id: unknown;
  product_created_at: unknown;
  product_updated_at: unknown;
  variant_id: unknown;
  variant_product_id: unknown;
  variant_locator: unknown;
  variant_name: unknown;
  variant_created_at: unknown;
}

interface TotalCountRow {
  total_count: unknown;
}

export interface CreateProductInput {
  productLocator: string;
  displayName?: string;
}

export interface GetProductInput {
  productId?: string;
  productLocator?: string;
}

export interface ListProductsInput {
  limit?: number;
  offset?: number;
}

export interface UpdateProductInput {
  productId: string;
  displayName?: string;
  clearDisplayName: boolean;
}

export interface DeleteProductInput {
  productId: string;
}

export class ProductClientError extends Error {
  readonly code: string;
  readonly details: Record<string, string>;

  constructor(code: string, message: string, details: Record<string, string> = {}) {
    super(message);
    this.code = code;
    this.details = details;
  }
}

const baseSelectProductRecordSql = `
SELECT
  p.id AS product_id,
  p.locator AS product_locator,
  p.display_name AS product_display_name,
  p.default_variant_id AS product_default_variant_id,
  p.created_at AS product_created_at,
  p.updated_at AS product_updated_at,
  v.id AS variant_id,
  v.product_id AS variant_product_id,
  v.locator AS variant_locator,
  v.name AS variant_name,
  v.created_at AS variant_created_at
FROM products p
INNER JOIN variants v ON v.id = p.default_variant_id
`;

const expectString = (value: unknown, field: string): string => {
  if (typeof value === "string") {
    return value;
  }

  if (typeof value === "number" || typeof value === "bigint") {
    return String(value);
  }

  throw new ProductClientError(
    "STORAGE_CORRUPTION",
    `Storage value is not a string for field ${field}`,
    { field },
  );
};

const toOptionalString = (value: unknown): string | undefined => {
  if (value === null || value === undefined) {
    return undefined;
  }

  if (typeof value === "string") {
    return value;
  }

  return String(value);
};

const expectDate = (value: unknown, field: string): Date => {
  if (value instanceof Date) {
    return value;
  }

  if (typeof value === "string" || typeof value === "number") {
    const parsed = new Date(value);
    if (!Number.isNaN(parsed.getTime())) {
      return parsed;
    }
  }

  throw new ProductClientError(
    "STORAGE_CORRUPTION",
    `Storage value is not a valid timestamp for field ${field}`,
    { field },
  );
};

const toCount = (value: unknown): number => {
  if (typeof value === "number") {
    return Number.isFinite(value) ? value : 0;
  }

  if (typeof value === "bigint") {
    return Number(value);
  }

  if (typeof value === "string") {
    const parsed = Number(value);
    return Number.isFinite(parsed) ? parsed : 0;
  }

  return 0;
};

const mapRowToProductRecord = (row: ProductRecordRow): ProductRecord => {
  const product: Product = {
    id: expectString(row.product_id, "product_id"),
    locator: expectString(row.product_locator, "product_locator"),
    displayName: toOptionalString(row.product_display_name),
    defaultVariantId: expectString(
      row.product_default_variant_id,
      "product_default_variant_id",
    ),
    createdAt: expectDate(row.product_created_at, "product_created_at"),
    updatedAt: expectDate(row.product_updated_at, "product_updated_at"),
  };

  const defaultVariant: Variant = {
    id: expectString(row.variant_id, "variant_id"),
    productId: expectString(row.variant_product_id, "variant_product_id"),
    locator: expectString(row.variant_locator, "variant_locator"),
    name: expectString(row.variant_name, "variant_name"),
    createdAt: expectDate(row.variant_created_at, "variant_created_at"),
  };

  return {
    product,
    defaultVariant,
  };
};

const normalizePagingInput = ({
  limit,
  offset,
}: ListProductsInput): { limit: number; offset: number } => {
  const normalizedLimit = limit ?? 50;
  const normalizedOffset = offset ?? 0;

  if (!Number.isInteger(normalizedLimit) || normalizedLimit < 1 || normalizedLimit > 200) {
    throw new ProductClientError(
      "INVALID_ARGUMENT",
      "limit must be an integer in [1, 200]",
      { limit: String(normalizedLimit) },
    );
  }

  if (!Number.isInteger(normalizedOffset) || normalizedOffset < 0) {
    throw new ProductClientError(
      "INVALID_ARGUMENT",
      "offset must be an integer >= 0",
      { offset: String(normalizedOffset) },
    );
  }

  return {
    limit: normalizedLimit,
    offset: normalizedOffset,
  };
};

export class ProductClient {
  private initializePromise: Promise<void> | undefined;

  async initialize(): Promise<void> {
    if (this.initializePromise) {
      return this.initializePromise;
    }

    this.initializePromise = (async () => {
      await duckDbClient.initialize();

      await duckDbClient.run(`
        CREATE TABLE IF NOT EXISTS products (
          id VARCHAR PRIMARY KEY,
          locator VARCHAR NOT NULL UNIQUE,
          display_name VARCHAR,
          default_variant_id VARCHAR NOT NULL UNIQUE,
          created_at TIMESTAMP NOT NULL,
          updated_at TIMESTAMP NOT NULL
        )
      `);

      await duckDbClient.run(`
        CREATE TABLE IF NOT EXISTS variants (
          id VARCHAR PRIMARY KEY,
          product_id VARCHAR NOT NULL UNIQUE,
          locator VARCHAR NOT NULL UNIQUE,
          name VARCHAR NOT NULL,
          created_at TIMESTAMP NOT NULL
        )
      `);

      await duckDbClient.run(
        "CREATE INDEX IF NOT EXISTS idx_products_updated_at ON products(updated_at)",
      );

      await duckDbClient.run(
        "CREATE INDEX IF NOT EXISTS idx_variants_product_id ON variants(product_id)",
      );
    })();

    return this.initializePromise;
  }

  async createProduct(input: CreateProductInput): Promise<ProductRecord> {
    await this.initialize();

    const rawLocator = input.productLocator?.trim();
    if (!rawLocator) {
      throw new ProductClientError(
        "INVALID_ARGUMENT",
        "product_locator is required",
        { product_locator: input.productLocator ?? "" },
      );
    }

    let identity;
    try {
      identity = deriveStageZeroProductIdentity(rawLocator);
    } catch (error) {
      throw new ProductClientError(
        "INVALID_ARGUMENT",
        error instanceof Error ? error.message : "Invalid product locator",
        { product_locator: rawLocator },
      );
    }

    const displayName = input.displayName?.trim();
    const nowIso = new Date().toISOString();

    return duckDbClient.transaction(async (tx) => {
      const existingById = await this.selectProductRecordBy(
        tx,
        "p.id = $product_id",
        { product_id: identity.productId },
      );

      if (existingById) {
        if (existingById.product.locator !== identity.canonicalProductLocator) {
          throw new ProductClientError(
            "ID_COLLISION_DETECTED",
            "Computed product ID collides with a different locator",
            {
              product_id: identity.productId,
              expected_locator: identity.canonicalProductLocator,
              actual_locator: existingById.product.locator,
            },
          );
        }

        return existingById;
      }

      const existingByLocator = await this.selectProductRecordBy(
        tx,
        "p.locator = $product_locator",
        { product_locator: identity.canonicalProductLocator },
      );

      if (existingByLocator) {
        if (existingByLocator.product.id !== identity.productId) {
          throw new ProductClientError(
            "ID_COLLISION_DETECTED",
            "Locator maps to an unexpected product ID",
            {
              locator: identity.canonicalProductLocator,
              expected_product_id: identity.productId,
              actual_product_id: existingByLocator.product.id,
            },
          );
        }

        return existingByLocator;
      }

      await tx.run(
        `
          INSERT INTO products (
            id,
            locator,
            display_name,
            default_variant_id,
            created_at,
            updated_at
          )
          VALUES (
            $id,
            $locator,
            $display_name,
            $default_variant_id,
            $created_at,
            $updated_at
          )
        `,
        {
          id: identity.productId,
          locator: identity.canonicalProductLocator,
          display_name: displayName && displayName.length > 0 ? displayName : null,
          default_variant_id: identity.defaultVariantId,
          created_at: nowIso,
          updated_at: nowIso,
        },
      );

      await tx.run(
        `
          INSERT INTO variants (
            id,
            product_id,
            locator,
            name,
            created_at
          )
          VALUES (
            $id,
            $product_id,
            $locator,
            $name,
            $created_at
          )
        `,
        {
          id: identity.defaultVariantId,
          product_id: identity.productId,
          locator: identity.canonicalDefaultVariantLocator,
          name: identity.defaultVariantName,
          created_at: nowIso,
        },
      );

      const created = await this.selectProductRecordBy(
        tx,
        "p.id = $product_id",
        { product_id: identity.productId },
      );

      if (!created) {
        throw new ProductClientError(
          "INTERNAL",
          "Product insert completed but no record was found",
          { product_id: identity.productId },
        );
      }

      return created;
    });
  }

  async getProduct(input: GetProductInput): Promise<ProductRecord | undefined> {
    await this.initialize();

    const hasProductId = Boolean(input.productId?.trim());
    const hasProductLocator = Boolean(input.productLocator?.trim());

    if (hasProductId === hasProductLocator) {
      throw new ProductClientError(
        "INVALID_ARGUMENT",
        "Exactly one lookup key must be provided",
        {
          product_id: input.productId ?? "",
          product_locator: input.productLocator ?? "",
        },
      );
    }

    if (hasProductId) {
      return this.selectProductRecordBy(
        duckDbClient,
        "p.id = $product_id",
        { product_id: input.productId!.trim() },
      );
    }

    let canonicalLocator: string;
    try {
      canonicalLocator = deriveStageZeroProductIdentity(input.productLocator!.trim()).canonicalProductLocator;
    } catch (error) {
      throw new ProductClientError(
        "INVALID_ARGUMENT",
        error instanceof Error ? error.message : "Invalid product locator",
        { product_locator: input.productLocator ?? "" },
      );
    }

    return this.selectProductRecordBy(
      duckDbClient,
      "p.locator = $product_locator",
      { product_locator: canonicalLocator },
    );
  }

  async listProducts(input: ListProductsInput): Promise<ProductListResult> {
    await this.initialize();

    const paging = normalizePagingInput(input);

    const rows = await duckDbClient.queryRows<ProductRecordRow>(
      `${baseSelectProductRecordSql}
       ORDER BY p.updated_at DESC
       LIMIT ${paging.limit}
       OFFSET ${paging.offset}`,
    );

    const totalRow = await duckDbClient.queryFirst<TotalCountRow>(
      "SELECT count(*) AS total_count FROM products",
    );

    return {
      records: rows.map(mapRowToProductRecord),
      totalCount: totalRow ? toCount(totalRow.total_count) : 0,
    };
  }

  async updateProduct(input: UpdateProductInput): Promise<ProductRecord | undefined> {
    await this.initialize();

    const productId = input.productId?.trim();
    if (!productId) {
      throw new ProductClientError(
        "INVALID_ARGUMENT",
        "product_id is required",
        { product_id: input.productId ?? "" },
      );
    }

    if (input.clearDisplayName && input.displayName !== undefined) {
      throw new ProductClientError(
        "INVALID_ARGUMENT",
        "display_name and clear_display_name cannot be used together",
        {
          clear_display_name: String(input.clearDisplayName),
        },
      );
    }

    return duckDbClient.transaction(async (tx) => {
      const existing = await this.selectProductRecordBy(tx, "p.id = $product_id", {
        product_id: productId,
      });

      if (!existing) {
        return undefined;
      }

      const normalizedDisplayName = input.displayName?.trim();
      const shouldClear = input.clearDisplayName;
      const hasDisplayUpdate = normalizedDisplayName !== undefined;

      if (!shouldClear && !hasDisplayUpdate) {
        return existing;
      }

      await tx.run(
        `
          UPDATE products
          SET display_name = $display_name,
              updated_at = $updated_at
          WHERE id = $product_id
        `,
        {
          product_id: productId,
          display_name: shouldClear
            ? null
            : normalizedDisplayName && normalizedDisplayName.length > 0
            ? normalizedDisplayName
            : null,
          updated_at: new Date().toISOString(),
        },
      );

      return this.selectProductRecordBy(tx, "p.id = $product_id", {
        product_id: productId,
      });
    });
  }

  async deleteProduct(input: DeleteProductInput): Promise<DeleteProductResult | undefined> {
    await this.initialize();

    const productId = input.productId?.trim();
    if (!productId) {
      throw new ProductClientError(
        "INVALID_ARGUMENT",
        "product_id is required",
        { product_id: input.productId ?? "" },
      );
    }

    return duckDbClient.transaction(async (tx) => {
      const existing = await this.selectProductRecordBy(tx, "p.id = $product_id", {
        product_id: productId,
      });

      if (!existing) {
        return undefined;
      }

      await tx.run(
        "DELETE FROM variants WHERE id = $variant_id",
        {
          variant_id: existing.defaultVariant.id,
        },
      );

      await tx.run(
        "DELETE FROM products WHERE id = $product_id",
        {
          product_id: productId,
        },
      );

      return {
        productId,
        defaultVariantId: existing.defaultVariant.id,
      };
    });
  }

  private async selectProductRecordBy(
    executor: DuckDbExecutor,
    whereClause: string,
    bindings: Record<string, unknown>,
  ): Promise<ProductRecord | undefined> {
    const row = await executor.queryFirst<ProductRecordRow>(
      `${baseSelectProductRecordSql} WHERE ${whereClause} LIMIT 1`,
      bindings,
    );

    return row ? mapRowToProductRecord(row) : undefined;
  }
}

export const productClient = new ProductClient();
