import { afterEach, beforeEach, describe, expect, it } from 'bun:test';

import { buildApp } from '../app';
import { createSqliteTestDatabase, type SqliteTestDatabase } from '../test/helpers/sqlite-test-db';

describe('variants routes integration', () => {
  let testDatabase: SqliteTestDatabase;

  beforeEach(async () => {
    testDatabase = createSqliteTestDatabase('variants-routes');
    await testDatabase.setup();
  });

  afterEach(async () => {
    await testDatabase.teardown();
  });

  it('creates, lists, updates, fetches, and deletes variants through http handlers', async () => {
    const app = buildApp();

    const createProductResponse = await app.handle(
      new Request('http://localhost/products/', {
        method: 'POST',
        headers: {
          'content-type': 'application/json',
        },
        body: JSON.stringify({
          locator: '@local:///tmp/variants-route-product',
          displayName: 'Variants Route Product',
        }),
      }),
    );

    expect(createProductResponse.status).toBe(201);

    const productPayload = (await createProductResponse.json()) as {
      ok: true;
      data: { id: string; locator: string };
    };

    const createVariantResponse = await app.handle(
      new Request('http://localhost/variants/', {
        method: 'POST',
        headers: {
          'content-type': 'application/json',
        },
        body: JSON.stringify({
          name: 'wt-main',
          locator: productPayload.data.locator,
          product: {
            connect: {
              id: productPayload.data.id,
            },
          },
        }),
      }),
    );

    expect(createVariantResponse.status).toBe(201);

    const createdVariant = (await createVariantResponse.json()) as {
      ok: true;
      data: { id: string; productId: string; name: string; locator: string };
    };

    expect(createdVariant.data.productId).toBe(productPayload.data.id);
    expect(createdVariant.data.name).toBe('wt-main');
    expect(createdVariant.data.locator).toBe(productPayload.data.locator);

    const listVariantsResponse = await app.handle(
      new Request(
        `http://localhost/variants/?productId=${productPayload.data.id}&locator=${encodeURIComponent(productPayload.data.locator)}`,
      ),
    );
    expect(listVariantsResponse.status).toBe(200);

    const listedVariants = (await listVariantsResponse.json()) as {
      ok: true;
      data: Array<{ id: string; name: string }>;
    };

    expect(listedVariants.data.length).toBe(2);

    const getVariantResponse = await app.handle(
      new Request(`http://localhost/variants/${createdVariant.data.id}`),
    );
    expect(getVariantResponse.status).toBe(200);

    const updateVariantResponse = await app.handle(
      new Request(`http://localhost/variants/${createdVariant.data.id}`, {
        method: 'PATCH',
        headers: {
          'content-type': 'application/json',
        },
        body: JSON.stringify({
          name: 'wt-main-updated',
        }),
      }),
    );
    expect(updateVariantResponse.status).toBe(200);

    const updatedVariant = (await updateVariantResponse.json()) as {
      ok: true;
      data: { id: string; name: string };
    };
    expect(updatedVariant.data.id).toBe(createdVariant.data.id);
    expect(updatedVariant.data.name).toBe('wt-main-updated');

    const deleteVariantResponse = await app.handle(
      new Request(`http://localhost/variants/${createdVariant.data.id}`, {
        method: 'DELETE',
      }),
    );
    expect(deleteVariantResponse.status).toBe(200);

    const getDeletedVariantResponse = await app.handle(
      new Request(`http://localhost/variants/${createdVariant.data.id}`),
    );
    expect(getDeletedVariantResponse.status).toBe(404);
  });
});
