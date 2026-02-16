#!/usr/bin/env bun

import { runDockerCompose } from "./helpers/docker_compose.sh.ts";

const args = Bun.argv.slice(2);
const commandArgs = args[0] === "run" ? args.slice(1) : args;
const command = commandArgs.length > 0
  ? commandArgs
  : ["moon", "run", "dark_core:test"];

await runDockerCompose(["run", "--rm", "ci", ...command]);
