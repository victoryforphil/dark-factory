const isPlainObject = (value: unknown): value is Record<string, unknown> => {
  if (typeof value !== 'object' || value === null) {
    return false;
  }

  if (Array.isArray(value)) {
    return false;
  }

  const prototype = Object.getPrototypeOf(value);
  return prototype === Object.prototype || prototype === null;
};

const mergePair = (left: unknown, right: unknown): unknown => {
  if (!isPlainObject(left)) {
    if (isPlainObject(right)) {
      return deepMerge(right);
    }

    if (Array.isArray(right)) {
      return [...right];
    }

    return right;
  }

  if (!isPlainObject(right)) {
    if (Array.isArray(right)) {
      return [...right];
    }

    return right;
  }

  const next: Record<string, unknown> = { ...left };

  for (const [key, rightValue] of Object.entries(right)) {
    if (!(key in next)) {
      next[key] = Array.isArray(rightValue) ? [...rightValue] : rightValue;
      continue;
    }

    const leftValue = next[key];
    next[key] = mergePair(leftValue, rightValue);
  }

  return next;
};

/** Merge config layers deeply; arrays are replaced (not concatenated). */
export const deepMerge = (...layers: unknown[]): Record<string, unknown> => {
  let result: unknown = {};

  for (const layer of layers) {
    if (layer === undefined) {
      continue;
    }

    result = mergePair(result, layer);
  }

  return isPlainObject(result) ? result : {};
};
