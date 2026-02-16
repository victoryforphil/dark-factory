const VARIANT_ID_PREFIX = 'var_';
const VARIANT_ID_WIDTH = 13;
const ACTOR_ID_PREFIX = 'act_';
const ACTOR_ID_WIDTH = 13;

const toFixedBase36 = (value: bigint, width: number): string => {
  return value.toString(36).padStart(width, '0');
};

const randomUint64 = (): bigint => {
  const bytes = crypto.getRandomValues(new Uint8Array(8));
  let value = 0n;

  for (const byte of bytes) {
    value = (value << 8n) | BigInt(byte);
  }

  return value;
};

export const buildRandomVariantId = (): string => {
  return `${VARIANT_ID_PREFIX}${toFixedBase36(randomUint64(), VARIANT_ID_WIDTH)}`;
};

export const buildRandomActorId = (): string => {
  return `${ACTOR_ID_PREFIX}${toFixedBase36(randomUint64(), ACTOR_ID_WIDTH)}`;
};
