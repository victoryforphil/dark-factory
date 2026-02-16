#!/usr/bin/env bun

import { runDockerCompose } from "./helpers/docker_compose.sh.ts";

const requestedTargets = Bun.argv.slice(2);
const targets = requestedTargets.length > 0
  ? requestedTargets
  : ["run", "agentbox", "ci", "devcontainer"];

console.log(`Docker // Build // Targets (${targets.join(",")})`);
await runDockerCompose(["build", ...targets]);
