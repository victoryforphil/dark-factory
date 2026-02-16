import { createHash } from "node:crypto";
import { posix as posixPath } from "node:path";

const localLocatorPrefix = "@local://";
const stageZeroVariantName = "default";
const windowsAbsolutePathPattern = /^[a-zA-Z]:\//;
const windowsRootPattern = /^[a-zA-Z]:\/$/;

const removeTrailingSlash = (absolutePath: string): string => {
  if (absolutePath === "/" || windowsRootPattern.test(absolutePath)) {
    return absolutePath;
  }

  return absolutePath.endsWith("/")
    ? absolutePath.slice(0, -1)
    : absolutePath;
};

const normalizeAbsolutePath = (rawPath: string): string => {
  const withUnixSeparators = rawPath.replace(/\\/g, "/");
  const normalized = posixPath.normalize(withUnixSeparators);

  if (!normalized.startsWith("/") && !windowsAbsolutePathPattern.test(normalized)) {
    throw new Error(
      `Core // Product // Locator must contain an absolute path (path=${rawPath})`,
    );
  }

  if (windowsAbsolutePathPattern.test(normalized)) {
    const driveLetter = normalized.slice(0, 1).toLowerCase();
    const rest = normalized.slice(1);
    return removeTrailingSlash(`${driveLetter}${rest}`);
  }

  return removeTrailingSlash(normalized);
};

export const canonicalizeProductLocator = (locator: string): string => {
  if (!locator.startsWith(localLocatorPrefix)) {
    throw new Error(
      `Core // Product // Locator must start with @local:// (locator=${locator})`,
    );
  }

  const absolutePath = locator.slice(localLocatorPrefix.length);
  if (absolutePath.length === 0) {
    throw new Error("Core // Product // Locator path must not be empty");
  }

  if (absolutePath.includes("#")) {
    throw new Error(
      `Core // Product // Product locator must not contain variant fragment (locator=${locator})`,
    );
  }

  const normalizedPath = normalizeAbsolutePath(absolutePath);
  return `${localLocatorPrefix}${normalizedPath}`;
};

export const buildDefaultVariantLocator = (canonicalProductLocator: string): string => {
  return `${canonicalProductLocator}#${stageZeroVariantName}`;
};

const sha256Hex = (value: string): string => {
  return createHash("sha256").update(value).digest("hex");
};

export const buildProductId = (canonicalProductLocator: string): string => {
  return `prd_${sha256Hex(canonicalProductLocator)}`;
};

export const buildVariantId = (canonicalVariantLocator: string): string => {
  return `var_${sha256Hex(canonicalVariantLocator)}`;
};

export interface StageZeroProductIdentity {
  canonicalProductLocator: string;
  productId: string;
  canonicalDefaultVariantLocator: string;
  defaultVariantId: string;
  defaultVariantName: string;
}

export const deriveStageZeroProductIdentity = (
  locator: string,
): StageZeroProductIdentity => {
  const canonicalProductLocator = canonicalizeProductLocator(locator);
  const canonicalDefaultVariantLocator = buildDefaultVariantLocator(canonicalProductLocator);

  return {
    canonicalProductLocator,
    productId: buildProductId(canonicalProductLocator),
    canonicalDefaultVariantLocator,
    defaultVariantId: buildVariantId(canonicalDefaultVariantLocator),
    defaultVariantName: stageZeroVariantName,
  };
};
