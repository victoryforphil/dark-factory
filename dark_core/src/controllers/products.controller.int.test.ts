import { afterEach, beforeEach, describe, expect, it } from 'bun:test';

import { createSqliteTestDatabase, type SqliteTestDatabase } from '../test/helpers/sqlite-test-db';
import { createProduct, listProducts } from './products.controller';

describe('products controller integration', () => {
  let testDatabase: SqliteTestDatabase;

  beforeEach(async () => {
    testDatabase = createSqliteTestDatabase('products-controller');
    await testDatabase.setup();
  });

  afterEach(async () => {
    await testDatabase.teardown();
  });

  it('creates and lists products using a unique sqlite database', async () => {
    const created = await createProduct({
      locator: 'controller-test-product',
      displayName: 'Controller Test Product',
    });

    expect(typeof created.id).toBe('string');
    expect(created.locator).toBe('controller-test-product');
    expect(created.displayName).toBe('Controller Test Product');

    const products = await listProducts({ limit: 10 });

    expect(products.length).toBe(1);
    expect(products[0]?.id).toBe(created.id);
    expect(products[0]?.locator).toBe('controller-test-product');
  });

  it('normalizes list limit values', async () => {
    await createProduct({ locator: 'limit-test-product-1' });
    await createProduct({ locator: 'limit-test-product-2' });

    const products = await listProducts({ limit: 1.9 });
    expect(products.length).toBe(1);
  });
});
