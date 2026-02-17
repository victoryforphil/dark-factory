import { afterEach, beforeEach, describe, expect, it } from 'bun:test';
import { access, mkdir, mkdtemp, rm } from 'node:fs/promises';
import { join } from 'node:path';
import { tmpdir } from 'node:os';

import { buildApp } from '../../app';
import { createSqliteTestDatabase, type SqliteTestDatabase } from '../../test/helpers/sqlite-test-db';
import { locatorIdToHostPath } from '../../utils/locator';
import { createProduct } from '../products/products.controller';
import {
  createMockAgentSession,
  resetMockAgentEngineForTests,
  sendMockAgentSessionPrompt,
} from '../providers/mockagent/mockagent.controller';
import {
  createVariant,
  deleteVariantById,
  getVariantById,
  listVariants,
  syncVariantGitInfo,
  updateVariantById,
} from './variants.controller';

describe('variants module integration', () => {
  let testDatabase: SqliteTestDatabase;

  beforeEach(async () => {
    testDatabase = createSqliteTestDatabase('variants-module');
    await testDatabase.setup();
    resetMockAgentEngineForTests({
      startTimeMs: 1_700_100_000_000,
      timeStepMs: 1,
    });
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

    const pollVariantResponse = await app.handle(
      new Request(`http://localhost/variants/${createdVariant.data.id}/poll`, {
        method: 'POST',
      }),
    );
    expect(pollVariantResponse.status).toBe(200);

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

    const polledVariant = await syncVariantGitInfo(createdVariant.id);
    expect(polledVariant.gitInfoLastPolledAt).toBeTruthy();

    const variants = await listVariants({ productId: product.id, locator: product.locator });
    expect(variants.length).toBe(2);

    const deletedVariant = await deleteVariantById(createdVariant.id);
    expect(deletedVariant.id).toBe(createdVariant.id);

    const afterDelete = await listVariants({ productId: product.id });
    expect(afterDelete.length).toBe(1);
    expect(afterDelete[0]?.name).toBe('default');
  });

  it('deletes clone directory when variant delete uses dry=false', async () => {
    const workspace = await mkdtemp(join(tmpdir(), 'dark-factory-variant-delete-'));
    const clonePath = join(workspace, 'clone-dir');

    try {
      await mkdir(clonePath, { recursive: true });
      await Bun.write(join(clonePath, 'README.md'), '# clone');

      const product = await createProduct({ locator: `@local://${workspace}` });
      const createdVariant = await createVariant({
        product: {
          connect: {
            id: product.id,
          },
        },
        name: 'clone-delete',
        locator: `@local://${clonePath}`,
      });

      await deleteVariantById(createdVariant.id, { dry: false });

      const exists = await access(clonePath).then(
        () => true,
        () => false,
      );
      expect(exists).toBeFalse();
    } finally {
      await rm(workspace, { recursive: true, force: true });
    }
  });

  it('keeps clone directory when variant delete uses dry=true', async () => {
    const workspace = await mkdtemp(join(tmpdir(), 'dark-factory-variant-delete-'));
    const clonePath = join(workspace, 'clone-dir-keep');

    try {
      await mkdir(clonePath, { recursive: true });
      await Bun.write(join(clonePath, 'README.md'), '# clone');

      const product = await createProduct({ locator: `@local://${workspace}` });
      const createdVariant = await createVariant({
        product: {
          connect: {
            id: product.id,
          },
        },
        name: 'clone-keep',
        locator: `@local://${clonePath}`,
      });

      await deleteVariantById(createdVariant.id, { dry: true });

      const exists = await access(clonePath).then(
        () => true,
        () => false,
      );
      expect(exists).toBeTrue();
    } finally {
      await rm(workspace, { recursive: true, force: true });
    }
  });

  it('honors poll query defaults and overrides on variant read routes', async () => {
    const app = buildApp();
    const product = await createProduct({ locator: '@local:///tmp/variants-poll-query-product' });
    const defaultVariant = (await listVariants({ productId: product.id, poll: false }))[0];
    const defaultVariantId = defaultVariant?.id;

    expect(defaultVariantId).toBeTruthy();
    expect(defaultVariant?.gitInfoLastPolledAt).toBeNull();

    const getWithoutPoll = await app.handle(
      new Request(`http://localhost/variants/${defaultVariantId}?poll=false`),
    );
    expect(getWithoutPoll.status).toBe(200);

    const notPolledPayload = (await getWithoutPoll.json()) as {
      ok: true;
      data: { gitInfoLastPolledAt: string | null };
    };
    expect(notPolledPayload.data.gitInfoLastPolledAt).toBeNull();

    const getWithDefaultPoll = await app.handle(new Request(`http://localhost/variants/${defaultVariantId}`));
    expect(getWithDefaultPoll.status).toBe(200);

    const polledPayload = (await getWithDefaultPoll.json()) as {
      ok: true;
      data: { gitInfoLastPolledAt: string | null };
    };
    expect(polledPayload.data.gitInfoLastPolledAt).toBeTruthy();
  });

  it('imports active provider sessions into actors for a variant route', async () => {
    const app = buildApp();
    const product = await createProduct({ locator: '@local:///tmp/variants-import-actors-product' });
    const defaultVariant = (await listVariants({ productId: product.id, poll: false })).find(
      (variant) => variant.name === 'default',
    );

    expect(defaultVariant).toBeTruthy();

    const directory = locatorIdToHostPath(defaultVariant!.locator);
    const sessionOne = await createMockAgentSession({
      directory,
      title: 'Imported actor one',
    });
    await sendMockAgentSessionPrompt({
      directory,
      id: sessionOne.id,
      prompt: 'Capture this assistant description',
      noReply: false,
    });

    await createMockAgentSession({
      directory,
      title: 'Imported actor two',
    });

    const firstImportResponse = await app.handle(
      new Request(`http://localhost/variants/${defaultVariant!.id}/actors/import`, {
        method: 'POST',
        headers: {
          'content-type': 'application/json',
        },
        body: JSON.stringify({
          provider: 'mock',
        }),
      }),
    );

    expect(firstImportResponse.status).toBe(200);
    const firstImportPayload = (await firstImportResponse.json()) as {
      ok: true;
      data: {
        discovered: number;
        created: number;
        updated: number;
        actors: Array<{ id: string; description?: string | null }>;
      };
    };

    expect(firstImportPayload.data.discovered).toBe(2);
    expect(firstImportPayload.data.created).toBe(2);
    expect(firstImportPayload.data.updated).toBe(0);
    expect(firstImportPayload.data.actors.length).toBe(2);
    expect(
      firstImportPayload.data.actors.some((actor) =>
        typeof actor.description === 'string' && actor.description.trim().length > 0,
      ),
    ).toBeTrue();

    const secondImportResponse = await app.handle(
      new Request(`http://localhost/variants/${defaultVariant!.id}/actors/import`, {
        method: 'POST',
        headers: {
          'content-type': 'application/json',
        },
        body: JSON.stringify({
          provider: 'mock',
        }),
      }),
    );

    expect(secondImportResponse.status).toBe(200);
    const secondImportPayload = (await secondImportResponse.json()) as {
      ok: true;
      data: {
        discovered: number;
        created: number;
        updated: number;
      };
    };

    expect(secondImportPayload.data.discovered).toBe(2);
    expect(secondImportPayload.data.created).toBe(0);
    expect(secondImportPayload.data.updated).toBe(2);

    const listActorsResponse = await app.handle(
      new Request(`http://localhost/actors/?variantId=${defaultVariant!.id}&provider=mock`),
    );

    expect(listActorsResponse.status).toBe(200);
    const listActorsPayload = (await listActorsResponse.json()) as {
      ok: true;
      data: Array<{ id: string; description?: string | null }>;
    };

    expect(listActorsPayload.data.length).toBe(2);
    expect(
      listActorsPayload.data.some((actor) =>
        typeof actor.description === 'string' && actor.description.trim().length > 0,
      ),
    ).toBeTrue();
  });
});
