#!/usr/bin/env bun

import { resolve } from "node:path";
import { findRepoRoot } from "../helpers/run_root.sh.ts";

type SourceKey = "opencode" | "elysia" | "prisma" | "moonrepo" | "ratatui_web" | "ratatui_docs";

const SCRAPER_SCRIPT_BY_SOURCE: Record<SourceKey, string> = {
  opencode: "scripts/scrapes/scrape_opencode_docs.sh.ts",
  elysia: "scripts/scrapes/scrape_elysia_docs.sh.ts",
  prisma: "scripts/scrapes/scrape_prisma_docs.sh.ts",
  moonrepo: "scripts/scrapes/scrape_moonrepo_docs.sh.ts",
  ratatui_web: "scripts/scrapes/scrape_ratatui_web_docs.sh.ts",
  ratatui_docs: "scripts/scrapes/scrape_ratatui_docs_docs.sh.ts",
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
