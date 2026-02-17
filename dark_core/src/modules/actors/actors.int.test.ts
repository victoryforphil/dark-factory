import { afterEach, beforeEach, describe, expect, it } from 'bun:test';

import { buildApp } from '../../app';
import { createSqliteTestDatabase, type SqliteTestDatabase } from '../../test/helpers/sqlite-test-db';
import { resetMockAgentEngineForTests } from '../providers/mockagent/mockagent.controller';
import { listVariants } from '../variants/variants.controller';

describe('actors module integration', () => {
  let testDatabase: SqliteTestDatabase;

  beforeEach(async () => {
    testDatabase = createSqliteTestDatabase('actors-module');
    await testDatabase.setup();
    resetMockAgentEngineForTests({
      startTimeMs: 1_700_000_000_000,
      timeStepMs: 1,
    });
  });

  afterEach(async () => {
    await testDatabase.teardown();
  });

  it('spawns actor for variant and supports poll/messages/commands', async () => {
    const app = buildApp();

    const createProductResponse = await app.handle(
      new Request('http://localhost/products/', {
        method: 'POST',
        headers: {
          'content-type': 'application/json',
        },
        body: JSON.stringify({
          locator: '@local:///tmp/actors-int-product',
          displayName: 'Actors Int Product',
        }),
      }),
    );
    expect(createProductResponse.status).toBe(201);

    const createdProduct = (await createProductResponse.json()) as { ok: true; data: { id: string } };
    const variants = await listVariants({ productId: createdProduct.data.id, poll: false });
    const defaultVariantId = variants.find((variant) => variant.name === 'default')?.id;
    expect(defaultVariantId).toBeTruthy();

    const createActorResponse = await app.handle(
      new Request('http://localhost/actors/', {
        method: 'POST',
        headers: {
          'content-type': 'application/json',
        },
        body: JSON.stringify({
          variantId: defaultVariantId,
          provider: 'mock',
          title: 'integration actor',
        }),
      }),
    );

    expect(createActorResponse.status).toBe(201);
    const createdActorPayload = (await createActorResponse.json()) as {
      ok: true;
      data: { id: string; status: string; provider: string };
    };
    expect(createdActorPayload.data.provider).toBe('mock');
    expect(createdActorPayload.data.status).toBe('ready');

    const actorId = createdActorPayload.data.id;

    const createVariantResponse = await app.handle(
      new Request(`http://localhost/products/${createdProduct.data.id}/variants`, {
        method: 'POST',
        headers: {
          'content-type': 'application/json',
        },
        body: JSON.stringify({
          locator: '@local:///tmp/actors-int-product-move',
          name: 'move-target',
        }),
      }),
    );
    expect(createVariantResponse.status).toBe(201);

    const createVariantPayload = (await createVariantResponse.json()) as {
      ok: true;
      data: { id: string; locator: string };
    };

    const pollResponse = await app.handle(
      new Request(`http://localhost/actors/${actorId}/poll`, {
        method: 'POST',
      }),
    );
    expect(pollResponse.status).toBe(200);

    const sendMessageResponse = await app.handle(
      new Request(`http://localhost/actors/${actorId}/messages`, {
        method: 'POST',
        headers: {
          'content-type': 'application/json',
        },
        body: JSON.stringify({
          prompt: 'hello integration',
        }),
      }),
    );
    expect(sendMessageResponse.status).toBe(200);

    const listMessagesResponse = await app.handle(
      new Request(`http://localhost/actors/${actorId}/messages?nLastMessages=1`),
    );
    expect(listMessagesResponse.status).toBe(200);

    const messagesPayload = (await listMessagesResponse.json()) as {
      ok: true;
      data: Array<{ role: string; text?: string }>;
    };
    expect(messagesPayload.data.length).toBe(1);
    expect(messagesPayload.data[0]?.role).toBe('assistant');

    const commandResponse = await app.handle(
      new Request(`http://localhost/actors/${actorId}/commands`, {
        method: 'POST',
        headers: {
          'content-type': 'application/json',
        },
        body: JSON.stringify({
          command: '/busy',
        }),
      }),
    );
    expect(commandResponse.status).toBe(200);

    const attachResponse = await app.handle(
      new Request(`http://localhost/actors/${actorId}/attach?model=openai/gpt-5&agent=general`),
    );
    expect(attachResponse.status).toBe(200);

    const listActorsResponse = await app.handle(
      new Request(`http://localhost/actors/?variantId=${defaultVariantId}&provider=mock`),
    );
    expect(listActorsResponse.status).toBe(200);

    const moveActorResponse = await app.handle(
      new Request(`http://localhost/actors/${actorId}`, {
        method: 'PATCH',
        headers: {
          'content-type': 'application/json',
        },
        body: JSON.stringify({
          variantId: createVariantPayload.data.id,
        }),
      }),
    );
    expect(moveActorResponse.status).toBe(200);

    const movedActorPayload = (await moveActorResponse.json()) as {
      ok: true;
      data: { variantId: string; workingLocator: string };
    };
    expect(movedActorPayload.data.variantId).toBe(createVariantPayload.data.id);
    expect(movedActorPayload.data.workingLocator).toBe(createVariantPayload.data.locator);

    const deleteActorResponse = await app.handle(
      new Request(`http://localhost/actors/${actorId}`, {
        method: 'DELETE',
      }),
    );
    expect(deleteActorResponse.status).toBe(200);

    const getDeletedActorResponse = await app.handle(
      new Request(`http://localhost/actors/${actorId}`),
    );
    expect(getDeletedActorResponse.status).toBe(404);
  });
});
