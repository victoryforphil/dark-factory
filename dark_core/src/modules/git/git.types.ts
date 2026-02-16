export interface GitWorktreeSummary {
  path: string;
  branch: string | null;
  head: string | null;
  bare: boolean;
  detached: boolean;
  locked: boolean;
  prunable: boolean;
}

export interface GitBaseInfo {
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
  scannedAt: string;
}

export interface ProductGitInfo extends GitBaseInfo {
  worktreeCount: number;
}

export interface VariantGitStatus {
  clean: boolean;
  staged: number;
  unstaged: number;
  untracked: number;
  conflicted: number;
  ignored: number;
  upstream: string | null;
  ahead: number;
  behind: number;
}

export interface VariantGitInfo extends GitBaseInfo {
  status: VariantGitStatus;
  worktrees: GitWorktreeSummary[];
}
