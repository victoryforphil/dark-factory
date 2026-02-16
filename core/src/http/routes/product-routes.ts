import {
  createProductRequestSchemaId,
  createProductResponseSchemaId,
  deleteProductRequestSchemaId,
  deleteProductResponseSchemaId,
  getProductRequestSchemaId,
  getProductResponseSchemaId,
  listProductsRequestSchemaId,
  listProductsResponseSchemaId,
  updateProductRequestSchemaId,
  updateProductResponseSchemaId,
} from "../../clients/protobuf-client";
import { ProductClientError, productClient } from "../../clients/product-client";
import { logger } from "../../logging";

const toApiError = (
  code: string,
  message: string,
  details: Record<string, string> = {},
) => ({
  error: {
    code,
    message,
    details,
  },
});

const toProtobufBody = (body: unknown): Uint8Array => {
  if (body instanceof Uint8Array) {
    return body;
  }

  if (body instanceof ArrayBuffer) {
    return new Uint8Array(body);
  }

  if (ArrayBuffer.isView(body)) {
    return new Uint8Array(body.buffer, body.byteOffset, body.byteLength);
  }

  throw new ProductClientError("INVALID_ARGUMENT", "Request body must be protobuf bytes");
};

const handleProductRouteError = (error: unknown) => {
  if (error instanceof ProductClientError) {
    return toApiError(error.code, error.message, error.details);
  }

  const reason = error instanceof Error ? error.message : "Unknown error";
  logger.error(`Core // Product API // Unexpected error (reason=${reason})`);

  return toApiError("INTERNAL", "Unexpected product API error", { reason });
};

export const registerProductRoutes = (app: any) =>
  app
    .post(
      "/v1/products/create",
      async ({ body, decode, headers }: any) => {
        try {
          const request = await decode(
            createProductRequestSchemaId,
            toProtobufBody(body),
            headers,
          );
          const created = await productClient.createProduct({
            productLocator: request.productLocator,
            displayName: request.displayName,
          });

          return {
            ok: {
              product: created.product,
              defaultVariant: created.defaultVariant,
            },
          };
        } catch (error) {
          return handleProductRouteError(error);
        }
      },
      {
        parse: "protobuf",
        responseSchema: createProductResponseSchemaId,
      },
    )
    .post(
      "/v1/products/get",
      async ({ body, decode, headers }: any) => {
        try {
          const request = await decode(
            getProductRequestSchemaId,
            toProtobufBody(body),
            headers,
          );
          const record = await productClient.getProduct({
            productId: request.productId,
            productLocator: request.productLocator,
          });

          if (!record) {
            return toApiError("NOT_FOUND", "Product was not found");
          }

          return {
            ok: {
              record: {
                product: record.product,
                defaultVariant: record.defaultVariant,
              },
            },
          };
        } catch (error) {
          return handleProductRouteError(error);
        }
      },
      {
        parse: "protobuf",
        responseSchema: getProductResponseSchemaId,
      },
    )
    .post(
      "/v1/products/list",
      async ({ body, decode, headers }: any) => {
        try {
          const request = await decode(
            listProductsRequestSchemaId,
            toProtobufBody(body),
            headers,
          );
          const result = await productClient.listProducts({
            limit: request.limit,
            offset: request.offset,
          });

          return {
            ok: {
              records: result.records.map((record) => ({
                product: record.product,
                defaultVariant: record.defaultVariant,
              })),
              totalCount: result.totalCount,
            },
          };
        } catch (error) {
          return handleProductRouteError(error);
        }
      },
      {
        parse: "protobuf",
        responseSchema: listProductsResponseSchemaId,
      },
    )
    .post(
      "/v1/products/update",
      async ({ body, decode, headers }: any) => {
        try {
          const request = await decode(
            updateProductRequestSchemaId,
            toProtobufBody(body),
            headers,
          );
          const record = await productClient.updateProduct({
            productId: request.productId,
            displayName: request.displayName,
            clearDisplayName: request.clearDisplayName,
          });

          if (!record) {
            return toApiError("NOT_FOUND", "Product was not found");
          }

          return {
            ok: {
              record: {
                product: record.product,
                defaultVariant: record.defaultVariant,
              },
            },
          };
        } catch (error) {
          return handleProductRouteError(error);
        }
      },
      {
        parse: "protobuf",
        responseSchema: updateProductResponseSchemaId,
      },
    )
    .post(
      "/v1/products/delete",
      async ({ body, decode, headers }: any) => {
        try {
          const request = await decode(
            deleteProductRequestSchemaId,
            toProtobufBody(body),
            headers,
          );
          const deleted = await productClient.deleteProduct({
            productId: request.productId,
          });

          if (!deleted) {
            return toApiError("NOT_FOUND", "Product was not found");
          }

          return {
            ok: {
              productId: deleted.productId,
              defaultVariantId: deleted.defaultVariantId,
            },
          };
        } catch (error) {
          return handleProductRouteError(error);
        }
      },
      {
        parse: "protobuf",
        responseSchema: deleteProductResponseSchemaId,
      },
    );
