import { stat } from 'node:fs/promises';
import { resolve } from 'node:path';

export const normalizeProviderDirectory = async (
  directory: string,
  providerName = 'Provider',
): Promise<string> => {
  const resolvedDirectory = resolve(directory);
  const directoryInfo = await stat(resolvedDirectory).catch(() => undefined);

  if (!directoryInfo || !directoryInfo.isDirectory()) {
    throw new Error(
      `${providerName} // Directory // Expected existing directory (directory=${resolvedDirectory})`,
    );
  }

  return resolvedDirectory;
};
