import type { ActorStatusLabel, ProviderMessage } from '../providers.common';
import type { OpenCodeStatusLike } from './opencode.types';

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
      .filter((part) => part.type === 'text' && typeof part.text === 'string')
      .map((part) => part.text)
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
