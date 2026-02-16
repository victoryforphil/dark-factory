import { z } from 'zod';

import type { ConfigSubsystemDefinition } from './types';

/** Builds a section schema and defaults missing section values to an empty object. */
export const createSubsystemSchema = <TShape extends z.ZodRawShape>(
  section: ConfigSubsystemDefinition<TShape>,
  strict: boolean,
) => {
  const baseSchema = strict
    ? z.object(section.shape).strict()
    : z.object(section.shape).passthrough();

  return z.preprocess((value) => value ?? {}, baseSchema);
};
