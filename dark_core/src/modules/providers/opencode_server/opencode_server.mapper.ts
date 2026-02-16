import type { ActorStatusLabel, ProviderMessage } from '../providers.common';
import type { OpenCodeStatusLike } from './opencode_server.types';

const collectText = (value: unknown, depth = 0): string[] => {
  if (depth > 4 || value === null || value === undefined) {
    return [];
  }

  if (typeof value === 'string') {
    const trimmed = value.trim();
    return trimmed ? [trimmed] : [];
  }

  if (Array.isArray(value)) {
    return value.flatMap((item) => collectText(item, depth + 1));
  }

  if (typeof value !== 'object') {
    return [];
  }

  const source = value as Record<string, unknown>;
  return ['text', 'content', 'value', 'message'].flatMap((key) => collectText(source[key], depth + 1));
};

const extractPartText = (part: { type: string; text?: string }): string => {
  const source = part as unknown as Record<string, unknown>;
  const segments = [source.text, source.content, source.value, source.message]
    .flatMap((entry) => collectText(entry))
    .filter((entry, index, all) => all.indexOf(entry) === index);

  return segments.join('\n').trim();
};

export const mapOpenCodeSessionStatus = (status: OpenCodeStatusLike | undefined): ActorStatusLabel => {
  if (!status) {
    return 'stopped';
  }

  if (status.type === 'idle') {
    return 'ready';
  }

  if (status.type === 'busy') {
    return 'busy';
  }

  if (status.type === 'retry') {
    return 'retrying';
  }

  return 'unknown';
};

export const mapOpenCodeMessages = (
  messages: Array<{ info: { id: string; role: string; time: { created: number } }; parts: Array<{ type: string; text?: string }> }>,
): ProviderMessage[] => {
  return messages.map((message) => {
    const text = message.parts
      .map((part) => extractPartText(part))
      .filter((partText) => partText.length > 0)
      .join('\n')
      .trim();

    return {
      id: message.info.id,
      role: message.info.role,
      createdAt: new Date(message.info.time.created).toISOString(),
      ...(text.length > 0 ? { text } : {}),
      raw: {
        info: message.info,
        parts: message.parts,
      },
    };
  });
};
