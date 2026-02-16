import { afterEach, beforeEach, describe, expect, it } from 'bun:test';

import { buildApp } from '../app';
import { createSqliteTestDatabase, type SqliteTestDatabase } from '../test/helpers/sqlite-test-db';
import { buildDeterministicProductId } from '../utils/product-locator';

describe('products routes integration', () => {
  let testDatabase: SqliteTestDatabase;

  beforeEach(async () => {
    testDatabase = createSqliteTestDatabase('products-routes');
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
    expect(created.data.id).toBe(buildDeterministicProductId('@local:///tmp/route-test-product'));
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

    const getResponse = await app.handle(
      new Request(`http://localhost/products/${created.data.id}`),
    );
    expect(getResponse.status).toBe(200);

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
});
