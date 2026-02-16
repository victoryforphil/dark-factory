#!/usr/bin/env bun

import { findRepoRoot } from "./helpers/run_root.sh.ts";

const repoRoot = findRepoRoot(import.meta.dir);
const process = Bun.spawn(["moon", "run", "dark_core:dev"], {
  cwd: repoRoot,
  stdin: "inherit",
  stdout: "inherit",
  stderr: "inherit",
});

const exitCode = await process.exited;
if (exitCode !== 0) {
  throw new Error(`Dev // Script // Failed (exit=${exitCode})`);
}
