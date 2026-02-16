#!/usr/bin/env bun

import { existsSync, mkdirSync, rmSync } from "node:fs";
import { join } from "node:path";
import { $ } from "bun";
import { findRepoRoot } from "./helpers/run_root.sh.ts";

// Resolve all paths from repository root so the script can be run
// from anywhere and still behave the same.
const repoRoot = findRepoRoot(import.meta.dir);
const schemaRoot = join(repoRoot, "schemas");
const coreRoot = join(repoRoot, "core");
const outputRoot = join(coreRoot, "src", "gen", "proto");
const entrySchema = "core/v1/core.proto";

// ts-proto is installed in core/node_modules and used as a protoc plugin.
const pluginPath = join(coreRoot, "node_modules", ".bin", "protoc-gen-ts_proto");

if (!existsSync(pluginPath)) {
  throw new Error(
    `Core // Protobuf // Missing ts-proto plugin (${pluginPath}). Run bun install in core first.`,
  );
}

// Generate all schema files under schemas/core to keep imports consistent.
const schemaFiles = [...new Bun.Glob("core/**/*.proto").scanSync({ cwd: schemaRoot })].sort();

if (schemaFiles.length === 0) {
  throw new Error(`Core // Protobuf // No schema files found (${schemaRoot})`);
}

// Clean generated output first so stale files from renamed schemas are removed.
rmSync(outputRoot, { recursive: true, force: true });
mkdirSync(outputRoot, { recursive: true });

console.log(
  `Core // Protobuf // Generating TS (schemas=${schemaFiles.length},out=${outputRoot})`,
);

// 1) Generate TypeScript classes/types with ts-proto.
await $`protoc --plugin=${pluginPath} --ts_proto_out=${outputRoot} --ts_proto_opt=esModuleInterop=true --ts_proto_opt=importSuffix=.js --ts_proto_opt=snakeToCamel=keys_json --proto_path=${schemaRoot} ${schemaFiles}`.cwd(
  repoRoot,
);

// 2) Emit a descriptor set for tooling/debugging and schema validation workflows.
await $`protoc --include_imports --descriptor_set_out=${join(schemaRoot, "core", "core.pb")} --proto_path=${schemaRoot} ${entrySchema}`.cwd(
  repoRoot,
);

console.log("Core // Protobuf // Generated TypeScript and descriptor set (core/core.pb)");
