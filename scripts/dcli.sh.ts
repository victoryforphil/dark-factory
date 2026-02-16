#!/usr/bin/env bun

import { findRepoRoot } from "./helpers/run_root.sh.ts";

const repoRoot = findRepoRoot(import.meta.dir);
const args = Bun.argv.slice(2);

const process = Bun.spawn(
  ["cargo", "run", "--quiet", "--manifest-path", "frontends/dark_cli/Cargo.toml", "--", ...args],
  {
    cwd: repoRoot,
    stdin: "inherit",
    stdout: "inherit",
    stderr: "inherit",
  },
);

const exitCode = await process.exited;
if (exitCode !== 0) {
  throw new Error(`CLI // Script // Failed (exit=${exitCode})`);
}
