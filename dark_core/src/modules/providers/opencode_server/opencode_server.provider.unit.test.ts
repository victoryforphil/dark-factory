import { describe, expect, it } from 'bun:test';

import {
  __opencodeProviderInternals,
} from './opencode_server.provider';

describe('opencode server provider internals', () => {
  it('reads parent id across supported key aliases', () => {
    expect(__opencodeProviderInternals.readSessionParentId({ id: 's1', parentId: 'root' })).toBe('root');
    expect(__opencodeProviderInternals.readSessionParentId({ id: 's1', parentID: 'root' })).toBe('root');
    expect(__opencodeProviderInternals.readSessionParentId({ id: 's1', parent_id: 'root' })).toBe('root');
    expect(
      __opencodeProviderInternals.readSessionParentId({
        id: 's1',
        parent: { id: 'root' },
      }),
    ).toBe('root');
    expect(__opencodeProviderInternals.readSessionParentId({ id: 's1' })).toBeUndefined();
  });

  it('builds nested sub-agent tree from parent links', () => {
    const sessions = [
      { id: 'root', title: 'Root' },
      { id: 'child-a', title: 'Child A', parentID: 'root', updatedAt: 1_700_000_000 },
      { id: 'child-b', title: 'Child B', parentId: 'root' },
      { id: 'grandchild', title: 'Grandchild', parent_id: 'child-a' },
      { id: 'other-root', title: 'Other Root' },
    ];

    const tree = __opencodeProviderInternals.buildSubAgentTree({
      rootSessionId: 'root',
      sessions,
      statuses: {
        root: { type: 'busy' },
        'child-a': { type: 'idle' },
        'child-b': { type: 'retry' },
        grandchild: { type: 'busy' },
      },
    });

    expect(tree.length).toBe(2);
    expect(tree[0]?.id).toBe('child-a');
    expect(tree[0]?.parentId).toBe('root');
    expect(tree[0]?.depth).toBe(0);
    expect(tree[0]?.status).toBe('ready');
    expect(tree[0]?.updatedAt).toBeDefined();
    expect(tree[0]?.children?.[0]?.id).toBe('grandchild');
    expect(tree[0]?.children?.[0]?.depth).toBe(1);
    expect(tree[1]?.id).toBe('child-b');
    expect(tree[1]?.status).toBe('retrying');
  });

  it('selects active sessions when status map is present', () => {
    const sessions = [
      { id: 'root-a', time: { updated: 1_700_000_001_000 } },
      { id: 'root-b', time: { updated: 1_700_000_002_000 } },
      { id: 'child-b', parentID: 'root-b', time: { updated: 1_700_000_003_000 } },
    ];

    const selected = __opencodeProviderInternals.selectSessionsForImport({
      sessions,
      statuses: {
        'root-b': { type: 'busy' },
        'child-b': { type: 'idle' },
      },
    });

    expect(selected.map((session) => session.id)).toEqual(['root-b', 'child-b']);
  });

  it('falls back to recent sessions when status map is empty', () => {
    const now = Date.now();
    const selected = __opencodeProviderInternals.selectSessionsForImport({
      sessions: [
        { id: 'recent-a', time: { updated: now - 10_000 } },
        { id: 'recent-b', time: { updated: now - 20_000 } },
        { id: 'old', time: { updated: now - 1000 * 60 * 60 * 24 * 10 } },
      ],
      statuses: {},
    });

    const selectedIds = selected.map((session) => session.id);
    expect(selectedIds).toContain('recent-a');
    expect(selectedIds).toContain('recent-b');
    expect(selectedIds).not.toContain('old');
  });
});
