import type { Elysia } from "elysia";

import {
  createProductRequestSchemaId,
  createProductResponseSchemaId,
} from "../../protobuf";

export const registerProductRoutes = (app: Elysia) =>
  app.post(
    "/v1/products:create",
    async ({ body, decode, headers }) => {
      const request = await decode(createProductRequestSchemaId, body, headers);

      return {
        error: {
          code: "NOT_IMPLEMENTED",
          message: "Create product API is scaffolded but not implemented yet.",
          details: {
            product_locator: request.productLocator,
          },
        },
      };
    },
    {
      parse: "protobuf",
      responseSchema: createProductResponseSchemaId,
    },
  );
