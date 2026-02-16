const stringSchema = { type: "string" };
const timestampSchema = { type: "string", format: "date-time" };

const apiErrorSchema = {
  type: "object",
  required: ["code", "message"],
  properties: {
    code: stringSchema,
    message: stringSchema,
    details: {
      type: "object",
      additionalProperties: stringSchema,
    },
  },
  additionalProperties: false,
};

const productSchema = {
  type: "object",
  required: [
    "id",
    "locator",
    "defaultVariantId",
    "createdAt",
    "updatedAt",
  ],
  properties: {
    id: stringSchema,
    locator: stringSchema,
    displayName: stringSchema,
    defaultVariantId: stringSchema,
    createdAt: timestampSchema,
    updatedAt: timestampSchema,
  },
  additionalProperties: false,
};

const variantSchema = {
  type: "object",
  required: ["id", "productId", "locator", "name", "createdAt"],
  properties: {
    id: stringSchema,
    productId: stringSchema,
    locator: stringSchema,
    name: stringSchema,
    createdAt: timestampSchema,
  },
  additionalProperties: false,
};

const productRecordSchema = {
  type: "object",
  required: ["product", "defaultVariant"],
  properties: {
    product: productSchema,
    defaultVariant: variantSchema,
  },
  additionalProperties: false,
};

const createProductRequestSchema = {
  type: "object",
  required: ["productLocator"],
  properties: {
    productLocator: {
      ...stringSchema,
      description: "Required. Format: @local://{abs_path}",
    },
    displayName: stringSchema,
  },
  additionalProperties: false,
};

const createProductSuccessSchema = {
  type: "object",
  required: ["product", "defaultVariant"],
  properties: {
    product: productSchema,
    defaultVariant: variantSchema,
  },
  additionalProperties: false,
};

const getProductRequestSchema = {
  oneOf: [
    {
      type: "object",
      required: ["productId"],
      properties: {
        productId: stringSchema,
      },
      additionalProperties: false,
    },
    {
      type: "object",
      required: ["productLocator"],
      properties: {
        productLocator: stringSchema,
      },
      additionalProperties: false,
    },
  ],
};

const getProductSuccessSchema = {
  type: "object",
  required: ["record"],
  properties: {
    record: productRecordSchema,
  },
  additionalProperties: false,
};

const listProductsRequestSchema = {
  type: "object",
  properties: {
    limit: {
      type: "integer",
      minimum: 1,
      maximum: 200,
    },
    offset: {
      type: "integer",
      minimum: 0,
    },
  },
  additionalProperties: false,
};

const listProductsSuccessSchema = {
  type: "object",
  required: ["records", "totalCount"],
  properties: {
    records: {
      type: "array",
      items: productRecordSchema,
    },
    totalCount: {
      type: "integer",
      minimum: 0,
    },
  },
  additionalProperties: false,
};

const updateProductRequestSchema = {
  type: "object",
  required: ["productId", "clearDisplayName"],
  properties: {
    productId: stringSchema,
    displayName: stringSchema,
    clearDisplayName: { type: "boolean" },
  },
  additionalProperties: false,
};

const updateProductSuccessSchema = {
  type: "object",
  required: ["record"],
  properties: {
    record: productRecordSchema,
  },
  additionalProperties: false,
};

const deleteProductRequestSchema = {
  type: "object",
  required: ["productId"],
  properties: {
    productId: stringSchema,
  },
  additionalProperties: false,
};

const deleteProductSuccessSchema = {
  type: "object",
  required: ["productId", "defaultVariantId"],
  properties: {
    productId: stringSchema,
    defaultVariantId: stringSchema,
  },
  additionalProperties: false,
};

const oneOfOkOrError = (okSchema: Record<string, unknown>) => ({
  oneOf: [
    {
      type: "object",
      required: ["ok"],
      properties: {
        ok: okSchema,
      },
      additionalProperties: false,
    },
    {
      type: "object",
      required: ["error"],
      properties: {
        error: apiErrorSchema,
      },
      additionalProperties: false,
    },
  ],
});

const protobufContent = (schema: Record<string, unknown>) => ({
  "application/x-protobuf": {
    schema,
  },
});

const operationDetail = (
  summary: string,
  requestSchema: Record<string, unknown>,
  successSchema: Record<string, unknown>,
) => ({
  tags: ["products"],
  summary,
  requestBody: {
    required: true,
    content: protobufContent(requestSchema),
  },
  responses: {
    200: {
      description: "Protobuf response envelope",
      content: protobufContent(oneOfOkOrError(successSchema)),
    },
  },
});

export const createProductOpenApiDetail = operationDetail(
  "Create product",
  createProductRequestSchema,
  createProductSuccessSchema,
);

export const getProductOpenApiDetail = operationDetail(
  "Get product",
  getProductRequestSchema,
  getProductSuccessSchema,
);

export const listProductsOpenApiDetail = operationDetail(
  "List products",
  listProductsRequestSchema,
  listProductsSuccessSchema,
);

export const updateProductOpenApiDetail = operationDetail(
  "Update product",
  updateProductRequestSchema,
  updateProductSuccessSchema,
);

export const deleteProductOpenApiDetail = operationDetail(
  "Delete product",
  deleteProductRequestSchema,
  deleteProductSuccessSchema,
);
