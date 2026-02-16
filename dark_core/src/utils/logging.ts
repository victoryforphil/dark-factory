import adze from 'adze';

export const logger = adze;

export type LogMetadataValue = string | number | boolean | null | undefined;

export const startLogTimer = (): number => {
  return Date.now();
};

export const logInfoDuration = (
  message: string,
  startedAtMs: number,
  metadata: Record<string, LogMetadataValue> = {},
): void => {
  const durationMs = Math.max(0, Date.now() - startedAtMs);

  logger
    .info(
      `${message} ${formatLogMetadata({
        ...metadata,
        durationMs,
      })}`,
    );
};

export const formatLogMetadata = (metadata: Record<string, LogMetadataValue>): string => {
  const normalizedEntries = Object.entries(metadata)
    .filter(([, value]) => value !== undefined)
    .sort(([leftKey], [rightKey]) => leftKey.localeCompare(rightKey));

  return `(meta=${JSON.stringify(Object.fromEntries(normalizedEntries))})`;
};

export default logger;
