import {
  ProtoRequestError,
  ProtoResponseError,
  protobuf,
  protobufParser,
} from "elysia-protobuf";

import {
  CreateProductRequest,
  CreateProductResponse,
  DeleteProductRequest,
  DeleteProductResponse,
  GetProductRequest,
  GetProductResponse,
  ListProductsRequest,
  ListProductsResponse,
  UpdateProductRequest,
  UpdateProductResponse,
} from "../gen/proto/core/v1/api/product_api";

export const createProductRequestSchemaId = "core.v1.product.create.request";
export const createProductResponseSchemaId = "core.v1.product.create.response";

export const getProductRequestSchemaId = "core.v1.product.get.request";
export const getProductResponseSchemaId = "core.v1.product.get.response";

export const listProductsRequestSchemaId = "core.v1.product.list.request";
export const listProductsResponseSchemaId = "core.v1.product.list.response";

export const updateProductRequestSchemaId = "core.v1.product.update.request";
export const updateProductResponseSchemaId = "core.v1.product.update.response";

export const deleteProductRequestSchemaId = "core.v1.product.delete.request";
export const deleteProductResponseSchemaId = "core.v1.product.delete.response";

const schemas = {
  [createProductRequestSchemaId]: CreateProductRequest,
  [createProductResponseSchemaId]: CreateProductResponse,
  [getProductRequestSchemaId]: GetProductRequest,
  [getProductResponseSchemaId]: GetProductResponse,
  [listProductsRequestSchemaId]: ListProductsRequest,
  [listProductsResponseSchemaId]: ListProductsResponse,
  [updateProductRequestSchemaId]: UpdateProductRequest,
  [updateProductResponseSchemaId]: UpdateProductResponse,
  [deleteProductRequestSchemaId]: DeleteProductRequest,
  [deleteProductResponseSchemaId]: DeleteProductResponse,
};

export const protobufPlugin = protobuf({ schemas: schemas as any });
export const protobufBodyParser = protobufParser();

export { ProtoRequestError, ProtoResponseError };
