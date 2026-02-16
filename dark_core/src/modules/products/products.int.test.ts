import { afterEach, beforeEach, describe, expect, it } from 'bun:test';

import { buildApp } from '../../app';
import { createSqliteTestDatabase, type SqliteTestDatabase } from '../../test/helpers/sqlite-test-db';
import { createVariant, deleteVariantById, listVariants } from '../variants/variants.controller';
import { buildDeterministicIdFromLocator } from '../../utils/locator';
import {
  createProduct,
  deleteProductById,
  getProductById,
  listProducts,
  updateProductById,
} from './products.controller';

describe('products module integration', () => {
  let testDatabase: SqliteTestDatabase;

  beforeEach(async () => {
    testDatabase = createSqliteTestDatabase('products-module');
    await testDatabase.setup();
  });

  afterEach(async () => {
    await testDatabase.teardown();
  });

  it('creates and lists products through http handlers', async () => {
    const app = buildApp();

    const createResponse = await app.handle(
      new Request('http://localhost/products/', {
        method: 'POST',
        headers: {
          'content-type': 'application/json',
        },
        body: JSON.stringify({
          locator: '@local:///tmp/route-test-product',
          displayName: 'Route Test Product',
        }),
      }),
    );

    expect(createResponse.status).toBe(201);

    const created = (await createResponse.json()) as {
      ok: true;
      data: { id: string; locator: string; displayName: string | null };
    };

    expect(created.ok).toBe(true);
    expect(created.data.id).toBe(buildDeterministicIdFromLocator('@local:///tmp/route-test-product'));
    expect(created.data.locator).toBe('@local:///tmp/route-test-product');

    const variantsResponse = await app.handle(
      new Request(`http://localhost/variants/?productId=${created.data.id}`),
    );
    expect(variantsResponse.status).toBe(200);

    const listedVariants = (await variantsResponse.json()) as {
      ok: true;
      data: Array<{ productId: string; name: string; locator: string }>;
    };

    expect(listedVariants.data.length).toBe(1);
    expect(listedVariants.data[0]?.productId).toBe(created.data.id);
    expect(listedVariants.data[0]?.name).toBe('default');
    expect(listedVariants.data[0]?.locator).toBe(created.data.locator);

    const listResponse = await app.handle(new Request('http://localhost/products/'));

    expect(listResponse.status).toBe(200);

    const listed = (await listResponse.json()) as {
      ok: true;
      data: Array<{ id: string; locator: string }>;
    };

    expect(listed.ok).toBe(true);
    expect(listed.data.length).toBe(1);
    expect(listed.data[0]?.id).toBe(created.data.id);
    expect(listed.data[0]?.locator).toBe('@local:///tmp/route-test-product');

    const listFullResponse = await app.handle(new Request('http://localhost/products/?include=full'));
    expect(listFullResponse.status).toBe(200);

    const listedFull = (await listFullResponse.json()) as {
      ok: true;
      data: Array<{
        id: string;
        variants?: Array<{ productId: string; name: string; locator: string }>;
      }>;
    };

    expect(Array.isArray(listedFull.data[0]?.variants)).toBe(true);
    expect(listedFull.data[0]?.variants?.[0]?.productId).toBe(created.data.id);
    expect(listedFull.data[0]?.variants?.[0]?.name).toBe('default');

    const getResponse = await app.handle(new Request(`http://localhost/products/${created.data.id}`));
    expect(getResponse.status).toBe(200);

    const getFullResponse = await app.handle(
      new Request(`http://localhost/products/${created.data.id}?include=full`),
    );
    expect(getFullResponse.status).toBe(200);

    const fetchedFull = (await getFullResponse.json()) as {
      ok: true;
      data: { id: string; variants?: Array<{ name: string; locator: string }> };
    };

    expect(fetchedFull.data.id).toBe(created.data.id);
    expect(fetchedFull.data.variants?.some((variant) => variant.name === 'default')).toBe(true);

    const updateResponse = await app.handle(
      new Request(`http://localhost/products/${created.data.id}`, {
        method: 'PATCH',
        headers: {
          'content-type': 'application/json',
        },
        body: JSON.stringify({
          displayName: 'Route Test Product Updated',
        }),
      }),
    );

    expect(updateResponse.status).toBe(200);

    const updated = (await updateResponse.json()) as {
      ok: true;
      data: { id: string; displayName: string | null };
    };

    expect(updated.data.id).toBe(created.data.id);
    expect(updated.data.displayName).toBe('Route Test Product Updated');

    const deleteResponse = await app.handle(
      new Request(`http://localhost/products/${created.data.id}`, {
        method: 'DELETE',
      }),
    );
    expect(deleteResponse.status).toBe(200);

    const getDeletedResponse = await app.handle(
      new Request(`http://localhost/products/${created.data.id}`),
    );
    expect(getDeletedResponse.status).toBe(404);
  });

  it('creates a local product and also creates default variant at same locator', async () => {
    const created = await createProduct({
      locator: '@local:///tmp/controller-test-product',
      displayName: 'Controller Test Product',
    });

    expect(created.id).toBe(buildDeterministicIdFromLocator('@local:///tmp/controller-test-product'));
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

  it('re-ensures default local variant when creating an existing local product', async () => {
    const created = await createProduct({
      locator: '@local:///tmp/recreate-default-variant',
    });

    const existingVariants = await listVariants({ productId: created.id, poll: false });
    expect(existingVariants.length).toBe(1);
    const defaultVariantId = existingVariants[0]?.id;
    expect(defaultVariantId).toBeTruthy();

    await deleteVariantById(defaultVariantId as string);

    const recreated = await createProduct({
      locator: '@local:///tmp/recreate-default-variant',
    });

    const recreatedVariants = await listVariants({ productId: recreated.id, poll: false });
    expect(recreatedVariants.length).toBe(1);
    expect(recreatedVariants[0]?.name).toBe('default');
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
