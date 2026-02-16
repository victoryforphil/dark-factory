import { config } from "./config";
import { logger } from "./logging";
import { createApp } from "./app";

const app = createApp().listen({
  hostname: config.http.address_listen,
  port: config.http.address_port,
});

logger.info(
  `Core // HTTP // Listening (env=${config.env},host=${app.server?.hostname},port=${app.server?.port})`
);
