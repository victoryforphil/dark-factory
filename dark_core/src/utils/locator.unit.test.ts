import { describe, expect, it } from 'bun:test';

import {
  buildGitLocator,
  buildDeterministicIdFromLocator,
  canonicalizeGitLocator,
  canonicalizeLocalLocator,
  canonicalizeSshLocator,
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

  it('canonicalizes git locator strings', () => {
    const locator = canonicalizeGitLocator('@git:// https://github.com/acme/dark-factory.git # main ');

    expect(locator).toBe('@git://https://github.com/acme/dark-factory.git#main');
  });

  it('canonicalizes ssh locator strings', () => {
    const locator = canonicalizeSshLocator('@ssh://devbox//srv/work/../project/');

    expect(locator).toBe('@ssh://devbox/srv/project');
  });

  it('builds git locator identifiers from remote and branch', () => {
    const locator = buildGitLocator('git@github.com:acme/dark-factory.git', 'master');

    expect(locator).toBe('@git://git@github.com:acme/dark-factory.git#master');
  });

  it('builds deterministic short product IDs', () => {
    const productId = buildDeterministicIdFromLocator('@local:///tmp/project');

    expect(productId).toBe('prd_1ymnvqkybkq94');
  });

  it('parses locator ids by type', () => {
    const local = parseLocatorId('@local:///tmp/project');
    const git = parseLocatorId('@git://https://github.com/acme/dark-factory.git#main');
    const ssh = parseLocatorId('@ssh://devbox/srv/project');
    const unknown = parseLocatorId('repo://dark-factory/product-a');

    expect(local).toEqual({
      type: 'local',
      locator: '@local:///tmp/project',
      canonicalPath: '/tmp/project',
    });
    expect(git).toEqual({
      type: 'git',
      locator: '@git://https://github.com/acme/dark-factory.git#main',
      remote: 'https://github.com/acme/dark-factory.git',
      ref: 'main',
    });
    expect(ssh).toEqual({
      type: 'ssh',
      locator: '@ssh://devbox/srv/project',
      host: 'devbox',
      path: '/srv/project',
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
