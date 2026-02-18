import { cp, mkdir, rm, stat } from 'node:fs/promises';
import { basename, join, resolve } from 'node:path';

import type { Product, Variant } from '../../../../generated/prisma/client';

import { getConfig } from '../../config';
import {
  hostAbsolutePathToLocatorId,
  locatorIdToHostPath,
  normalizeLocator,
  parseLocatorId,
} from '../../utils/locator';
import { buildRandomVariantId } from '../../utils/id';
import Log, { formatLogMetadata } from '../../utils/logging';
import { NotFoundError } from '../common/controller.errors';
import { getPrismaClient } from '../prisma/prisma.client';
import { buildSshInvocation } from '../ssh/ssh.controller';
import { syncVariantGitInfo } from '../variants/variants.controller';

type VariantCloneType = 'auto' | 'local.copy' | 'git.clone_branch';

export interface CloneVariantForProductInput {
  productId: string;
  name?: string;
  targetPath?: string;
  cloneType?: VariantCloneType;
  branchName?: string;
  sourceVariantId?: string;
  runAsync?: boolean;
}

export interface VariantCloneResult {
  variant: Variant;
  clone: {
    cloneType: Exclude<VariantCloneType, 'auto'>;
    sourceLocator: string;
    sourceLocatorKind: 'local' | 'git' | 'ssh';
    targetPath: string;
    targetLocator: string;
    branchName: string | null;
    generatedTargetPath: boolean;
    generatedBranchName: boolean;
    attemptedCommand: string | null;
    usedNoLocalRetry: boolean;
    isAsync: boolean;
  };
}

interface ResolvedCloneTarget {
  targetPath: string;
  targetLocator: string;
  targetKind: 'local' | 'ssh';
  generatedTargetPath: boolean;
}

interface GitCommandResult {
  ok: boolean;
  stdout: string;
  stderr: string;
  exitCode: number;
  command: string;
}

interface RunGitOptions {
  timeoutMs?: number;
  nonInteractive?: boolean;
  logOutput?: boolean;
  onOutputLine?: (line: string, level: 'info' | 'warn') => void;
}

interface CloneExecutionMetadata {
  attemptedCommand: string;
  usedNoLocalRetry: boolean;
}

const DEFAULT_VARIANT_NAME_PREFIX = 'clone';
const DEFAULT_CLONE_DIR_SUFFIX_WIDTH = 2;
const GIT_SHALLOW_CLONE_DEPTH = 32;
const GIT_CLONE_TIMEOUT_MS = 15 * 60_000;
const GIT_CLONE_TRANSIENT_RETRY_MAX = 2;
const GIT_OUTPUT_LOG_RATE_MS = 1_000;
const GIT_OUTPUT_METADATA_RATE_MS = 1_000;

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

const shellQuote = (value: string): string => {
  if (value.length === 0) {
    return "''";
  }

  if (/^[A-Za-z0-9_./:@%+,-]+$/.test(value)) {
    return value;
  }

  return `'${value.replace(/'/g, `'\\''`)}'`;
};

const formatGitCommand = (args: string[], nonInteractive: boolean): string => {
  const envPrefix = nonInteractive
    ? "GIT_TERMINAL_PROMPT=0 GIT_ASKPASS=echo GIT_SSH_COMMAND='ssh -oBatchMode=yes' "
    : '';

  return `${envPrefix}git ${args.map(shellQuote).join(' ')}`;
};

const runGit = async (
  args: string[],
  cwd?: string,
  options: RunGitOptions = {},
): Promise<GitCommandResult> => {
  const nonInteractive = options.nonInteractive ?? false;
  const command = formatGitCommand(args, nonInteractive);
  const commandCwd = cwd ?? process.cwd();
  const timeoutMs = options.timeoutMs ?? 0;

  Log.info(
    `Core // Variant Clones Controller // Git command executing ${formatLogMetadata({
      command,
      cwd: commandCwd,
      timeoutMs: timeoutMs > 0 ? timeoutMs : null,
    })}`,
  );

  const env = {
    ...process.env,
    ...(nonInteractive
      ? {
          GIT_TERMINAL_PROMPT: '0',
          GIT_ASKPASS: 'echo',
          // Fail fast for SSH remotes when auth is unavailable.
          GIT_SSH_COMMAND: 'ssh -oBatchMode=yes',
        }
      : {}),
  };

  const child = Bun.spawn(['git', ...args], {
    cwd: commandCwd,
    env,
    stdout: 'pipe',
    stderr: 'pipe',
    stdin: 'ignore',
  });

  let timedOut = false;
  let timeoutHandle: ReturnType<typeof setTimeout> | null = null;
  if (timeoutMs > 0) {
    timeoutHandle = setTimeout(() => {
      timedOut = true;
      child.kill();
    }, timeoutMs);
  }

  const readStreamWithOptionalLiveLogs = async (
    stream: ReadableStream<Uint8Array> | null,
    level: 'info' | 'warn',
  ): Promise<string> => {
    if (!stream) {
      return '';
    }

    if (!options.logOutput) {
      return new Response(stream).text();
    }

    const reader = stream.getReader();
    const decoder = new TextDecoder();
    let captured = '';
    let buffered = '';
    let lastLoggedAtMs = 0;
    let lastMetadataAtMs = 0;
    let hasLoggedAnyLine = false;
    let lastLoggedLine: string | null = null;

    const processLine = (line: string): void => {
      const trimmed = line.trimEnd();
      if (trimmed.length === 0) {
        return;
      }

      const nowMs = Date.now();
      const highPriority = isGitStartLine(trimmed) || isGitCompletionLine(trimmed) || isFatalGitLine(trimmed);

      const canLog =
        highPriority ||
        !hasLoggedAnyLine ||
        (nowMs - lastLoggedAtMs >= GIT_OUTPUT_LOG_RATE_MS && lastLoggedLine !== trimmed);
      if (canLog) {
        const message = `Core // Variant Clones Controller // Git ${level} ${formatLogMetadata({ command, line: trimmed })}`;
        if (level === 'warn') {
          Log.warn(message);
        } else {
          Log.info(message);
        }

        hasLoggedAnyLine = true;
        lastLoggedAtMs = nowMs;
        lastLoggedLine = trimmed;
      }

      const canUpdateMetadata =
        highPriority ||
        !hasLoggedAnyLine ||
        nowMs - lastMetadataAtMs >= GIT_OUTPUT_METADATA_RATE_MS;
      if (canUpdateMetadata) {
        options.onOutputLine?.(trimmed, level);
        lastMetadataAtMs = nowMs;
      }
    };

    while (true) {
      const { done, value } = await reader.read();
      if (done) {
        break;
      }

      const chunk = decoder.decode(value, { stream: true });
      captured += chunk;
      buffered += chunk;

      let splitIndex = buffered.search(/[\r\n]/);
      while (splitIndex >= 0) {
        const line = buffered.slice(0, splitIndex);
        buffered = buffered.slice(splitIndex + 1);
        processLine(line);
        splitIndex = buffered.search(/[\r\n]/);
      }
    }

    const tail = decoder.decode();
    if (tail.length > 0) {
      captured += tail;
      buffered += tail;
    }

    processLine(buffered);

    return captured;
  };

  const [stdout, stderr, exitCode] = await Promise.all([
    readStreamWithOptionalLiveLogs(child.stdout, 'info'),
    readStreamWithOptionalLiveLogs(child.stderr, 'warn'),
    child.exited,
  ]);

  if (timeoutHandle) {
    clearTimeout(timeoutHandle);
  }

  if (timedOut) {
    return {
      ok: false,
      stdout,
      stderr: `${stderr}\nGit command timed out after ${timeoutMs}ms: ${command}`,
      exitCode: -1,
      command,
    };
  }

  return {
    ok: exitCode === 0,
    stdout,
    stderr,
    exitCode,
    command,
  };
};

const shouldRetryGitCloneAsNoLocal = (stderr: string): boolean => {
  const normalized = stderr.toLowerCase();
  return (
    normalized.includes('invalid index-pack output') ||
    normalized.includes('tmp_pack') ||
    normalized.includes('could not open')
  );
};

const shouldRetryGitCloneTransient = (stderr: string): boolean => {
  const normalized = stderr.toLowerCase();
  return (
    normalized.includes('early eof') ||
    normalized.includes('fetch-pack') ||
    normalized.includes('index-pack failed') ||
    normalized.includes('connection reset') ||
    normalized.includes('remote end hung up unexpectedly') ||
    normalized.includes('unexpected disconnect')
  );
};

const shouldRetryGitCloneWithoutFilter = (stderr: string): boolean => {
  const normalized = stderr.toLowerCase();
  return (
    normalized.includes('filtering not recognized by server') ||
    normalized.includes('does not support filter') ||
    normalized.includes('server does not support')
  );
};

const isFatalGitLine = (line: string): boolean => {
  const normalized = line.toLowerCase();
  return normalized.includes('fatal:') || normalized.includes('error:');
};

const isGitCompletionLine = (line: string): boolean => {
  const normalized = line.toLowerCase();
  return (
    normalized.includes(' 100% ') ||
    normalized.includes('100% (') ||
    normalized.endsWith(', done.') ||
    normalized.includes('updating files: 100%')
  );
};

const isGitStartLine = (line: string): boolean => {
  return line.startsWith('Cloning into ');
};

const isJsonRecord = (value: unknown): value is Record<string, unknown> => {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
};

const withCloneMetadata = (
  existingGitInfo: Variant['gitInfo'],
  metadata: Record<string, unknown>,
): Variant['gitInfo'] => {
  const base = isJsonRecord(existingGitInfo) ? { ...existingGitInfo } : {};
  return {
    ...base,
    _clone: metadata,
  };
};

const withMergedCloneMetadata = (
  existingGitInfo: Variant['gitInfo'],
  patch: Record<string, unknown>,
): Variant['gitInfo'] => {
  const base = isJsonRecord(existingGitInfo) ? { ...existingGitInfo } : {};
  const existingClone = isJsonRecord(base._clone) ? base._clone : {};
  return {
    ...base,
    _clone: {
      ...existingClone,
      ...patch,
    },
  };
};

const resolveTargetFromInput = (targetPath: string): Omit<ResolvedCloneTarget, 'generatedTargetPath'> => {
  const normalized = normalizeLocator(targetPath);
  const parsed = parseLocatorId(normalized);
  if (parsed.type === 'local') {
    const hostPath = locatorIdToHostPath(parsed.locator);
    return {
      targetPath: hostPath,
      targetLocator: parsed.locator,
      targetKind: 'local',
    };
  }

  if (parsed.type === 'ssh') {
    return {
      targetPath: parsed.path,
      targetLocator: parsed.locator,
      targetKind: 'ssh',
    };
  }

  if (targetPath.startsWith('/')) {
    const resolved = resolve(targetPath);
    return {
      targetPath: resolved,
      targetLocator: hostAbsolutePathToLocatorId(resolved),
      targetKind: 'local',
    };
  }

  const resolved = resolve(process.cwd(), targetPath);
  return {
    targetPath: resolved,
    targetLocator: hostAbsolutePathToLocatorId(resolved),
    targetKind: 'local',
  };
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

const createAutoTargetPath = async (
  workspaceRootPath: string,
  product: Product,
  requestedName?: string,
): Promise<string> => {
  const productPrefix = deriveDirectoryPrefix(product);
  const requestedNamePrefix = trimToNull(requestedName);
  const prefix = requestedNamePrefix
    ? `${productPrefix}_${toSlug(requestedNamePrefix)}`
    : productPrefix;

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
  requestedName: string | undefined,
  targetPath?: string,
): Promise<ResolvedCloneTarget> => {
  if (targetPath && targetPath.trim().length > 0) {
    const resolved = resolveTargetFromInput(targetPath);
    if (resolved.targetKind === 'local') {
      await mkdir(resolve(resolved.targetPath, '..'), { recursive: true });
    }

    if (resolved.targetKind === 'local' && (await pathExists(resolved.targetPath))) {
      throw new Error(
        `Variants // Clone // Target path already exists ${formatLogMetadata({ targetPath: resolved.targetPath })}`,
      );
    }

    return {
      targetPath: resolved.targetPath,
      targetLocator: resolved.targetLocator,
      targetKind: resolved.targetKind,
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
  const generatedPath = await createAutoTargetPath(workspaceRootPath, product, requestedName);

  return {
    targetPath: generatedPath,
    targetLocator: hostAbsolutePathToLocatorId(generatedPath),
    targetKind: 'local',
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

const doesRemoteBranchExist = async (remote: string, branchName: string): Promise<boolean> => {
  const branchRef = trimToNull(branchName);
  if (!branchRef) {
    return false;
  }

  const result = await runGit(['ls-remote', '--heads', remote, branchRef], process.cwd(), {
    nonInteractive: true,
    timeoutMs: 30_000,
  });
  if (!result.ok) {
    throw new Error(
      `Variants // Clone // Git branch discovery failed ${formatLogMetadata({
        branchName: branchRef,
        command: result.command,
        exitCode: result.exitCode,
        remote,
        stderr: trimToNull(result.stderr),
      })}`,
    );
  }

  return result.stdout
    .split('\n')
    .map((line) => line.trim())
    .some((line) => line.endsWith(`/heads/${branchRef}`));
};

const checkoutOrCreateBranch = async (
  targetPath: string,
  branchName: string,
  branchExistsOnRemote: boolean,
  onOutputLine?: (line: string, level: 'info' | 'warn') => void,
): Promise<void> => {
  if (branchExistsOnRemote) {
    const checkoutResult = await runGit(['checkout', branchName], targetPath, {
      logOutput: true,
      onOutputLine,
    });
    if (!checkoutResult.ok) {
      throw new Error(
        `Variants // Clone // Git checkout existing branch failed ${formatLogMetadata({
          branchName,
          command: checkoutResult.command,
          exitCode: checkoutResult.exitCode,
          stderr: trimToNull(checkoutResult.stderr),
        })}`,
      );
    }

    const pullResult = await runGit(['pull', '--ff-only', 'origin', branchName], targetPath, {
      logOutput: true,
      onOutputLine,
    });
    if (!pullResult.ok) {
      throw new Error(
        `Variants // Clone // Git pull existing branch failed ${formatLogMetadata({
          branchName,
          command: pullResult.command,
          exitCode: pullResult.exitCode,
          stderr: trimToNull(pullResult.stderr),
        })}`,
      );
    }

    return;
  }

  const checkoutResult = await runGit(['checkout', '-b', branchName], targetPath, {
    logOutput: true,
    onOutputLine,
  });
  if (!checkoutResult.ok) {
    throw new Error(
      `Variants // Clone // Git branch create failed ${formatLogMetadata({
        branchName,
        command: checkoutResult.command,
        exitCode: checkoutResult.exitCode,
        stderr: trimToNull(checkoutResult.stderr),
      })}`,
    );
  }
};

const cloneFromGitRemoteOverSsh = async (
  productLocator: string,
  targetLocator: string,
  branchName: string,
  branchProvided: boolean,
  _onOutputLine?: (line: string, level: 'info' | 'warn') => void,
): Promise<CloneExecutionMetadata> => {
  const productParsed = parseLocatorId(productLocator);
  if (productParsed.type !== 'git') {
    throw new Error(
      `Variants // Clone // Git clone requires git product locator ${formatLogMetadata({
        productLocator,
      })}`,
    );
  }

  const targetParsed = parseLocatorId(targetLocator);
  if (targetParsed.type !== 'ssh') {
    throw new Error(
      `Variants // Clone // Remote clone requires @ssh locator ${formatLogMetadata({
        targetLocator,
      })}`,
    );
  }

  const branchExistsOnRemote = branchProvided
    ? await doesRemoteBranchExist(productParsed.remote, branchName)
    : false;
  const cloneSourceBranch = branchExistsOnRemote ? branchName : productParsed.ref;

  const cloneArgs = [
    'clone',
    '--progress',
    '--filter=blob:none',
    '--branch',
    cloneSourceBranch,
    '--single-branch',
    productParsed.remote,
    targetParsed.path,
  ];
  if (!branchExistsOnRemote) {
    cloneArgs.splice(2, 0, '--depth', String(GIT_SHALLOW_CLONE_DEPTH));
  }

  const parentPath = resolve(targetParsed.path, '..');
  const checkoutCommand = branchExistsOnRemote
    ? `git checkout ${shellQuote(branchName)} && git pull --ff-only origin ${shellQuote(branchName)}`
    : `git checkout -b ${shellQuote(branchName)}`;
  const script = [
    `mkdir -p ${shellQuote(parentPath)}`,
    `if [ -e ${shellQuote(targetParsed.path)} ]; then echo ${shellQuote(`Target path already exists: ${targetParsed.path}`)} >&2; exit 17; fi`,
    `git ${cloneArgs.map(shellQuote).join(' ')}`,
    `cd ${shellQuote(targetParsed.path)}`,
    checkoutCommand,
  ].join(' && ');

  const invocation = buildSshInvocation(targetParsed.host, ['-o', 'BatchMode=yes']);
  const sshCommandParts = ['ssh', ...invocation.args, invocation.destination, script];
  const attemptedCommand = sshCommandParts.map(shellQuote).join(' ');

  const child = Bun.spawn(sshCommandParts, {
    cwd: process.cwd(),
    stdout: 'pipe',
    stderr: 'pipe',
    stdin: 'ignore',
  });
  const [stdout, stderr, exitCode] = await Promise.all([
    new Response(child.stdout).text(),
    new Response(child.stderr).text(),
    child.exited,
  ]);

  if (exitCode !== 0) {
    const errorText = trimToNull(stderr) ?? trimToNull(stdout);
    if (errorText?.includes('Target path already exists:')) {
      throw new Error(
        `Variants // Clone // Target path already exists ${formatLogMetadata({
          targetPath: targetParsed.path,
        })}`,
      );
    }

    throw new Error(
      `Variants // Clone // Remote git clone failed ${formatLogMetadata({
        branchName: cloneSourceBranch,
        command: attemptedCommand,
        exitCode,
        stderr: trimToNull(stderr),
      })}`,
    );
  }

  return {
    attemptedCommand,
    usedNoLocalRetry: false,
  };
};

const cloneFromGitRemote = async (
  productLocator: string,
  targetPath: string,
  branchName: string,
  branchProvided: boolean,
  onOutputLine?: (line: string, level: 'info' | 'warn') => void,
): Promise<CloneExecutionMetadata> => {
  const parsed = parseLocatorId(productLocator);
  if (parsed.type !== 'git') {
    throw new Error(
      `Variants // Clone // Git clone requires git product locator ${formatLogMetadata({
        productLocator,
      })}`,
    );
  }

  const branchExistsOnRemote = branchProvided
    ? await doesRemoteBranchExist(parsed.remote, branchName)
    : false;
  const cloneSourceBranch = branchExistsOnRemote ? branchName : parsed.ref;

  const cloneArgs = [
    'clone',
    '--progress',
    '--filter=blob:none',
    '--branch',
    cloneSourceBranch,
    '--single-branch',
    parsed.remote,
    targetPath,
  ];
  if (!branchExistsOnRemote) {
    cloneArgs.splice(2, 0, '--depth', String(GIT_SHALLOW_CLONE_DEPTH));
  }
  let cloneCommandArgs = cloneArgs;
  let cloneResult = await runGit(cloneCommandArgs, process.cwd(), {
    nonInteractive: true,
    timeoutMs: GIT_CLONE_TIMEOUT_MS,
    logOutput: true,
    onOutputLine,
  });
  let usedNoLocalRetry = false;
  let usedFilterFallback = false;
  let transientRetries = 0;
  while (!cloneResult.ok) {
    if (!usedFilterFallback && shouldRetryGitCloneWithoutFilter(cloneResult.stderr)) {
      usedFilterFallback = true;
      cloneCommandArgs = cloneCommandArgs.filter((part) => part !== '--filter=blob:none');
      Log.warn(
        `Core // Variant Clones Controller // Clone retry without --filter ${formatLogMetadata({
          branchName: cloneSourceBranch,
          stderr: trimToNull(cloneResult.stderr),
          targetPath,
        })}`,
      );
      await rm(targetPath, { recursive: true, force: true });
      cloneResult = await runGit(cloneCommandArgs, process.cwd(), {
        nonInteractive: true,
        timeoutMs: GIT_CLONE_TIMEOUT_MS,
        logOutput: true,
        onOutputLine,
      });
      continue;
    }

    if (!usedNoLocalRetry && shouldRetryGitCloneAsNoLocal(cloneResult.stderr)) {
      Log.warn(
        `Core // Variant Clones Controller // Clone retry with --no-local ${formatLogMetadata({
          branchName: cloneSourceBranch,
          stderr: trimToNull(cloneResult.stderr),
          targetPath,
        })}`,
      );

      await rm(targetPath, { recursive: true, force: true });
      cloneCommandArgs = ['clone', '--no-local', ...cloneArgs.slice(1)];
      cloneResult = await runGit(cloneCommandArgs, process.cwd(), {
        nonInteractive: true,
        timeoutMs: GIT_CLONE_TIMEOUT_MS,
        logOutput: true,
        onOutputLine,
      });
      usedNoLocalRetry = true;
      continue;
    }

    if (
      shouldRetryGitCloneTransient(cloneResult.stderr) &&
      transientRetries < GIT_CLONE_TRANSIENT_RETRY_MAX
    ) {
      transientRetries += 1;
      Log.warn(
        `Core // Variant Clones Controller // Clone transient retry ${formatLogMetadata({
          attempt: transientRetries,
          branchName: cloneSourceBranch,
          maxRetries: GIT_CLONE_TRANSIENT_RETRY_MAX,
          stderr: trimToNull(cloneResult.stderr),
          targetPath,
        })}`,
      );

      await rm(targetPath, { recursive: true, force: true });
      cloneResult = await runGit(cloneCommandArgs, process.cwd(), {
        nonInteractive: true,
        timeoutMs: GIT_CLONE_TIMEOUT_MS,
        logOutput: true,
        onOutputLine,
      });
      continue;
    }

    break;
  }

  if (!cloneResult.ok) {
    throw new Error(
      `Variants // Clone // Git clone failed ${formatLogMetadata({
        branchName: cloneSourceBranch,
        command: cloneResult.command,
        exitCode: cloneResult.exitCode,
        transientRetries,
        stderr: trimToNull(cloneResult.stderr),
      })}`,
    );
  }

  await checkoutOrCreateBranch(targetPath, branchName, branchExistsOnRemote, onOutputLine);

  return {
    attemptedCommand: cloneResult.command,
    usedNoLocalRetry: usedNoLocalRetry || usedFilterFallback,
  };
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
  const { targetPath, targetLocator, targetKind, generatedTargetPath } = await resolveCloneTarget(
    product,
    input.name,
    input.targetPath,
  );
  const branch = buildBranchName(product, input.branchName);
  const branchProvided = Boolean(trimToNull(input.branchName));
  const variantName = await buildVariantName(product.id, input.name);
  const variantId = buildRandomVariantId();
  const startedAt = new Date();

  Log.info(
    `Core // Variant Clones Controller // Clone plan resolved ${formatLogMetadata({
      cloneType: resolvedCloneType,
      generatedBranchName: branch.generated,
      generatedTargetPath,
      productId: product.id,
      sourceLocator,
      targetKind,
      targetLocator,
      targetPath,
      workspaceLocator: product.workspaceLocator,
    })}`,
  );

  if (targetKind === 'local') {
    await mkdir(resolve(targetPath, '..'), { recursive: true });
    Log.debug(
      `Core // Variant Clones Controller // Clone target parent ensured ${formatLogMetadata({
        productId: product.id,
        targetPath,
      })}`,
    );
  }

  const seededVariant = await prisma.variant.create({
    data: {
      id: variantId,
      productId: product.id,
      name: variantName,
      locator: targetLocator,
      gitInfo: withCloneMetadata(null, {
        status: 'cloning',
        phase: 'clone.pending',
        startedAt: startedAt.toISOString(),
        sourceLocator,
        targetPath,
        branchName: resolvedCloneType === 'git.clone_branch' ? branch.value : null,
      }),
      gitInfoUpdatedAt: startedAt,
    },
  });

  let attemptedCommand: string | null = null;
  let usedNoLocalRetry = false;
  let lastCloneLine: string | null = null;
  let lastProgressUpdateMs = 0;

  const updateCloneProgress = async (line: string): Promise<void> => {
    const trimmed = line.trim();
    if (!trimmed) {
      return;
    }

    const nowMs = Date.now();
    if (trimmed === lastCloneLine && nowMs - lastProgressUpdateMs < 1_000) {
      return;
    }

    if (nowMs - lastProgressUpdateMs < 1_000) {
      return;
    }

    lastCloneLine = trimmed;
    lastProgressUpdateMs = nowMs;

    const current = await prisma.variant.findUnique({ where: { id: seededVariant.id } });
    if (!current) {
      return;
    }

    await prisma.variant
      .update({
        where: { id: seededVariant.id },
        data: {
          gitInfo: withMergedCloneMetadata(current.gitInfo, {
            status: 'cloning',
            phase: 'clone.pending',
            lastLine: trimmed,
            startedAt: startedAt.toISOString(),
            sourceLocator,
            targetPath,
            branchName: resolvedCloneType === 'git.clone_branch' ? branch.value : null,
          }),
          gitInfoUpdatedAt: new Date(),
        },
      })
      .catch(() => null);
  };

  const executeCloneLifecycle = async (): Promise<Variant> => {
    try {
      if (resolvedCloneType === 'local.copy') {
        if (targetKind !== 'local') {
          throw new Error(
            `Variants // Clone // local.copy requires local target path ${formatLogMetadata({
              targetLocator,
            })}`,
          );
        }

        Log.info(
          `Core // Variant Clones Controller // Clone execution local copy ${formatLogMetadata({
            productId: product.id,
            sourceLocator,
            targetPath,
            variantId,
          })}`,
        );
        await cloneFromLocalSource(sourceLocator, targetPath);
        attemptedCommand = `cp -R ${sourceLocator} ${targetPath}`;
      } else {
        Log.info(
          `Core // Variant Clones Controller // Clone execution git clone+branch ${formatLogMetadata({
            branchName: branch.value,
            productId: product.id,
            productLocator: product.locator,
            targetPath,
            variantId,
          })}`,
        );
        const execution =
          targetKind === 'local'
            ? await cloneFromGitRemote(
                product.locator,
                targetPath,
                branch.value,
                branchProvided,
                (line) => {
                  void updateCloneProgress(line);
                },
              )
            : await cloneFromGitRemoteOverSsh(
                product.locator,
                targetLocator,
                branch.value,
                branchProvided,
                (line) => {
                  void updateCloneProgress(line);
                },
              );
        attemptedCommand = execution.attemptedCommand;
        usedNoLocalRetry = execution.usedNoLocalRetry;
        Log.info(
          `Core // Variant Clones Controller // Clone command attempted ${formatLogMetadata({
            command: attemptedCommand,
            usedNoLocalRetry,
            variantId,
          })}`,
        );
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      const failedAt = new Date();
      const current = await prisma.variant.findUnique({ where: { id: seededVariant.id } });
      await prisma.variant
        .update({
          where: { id: seededVariant.id },
          data: {
            gitInfo: withMergedCloneMetadata(current?.gitInfo ?? seededVariant.gitInfo, {
              status: 'failed',
              phase: 'clone.failed',
              startedAt: startedAt.toISOString(),
              failedAt: failedAt.toISOString(),
              sourceLocator,
              targetPath,
              branchName: resolvedCloneType === 'git.clone_branch' ? branch.value : null,
              attemptedCommand,
              usedNoLocalRetry,
              lastLine: lastCloneLine,
              error: message,
            }),
            gitInfoUpdatedAt: failedAt,
          },
        })
        .catch(() => null);

      throw error;
    }

    const syncedVariant = await syncVariantGitInfo(seededVariant.id);
    const finishedAt = new Date();
    const variant = await prisma.variant.update({
      where: { id: seededVariant.id },
      data: {
        gitInfo: withMergedCloneMetadata(syncedVariant.gitInfo, {
          status: 'ready',
          phase: 'clone.ready',
          startedAt: startedAt.toISOString(),
          finishedAt: finishedAt.toISOString(),
          sourceLocator,
          targetPath,
          branchName: resolvedCloneType === 'git.clone_branch' ? branch.value : null,
          attemptedCommand,
          usedNoLocalRetry,
          lastLine: lastCloneLine,
        }),
        gitInfoUpdatedAt: finishedAt,
      },
    });

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

    return variant;
  };

  const runAsync = input.runAsync === true;
  const sourceLocatorKind = (() => {
    const parsed = parseLocatorId(sourceLocator);
    if (parsed.type === 'git') {
      return 'git' as const;
    }

    if (parsed.type === 'ssh') {
      return 'ssh' as const;
    }

    return 'local' as const;
  })();

  if (runAsync) {
    void executeCloneLifecycle().catch((error) => {
      Log.error(
        `Core // Variant Clones Controller // Async clone failed ${formatLogMetadata({
          error: error instanceof Error ? error.message : String(error),
          productId: product.id,
          targetPath,
          variantId,
        })}`,
      );
    });

    return {
      variant: seededVariant,
      clone: {
        cloneType: resolvedCloneType,
        sourceLocator,
        sourceLocatorKind,
        targetPath,
        targetLocator,
        branchName: resolvedCloneType === 'git.clone_branch' ? branch.value : null,
        generatedTargetPath,
        generatedBranchName: resolvedCloneType === 'git.clone_branch' ? branch.generated : false,
        attemptedCommand,
        usedNoLocalRetry,
        isAsync: true,
      },
    };
  }

  const variant = await executeCloneLifecycle();

  return {
    variant,
    clone: {
      cloneType: resolvedCloneType,
      sourceLocator,
      sourceLocatorKind,
      targetPath,
      targetLocator,
      branchName: resolvedCloneType === 'git.clone_branch' ? branch.value : null,
      generatedTargetPath,
      generatedBranchName: resolvedCloneType === 'git.clone_branch' ? branch.generated : false,
      attemptedCommand,
      usedNoLocalRetry,
      isAsync: false,
    },
  };
};
