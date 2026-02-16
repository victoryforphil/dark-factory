import { Elysia } from "elysia";
import { config } from "./config";
import { logger } from "./logging";

const serviceName = "dark-factory-core";

const app = new Elysia()
  .get("/", () => ({
    service: serviceName,
    status: "ok",
    message: "Dark Factory core is running",
    concepts: ["world", "env", "actor"],
    env: config.env,
  }))
  .get("/health", () => ({ status: "ok" }))
  .listen({
    hostname: config.http.address_listen,
    port: config.http.address_port,
  });

logger.info(
  `Core // HTTP // Listening (env=${config.env},host=${app.server?.hostname},port=${app.server?.port})`
);
