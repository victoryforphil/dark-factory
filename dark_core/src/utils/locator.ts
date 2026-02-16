import { normalize, posix } from 'node:path';

const LOCAL_LOCATOR_PREFIX = '@local://';
const PRODUCT_ID_PREFIX = 'prd_';
const PRODUCT_ID_WIDTH = 13;
const WINDOWS_ABSOLUTE_PATH_PATTERN = /^[A-Za-z]:[\\/]/;
const WINDOWS_ROOT_PATH_PATTERN = /^[A-Za-z]:\/$/;

export type LocatorId =
  | {
      type: 'local';
      locator: string;
      canonicalPath: string;
    }
  | {
      type: 'unknown';
      locator: string;
    };

const normalizePathSeparators = (value: string): string => {
  return value.replace(/\\/g, '/');
};

const toWindowsPath = (value: string): string => {
  return value.replace(/\//g, '\\');
};

const isAbsoluteLocalPath = (value: string): boolean => {
  return value.startsWith('/') || WINDOWS_ABSOLUTE_PATH_PATTERN.test(value);
};

const isRootPath = (value: string): boolean => {
  return value === '/' || WINDOWS_ROOT_PATH_PATTERN.test(value);
};

const normalizeLocalPath = (value: string): string => {
  const withForwardSlashes = normalizePathSeparators(value);
  const normalizedPath = posix.normalize(withForwardSlashes);

  const withoutTrailingSlash =
    normalizedPath.endsWith('/') && !isRootPath(normalizedPath)
      ? normalizedPath.slice(0, -1)
      : normalizedPath;

  if (process.platform === 'win32') {
    return withoutTrailingSlash.replace(/^([A-Z]):/, (_, drive: string) => `${drive.toLowerCase()}:`);
  }

  return withoutTrailingSlash;
};

const localPathToHostPath = (canonicalPath: string): string => {
  if (process.platform === 'win32') {
    return normalize(toWindowsPath(canonicalPath));
  }

  return normalize(canonicalPath);
};

const hostAbsolutePathToLocalPath = (absolutePath: string): string => {
  if (process.platform === 'win32') {
    const normalizedAbsolutePath = normalize(absolutePath);
    const withForwardSlashes = normalizePathSeparators(normalizedAbsolutePath);
    return normalizeLocalPath(withForwardSlashes);
  }

  return normalizeLocalPath(absolutePath);
};

const toFixedBase36 = (value: bigint, width: number): string => {
  return value.toString(36).padStart(width, '0');
};

const sha256ToFirst64Bits = (value: string): bigint => {
  const digestHex = new Bun.CryptoHasher('sha256').update(value).digest('hex');
  const first64BitsHex = digestHex.slice(0, 16);
  return BigInt(`0x${first64BitsHex}`);
};

export const isLocalLocator = (locator: string): boolean => {
  return locator.startsWith(LOCAL_LOCATOR_PREFIX);
};

export const parseLocatorId = (locator: string): LocatorId => {
  const trimmedLocator = locator.trim();

  if (!isLocalLocator(trimmedLocator)) {
    return {
      type: 'unknown',
      locator: trimmedLocator,
    };
  }

  const canonicalLocator = canonicalizeLocalLocator(trimmedLocator);
  return {
    type: 'local',
    locator: canonicalLocator,
    canonicalPath: canonicalLocator.slice(LOCAL_LOCATOR_PREFIX.length),
  };
};

export const canonicalizeLocalLocator = (locator: string): string => {
  if (!isLocalLocator(locator)) {
    throw new Error(`Products // Locator // Expected @local:// locator (locator=${locator})`);
  }

  const localPath = locator.slice(LOCAL_LOCATOR_PREFIX.length);

  if (!isAbsoluteLocalPath(localPath)) {
    throw new Error(
      `Products // Locator // Expected absolute local path in locator (locator=${locator})`,
    );
  }

  const canonicalLocalPath = normalizeLocalPath(localPath);
  return `${LOCAL_LOCATOR_PREFIX}${canonicalLocalPath}`;
};

export const normalizeLocator = (locator: string): string => {
  const trimmedLocator = locator.trim();

  if (isLocalLocator(trimmedLocator)) {
    return canonicalizeLocalLocator(trimmedLocator);
  }

  if (isAbsoluteLocalPath(trimmedLocator)) {
    const canonicalLocalPath = normalizeLocalPath(trimmedLocator);
    return `${LOCAL_LOCATOR_PREFIX}${canonicalLocalPath}`;
  }

  return trimmedLocator;
};

export const locatorIdToHostPath = (locator: string): string => {
  const parsed = parseLocatorId(locator);

  switch (parsed.type) {
    case 'local':
      return localPathToHostPath(parsed.canonicalPath);
    case 'unknown':
      throw new Error(
        `Products // Locator // Unsupported locator format for host path conversion (locator=${parsed.locator})`,
      );
  }
};

export const hostAbsolutePathToLocatorId = (absolutePath: string): string => {
  const normalizedHostPath = process.platform === 'win32' ? normalize(absolutePath) : absolutePath;

  if (!isAbsoluteLocalPath(normalizedHostPath)) {
    throw new Error(
      `Products // Locator // Expected absolute host path for locator conversion (path=${absolutePath})`,
    );
  }

  const canonicalLocalPath = hostAbsolutePathToLocalPath(normalizedHostPath);
  return `${LOCAL_LOCATOR_PREFIX}${canonicalLocalPath}`;
};

export const buildDeterministicIdFromLocator = (canonicalLocator: string): string => {
  const token = toFixedBase36(sha256ToFirst64Bits(canonicalLocator), PRODUCT_ID_WIDTH);
  return `${PRODUCT_ID_PREFIX}${token}`;
};
