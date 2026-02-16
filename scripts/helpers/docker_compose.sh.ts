#!/usr/bin/env bun

import { join } from "node:path";
import { findRepoRoot } from "./run_root.sh.ts";

export const repoRoot = findRepoRoot(import.meta.dir);
export const composeFilePath = join(repoRoot, "docker", "compose.devcontainers.yml");

export async function runDockerCompose(args: string[]): Promise<void> {
  const process = Bun.spawn(["docker", "compose", "-f", composeFilePath, ...args], {
    cwd: repoRoot,
    stdin: "inherit",
    stdout: "inherit",
    stderr: "inherit",
  });

  const exitCode = await process.exited;
  if (exitCode !== 0) {
    throw new Error(`Docker // Compose // Failed (exit=${exitCode},args=${args.join(" ")})`);
  }
}
