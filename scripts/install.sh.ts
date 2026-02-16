#!/usr/bin/env bun

import { join } from "node:path";
import { findRepoRoot } from "./helpers/run_root.sh.ts";
import { runCommandSteps } from "./helpers/run_steps.sh.ts";

const repoRoot = findRepoRoot(import.meta.dir);

await runCommandSteps([
  {
    name: "Install proto toolchain",
    command: "proto",
    args: ["install"],
    cwd: repoRoot,
  },
  {
    name: "Install dark_core dependencies",
    command: "bun",
    args: ["install", "--cwd", join(repoRoot, "dark_core")],
    cwd: repoRoot,
  },
  {
    name: "Install workspace projects",
    command: "moon",
    args: [":install"],
    cwd: repoRoot,
  },
]);
