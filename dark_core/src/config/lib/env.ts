import type { ConfigEnvironment, EnvBinding } from './types';

const BOOLEAN_PATTERN = /^(true|false)$/i;
const INTEGER_PATTERN = /^-?(0|[1-9]\d*)$/;
const FLOAT_PATTERN = /^-?(0|[1-9]\d*)\.\d+$/;

const parseNumeric = (value: string): number | undefined => {
  if (!INTEGER_PATTERN.test(value) && !FLOAT_PATTERN.test(value)) {
    return undefined;
  }

  const parsed = Number(value);
  return Number.isFinite(parsed) ? parsed : undefined;
};

const setNestedValue = (
  target: Record<string, unknown>,
  path: string,
  value: unknown,
): void => {
  const segments = path.split('.').filter((segment) => segment.length > 0);
  if (segments.length === 0) {
    return;
  }

  let cursor: Record<string, unknown> = target;

  for (let index = 0; index < segments.length; index += 1) {
    const segment = segments[index]!;
    const isFinal = index === segments.length - 1;

    if (isFinal) {
      cursor[segment] = value;
      return;
    }

    const nextValue = cursor[segment];
    if (typeof nextValue !== 'object' || nextValue === null || Array.isArray(nextValue)) {
      cursor[segment] = {};
    }

    cursor = cursor[segment] as Record<string, unknown>;
  }
};

/** Parse raw environment string values into primitives when possible. */
export const parseEnvValue = (rawValue: string): unknown => {
  const value = rawValue.trim();

  if (value.length === 0) {
    return undefined;
  }

  if (BOOLEAN_PATTERN.test(value)) {
    return value.toLowerCase() === 'true';
  }

  const numericValue = parseNumeric(value);
  if (numericValue !== undefined) {
    return numericValue;
  }

  if (value.startsWith('{') || value.startsWith('[')) {
    try {
      return JSON.parse(value);
    } catch {
      return value;
    }
  }

  return value;
};

/** Build a nested env overlay object from explicit env bindings. */
export const envOverlayFromBindings = (
  env: ConfigEnvironment,
  bindings: ReadonlyArray<EnvBinding>,
): Record<string, unknown> => {
  const overlay: Record<string, unknown> = {};

  for (const binding of bindings) {
    const rawValue = env[binding.env];
    if (rawValue === undefined) {
      continue;
    }

    const parsedValue = parseEnvValue(rawValue);
    if (parsedValue === undefined) {
      continue;
    }

    setNestedValue(overlay, binding.path, parsedValue);
  }

  return overlay;
};
