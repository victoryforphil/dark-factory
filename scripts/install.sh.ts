#!/usr/bin/env bun

import { join } from "node:path";
import { findRepoRoot } from "./helpers/run_root.sh.ts";
import { runCommandSteps } from "./helpers/run_steps.sh.ts";

const repoRoot = findRepoRoot(import.meta.dir);

await runCommandSteps([
  // bash <(curl -fsSL https://moonrepo.dev/install/proto.sh) 1.2.3 --yes
  {
    name: "Install proto",
    command: "bash",
    args: ["-c", "curl -fsSL https://moonrepo.dev/install/proto.sh | bash"],
    cwd: repoRoot,
  },
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
  // Install OpenCode
  // curl -fsSL https://opencode.ai/install | bash
  {
    name: "Install OpenCode",
    command: "bash",
    args: ["-c", "curl -fsSL https://opencode.ai/install | bash"],
    cwd: repoRoot,
  }
]);
