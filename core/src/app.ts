import { Elysia } from "elysia";
import { openapi } from "@elysiajs/openapi";

import {
  ProtoRequestError,
  ProtoResponseError,
  protobufBodyParser,
  protobufPlugin,
} from "./protobuf";
import { handleAppError } from "./http/error-handler";
import { registerSystemRoutes } from "./http/routes/system-routes";
import { registerProductRoutes } from "./http/routes/product-routes";

export const createApp = () => {
  const app = new Elysia()
    .use(
      openapi({
        documentation: {
          info: {
            title: "Dark Factory Core API",
            version: "0.1.0",
            description: "OpenAPI documentation for the Dark Factory core service.",
          },
        },
        path: "/openapi",
        provider: "swagger-ui",
        specPath: "/openapi/json",
      }),
    )
    .use(protobufBodyParser)
    .use(protobufPlugin)
    .error({
      PROTO_REQUEST_ERROR: ProtoRequestError,
      PROTO_RESPONSE_ERROR: ProtoResponseError,
    })
    .onError(handleAppError);

  registerSystemRoutes(app);
  registerProductRoutes(app);

  return app;
};
