import { defineConfig } from "prisma/config";

export default defineConfig({
  schema: "prisma/schema.prisma",
  datasource: {
    url: process.env.DARKFACTORY_SQLITE_URL ?? "file:./darkfactory.db",
  },
});
