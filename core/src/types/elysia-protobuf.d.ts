declare module "elysia-protobuf" {
  export class ProtoRequestError extends Error {}
  export class ProtoResponseError extends Error {}

  export type ProtobufSchemas = Record<string, unknown>;

  export const protobuf: (options: { schemas: ProtobufSchemas }) => any;
  export const protobufParser: () => any;
}
