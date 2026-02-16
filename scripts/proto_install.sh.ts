#!/usr/bin/env bun
import { $ } from "bun";
import { findRepoRoot } from "./helpers/run_root.sh.ts";

const repoRoot = findRepoRoot(import.meta.dir);
await $`proto install`.cwd(repoRoot);
