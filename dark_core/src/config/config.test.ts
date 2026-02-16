import { mkdtempSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { tmpdir } from 'node:os';

import { describe, expect, it } from 'bun:test';

import { coreConfigEnvBindings } from './core.config';
import { loadConfig } from './lib/load';
import { envOverlayFromBindings, parseEnvValue } from './lib/env';

describe('config env parser', () => {
  it('parses booleans, numbers, JSON, and empty values', () => {
    expect(parseEnvValue('true')).toBe(true);
    expect(parseEnvValue('FALSE')).toBe(false);
    expect(parseEnvValue('41')).toBe(41);
    expect(parseEnvValue('1.5')).toBe(1.5);
    expect(parseEnvValue('["a","b"]')).toEqual(['a', 'b']);
    expect(parseEnvValue('')).toBeUndefined();
  });

  it('maps globally named env keys using explicit bindings', () => {
    const overlay = envOverlayFromBindings(
      {
        DARKFACTORY_SERVER_PORT: '5020',
        DARKFACTORY_PRISMA_LOG_QUERIES: 'true',
      },
      coreConfigEnvBindings,
    );

    expect(overlay).toEqual({
      server: { listenPort: 5020 },
      prisma: { logQueries: true },
    });
  });
});

describe('loadConfig', () => {
  it('applies defaults then TOML then env overrides', () => {
    const workdir = mkdtempSync(join(tmpdir(), 'dark-core-config-'));
    const configPath = join(workdir, 'config.toml');

    writeFileSync(
      configPath,
      ['[server]', 'listenHost = "0.0.0.0"', 'listenPort = 6000', '', '[prisma]', 'logQueries = true'].join(
        '\n',
      ),
      'utf8',
    );

    const loaded = loadConfig({
      path: configPath,
      strict: true,
      env: {
        DARKFACTORY_SERVER_PORT: '7001',
      },
    });

    expect(loaded.server.listenHost).toBe('0.0.0.0');
    expect(loaded.server.listenPort).toBe(7001);
    expect(loaded.prisma.logQueries).toBe(true);
  });

  it('rejects unknown keys in strict mode and allows them when configured', () => {
    const workdir = mkdtempSync(join(tmpdir(), 'dark-core-config-'));
    const configPath = join(workdir, 'config.toml');

    writeFileSync(configPath, ['[server]', 'unknown_field = "hello"'].join('\n'), 'utf8');

    expect(() => loadConfig({ path: configPath, strict: true, env: {} })).toThrow();

    expect(() => loadConfig({ path: configPath, allowUnknown: true, env: {} })).not.toThrow();
  });

  it('supports explicit env bindings without prefix', () => {
    const loaded = loadConfig({
      path: join(mkdtempSync(join(tmpdir(), 'dark-core-config-')), 'missing.toml'),
      strict: true,
      env: {
        DARKFACTORY_SERVER_PORT: '9090',
      },
    });

    expect(loaded.server.listenPort).toBe(9090);
  });
});
