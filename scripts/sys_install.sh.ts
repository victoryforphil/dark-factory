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
    "_darkfactory_run_with_moon() {",
    "  local moon_task=\"$1\"",
    "  shift",
    "  local binary_path=\"$1\"",
    "  shift",
    "  local moon_verbose=0",
    "  local binary_args=()",
    "  local arg",
    "  local moon_log",
    "  local moon_exit",
    "",
    "  for arg in \"$@\"; do",
    "    if [ \"$arg\" = \"--moon-verbose\" ] || [ \"$arg\" = \"--verbose\" ]; then",
    "      moon_verbose=1",
    "      continue",
    "    fi",
    "",
    "    binary_args+=(\"$arg\")",
    "  done",
    "",
    "  if [ \"$moon_verbose\" -eq 1 ]; then",
    "    (cd \"$DARKFACTORY_SRC_PATH\" && moon run \"$moon_task\") || return $?",
    "  else",
    "    moon_log=$(mktemp -t darkfactory-moon.XXXXXX)",
    "    (cd \"$DARKFACTORY_SRC_PATH\" && moon run --quiet \"$moon_task\" >\"$moon_log\" 2>&1)",
    "    moon_exit=$?",
    "",
    "    if [ \"$moon_exit\" -ne 0 ]; then",
    "      cat \"$moon_log\" 1>&2",
    "      rm -f \"$moon_log\"",
    "      return \"$moon_exit\"",
    "    fi",
    "",
    "    rm -f \"$moon_log\"",
    "  fi",
    "",
    "  if [ ! -x \"$binary_path\" ]; then",
    "    printf 'Sys Install // Error // Expected binary missing (%s)\\n' \"$binary_path\" 1>&2",
    "    return 1",
    "  fi",
    "",
    "  \"$binary_path\" \"${binary_args[@]}\"",
    "}",
    "",
    "dcli() {",
    "  _darkfactory_run_with_moon \"dark_cli:build\" \"$DARKFACTORY_SRC_PATH/target/debug/dark_cli\" \"$@\"",
    "}",
    "",
    "dchat() {",
    "  _darkfactory_run_with_moon \"dark_chat:build\" \"$DARKFACTORY_SRC_PATH/target/debug/dark_chat\" \"$@\"",
    "}",
    "",
    "dtui() {",
    "  _darkfactory_run_with_moon \"dark_tui:build\" \"$DARKFACTORY_SRC_PATH/target/debug/dark_tui\" \"$@\"",
    "}",
    "",
    "alias dark_cli='dcli'",
    "alias dark_chat='dchat'",
    "alias dark_tui='dtui'",
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
