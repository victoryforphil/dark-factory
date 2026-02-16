#!/usr/bin/env bun

import { resolve } from "node:path";
import { findRepoRoot } from "./helpers/run_root.sh.ts";

type SourceKey = "opencode" | "elysia" | "prisma";

const SCRAPER_SCRIPT_BY_SOURCE: Record<SourceKey, string> = {
  opencode: "scripts/scrape_opencode_docs.sh.ts",
  elysia: "scripts/scrape_elysia_docs.sh.ts",
  prisma: "scripts/scrape_prisma_docs.sh.ts",
};

const sourceArg = Bun.argv[2] as SourceKey | undefined;
const outputArg = Bun.argv[3];
const source = sourceArg ?? "opencode";

if (!(source in SCRAPER_SCRIPT_BY_SOURCE)) {
  const availableSources = Object.keys(SCRAPER_SCRIPT_BY_SOURCE).join(",");
  throw new Error(
    `Docs // Scrape // Unsupported source (source=${source},available=${availableSources})`,
  );
}

const repoRoot = findRepoRoot(import.meta.dir);
const scriptPath = resolve(repoRoot, SCRAPER_SCRIPT_BY_SOURCE[source]);
const args = outputArg ? [scriptPath, outputArg] : [scriptPath];

console.log(`Docs // Scrape // Dispatching scraper (source=${source},script=${scriptPath})`);

const process = Bun.spawn(["bun", ...args], {
  cwd: repoRoot,
  stdin: "inherit",
  stdout: "inherit",
  stderr: "inherit",
});

const exitCode = await process.exited;
if (exitCode !== 0) {
  throw new Error(`Docs // Scrape // Scraper failed (source=${source},exit=${exitCode})`);
}
