#!/usr/bin/env bun
import { dirname, join, resolve } from "node:path";
import { existsSync } from "node:fs";

export function findRepoRoot(startDir: string = import.meta.dir): string {
  let current = resolve(startDir);

  while (true) {
    if (existsSync(join(current, ".git"))) {
      return current;
    }

    const parent = dirname(current);
    if (parent === current) {
      throw new Error(`Could not find repository root from: ${startDir}`);
    }

    current = parent;
  }
}
