import { resolve } from 'node:path';
import { stat } from 'node:fs/promises';

import { locatorIdToHostPath, parseLocatorId } from '../../utils/locator';
import type { GitWorktreeSummary, ProductGitInfo, VariantGitInfo, VariantGitStatus } from './git.types';

interface GitCommandResult {
  ok: boolean;
  stdout: string;
  stderr: string;
}

interface GitRepositorySnapshot {
  repoName: string;
  remoteName: string | null;
  remoteUrl: string | null;
  authorName: string | null;
  authorEmail: string | null;
  branch: string | null;
  commit: string | null;
  repoRoot: string;
  gitDir: string;
  gitCommonDir: string;
  isLinkedWorktree: boolean;
  worktreePath: string;
  status: VariantGitStatus;
  worktrees: GitWorktreeSummary[];
}

const trimToNull = (value: string): string | null => {
  const trimmed = value.trim();
  return trimmed.length > 0 ? trimmed : null;
};

const runGit = async (args: string[], cwd: string): Promise<GitCommandResult> => {
  const command = Bun.spawn(['git', ...args], {
    cwd,
    stdout: 'pipe',
    stderr: 'pipe',
    stdin: 'ignore',
  });

  const [stdout, stderr, exitCode] = await Promise.all([
    new Response(command.stdout).text(),
    new Response(command.stderr).text(),
    command.exited,
  ]);

  return {
    ok: exitCode === 0,
    stdout,
    stderr,
  };
};

const parseWorktreeList = (raw: string): GitWorktreeSummary[] => {
  const lines = raw.split('\n');
  const worktrees: GitWorktreeSummary[] = [];
  let current: Partial<GitWorktreeSummary> | null = null;

  const flush = () => {
    if (!current?.path) {
      current = null;
      return;
    }

    worktrees.push({
      path: current.path,
      branch: current.branch ?? null,
      head: current.head ?? null,
      bare: current.bare ?? false,
      detached: current.detached ?? false,
      locked: current.locked ?? false,
      prunable: current.prunable ?? false,
    });

    current = null;
  };

  for (const line of lines) {
    if (line.startsWith('worktree ')) {
      flush();
      current = { path: line.slice('worktree '.length).trim() };
      continue;
    }

    if (!current) {
      continue;
    }

    if (line.length === 0) {
      flush();
      continue;
    }

    if (line.startsWith('HEAD ')) {
      current.head = line.slice('HEAD '.length).trim();
      continue;
    }

    if (line.startsWith('branch ')) {
      const rawBranch = line.slice('branch '.length).trim();
      current.branch = rawBranch.startsWith('refs/heads/')
        ? rawBranch.slice('refs/heads/'.length)
        : rawBranch;
      continue;
    }

    if (line === 'bare') {
      current.bare = true;
      continue;
    }

    if (line === 'detached') {
      current.detached = true;
      continue;
    }

    if (line.startsWith('locked')) {
      current.locked = true;
      continue;
    }

    if (line.startsWith('prunable')) {
      current.prunable = true;
    }
  }

  flush();

  return worktrees;
};

const parseStatus = (raw: string): VariantGitStatus => {
  let staged = 0;
  let unstaged = 0;
  let untracked = 0;
  let conflicted = 0;
  let ignored = 0;
  let upstream: string | null = null;
  let ahead = 0;
  let behind = 0;

  for (const line of raw.split('\n')) {
    if (line.startsWith('# branch.upstream ')) {
      upstream = trimToNull(line.slice('# branch.upstream '.length));
      continue;
    }

    if (line.startsWith('# branch.ab ')) {
      const match = line.match(/\+([0-9]+)\s-([0-9]+)/);

      if (match) {
        ahead = Number(match[1] ?? '0');
        behind = Number(match[2] ?? '0');
      }

      continue;
    }

    if (line.startsWith('? ')) {
      untracked += 1;
      continue;
    }

    if (line.startsWith('! ')) {
      ignored += 1;
      continue;
    }

    if (line.startsWith('u ')) {
      conflicted += 1;
      continue;
    }

    if (line.startsWith('1 ') || line.startsWith('2 ')) {
      const parts = line.split(' ');
      const xy = parts[1] ?? '..';
      const x = xy[0] ?? '.';
      const y = xy[1] ?? '.';

      if (x !== '.' && x !== ' ') {
        staged += 1;
      }

      if (y !== '.' && y !== ' ') {
        unstaged += 1;
      }
    }
  }

  return {
    clean: staged + unstaged + untracked + conflicted === 0,
    staged,
    unstaged,
    untracked,
    conflicted,
    ignored,
    upstream,
    ahead,
    behind,
  };
};

const resolveLocalDirectoryFromLocator = async (locator: string): Promise<string | null> => {
  const parsed = parseLocatorId(locator);

  if (parsed.type !== 'local') {
    return null;
  }

  let directoryPath: string;

  try {
    directoryPath = locatorIdToHostPath(parsed.locator);
  } catch {
    return null;
  }

  const info = await stat(directoryPath).catch(() => null);

  if (!info?.isDirectory()) {
    return null;
  }

  return directoryPath;
};

const loadGitRepositorySnapshot = async (locator: string): Promise<GitRepositorySnapshot | null> => {
  const directory = await resolveLocalDirectoryFromLocator(locator);

  if (!directory) {
    return null;
  }

  const insideWorkTree = await runGit(['rev-parse', '--is-inside-work-tree'], directory);

  if (!insideWorkTree.ok || trimToNull(insideWorkTree.stdout) !== 'true') {
    return null;
  }

  const [
    topLevel,
    gitDir,
    gitCommonDir,
    remoteUrl,
    authorName,
    authorEmail,
    branch,
    commit,
    status,
    worktreeList,
  ] = await Promise.all([
    runGit(['rev-parse', '--show-toplevel'], directory),
    runGit(['rev-parse', '--git-dir'], directory),
    runGit(['rev-parse', '--git-common-dir'], directory),
    runGit(['config', '--get', 'remote.origin.url'], directory),
    runGit(['config', '--get', 'user.name'], directory),
    runGit(['config', '--get', 'user.email'], directory),
    runGit(['branch', '--show-current'], directory),
    runGit(['rev-parse', 'HEAD'], directory),
    runGit(['status', '--porcelain=2', '--branch'], directory),
    runGit(['worktree', 'list', '--porcelain'], directory),
  ]);

  if (!topLevel.ok || !gitDir.ok || !gitCommonDir.ok || !status.ok) {
    return null;
  }

  const repoRoot = trimToNull(topLevel.stdout);
  const gitDirValue = trimToNull(gitDir.stdout);
  const gitCommonDirValue = trimToNull(gitCommonDir.stdout);

  if (!repoRoot || !gitDirValue || !gitCommonDirValue) {
    return null;
  }

  const absoluteGitDir = resolve(repoRoot, gitDirValue);
  const absoluteGitCommonDir = resolve(repoRoot, gitCommonDirValue);
  const worktrees = worktreeList.ok ? parseWorktreeList(worktreeList.stdout) : [];

  const repoNameParts = repoRoot.split('/').filter(Boolean);
  const repoName = repoNameParts[repoNameParts.length - 1] ?? repoRoot;

  return {
    repoName,
    remoteName: remoteUrl.ok && trimToNull(remoteUrl.stdout) ? 'origin' : null,
    remoteUrl: remoteUrl.ok ? trimToNull(remoteUrl.stdout) : null,
    authorName: authorName.ok ? trimToNull(authorName.stdout) : null,
    authorEmail: authorEmail.ok ? trimToNull(authorEmail.stdout) : null,
    branch: branch.ok ? trimToNull(branch.stdout) : null,
    commit: commit.ok ? trimToNull(commit.stdout) : null,
    repoRoot,
    gitDir: absoluteGitDir,
    gitCommonDir: absoluteGitCommonDir,
    isLinkedWorktree: absoluteGitDir !== absoluteGitCommonDir,
    worktreePath: repoRoot,
    status: parseStatus(status.stdout),
    worktrees,
  };
};

export const scanProductGitInfo = async (locator: string): Promise<ProductGitInfo | null> => {
  const snapshot = await loadGitRepositorySnapshot(locator);

  if (!snapshot) {
    return null;
  }

  return {
    repoName: snapshot.repoName,
    remoteName: snapshot.remoteName,
    remoteUrl: snapshot.remoteUrl,
    authorName: snapshot.authorName,
    authorEmail: snapshot.authorEmail,
    branch: snapshot.branch,
    commit: snapshot.commit,
    repoRoot: snapshot.repoRoot,
    gitDir: snapshot.gitDir,
    gitCommonDir: snapshot.gitCommonDir,
    isLinkedWorktree: snapshot.isLinkedWorktree,
    worktreePath: snapshot.worktreePath,
    worktreeCount: snapshot.worktrees.length,
    scannedAt: new Date().toISOString(),
  };
};

export const scanVariantGitInfo = async (locator: string): Promise<VariantGitInfo | null> => {
  const snapshot = await loadGitRepositorySnapshot(locator);

  if (!snapshot) {
    return null;
  }

  return {
    repoName: snapshot.repoName,
    remoteName: snapshot.remoteName,
    remoteUrl: snapshot.remoteUrl,
    authorName: snapshot.authorName,
    authorEmail: snapshot.authorEmail,
    branch: snapshot.branch,
    commit: snapshot.commit,
    repoRoot: snapshot.repoRoot,
    gitDir: snapshot.gitDir,
    gitCommonDir: snapshot.gitCommonDir,
    isLinkedWorktree: snapshot.isLinkedWorktree,
    worktreePath: snapshot.worktreePath,
    status: snapshot.status,
    worktrees: snapshot.worktrees,
    scannedAt: new Date().toISOString(),
  };
};
