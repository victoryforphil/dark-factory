#!/usr/bin/env bun

import { existsSync } from "node:fs";
import { appendFile, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { findRepoRoot } from "./helpers/run_root.sh.ts";

type Mode = "start" | "run" | "status" | "stop";

type Options = {
  intervalMinutes: number;
  cycles: number;
  scanCount: number;
};

type SessionSummary = {
  id: string;
  title?: string;
  directory?: string;
};

type ReflectState = {
  status: "running" | "completed" | "stopped" | "error";
  pid: number | null;
  startedAt: string;
  updatedAt: string;
  intervalMinutes: number;
  cycles: number;
  scanCount: number;
  completedCycles: number;
  lastReviewedSessionId?: string;
  lastReflectionPath?: string;
  lastError?: string;
  logPath: string;
};

const defaults: Options = {
  intervalMinutes: 5,
  cycles: 3,
  scanCount: 10,
};

const repoRoot = findRepoRoot(import.meta.dir);
const runtimeDir = join(repoRoot, ".opencode", "runtime");
const reflectionsDir = join(repoRoot, "docs", "reflections");
const statePath = join(runtimeDir, "reflect_constant.state.json");
const logPath = join(runtimeDir, "reflect_constant.log");
const reflectModel = Bun.env.REFLECT_CONSTANT_MODEL ?? "openai/gpt-5.3-codex";

const parsed = parseArgs(Bun.argv.slice(2));

switch (parsed.mode) {
  case "start":
    await startLoop(parsed.options);
    break;
  case "run":
    await runLoop(parsed.options);
    break;
  case "status":
    await showStatus();
    break;
  case "stop":
    await stopLoop();
    break;
  default:
    throw new Error(`Reflect Constant // Script // Unsupported mode (${parsed.mode})`);
}

function parseArgs(argv: string[]): { mode: Mode; options: Options } {
  const modeCandidate = argv[0];
  const mode: Mode = isMode(modeCandidate) ? modeCandidate : "run";
  const args = isMode(modeCandidate) ? argv.slice(1) : argv;

  const options: Options = { ...defaults };

  for (let index = 0; index < args.length; index += 1) {
    const token = args[index];

    if (token === "--interval-minutes") {
      options.intervalMinutes = parseNumberFlag(args[index + 1], token);
      index += 1;
      continue;
    }

    if (token === "--cycles") {
      options.cycles = parseNumberFlag(args[index + 1], token);
      index += 1;
      continue;
    }

    if (token === "--scan-count") {
      options.scanCount = parseNumberFlag(args[index + 1], token);
      index += 1;
      continue;
    }

    throw new Error(`Reflect Constant // Script // Unknown flag (${token})`);
  }

  if (options.intervalMinutes <= 0) {
    throw new Error("Reflect Constant // Script // interval-minutes must be > 0");
  }

  if (options.cycles < 0) {
    throw new Error("Reflect Constant // Script // cycles must be >= 0");
  }

  if (options.scanCount <= 0) {
    throw new Error("Reflect Constant // Script // scan-count must be > 0");
  }

  return { mode, options };
}

function isMode(value: string | undefined): value is Mode {
  return value === "start" || value === "run" || value === "status" || value === "stop";
}

function parseNumberFlag(raw: string | undefined, flag: string): number {
  if (!raw) {
    throw new Error(`Reflect Constant // Script // Missing flag value (${flag})`);
  }

  const value = Number(raw);
  if (!Number.isFinite(value)) {
    throw new Error(`Reflect Constant // Script // Invalid numeric flag (${flag},value=${raw})`);
  }

  return value;
}

async function ensureRuntimeDir(): Promise<void> {
  await mkdir(runtimeDir, { recursive: true });
}

async function ensureReflectionsDir(): Promise<void> {
  await mkdir(reflectionsDir, { recursive: true });
}

async function writeState(state: ReflectState): Promise<void> {
  await ensureRuntimeDir();
  await writeFile(statePath, `${JSON.stringify(state, null, 2)}\n`, "utf8");
}

async function readState(): Promise<ReflectState | null> {
  if (!existsSync(statePath)) {
    return null;
  }

  const raw = await readFile(statePath, "utf8");
  return JSON.parse(raw) as ReflectState;
}

async function logLine(message: string): Promise<void> {
  const line = `${new Date().toISOString()} Reflect Constant // ${message}`;
  console.log(line);
  await ensureRuntimeDir();
  await appendFile(logPath, `${line}\n`, "utf8");
}

function isPidAlive(pid: number): boolean {
  try {
    process.kill(pid, 0);
    return true;
  } catch {
    return false;
  }
}

async function startLoop(options: Options): Promise<void> {
  const existing = await readState();
  if (existing?.pid && isPidAlive(existing.pid)) {
    throw new Error(
      `Reflect Constant // Script // Loop already running (pid=${existing.pid})`,
    );
  }

  await ensureRuntimeDir();

  const scriptPath = join(repoRoot, "scripts", "reflect_constant.sh.ts");
  const child = Bun.spawn(
    [
      "bun",
      scriptPath,
      "run",
      "--interval-minutes",
      String(options.intervalMinutes),
      "--cycles",
      String(options.cycles),
      "--scan-count",
      String(options.scanCount),
    ],
    {
      cwd: repoRoot,
      stdin: "ignore",
      stdout: "ignore",
      stderr: "ignore",
      detached: true,
    },
  );

  const startedAt = new Date().toISOString();
  await writeState({
    status: "running",
    pid: child.pid,
    startedAt,
    updatedAt: startedAt,
    intervalMinutes: options.intervalMinutes,
    cycles: options.cycles,
    scanCount: options.scanCount,
    completedCycles: 0,
    logPath,
  });

  console.log(`Reflect Constant // Script // Started (pid=${child.pid},state=${statePath})`);
  console.log(`Reflect Constant // Script // Log (${logPath})`);
}

async function showStatus(): Promise<void> {
  const state = await readState();
  if (!state) {
    console.log("Reflect Constant // Script // No state found");
    return;
  }

  const alive = state.pid ? isPidAlive(state.pid) : false;
  console.log(
    `Reflect Constant // Script // Status (state=${state.status},pid=${state.pid ?? "none"},alive=${alive},completed=${state.completedCycles})`,
  );
  console.log(`Reflect Constant // Script // Updated (${state.updatedAt})`);
  console.log(`Reflect Constant // Script // Log (${state.logPath})`);
  if (state.lastReflectionPath) {
    console.log(`Reflect Constant // Script // Last Reflection (${state.lastReflectionPath})`);
  }
  if (state.lastReviewedSessionId) {
    console.log(
      `Reflect Constant // Script // Last Session (id=${state.lastReviewedSessionId})`,
    );
  }
  if (state.lastError) {
    console.log(`Reflect Constant // Script // Last Error (${state.lastError})`);
  }
}

async function stopLoop(): Promise<void> {
  const state = await readState();
  if (!state?.pid) {
    console.log("Reflect Constant // Script // Nothing to stop");
    return;
  }

  if (!isPidAlive(state.pid)) {
    await writeState({
      ...state,
      status: "stopped",
      pid: null,
      updatedAt: new Date().toISOString(),
    });
    console.log(`Reflect Constant // Script // Process already stopped (pid=${state.pid})`);
    return;
  }

  process.kill(state.pid, "SIGTERM");
  await Bun.sleep(300);

  if (isPidAlive(state.pid)) {
    process.kill(state.pid, "SIGKILL");
  }

  await writeState({
    ...state,
    status: "stopped",
    pid: null,
    updatedAt: new Date().toISOString(),
  });

  console.log(`Reflect Constant // Script // Stopped (pid=${state.pid})`);
}

async function runLoop(options: Options): Promise<void> {
  let stopping = false;

  process.on("SIGTERM", () => {
    stopping = true;
  });

  const startedAt = new Date().toISOString();
  let state: ReflectState = {
    status: "running",
    pid: process.pid,
    startedAt,
    updatedAt: startedAt,
    intervalMinutes: options.intervalMinutes,
    cycles: options.cycles,
    scanCount: options.scanCount,
    completedCycles: 0,
    logPath,
  };
  await writeState(state);
  await logLine(
    `Loop started (pid=${process.pid},intervalMinutes=${options.intervalMinutes},cycles=${options.cycles})`,
  );

  const runIndefinitely = options.cycles === 0;
  const totalCycles = runIndefinitely ? Number.MAX_SAFE_INTEGER : options.cycles;

  for (let cycle = 1; cycle <= totalCycles; cycle += 1) {
    if (stopping) {
      await logLine("Stopping after SIGTERM");
      break;
    }

    await logLine(`Cycle ${cycle} started`);
    const session = await pickReviewSession(options.scanCount);

    if (!session) {
      await logLine(`Cycle ${cycle} skipped (no reviewable session found)`);
    } else {
      const runResult = await runReflectorCycle(cycle, session.id);
      if (runResult.ok) {
        await logLine(`Cycle ${cycle} completed (session=${session.id})`);
        state = {
          ...state,
          completedCycles: cycle,
          lastReviewedSessionId: session.id,
          lastReflectionPath: runResult.reflectionPath,
          updatedAt: new Date().toISOString(),
          lastError: undefined,
        };
      } else {
        await logLine(`Cycle ${cycle} failed (${runResult.error})`);
        state = {
          ...state,
          completedCycles: cycle,
          lastReviewedSessionId: session.id,
          lastReflectionPath: runResult.reflectionPath,
          updatedAt: new Date().toISOString(),
          lastError: runResult.error,
        };
      }
      await writeState(state);
    }

    if (cycle < totalCycles && !stopping) {
      await Bun.sleep(options.intervalMinutes * 60_000);
    }
  }

  const finalStatus: ReflectState["status"] = stopping
    ? "stopped"
    : state.lastError
      ? "error"
      : "completed";
  state = {
    ...state,
    status: finalStatus,
    pid: null,
    updatedAt: new Date().toISOString(),
  };
  await writeState(state);
  await logLine(`Loop finished (status=${finalStatus},completedCycles=${state.completedCycles})`);
}

async function pickReviewSession(scanCount: number): Promise<SessionSummary | null> {
  const result = await runCapture(
    "opencode",
    ["session", "list", "--max-count", String(scanCount), "--format", "json"],
    repoRoot,
  );

  if (result.exitCode !== 0) {
    await logLine(`Failed to list sessions (exit=${result.exitCode})`);
    return null;
  }

  let sessions: SessionSummary[] = [];
  try {
    sessions = JSON.parse(result.stdout) as SessionSummary[];
  } catch {
    await logLine("Failed to parse session list JSON");
    return null;
  }

  const preferred = sessions.find((session) => {
    if (!session.id) {
      return false;
    }

    if (session.directory && session.directory !== repoRoot) {
      return false;
    }

    const title = (session.title ?? "").toLowerCase();
    return !title.includes("reflect");
  });

  return preferred ?? sessions[0] ?? null;
}

async function runReflectorCycle(
  cycle: number,
  sessionId: string,
): Promise<
  | { ok: true; reflectionPath: string }
  | { ok: false; error: string; reflectionPath: string }
> {
  const exportResult = await runCapture("opencode", ["export", sessionId], repoRoot);
  if (exportResult.exitCode !== 0) {
    const failedPath = await writeReflectionArtifact({
      cycle,
      sessionId,
      status: "failed",
      stdout: "",
      stderr: `Export failed with exit code ${exportResult.exitCode}`,
    });
    return {
      ok: false,
      error: `export failed (session=${sessionId},exit=${exportResult.exitCode})`,
      reflectionPath: failedPath,
    };
  }

  await ensureRuntimeDir();
  const exportPath = join(runtimeDir, `reflect_session_${sessionId}.json`);
  await writeFile(exportPath, exportResult.stdout, "utf8");

  const prompt = [
    "Use @reflector to review the attached session export for reusable process improvements.",
    "Return compact bullets only with sections: Wins (1-3), Misses (0-2), Lessons (0-3), Maintenance updates (0-3).",
    "Lessons must fit docs/lessons/*.lessons.md and maintenance updates should target AGENTS.md or .opencode/* only.",
    "Avoid secrets, design-direction edits, and one-off human style preference changes.",
  ].join(" ");

  const reflectResult = await runCapture(
    "opencode",
    [
      "run",
      "--model",
      reflectModel,
      "--dir",
      repoRoot,
      "--file",
      exportPath,
      "--",
      prompt,
    ],
    repoRoot,
  );

  const failedByOutput = hasReflectorRunError(
    reflectResult.exitCode,
    reflectResult.stdout,
    reflectResult.stderr,
  );

  await appendFile(
    logPath,
    `${new Date().toISOString()} Reflect Constant // Cycle ${cycle} output start\n${reflectResult.stdout}\n${new Date().toISOString()} Reflect Constant // Cycle ${cycle} stderr start\n${reflectResult.stderr}\n${new Date().toISOString()} Reflect Constant // Cycle ${cycle} output end\n`,
    "utf8",
  );

  const reflectionPath = await writeReflectionArtifact({
    cycle,
    sessionId,
    status: failedByOutput ? "failed" : "completed",
    stdout: reflectResult.stdout,
    stderr: reflectResult.stderr,
  });

  try {
    await rm(exportPath, { force: true });
  } catch {
    // no-op
  }

  if (failedByOutput) {
    return {
      ok: false,
      error: `reflector failed (session=${sessionId},exit=${reflectResult.exitCode})`,
      reflectionPath,
    };
  }

  return { ok: true, reflectionPath };
}

function hasReflectorRunError(exitCode: number, stdout: string, stderr: string): boolean {
  if (exitCode !== 0) {
    return true;
  }

  const combined = `${stdout}\n${stderr}`;
  return /(ProviderModelNotFoundError|Model not found:|\bError:\b)/.test(combined);
}

function cleanTerminalOutput(text: string): string {
  return text.replace(/\u001b\[[0-9;]*m/g, "").trim();
}

function formatTimestampForFile(isoTimestamp: string): string {
  return isoTimestamp.replace(/[-:]/g, "").replace(/\.\d{3}Z$/, "Z");
}

async function writeReflectionArtifact(input: {
  cycle: number;
  sessionId: string;
  status: "completed" | "failed";
  stdout: string;
  stderr: string;
}): Promise<string> {
  await ensureReflectionsDir();

  const createdAt = new Date().toISOString();
  const safeTimestamp = formatTimestampForFile(createdAt);
  const reflectionPath = join(
    reflectionsDir,
    `${safeTimestamp}_${input.sessionId}.reflection.md`,
  );

  const cleanedStdout = cleanTerminalOutput(input.stdout);
  const cleanedStderr = cleanTerminalOutput(input.stderr);

  const lines = [
    `- Cycle: ${input.cycle}`,
    `- Session: ${input.sessionId}`,
    `- Status: ${input.status}`,
    `- Generated: ${createdAt}`,
    "",
    "## Reflector Output",
    cleanedStdout.length > 0 ? cleanedStdout : "none",
    "",
    "## Reflector Errors",
    cleanedStderr.length > 0 ? cleanedStderr : "none",
    "",
  ];

  await writeFile(reflectionPath, `${lines.join("\n")}`, "utf8");
  return reflectionPath;
}

async function runCapture(
  command: string,
  args: string[],
  cwd: string,
): Promise<{ exitCode: number; stdout: string; stderr: string }> {
  const process = Bun.spawn([command, ...args], {
    cwd,
    stdout: "pipe",
    stderr: "pipe",
  });

  const stdout = process.stdout
    ? await new Response(process.stdout).text()
    : "";
  const stderr = process.stderr
    ? await new Response(process.stderr).text()
    : "";
  const exitCode = await process.exited;

  return {
    exitCode,
    stdout,
    stderr,
  };
}
