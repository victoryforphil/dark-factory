-- CreateTable
CREATE TABLE "products" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "locator" TEXT NOT NULL,
    "display_name" TEXT,
    "workspace_locator" TEXT,
    "git_info" JSONB,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME NOT NULL
);

-- CreateTable
CREATE TABLE "variants" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "product_id" TEXT NOT NULL,
    "name" TEXT NOT NULL DEFAULT 'default',
    "locator" TEXT NOT NULL,
    "git_info" JSONB,
    "git_info_updated_at" DATETIME,
    "git_info_last_polled_at" DATETIME,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME NOT NULL,
    CONSTRAINT "variants_product_id_fkey" FOREIGN KEY ("product_id") REFERENCES "products" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "actors" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "variant_id" TEXT NOT NULL,
    "provider" TEXT NOT NULL,
    "actor_locator" TEXT NOT NULL,
    "working_locator" TEXT NOT NULL,
    "provider_session_id" TEXT,
    "status" TEXT NOT NULL DEFAULT 'unknown',
    "title" TEXT,
    "description" TEXT,
    "connection_info" JSONB,
    "attach_command" TEXT,
    "sub_agents" JSONB,
    "metadata" JSONB,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME NOT NULL,
    CONSTRAINT "actors_variant_id_fkey" FOREIGN KEY ("variant_id") REFERENCES "variants" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);

-- CreateIndex
CREATE UNIQUE INDEX "products_locator_key" ON "products"("locator");

-- CreateIndex
CREATE INDEX "products_locator_idx" ON "products"("locator");

-- CreateIndex
CREATE INDEX "variants_product_id_idx" ON "variants"("product_id");

-- CreateIndex
CREATE INDEX "variants_locator_idx" ON "variants"("locator");

-- CreateIndex
CREATE UNIQUE INDEX "variants_product_id_name_key" ON "variants"("product_id", "name");

-- CreateIndex
CREATE INDEX "actors_variant_id_idx" ON "actors"("variant_id");

-- CreateIndex
CREATE INDEX "actors_provider_idx" ON "actors"("provider");

-- CreateIndex
CREATE INDEX "actors_status_idx" ON "actors"("status");

-- CreateIndex
CREATE INDEX "actors_provider_session_id_idx" ON "actors"("provider_session_id");

-- CreateIndex
CREATE UNIQUE INDEX "actors_variant_id_provider_provider_session_id_key" ON "actors"("variant_id", "provider", "provider_session_id");
