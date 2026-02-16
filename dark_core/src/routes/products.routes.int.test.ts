import { afterEach, beforeEach, describe, expect, it } from 'bun:test';

import { buildApp } from '../app';
import { createSqliteTestDatabase, type SqliteTestDatabase } from '../test/helpers/sqlite-test-db';

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
          locator: 'route-test-product',
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
    expect(typeof created.data.id).toBe('string');
    expect(created.data.locator).toBe('route-test-product');

    const listResponse = await app.handle(new Request('http://localhost/products/'));

    expect(listResponse.status).toBe(200);

    const listed = (await listResponse.json()) as {
      ok: true;
      data: Array<{ id: string; locator: string }>;
    };

    expect(listed.ok).toBe(true);
    expect(listed.data.length).toBe(1);
    expect(listed.data[0]?.id).toBe(created.data.id);
    expect(listed.data[0]?.locator).toBe('route-test-product');
  });
});
