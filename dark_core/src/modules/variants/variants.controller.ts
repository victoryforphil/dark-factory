import { Prisma, type Variant } from '../../../../generated/prisma/client';
import { rm, stat } from 'node:fs/promises';

import { getPrismaClient } from '../prisma/prisma.client';
import Log, { formatLogMetadata } from '../../utils/logging';
import { NotFoundError } from '../common/controller.errors';
import type { CursorListQuery } from '../common/controller.types';
import { scanVariantGitInfo } from '../git/git.scan';
import { buildRandomVariantId } from '../../utils/id';
import { isLocalLocator, locatorIdToHostPath } from '../../utils/locator';

const DEFAULT_LIST_LIMIT = 25;
const MAX_LIST_LIMIT = 100;
const DEFAULT_POLL_BEFORE_READ = true;

export interface ListVariantsQuery extends CursorListQuery {
  productId?: string;
  locator?: string;
  name?: string;
  poll?: boolean;
}

export interface GetVariantOptions {
  poll?: boolean;
}

export interface CreateVariantInput {
  product: {
    connect: {
      id: string;
    };
  };
  name?: string;
  locator: string;
}

export interface UpdateVariantInput {
  name?: string;
  locator?: string;
}

export interface DeleteVariantOptions {
  dry?: boolean;
}

export interface CheckoutVariantBranchInput {
  branchName: string;
}

const normalizeLimit = (value?: number): number => {
  if (typeof value !== 'number' || Number.isNaN(value)) {
    return DEFAULT_LIST_LIMIT;
  }

  return Math.max(1, Math.min(MAX_LIST_LIMIT, Math.floor(value)));
};

const toPrismaJson = (value: Variant['gitInfo']): Prisma.InputJsonValue | Prisma.DbNull => {
  if (value === null || value === undefined) {
    return Prisma.DbNull;
  }

  return value as Prisma.InputJsonValue;
};

const isJsonRecord = (value: unknown): value is Record<string, unknown> => {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
};

const mergeCloneMetadata = (
  gitInfo: Variant['gitInfo'],
  previousGitInfo: Variant['gitInfo'],
): Variant['gitInfo'] => {
  const previousClone = isJsonRecord(previousGitInfo) ? previousGitInfo._clone : undefined;
  if (previousClone === undefined) {
    return gitInfo;
  }

  const nextBase = isJsonRecord(gitInfo) ? { ...gitInfo } : {};
  return {
    ...nextBase,
    _clone: previousClone,
  };
};

export const listVariants = async (query: ListVariantsQuery = {}): Promise<Variant[]> => {
  const prisma = getPrismaClient();
  const limit = normalizeLimit(query.limit);
  const poll = query.poll ?? DEFAULT_POLL_BEFORE_READ;

  Log.debug(
    `Core // Variants Controller // Listing variants ${formatLogMetadata({
      cursor: query.cursor ?? null,
      limit,
      locator: query.locator ?? null,
      name: query.name ?? null,
      poll,
      productId: query.productId ?? null,
    })}`,
  );

  const variants = await prisma.variant.findMany({
    where: {
      ...(query.productId ? { productId: query.productId } : {}),
      ...(query.locator ? { locator: query.locator } : {}),
      ...(query.name ? { name: query.name } : {}),
    },
    take: limit,
    orderBy: { createdAt: 'desc' },
    ...(query.cursor ? { cursor: { id: query.cursor }, skip: 1 } : {}),
  });

  if (!poll || variants.length === 0) {
    return variants;
  }

  Log.info(
    `Core // Variants Controller // Polling before list response ${formatLogMetadata({
      count: variants.length,
      locator: query.locator ?? null,
      productId: query.productId ?? null,
    })}`,
  );

  return Promise.all(variants.map((variant) => syncVariantGitInfo(variant.id)));
};

export const getVariantById = async (id: string, options: GetVariantOptions = {}): Promise<Variant> => {
  const poll = options.poll ?? DEFAULT_POLL_BEFORE_READ;

  if (poll) {
    Log.info(
      `Core // Variants Controller // Polling before get response ${formatLogMetadata({ id })}`,
    );
    return syncVariantGitInfo(id);
  }

  const prisma = getPrismaClient();

  Log.debug(`Core // Variants Controller // Getting variant ${formatLogMetadata({ id })}`);

  const variant = await prisma.variant.findUnique({ where: { id } });

  if (!variant) {
    Log.warn(`Core // Variants Controller // Variant not found ${formatLogMetadata({ id })}`);
    throw new NotFoundError(`Variant ${id} was not found`);
  }

  return variant;
};

export const createVariant = async (input: CreateVariantInput): Promise<Variant> => {
  const prisma = getPrismaClient();

  const createdVariant = await prisma.variant.create({
    data: {
      id: buildRandomVariantId(),
      name: input.name,
      locator: input.locator,
      product: {
        connect: {
          id: input.product.connect.id,
        },
      },
    },
  });

  Log.info(
    `Core // Variants Controller // Variant created ${formatLogMetadata({
      id: createdVariant.id,
      locator: createdVariant.locator,
      name: createdVariant.name,
      productId: createdVariant.productId,
    })}`,
  );

  return syncVariantGitInfo(createdVariant.id);
};

export const syncVariantGitInfo = async (id: string): Promise<Variant> => {
  const prisma = getPrismaClient();

  const existingVariant = await prisma.variant.findUnique({ where: { id } });

  if (!existingVariant) {
    Log.warn(
      `Core // Variants Controller // Git sync skipped, variant not found ${formatLogMetadata({ id })}`,
    );
    throw new NotFoundError(`Variant ${id} was not found`);
  }

  const gitInfo = await scanVariantGitInfo(existingVariant.locator);
  const mergedGitInfo = mergeCloneMetadata(gitInfo, existingVariant.gitInfo);
  const now = new Date();

  const updatedVariant = await prisma.variant.update({
    where: { id },
    data: {
      gitInfo: toPrismaJson(mergedGitInfo),
      gitInfoLastPolledAt: now,
      gitInfoUpdatedAt: mergedGitInfo ? now : null,
    },
  });

  Log.debug(
    `Core // Variants Controller // Git info synchronized ${formatLogMetadata({
      hasGitInfo: Boolean(gitInfo),
      id,
      locator: existingVariant.locator,
    })}`,
  );

  return updatedVariant;
};

export const updateVariantById = async (
  id: string,
  input: UpdateVariantInput,
): Promise<Variant> => {
  const prisma = getPrismaClient();

  const existingVariant = await prisma.variant.findUnique({ where: { id } });

  if (!existingVariant) {
    Log.warn(
      `Core // Variants Controller // Update skipped, variant not found ${formatLogMetadata({ id })}`,
    );
    throw new NotFoundError(`Variant ${id} was not found`);
  }

  const updatedVariant = await prisma.variant.update({
    where: { id },
    data: {
      ...(input.name !== undefined ? { name: input.name } : {}),
      ...(input.locator !== undefined ? { locator: input.locator } : {}),
    },
  });

  Log.info(`Core // Variants Controller // Variant updated ${formatLogMetadata({ id })}`);

  return updatedVariant;
};

export const deleteVariantById = async (
  id: string,
  options: DeleteVariantOptions = {},
): Promise<Variant> => {
  const prisma = getPrismaClient();
  const dry = options.dry ?? false;

  Log.info(
    `Core // Variants Controller // Delete requested ${formatLogMetadata({
      dry,
      id,
    })}`,
  );

  const existingVariant = await prisma.variant.findUnique({ where: { id } });

  if (!existingVariant) {
    Log.warn(
      `Core // Variants Controller // Delete skipped, variant not found ${formatLogMetadata({ id })}`,
    );
    throw new NotFoundError(`Variant ${id} was not found`);
  }

  Log.debug(
    `Core // Variants Controller // Delete resolved target ${formatLogMetadata({
      dry,
      id,
      locator: existingVariant.locator,
      productId: existingVariant.productId,
      variantName: existingVariant.name,
    })}`,
  );

  await undoVariantCloneIfRequested(existingVariant, dry);

  const deletedVariant = await prisma.variant.delete({ where: { id } });
  Log.info(
    `Core // Variants Controller // Variant deleted ${formatLogMetadata({
      dry,
      id,
    })}`,
  );
  return deletedVariant;
};

interface GitCommandResult {
  ok: boolean;
  stdout: string;
  stderr: string;
  exitCode: number;
}

const trimToNull = (value: string | null | undefined): string | null => {
  if (!value) {
    return null;
  }

  const trimmed = value.trim();
  return trimmed.length > 0 ? trimmed : null;
};

const runGit = async (args: string[], cwd: string): Promise<GitCommandResult> => {
  const child = Bun.spawn(['git', ...args], {
    cwd,
    stdout: 'pipe',
    stderr: 'pipe',
    stdin: 'ignore',
  });

  const [stdout, stderr, exitCode] = await Promise.all([
    new Response(child.stdout).text(),
    new Response(child.stderr).text(),
    child.exited,
  ]);

  return {
    ok: exitCode === 0,
    stdout,
    stderr,
    exitCode,
  };
};

const ensureLocalGitVariantPath = async (id: string): Promise<string> => {
  const variant = await getVariantById(id, { poll: false });
  if (!isLocalLocator(variant.locator)) {
    throw new Error(
      `Variants // Branch // Variant locator must be local ${formatLogMetadata({
        id,
        locator: variant.locator,
      })}`,
    );
  }

  const path = locatorIdToHostPath(variant.locator);
  const existing = await stat(path).catch(() => null);
  if (!existing || !existing.isDirectory()) {
    throw new Error(
      `Variants // Branch // Variant path missing ${formatLogMetadata({
        id,
        path,
      })}`,
    );
  }

  const insideWorkTree = await runGit(['rev-parse', '--is-inside-work-tree'], path);
  if (!insideWorkTree.ok || !insideWorkTree.stdout.trim().includes('true')) {
    throw new Error(
      `Variants // Branch // Variant path is not a git worktree ${formatLogMetadata({
        id,
        path,
      })}`,
    );
  }

  return path;
};

const hasLocalBranch = async (path: string, branchName: string): Promise<boolean> => {
  const result = await runGit(['show-ref', '--verify', '--quiet', `refs/heads/${branchName}`], path);
  return result.ok;
};

const hasRemoteBranch = async (path: string, branchName: string): Promise<boolean> => {
  const remoteResult = await runGit(['remote', 'get-url', 'origin'], path);
  if (!remoteResult.ok) {
    return false;
  }

  const remoteBranchResult = await runGit(['ls-remote', '--heads', 'origin', branchName], path);
  if (!remoteBranchResult.ok) {
    throw new Error(
      `Variants // Branch // Git remote branch lookup failed ${formatLogMetadata({
        branchName,
        exitCode: remoteBranchResult.exitCode,
        path,
        stderr: trimToNull(remoteBranchResult.stderr),
      })}`,
    );
  }

  const exists = remoteBranchResult.stdout
    .split('\n')
    .map((line) => line.trim())
    .some((line) => line.endsWith(`/heads/${branchName}`));
  return exists;
};

export const checkoutVariantBranchById = async (
  id: string,
  input: CheckoutVariantBranchInput,
): Promise<Variant> => {
  const branchName = input.branchName.trim();
  if (!branchName) {
    throw new Error('Variants // Branch // Branch name is required');
  }

  const path = await ensureLocalGitVariantPath(id);
  const localExists = await hasLocalBranch(path, branchName);
  const remoteExists = await hasRemoteBranch(path, branchName);

  if (localExists) {
    const checkoutResult = await runGit(['checkout', branchName], path);
    if (!checkoutResult.ok) {
      throw new Error(
        `Variants // Branch // Git checkout failed ${formatLogMetadata({
          branchName,
          exitCode: checkoutResult.exitCode,
          path,
          stderr: trimToNull(checkoutResult.stderr),
        })}`,
      );
    }
  } else if (remoteExists) {
    const checkoutResult = await runGit(['checkout', '-b', branchName], path);
    if (!checkoutResult.ok) {
      throw new Error(
        `Variants // Branch // Git checkout create failed ${formatLogMetadata({
          branchName,
          exitCode: checkoutResult.exitCode,
          path,
          stderr: trimToNull(checkoutResult.stderr),
        })}`,
      );
    }
  } else {
    const checkoutResult = await runGit(['checkout', '-b', branchName], path);
    if (!checkoutResult.ok) {
      throw new Error(
        `Variants // Branch // Git branch create failed ${formatLogMetadata({
          branchName,
          exitCode: checkoutResult.exitCode,
          path,
          stderr: trimToNull(checkoutResult.stderr),
        })}`,
      );
    }
  }

  if (localExists || remoteExists) {
    const pullResult = await runGit(['pull', '--ff-only', 'origin', branchName], path);
    if (!pullResult.ok) {
      throw new Error(
        `Variants // Branch // Git pull failed ${formatLogMetadata({
          branchName,
          exitCode: pullResult.exitCode,
          path,
          stderr: trimToNull(pullResult.stderr),
        })}`,
      );
    }
  }

  Log.info(
    `Core // Variants Controller // Branch switched ${formatLogMetadata({
      branchName,
      id,
      localExists,
      remoteExists,
    })}`,
  );

  return syncVariantGitInfo(id);
};

const ensureGitCloneSafeToDelete = async (path: string): Promise<void> => {
  Log.debug(
    `Core // Variants Controller // Undo git checks started ${formatLogMetadata({ path })}`,
  );
  const insideWorkTree = await runGit(['rev-parse', '--is-inside-work-tree'], path);
  Log.debug(
    `Core // Variants Controller // Undo git check inside worktree ${formatLogMetadata({
      ok: insideWorkTree.ok,
      path,
      stdout: trimToNull(insideWorkTree.stdout),
    })}`,
  );
  if (!insideWorkTree.ok || !insideWorkTree.stdout.trim().includes('true')) {
    Log.info(
      `Core // Variants Controller // Undo git checks skipped, not a git worktree ${formatLogMetadata({
        path,
      })}`,
    );
    return;
  }

  const status = await runGit(['status', '--porcelain'], path);
  Log.debug(
    `Core // Variants Controller // Undo git status check ${formatLogMetadata({
      exitCode: status.exitCode,
      hasChanges: Boolean(trimToNull(status.stdout)),
      ok: status.ok,
      path,
    })}`,
  );
  if (!status.ok) {
    throw new Error(
      `Variants // Delete // Undo blocked: unable to read git status ${formatLogMetadata({
        exitCode: status.exitCode,
        path,
        stderr: trimToNull(status.stderr),
      })}`,
    );
  }

  if (trimToNull(status.stdout)) {
    throw new Error(
      `Variants // Delete // Undo blocked: working tree has changes ${formatLogMetadata({ path })}`,
    );
  }

  const branchResult = await runGit(['rev-parse', '--abbrev-ref', 'HEAD'], path);
  const branch = trimToNull(branchResult.stdout);
  Log.debug(
    `Core // Variants Controller // Undo git branch check ${formatLogMetadata({
      branch,
      exitCode: branchResult.exitCode,
      ok: branchResult.ok,
      path,
    })}`,
  );
  if (!branchResult.ok || !branch || branch === 'HEAD') {
    if (branch === 'HEAD') {
      const headResult = await runGit(['rev-parse', '--verify', 'HEAD'], path);
      if (!headResult.ok) {
        Log.warn(
          `Core // Variants Controller // Undo git checks downgraded for incomplete clone ${formatLogMetadata({
            path,
          })}`,
        );
        return;
      }
    }

    throw new Error(
      `Variants // Delete // Undo blocked: expected active branch ${formatLogMetadata({
        path,
      })}`,
    );
  }

  const fetchResult = await runGit(
    ['fetch', 'origin', `${branch}:refs/remotes/origin/${branch}`],
    path,
  );
  Log.debug(
    `Core // Variants Controller // Undo git fetch remote branch ${formatLogMetadata({
      branch,
      exitCode: fetchResult.exitCode,
      ok: fetchResult.ok,
      path,
    })}`,
  );
  if (!fetchResult.ok) {
    throw new Error(
      `Variants // Delete // Undo blocked: failed to fetch origin branch ${formatLogMetadata({
        branch,
        exitCode: fetchResult.exitCode,
        path,
        stderr: trimToNull(fetchResult.stderr),
      })}`,
    );
  }

  const pushedResult = await runGit(
    ['merge-base', '--is-ancestor', 'HEAD', `origin/${branch}`],
    path,
  );
  Log.debug(
    `Core // Variants Controller // Undo git pushed check ${formatLogMetadata({
      branch,
      exitCode: pushedResult.exitCode,
      ok: pushedResult.ok,
      path,
    })}`,
  );
  if (!pushedResult.ok) {
    throw new Error(
      `Variants // Delete // Undo blocked: branch head not pushed to origin ${formatLogMetadata({
        branch,
        path,
      })}`,
    );
  }
};

const undoVariantCloneIfRequested = async (variant: Variant, dry: boolean): Promise<void> => {
  Log.info(
    `Core // Variants Controller // Undo clone decision ${formatLogMetadata({
      dry,
      id: variant.id,
      locator: variant.locator,
    })}`,
  );
  if (dry) {
    Log.info(
      `Core // Variants Controller // Undo clone skipped by dry flag ${formatLogMetadata({
        id: variant.id,
      })}`,
    );
    return;
  }

  if (!isLocalLocator(variant.locator)) {
    throw new Error(
      `Variants // Delete // Undo blocked: variant locator must be local ${formatLogMetadata({
        id: variant.id,
        locator: variant.locator,
      })}`,
    );
  }

  const path = locatorIdToHostPath(variant.locator);
  Log.debug(
    `Core // Variants Controller // Undo clone resolved path ${formatLogMetadata({
      id: variant.id,
      path,
    })}`,
  );
  const existing = await stat(path).catch(() => null);
  if (!existing) {
    Log.warn(
      `Core // Variants Controller // Undo skipped, target path missing ${formatLogMetadata({
        id: variant.id,
        path,
      })}`,
    );
    return;
  }

  if (!existing.isDirectory()) {
    throw new Error(
      `Variants // Delete // Undo blocked: target path is not a directory ${formatLogMetadata({
        id: variant.id,
        path,
      })}`,
    );
  }

  await ensureGitCloneSafeToDelete(path);
  await rm(path, { recursive: true, force: false });

  Log.info(
    `Core // Variants Controller // Clone directory removed ${formatLogMetadata({
      id: variant.id,
      path,
    })}`,
  );
};
