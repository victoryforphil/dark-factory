#!/usr/bin/env bun

import { findRepoRoot } from "./helpers/run_root.sh.ts";

const repoRoot = findRepoRoot(import.meta.dir);
const launchCwd = process.cwd();
const args = Bun.argv.slice(2);
const manifestPath = `${repoRoot}/frontends/dark_tui/Cargo.toml`;

const childProcess = Bun.spawn(
  ["cargo", "run", "--quiet", "--manifest-path", manifestPath, "--", ...args],
  {
    cwd: launchCwd,
    stdin: "inherit",
    stdout: "inherit",
    stderr: "inherit",
  },
);

const exitCode = await childProcess.exited;
if (exitCode !== 0) {
  process.exit(exitCode);
}
