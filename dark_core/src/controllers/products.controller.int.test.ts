import { afterEach, beforeEach, describe, expect, it } from 'bun:test';

import { createSqliteTestDatabase, type SqliteTestDatabase } from '../test/helpers/sqlite-test-db';
import {
  createProduct,
  deleteProductById,
  getProductById,
  listProducts,
  updateProductById,
} from './products.controller';
import { buildDeterministicProductId } from '../utils/product-locator';
import { createVariant, listVariants } from './variants.controller';

describe('products controller integration', () => {
  let testDatabase: SqliteTestDatabase;

  beforeEach(async () => {
    testDatabase = createSqliteTestDatabase('products-controller');
    await testDatabase.setup();
  });

  afterEach(async () => {
    await testDatabase.teardown();
  });

  it('creates a local product and also creates default variant at same locator', async () => {
    const created = await createProduct({
      locator: '@local:///tmp/controller-test-product',
      displayName: 'Controller Test Product',
    });

    expect(created.id).toBe(
      buildDeterministicProductId('@local:///tmp/controller-test-product'),
    );
    expect(created.locator).toBe('@local:///tmp/controller-test-product');
    expect(created.displayName).toBe('Controller Test Product');

    const variants = await listVariants({ productId: created.id });

    expect(variants.length).toBe(1);
    expect(variants[0]?.name).toBe('default');
    expect(variants[0]?.locator).toBe(created.locator);
  });

  it('supports product CRUD operations', async () => {
    const created = await createProduct({
      locator: '@local:///tmp/controller-crud-product',
      displayName: 'Before Update',
    });

    const fetched = await getProductById(created.id);
    expect(fetched.id).toBe(created.id);

    const updated = await updateProductById(created.id, {
      displayName: 'After Update',
    });

    expect(updated.displayName).toBe('After Update');

    const deleted = await deleteProductById(created.id);
    expect(deleted.id).toBe(created.id);

    const listed = await listProducts({ limit: 10 });
    expect(listed.find((product) => product.id === created.id)).toBeUndefined();
  });

  it('treats repeated creates for same canonical locator as idempotent', async () => {
    const created = await createProduct({
      locator: '/tmp/idempotent-product/',
      displayName: 'Initial Name',
    });

    const repeated = await createProduct({
      locator: '@local:///tmp/idempotent-product',
      displayName: 'Ignored Name',
    });

    expect(repeated.id).toBe(created.id);
    expect(repeated.locator).toBe('@local:///tmp/idempotent-product');
    expect(repeated.displayName).toBe('Initial Name');

    const products = await listProducts({ limit: 10 });
    expect(products.length).toBe(1);

    const variants = await listVariants({ productId: created.id });
    expect(variants.length).toBe(1);
    expect(variants[0]?.name).toBe('default');
  });

  it('allows multiple variants at the same location for one product', async () => {
    const product = await createProduct({ locator: '@local:///tmp/same-location-product' });

    await createVariant({
      product: { connect: { id: product.id } },
      name: 'wt-main',
      locator: product.locator,
    });

    const variants = await listVariants({ productId: product.id, locator: product.locator });

    expect(variants.length).toBe(2);
    expect(variants.some((variant) => variant.name === 'default')).toBe(true);
    expect(variants.some((variant) => variant.name === 'wt-main')).toBe(true);
  });

  it('lists products using a unique sqlite database', async () => {
    const created = await createProduct({
      locator: '@local:///tmp/controller-test-list-product',
      displayName: 'Controller Test Product',
    });

    const products = await listProducts({ limit: 10 });

    expect(products.length).toBe(1);
    expect(products[0]?.id).toBe(created.id);
    expect(products[0]?.locator).toBe('@local:///tmp/controller-test-list-product');
  });

  it('normalizes list limit values', async () => {
    await createProduct({ locator: 'limit-test-product-1' });
    await createProduct({ locator: 'limit-test-product-2' });

    const products = await listProducts({ limit: 1.9 });
    expect(products.length).toBe(1);
  });
});
