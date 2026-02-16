import { config } from "../../config";

const serviceName = "dark-factory-core";

export const registerSystemRoutes = (app: any) =>
  app
    .get("/", () => ({
      service: serviceName,
      status: "ok",
      message: "Dark Factory core is running",
      concepts: ["product", "variant", "actor"],
      env: config.env,
    }))
    .get("/health", () => ({ status: "ok" }));
