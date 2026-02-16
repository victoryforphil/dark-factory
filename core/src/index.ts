import { Elysia } from "elysia";
import { config } from "./config";
import { logger, requestLogger } from "./logging";
import {
  ProtoRequestError,
  ProtoResponseError,
  createProductRequestSchemaId,
  createProductResponseSchemaId,
  protobufBodyParser,
  protobufPlugin,
} from "./protobuf";

const serviceName = "dark-factory-core";

const app = new Elysia()
  .use(requestLogger)
  .use(protobufBodyParser)
  .use(protobufPlugin)
  .error({
    PROTO_REQUEST_ERROR: ProtoRequestError,
    PROTO_RESPONSE_ERROR: ProtoResponseError,
  })
  .onError(({ code, error, set }) => {
    if (code === "PROTO_REQUEST_ERROR") {
      set.status = 400;
      return {
        message: (error as Error).message,
      };
    }

    if (code === "PROTO_RESPONSE_ERROR") {
      set.status = 500;
      return {
        message: (error as Error).message,
      };
    }

    return;
  })
  .get("/", () => ({
    service: serviceName,
    status: "ok",
    message: "Dark Factory core is running",
    concepts: ["product", "variant", "actor"],
    env: config.env,
  }))
  .get("/health", () => ({ status: "ok" }))
  .post(
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
  )
  .listen({
    hostname: config.http.address_listen,
    port: config.http.address_port,
  });

logger.info(
  `Core // HTTP // Listening (env=${config.env},host=${app.server?.hostname},port=${app.server?.port})`
);
