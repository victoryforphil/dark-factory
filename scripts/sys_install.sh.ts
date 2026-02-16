#!/usr/bin/env bun

import { mkdirSync, readFileSync, realpathSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";

import { findRepoRoot } from "./helpers/run_root.sh.ts";
import { resolveShellRcTarget, shellLabel } from "./helpers/shell_rc.sh.ts";

const START_MARKER = "# >>> dark-factory aliases >>>";
const END_MARKER = "# <<< dark-factory aliases <<<";

const dryRun = Bun.argv.includes("--dry-run");

const repoRoot = findRepoRoot(import.meta.dir);
const repoRealPath = realpathSync(repoRoot);
const rcTarget = resolveShellRcTarget();

mkdirSync(dirname(rcTarget.path), { recursive: true });

const existingContent = readFile(rcTarget.path);
const updatedContent = upsertManagedBlock(existingContent, buildAliasBlock(repoRealPath));

if (existingContent === updatedContent) {
  console.log(
    `Sys Install // Shell RC // Already configured (file=${rcTarget.path},shell=${shellLabel(rcTarget)})`,
  );
  process.exit(0);
}

if (dryRun) {
  console.log(`Sys Install // Dry Run // Would update ${rcTarget.path}`);
  console.log("Sys Install // Dry Run // Managed block preview:");
  console.log(buildAliasBlock(repoRealPath));
  process.exit(0);
}

writeFileSync(rcTarget.path, updatedContent, "utf8");

console.log(
  `Sys Install // Shell RC // Updated aliases (file=${rcTarget.path},shell=${shellLabel(rcTarget)})`,
);
console.log(`Sys Install // Next // Run: source ${rcTarget.path}`);

function readFile(path: string): string {
  try {
    return readFileSync(path, "utf8");
  } catch {
    return "";
  }
}

function buildAliasBlock(repoPath: string): string {
  const escapedPath = repoPath.replace(/"/g, '\\"');

  return [
    START_MARKER,
    `export DARKFACTORY_SRC_PATH="${escapedPath}"`,
    "alias dcli='cd \"$DARKFACTORY_SRC_PATH\" && cargo run --release -p dark_cli --'",
    "alias dtui='cd \"$DARKFACTORY_SRC_PATH\" && cargo run --release -p dark_tui --'",
    "alias dark_cli='cd \"$DARKFACTORY_SRC_PATH\" && cargo run --release -p dark_cli --'",
    "alias dark_tui='cd \"$DARKFACTORY_SRC_PATH\" && cargo run --release -p dark_tui --'",
    END_MARKER,
  ].join("\n");
}

function upsertManagedBlock(content: string, block: string): string {
  const escapedStart = escapeRegex(START_MARKER);
  const escapedEnd = escapeRegex(END_MARKER);
  const blockPattern = new RegExp(`${escapedStart}[\\s\\S]*?${escapedEnd}\\n*`, "m");

  if (blockPattern.test(content)) {
    const replaced = content.replace(blockPattern, `${block}\n`);
    return normalizeTrailingNewline(replaced);
  }

  if (!content.trim()) {
    return `${block}\n`;
  }

  const normalized = normalizeTrailingNewline(content);
  return `${normalized}\n${block}\n`;
}

function normalizeTrailingNewline(value: string): string {
  return `${value.replace(/\s*$/, "")}\n`;
}

function escapeRegex(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
