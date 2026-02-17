import { cp, mkdir, stat } from 'node:fs/promises';
import { basename, join, resolve } from 'node:path';

import type { Product, Variant } from '../../../../generated/prisma/client';

import { getConfig } from '../../config';
import { hostAbsolutePathToLocatorId, locatorIdToHostPath, parseLocatorId } from '../../utils/locator';
import { buildRandomVariantId } from '../../utils/id';
import Log, { formatLogMetadata } from '../../utils/logging';
import { NotFoundError } from '../common/controller.errors';
import { getPrismaClient } from '../prisma/prisma.client';
import { syncVariantGitInfo } from '../variants/variants.controller';

type VariantCloneType = 'auto' | 'local.copy' | 'git.clone_branch';

export interface CloneVariantForProductInput {
  productId: string;
  name?: string;
  targetPath?: string;
  cloneType?: VariantCloneType;
  branchName?: string;
  sourceVariantId?: string;
}

export interface VariantCloneResult {
  variant: Variant;
  clone: {
    cloneType: Exclude<VariantCloneType, 'auto'>;
    sourceLocator: string;
    sourceLocatorKind: 'local' | 'git';
    targetPath: string;
    targetLocator: string;
    branchName: string | null;
    generatedTargetPath: boolean;
    generatedBranchName: boolean;
  };
}

interface ResolvedCloneTarget {
  targetPath: string;
  generatedTargetPath: boolean;
}

interface GitCommandResult {
  ok: boolean;
  stdout: string;
  stderr: string;
  exitCode: number;
}

const DEFAULT_VARIANT_NAME_PREFIX = 'clone';
const DEFAULT_CLONE_DIR_SUFFIX_WIDTH = 4;

const trimToNull = (value: string | null | undefined): string | null => {
  if (!value) {
    return null;
  }

  const trimmed = value.trim();
  return trimmed.length > 0 ? trimmed : null;
};

const toSlug = (value: string): string => {
  const normalized = value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');

  return normalized.length > 0 ? normalized : 'dark-factory';
};

const shortToken = (): string => {
  return buildRandomVariantId().replace(/^var_/, '').slice(0, 6).toLowerCase();
};

const pathExists = async (path: string): Promise<boolean> => {
  const info = await stat(path).catch(() => null);
  return Boolean(info);
};

const runGit = async (args: string[], cwd?: string): Promise<GitCommandResult> => {
  const child = Bun.spawn(['git', ...args], {
    ...(cwd ? { cwd } : {}),
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

const resolveLocalPathFromInput = (targetPath: string): string => {
  const parsed = parseLocatorId(targetPath);
  if (parsed.type === 'local') {
    return locatorIdToHostPath(parsed.locator);
  }

  if (targetPath.startsWith('/')) {
    return resolve(targetPath);
  }

  return resolve(process.cwd(), targetPath);
};

const resolveSourceVariantLocator = async (
  product: Product,
  sourceVariantId?: string,
): Promise<string> => {
  const prisma = getPrismaClient();

  if (sourceVariantId) {
    const sourceVariant = await prisma.variant.findUnique({ where: { id: sourceVariantId } });
    if (!sourceVariant || sourceVariant.productId !== product.id) {
      throw new NotFoundError(
        `Variants // Clone // Source variant not found ${formatLogMetadata({
          productId: product.id,
          sourceVariantId,
        })}`,
      );
    }
    return sourceVariant.locator;
  }

  const defaultVariant = await prisma.variant.findUnique({
    where: {
      productId_name: {
        productId: product.id,
        name: 'default',
      },
    },
  });

  if (defaultVariant) {
    return defaultVariant.locator;
  }

  return product.locator;
};

const resolveCloneType = (
  locator: string,
  cloneType: VariantCloneType,
): Exclude<VariantCloneType, 'auto'> => {
  if (cloneType !== 'auto') {
    return cloneType;
  }

  const parsed = parseLocatorId(locator);
  if (parsed.type === 'git') {
    return 'git.clone_branch';
  }

  if (parsed.type === 'local') {
    return 'local.copy';
  }

  throw new Error(
    `Variants // Clone // Unsupported product locator ${formatLogMetadata({ locator })}`,
  );
};

const resolveWorkspaceRootPath = (product: Product): string | null => {
  const workspaceLocator = trimToNull(product.workspaceLocator);
  if (workspaceLocator) {
    const parsed = parseLocatorId(workspaceLocator);
    if (parsed.type !== 'local') {
      throw new Error(
        `Variants // Clone // Product workspace locator must be local ${formatLogMetadata({
          productId: product.id,
          workspaceLocator,
        })}`,
      );
    }

    return locatorIdToHostPath(parsed.locator);
  }

  const fallback = trimToNull(getConfig().variants.defaultWorkspaceLocator);
  if (!fallback) {
    return null;
  }

  const parsedFallback = parseLocatorId(fallback);
  if (parsedFallback.type !== 'local') {
    throw new Error(
      `Variants // Clone // Config default workspace locator must be local ${formatLogMetadata({
        value: fallback,
      })}`,
    );
  }

  return locatorIdToHostPath(parsedFallback.locator);
};

const deriveDirectoryPrefix = (product: Product): string => {
  const preferred =
    trimToNull(product.displayName) ??
    trimToNull(basename(product.locator.split('://').slice(-1)[0] ?? '')) ??
    'dark-factory';

  return toSlug(preferred);
};

const createAutoTargetPath = async (workspaceRootPath: string, product: Product): Promise<string> => {
  const prefix = deriveDirectoryPrefix(product);

  for (let index = 0; index < 10_000; index += 1) {
    const suffix = String(index).padStart(DEFAULT_CLONE_DIR_SUFFIX_WIDTH, '0');
    const candidate = join(workspaceRootPath, `${prefix}_${suffix}`);
    if (!(await pathExists(candidate))) {
      return candidate;
    }
  }

  return join(workspaceRootPath, `${prefix}_${shortToken()}`);
};

const resolveCloneTarget = async (
  product: Product,
  targetPath?: string,
): Promise<ResolvedCloneTarget> => {
  if (targetPath && targetPath.trim().length > 0) {
    const resolved = resolveLocalPathFromInput(targetPath);
    await mkdir(resolve(resolved, '..'), { recursive: true });

    if (await pathExists(resolved)) {
      throw new Error(
        `Variants // Clone // Target path already exists ${formatLogMetadata({ targetPath: resolved })}`,
      );
    }

    return {
      targetPath: resolved,
      generatedTargetPath: false,
    };
  }

  const workspaceRootPath = resolveWorkspaceRootPath(product);
  if (!workspaceRootPath) {
    throw new Error(
      `Variants // Clone // Workspace unresolved ${formatLogMetadata({ productId: product.id })}`,
    );
  }

  await mkdir(workspaceRootPath, { recursive: true });
  const generatedPath = await createAutoTargetPath(workspaceRootPath, product);

  return {
    targetPath: generatedPath,
    generatedTargetPath: true,
  };
};

const buildVariantName = async (productId: string, requestedName: string | undefined): Promise<string> => {
  const prisma = getPrismaClient();
  const normalizedRequested = trimToNull(requestedName);
  if (normalizedRequested) {
    return normalizedRequested;
  }

  for (let index = 0; index < 10_000; index += 1) {
    const candidate = `${DEFAULT_VARIANT_NAME_PREFIX}_${String(index).padStart(4, '0')}`;
    const exists = await prisma.variant.findUnique({
      where: {
        productId_name: {
          productId,
          name: candidate,
        },
      },
    });

    if (!exists) {
      return candidate;
    }
  }

  return `${DEFAULT_VARIANT_NAME_PREFIX}_${shortToken()}`;
};

const buildBranchName = (product: Product, requestedBranchName?: string): { value: string; generated: boolean } => {
  const normalized = trimToNull(requestedBranchName);
  if (normalized) {
    return {
      value: normalized,
      generated: false,
    };
  }

  const slug = deriveDirectoryPrefix(product);
  return {
    value: `df/${slug}-${shortToken()}`,
    generated: true,
  };
};

const cloneFromLocalSource = async (sourceLocator: string, targetPath: string): Promise<void> => {
  const parsed = parseLocatorId(sourceLocator);
  if (parsed.type !== 'local') {
    throw new Error(
      `Variants // Clone // Local clone requires local source ${formatLogMetadata({ sourceLocator })}`,
    );
  }

  const sourcePath = locatorIdToHostPath(parsed.locator);
  await cp(sourcePath, targetPath, {
    recursive: true,
    errorOnExist: true,
    force: false,
  });
};

const cloneFromGitRemote = async (
  productLocator: string,
  targetPath: string,
  branchName: string,
): Promise<void> => {
  const parsed = parseLocatorId(productLocator);
  if (parsed.type !== 'git') {
    throw new Error(
      `Variants // Clone // Git clone requires git product locator ${formatLogMetadata({
        productLocator,
      })}`,
    );
  }

  const cloneResult = await runGit(
    ['clone', '--branch', parsed.ref, '--single-branch', parsed.remote, targetPath],
    process.cwd(),
  );
  if (!cloneResult.ok) {
    throw new Error(
      `Variants // Clone // Git clone failed ${formatLogMetadata({
        exitCode: cloneResult.exitCode,
        stderr: trimToNull(cloneResult.stderr),
      })}`,
    );
  }

  const checkoutResult = await runGit(['checkout', '-b', branchName], targetPath);
  if (!checkoutResult.ok) {
    throw new Error(
      `Variants // Clone // Git branch create failed ${formatLogMetadata({
        branchName,
        exitCode: checkoutResult.exitCode,
        stderr: trimToNull(checkoutResult.stderr),
      })}`,
    );
  }
};

export const cloneVariantForProduct = async (
  input: CloneVariantForProductInput,
): Promise<VariantCloneResult> => {
  const prisma = getPrismaClient();

  Log.info(
    `Core // Variant Clones Controller // Clone requested ${formatLogMetadata({
      branchName: input.branchName ?? null,
      cloneType: input.cloneType ?? 'auto',
      name: input.name ?? null,
      productId: input.productId,
      sourceVariantId: input.sourceVariantId ?? null,
      targetPath: input.targetPath ?? null,
    })}`,
  );

  const product = await prisma.product.findUnique({ where: { id: input.productId } });
  if (!product) {
    throw new NotFoundError(`Product ${input.productId} was not found`);
  }

  const sourceLocator = await resolveSourceVariantLocator(product, input.sourceVariantId);
  const resolvedCloneType = resolveCloneType(product.locator, input.cloneType ?? 'auto');
  const { targetPath, generatedTargetPath } = await resolveCloneTarget(product, input.targetPath);
  const branch = buildBranchName(product, input.branchName);

  await mkdir(resolve(targetPath, '..'), { recursive: true });

  if (resolvedCloneType === 'local.copy') {
    await cloneFromLocalSource(sourceLocator, targetPath);
  } else {
    await cloneFromGitRemote(product.locator, targetPath, branch.value);
  }

  const targetLocator = hostAbsolutePathToLocatorId(targetPath);
  const variantName = await buildVariantName(product.id, input.name);

  const createdVariant = await prisma.variant.create({
    data: {
      id: buildRandomVariantId(),
      productId: product.id,
      name: variantName,
      locator: targetLocator,
    },
  });
  const variant = await syncVariantGitInfo(createdVariant.id);

  Log.info(
    `Core // Variant Clones Controller // Clone created ${formatLogMetadata({
      branchName: resolvedCloneType === 'git.clone_branch' ? branch.value : null,
      cloneType: resolvedCloneType,
      generatedBranchName: resolvedCloneType === 'git.clone_branch' ? branch.generated : false,
      generatedTargetPath,
      productId: product.id,
      sourceLocator,
      targetLocator,
      targetPath,
      variantId: variant.id,
      variantName: variant.name,
    })}`,
  );

  return {
    variant,
    clone: {
      cloneType: resolvedCloneType,
      sourceLocator,
      sourceLocatorKind: parseLocatorId(sourceLocator).type === 'git' ? 'git' : 'local',
      targetPath,
      targetLocator,
      branchName: resolvedCloneType === 'git.clone_branch' ? branch.value : null,
      generatedTargetPath,
      generatedBranchName: resolvedCloneType === 'git.clone_branch' ? branch.generated : false,
    },
  };
};
