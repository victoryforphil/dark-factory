import type { ZodRawShape, ZodTypeAny } from 'zod';

export type ConfigEnvironment = Record<string, string | undefined>;

export interface EnvBinding {
  path: string;
  env: string;
}

export interface ConfigValueDefinition<TSchema extends ZodTypeAny = ZodTypeAny> {
  schema: TSchema;
  env?: string;
}

export interface ConfigSubsystemDefinition<TShape extends ZodRawShape = ZodRawShape> {
  shape: TShape;
  env: ReadonlyArray<EnvBinding>;
}
