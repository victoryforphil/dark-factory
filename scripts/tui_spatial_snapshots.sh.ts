#!/usr/bin/env bun

import { findRepoRoot } from "./helpers/run_root.sh.ts";

const repoRoot = findRepoRoot(import.meta.dir);
const args = Bun.argv.slice(2);

const updateSnapshots = args.includes("--update");
const passthrough = args.filter((arg) => arg !== "--update");

const command = ["cargo", "test", "-p", "dark_tui", "spatial_snapshots", ...passthrough];
const env = {
  ...process.env,
  ...(updateSnapshots ? { INSTA_UPDATE: "always" } : {}),
};

const child = Bun.spawn(command, {
  cwd: repoRoot,
  env,
  stdin: "inherit",
  stdout: "inherit",
  stderr: "inherit",
});

const exitCode = await child.exited;
if (exitCode !== 0) {
  process.exit(exitCode);
}
