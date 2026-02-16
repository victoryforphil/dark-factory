import {
  ProtoRequestError,
  ProtoResponseError,
  protobuf,
  protobufParser,
} from "elysia-protobuf";
import {
  CreateProductRequest,
  CreateProductResponse,
} from "./gen/proto/core/v1/api/product_api";

export const createProductRequestSchemaId = "core.v1.create_product.request";
export const createProductResponseSchemaId = "core.v1.create_product.response";

// Keep schema IDs centralized so route handlers and plugin config stay in sync.
const schemas = {
  [createProductRequestSchemaId]: CreateProductRequest,
  [createProductResponseSchemaId]: CreateProductResponse,
};

// Parser and plugin are exported separately for explicit app bootstrap ordering.
export const protobufPlugin = protobuf({ schemas: schemas as any });
export const protobufBodyParser = protobufParser();

export { ProtoRequestError, ProtoResponseError };
