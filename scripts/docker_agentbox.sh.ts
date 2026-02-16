#!/usr/bin/env bun

import { runDockerCompose } from "./helpers/docker_compose.sh.ts";

type Mode = "up" | "attach" | "run" | "exec";

const argv = Bun.argv.slice(2);
const mode = parseMode(argv[0]);
const args = mode ? argv.slice(1) : argv;
const selectedMode = mode ?? "run";

if (selectedMode === "up") {
  await runDockerCompose(["up", "-d", "agentbox"]);
  process.exit(0);
}

if (selectedMode === "attach") {
  await runDockerCompose([
    "exec",
    "agentbox",
    "bash",
    "-lc",
    "if command -v zsh >/dev/null 2>&1; then exec zsh -l; else exec bash -l; fi",
  ]);
  process.exit(0);
}

const noTty = args.includes("--no-tty");
const filtered = args.filter((token) => token !== "--no-tty");
const dividerIndex = filtered.indexOf("--");

const command = dividerIndex >= 0
  ? filtered.slice(dividerIndex + 1)
  : filtered;

if (command.length === 0) {
  throw new Error("Docker // Agentbox // Missing command (use run -- <command>)");
}

await runDockerCompose(["up", "-d", "agentbox"]);

const execArgs = ["exec"];
if (noTty) {
  execArgs.push("-T");
}

execArgs.push("agentbox", ...command);
await runDockerCompose(execArgs);

function parseMode(value: string | undefined): Mode | null {
  if (value === "up" || value === "attach" || value === "run" || value === "exec") {
    return value;
  }

  return null;
}
