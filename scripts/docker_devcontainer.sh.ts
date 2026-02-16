#!/usr/bin/env bun

import { runDockerCompose } from "./helpers/docker_compose.sh.ts";

type Mode = "up" | "attach" | "exec" | "run";

const args = Bun.argv.slice(2);
const mode = parseMode(args[0]);
const rest = mode ? args.slice(1) : args;
const selectedMode = mode ?? "attach";

if (selectedMode === "up") {
  const detached = !rest.includes("--foreground");
  const composeArgs = detached
    ? ["up", "-d", "devcontainer"]
    : ["up", "devcontainer"];
  await runDockerCompose(composeArgs);
  process.exit(0);
}

if (selectedMode === "attach") {
  await runDockerCompose([
    "exec",
    "devcontainer",
    "bash",
    "-lc",
    "if command -v zsh >/dev/null 2>&1; then exec zsh -l; else exec bash -l; fi",
  ]);
  process.exit(0);
}

const dividerIndex = rest.indexOf("--");
const command = dividerIndex >= 0 ? rest.slice(dividerIndex + 1) : rest;
if (command.length === 0) {
  throw new Error("Docker // Devcontainer // Missing exec command");
}

await runDockerCompose(["exec", "devcontainer", ...command]);

function parseMode(value: string | undefined): Mode | null {
  if (value === "up" || value === "attach" || value === "exec" || value === "run") {
    return value;
  }

  return null;
}
