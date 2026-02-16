#!/usr/bin/env bun

import { join } from "node:path";
import { findRepoRoot } from "./helpers/run_root.sh.ts";
import { runCommandSteps } from "./helpers/run_steps.sh.ts";

const repoRoot = findRepoRoot(import.meta.dir);

await runCommandSteps([
 
]);
