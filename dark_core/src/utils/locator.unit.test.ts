import { describe, expect, it } from 'bun:test';

import {
  buildDeterministicIdFromLocator,
  canonicalizeLocalLocator,
  hostAbsolutePathToLocatorId,
  locatorIdToHostPath,
  normalizeLocator,
  parseLocatorId,
} from './locator';

describe('locator utilities', () => {
  it('canonicalizes local locator strings', () => {
    const locator = canonicalizeLocalLocator('@local:///tmp/./demo/../project/');

    expect(locator).toBe('@local:///tmp/project');
  });

  it('normalizes raw absolute filesystem paths into local locators', () => {
    const locator = normalizeLocator('/tmp/../tmp/project/');

    expect(locator).toBe('@local:///tmp/project');
  });

  it('keeps non-local locators unchanged', () => {
    const locator = normalizeLocator('repo://dark-factory/product-a');

    expect(locator).toBe('repo://dark-factory/product-a');
  });

  it('builds deterministic short product IDs', () => {
    const productId = buildDeterministicIdFromLocator('@local:///tmp/project');

    expect(productId).toBe('prd_1ymnvqkybkq94');
  });

  it('parses locator ids by type', () => {
    const local = parseLocatorId('@local:///tmp/project');
    const unknown = parseLocatorId('repo://dark-factory/product-a');

    expect(local).toEqual({
      type: 'local',
      locator: '@local:///tmp/project',
      canonicalPath: '/tmp/project',
    });
    expect(unknown).toEqual({
      type: 'unknown',
      locator: 'repo://dark-factory/product-a',
    });
  });

  it('converts local locator ids back to host absolute paths', () => {
    const hostPath = locatorIdToHostPath('@local:///tmp/project');
    expect(hostPath).toBe('/tmp/project');
  });

  it('converts host absolute paths into local locator ids', () => {
    const locator = hostAbsolutePathToLocatorId('/tmp/project');
    expect(locator).toBe('@local:///tmp/project');
  });
});
