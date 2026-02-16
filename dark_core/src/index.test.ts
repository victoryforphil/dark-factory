import { describe, expect, it } from 'bun:test';

import { buildApp } from './app';

describe('dark_core app', () => {
  it('serves root health payload', async () => {
    const app = buildApp();
    const response = await app.handle(new Request('http://localhost/'));

    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toEqual({
      service: 'dark_core',
      status: 'ok',
    });
  });
});
