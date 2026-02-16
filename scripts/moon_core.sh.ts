#!/usr/bin/env bun

import { $ } from "bun";
import { findRepoRoot } from "./helpers/run_root.sh.ts";

// Always run moon from workspace root so project targeting works reliably.
const repoRoot = findRepoRoot(import.meta.dir);

// Default to a non-watch task for quick verification, but allow overrides:
//   bun scripts/moon_core.sh.ts dev
//   bun scripts/moon_core.sh.ts start
const task = Bun.argv[2] ?? "start";
const target = `core:${task}`;

console.log(`Moon // Core // Running target (${target})`);
await $`moon run ${target}`.cwd(repoRoot);
