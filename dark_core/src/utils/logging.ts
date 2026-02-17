import adze from 'adze';

export const logger = adze;

export interface LogMetadataRecord {
  [key: string]: LogMetadataValue;
}

export type LogMetadataValue =
  | string
  | number
  | boolean
  | null
  | undefined
  | LogMetadataRecord
  | LogMetadataValue[];

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

export const logRouteStart = (
  route: string,
  metadata: Record<string, LogMetadataValue> = {},
): number => {
  const startedAt = startLogTimer();
  logger.info(`Core // Route // ${route} started ${formatLogMetadata(metadata)}`);
  return startedAt;
};

export const logRouteSuccess = (
  route: string,
  startedAtMs: number,
  metadata: Record<string, LogMetadataValue> = {},
): void => {
  logInfoDuration(`Core // Route // ${route} succeeded`, startedAtMs, metadata);
};

export const formatLogMetadata = (metadata: Record<string, LogMetadataValue>): string => {
  const normalizedEntries = Object.entries(metadata)
    .filter(([, value]) => value !== undefined)
    .sort(([leftKey], [rightKey]) => leftKey.localeCompare(rightKey));

  return `(meta=${JSON.stringify(Object.fromEntries(normalizedEntries))})`;
};

export default logger;
