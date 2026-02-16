import { afterEach, beforeEach, describe, expect, it } from 'bun:test';

import { createSqliteTestDatabase, type SqliteTestDatabase } from '../test/helpers/sqlite-test-db';
import { createProduct } from './products.controller';
import {
  createVariant,
  deleteVariantById,
  getVariantById,
  listVariants,
  updateVariantById,
} from './variants.controller';

describe('variants controller integration', () => {
  let testDatabase: SqliteTestDatabase;

  beforeEach(async () => {
    testDatabase = createSqliteTestDatabase('variants-controller');
    await testDatabase.setup();
  });

  afterEach(async () => {
    await testDatabase.teardown();
  });

  it('supports variant CRUD operations', async () => {
    const product = await createProduct({ locator: '@local:///tmp/variants-controller-product' });

    const createdVariant = await createVariant({
      product: {
        connect: {
          id: product.id,
        },
      },
      name: 'variant-a',
      locator: product.locator,
    });

    expect(createdVariant.productId).toBe(product.id);
    expect(createdVariant.name).toBe('variant-a');
    expect(createdVariant.locator).toBe(product.locator);

    const fetchedVariant = await getVariantById(createdVariant.id);
    expect(fetchedVariant.id).toBe(createdVariant.id);

    const updatedVariant = await updateVariantById(createdVariant.id, {
      name: 'variant-a-updated',
    });
    expect(updatedVariant.name).toBe('variant-a-updated');

    const variants = await listVariants({ productId: product.id, locator: product.locator });
    expect(variants.length).toBe(2);

    const deletedVariant = await deleteVariantById(createdVariant.id);
    expect(deletedVariant.id).toBe(createdVariant.id);

    const afterDelete = await listVariants({ productId: product.id });
    expect(afterDelete.length).toBe(1);
    expect(afterDelete[0]?.name).toBe('default');
  });
});
