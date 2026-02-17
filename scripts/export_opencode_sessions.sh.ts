#!/usr/bin/env bun

import { randomUUID } from "node:crypto";
import { mkdir, rm, writeFile } from "node:fs/promises";
import { basename, isAbsolute, join, relative } from "node:path";
import { findRepoRoot } from "./helpers/run_root.sh.ts";

type SessionListItem = {
  id: string;
  title?: string;
  directory?: string;
  created?: number;
  updated?: number;
};

type SessionExport = {
  info: {
    id: string;
    title?: string;
    directory?: string;
    slug?: string;
    version?: string;
    projectID?: string;
    summary?: {
      additions?: number;
      deletions?: number;
      files?: number;
    };
    time?: {
      created?: number;
      updated?: number;
    };
  };
  messages: SessionMessage[];
};

type SessionMessage = {
  info?: {
    id?: string;
    role?: string;
    sessionID?: string;
    parentID?: string;
    agent?: string;
    mode?: string;
    modelID?: string;
    providerID?: string;
    finish?: string;
    time?: {
      created?: number;
      completed?: number;
    };
    summary?: {
      diffs?: MessageDiff[];
    };
  };
  parts?: SessionPart[];
};

type SessionPart =
  | { type: "text"; text?: string }
  | { type: "reasoning"; text?: string }
  | {
      type: "tool";
      tool?: string;
      callID?: string;
      state?: {
        status?: string;
        input?: unknown;
        output?: unknown;
      };
    }
  | { type: "patch"; hash?: string; files?: string[] }
  | { type: "agent"; name?: string; source?: { value?: string } }
  | { type: "file"; mime?: string; filename?: string; url?: string }
  | { type: string; [key: string]: unknown };

type MessageDiff = {
  file: string;
  before?: string;
  after?: string;
  additions?: number;
  deletions?: number;
  status?: string;
};

type ScriptOptions = {
  count: number;
  outputDir: string;
  maxToolOutputChars: number;
};

type ExportTreeNode = {
  sessionId: string;
  title: string;
  role: "main" | "subagent";
  relativeDir: string;
  childTaskSessionIds: string[];
  children: ExportTreeNode[];
};

const DEFAULT_COUNT = 10;
const DEFAULT_OUTPUT_DIR = "docs/chats/opencode";
const DEFAULT_MAX_TOOL_OUTPUT_CHARS = 20_000;

const repoRoot = findRepoRoot(import.meta.dir);
const options = parseArgs(Bun.argv.slice(2));

const outputRoot = isAbsolute(options.outputDir)
  ? options.outputDir
  : join(repoRoot, options.outputDir);

await rm(outputRoot, { recursive: true, force: true });
await mkdir(outputRoot, { recursive: true });

const sessionList = await listSessions(Math.max(options.count * 3, options.count));
const filtered = sessionList
  .filter((session) => !session.directory || session.directory === repoRoot)
  .slice(0, options.count);

if (filtered.length === 0) {
  throw new Error("Export Sessions // Script // No sessions available for this repository");
}

const exportTimestamp = new Date().toISOString();
const sessionCache = new Map<string, SessionExport>();

for (const [index, session] of filtered.entries()) {
  const folderName = buildTopLevelFolderName(index + 1, session.id, session.title, session.updated);
  const sessionDir = join(outputRoot, folderName);
  await rm(sessionDir, { recursive: true, force: true });
  await mkdir(sessionDir, { recursive: true });

  const seenInTree = new Set<string>();
  const tree = await exportSessionTree({
    sessionId: session.id,
    role: "main",
    sessionDir,
    sessionCache,
    seenInTree,
  });

  const readmePath = join(sessionDir, "README.md");
  await writeFile(readmePath, renderSessionReadme(tree, exportTimestamp), "utf8");
  await writeFile(join(sessionDir, "tree.json"), `${JSON.stringify(tree, null, 2)}\n`, "utf8");
}

console.log(
  `Export Sessions // Script // Completed (count=${filtered.length},output=${relative(repoRoot, outputRoot)})`,
);

function parseArgs(argv: string[]): ScriptOptions {
  const options: ScriptOptions = {
    count: DEFAULT_COUNT,
    outputDir: DEFAULT_OUTPUT_DIR,
    maxToolOutputChars: DEFAULT_MAX_TOOL_OUTPUT_CHARS,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const token = argv[index];

    if (token === "--count" || token === "-n") {
      options.count = parsePositiveInt(argv[index + 1], token);
      index += 1;
      continue;
    }

    if (token === "--output-dir" || token === "-o") {
      const value = argv[index + 1];
      if (!value) {
        throw new Error(`Export Sessions // Script // Missing value (${token})`);
      }
      options.outputDir = value;
      index += 1;
      continue;
    }

    if (token === "--max-tool-output-chars") {
      options.maxToolOutputChars = parsePositiveInt(argv[index + 1], token);
      index += 1;
      continue;
    }

    throw new Error(`Export Sessions // Script // Unknown flag (${token})`);
  }

  return options;
}

function parsePositiveInt(raw: string | undefined, flag: string): number {
  if (!raw) {
    throw new Error(`Export Sessions // Script // Missing value (${flag})`);
  }

  const parsed = Number.parseInt(raw, 10);
  if (!Number.isFinite(parsed) || parsed <= 0) {
    throw new Error(`Export Sessions // Script // Invalid positive integer (${flag},value=${raw})`);
  }

  return parsed;
}

async function listSessions(maxCount: number): Promise<SessionListItem[]> {
  const result = await runCapture("opencode", [
    "session",
    "list",
    "--max-count",
    String(maxCount),
    "--format",
    "json",
  ]);

  if (result.exitCode !== 0) {
    throw new Error(
      `Export Sessions // Script // Failed to list sessions (exit=${result.exitCode})`,
    );
  }

  try {
    const parsed = JSON.parse(result.stdout) as SessionListItem[];
    return parsed.filter((item) => Boolean(item.id));
  } catch {
    throw new Error("Export Sessions // Script // Failed to parse session list JSON");
  }
}

async function exportSessionTree(input: {
  sessionId: string;
  role: "main" | "subagent";
  sessionDir: string;
  sessionCache: Map<string, SessionExport>;
  seenInTree: Set<string>;
}): Promise<ExportTreeNode> {
  const { sessionId, role, sessionDir, sessionCache, seenInTree } = input;

  if (seenInTree.has(sessionId)) {
    return {
      sessionId,
      title: "Cycle detected",
      role,
      relativeDir: relative(repoRoot, sessionDir),
      childTaskSessionIds: [],
      children: [],
    };
  }

  seenInTree.add(sessionId);

  const exported = await loadSessionExport(sessionId, sessionCache);
  await writeFile(join(sessionDir, "session.export.json"), `${JSON.stringify(exported, null, 2)}\n`, "utf8");

  const childTaskSessionIds = collectTaskSessionIds(exported.messages);
  await writeFile(
    join(sessionDir, "metadata.json"),
    `${JSON.stringify(
      {
        sessionId,
        role,
        title: exported.info.title ?? "Untitled",
        childTaskSessionIds,
      },
      null,
      2,
    )}\n`,
    "utf8",
  );

  const diffsDir = join(sessionDir, "diffs");
  await mkdir(diffsDir, { recursive: true });
  await writeMessageDiffFiles(exported, diffsDir);

  const transcript = renderSessionMarkdown(exported, {
    maxToolOutputChars: options.maxToolOutputChars,
    relativeSessionDir: relative(repoRoot, sessionDir),
    childTaskSessionIds,
  });
  await writeFile(join(sessionDir, "session.md"), transcript, "utf8");

  const children: ExportTreeNode[] = [];
  if (childTaskSessionIds.length > 0) {
    const childRoot = join(sessionDir, "subagents");
    await mkdir(childRoot, { recursive: true });

    for (const [index, childSessionId] of childTaskSessionIds.entries()) {
      const childExport = await loadSessionExport(childSessionId, sessionCache);
      const childFolderName = buildChildFolderName(index + 1, childSessionId, childExport.info.title);
      const childDir = join(childRoot, childFolderName);
      await mkdir(childDir, { recursive: true });

      const childNode = await exportSessionTree({
        sessionId: childSessionId,
        role: "subagent",
        sessionDir: childDir,
        sessionCache,
        seenInTree,
      });
      children.push(childNode);
    }
  }

  return {
    sessionId,
    title: exported.info.title ?? "Untitled",
    role,
    relativeDir: relative(repoRoot, sessionDir),
    childTaskSessionIds,
    children,
  };
}

async function loadSessionExport(
  sessionId: string,
  sessionCache: Map<string, SessionExport>,
): Promise<SessionExport> {
  const cached = sessionCache.get(sessionId);
  if (cached) {
    return cached;
  }

  const result = await runCapture("opencode", ["export", sessionId]);
  if (result.exitCode !== 0) {
    throw new Error(
      `Export Sessions // Script // Failed to export session (id=${sessionId},exit=${result.exitCode})`,
    );
  }

  const jsonStart = result.stdout.indexOf("{");
  if (jsonStart < 0) {
    throw new Error(
      `Export Sessions // Script // Missing JSON payload in export output (id=${sessionId})`,
    );
  }

  let parsed: SessionExport;
  try {
    parsed = JSON.parse(result.stdout.slice(jsonStart)) as SessionExport;
  } catch {
    throw new Error(
      `Export Sessions // Script // Failed to parse export JSON (id=${sessionId})`,
    );
  }

  sessionCache.set(sessionId, parsed);
  return parsed;
}

function collectTaskSessionIds(messages: SessionMessage[]): string[] {
  const sessionIds = new Set<string>();

  for (const message of messages) {
    for (const part of message.parts ?? []) {
      if (part.type !== "tool" || part.tool !== "task") {
        continue;
      }

      const output = part.state?.output;
      if (typeof output === "string") {
        for (const match of output.matchAll(/task_id:\s*(ses_[A-Za-z0-9]+)/g)) {
          const sessionId = match[1];
          if (sessionId) {
            sessionIds.add(sessionId);
          }
        }
      }
    }
  }

  return [...sessionIds];
}

async function writeMessageDiffFiles(exported: SessionExport, diffsDir: string): Promise<void> {
  let diffCount = 0;

  for (const [messageIndex, message] of exported.messages.entries()) {
    const diffs = message.info?.summary?.diffs;
    if (!Array.isArray(diffs) || diffs.length === 0) {
      continue;
    }

    for (const [diffIndex, diff] of diffs.entries()) {
      diffCount += 1;
      const safeFileName = sanitizeForFile(basename(diff.file) || diff.file || "change");
      const targetPath = join(
        diffsDir,
        `${String(messageIndex + 1).padStart(3, "0")}_${String(diffIndex + 1).padStart(2, "0")}_${safeFileName}.md`,
      );

      const diffText = await renderUnifiedDiff(diff.file, diff.before ?? "", diff.after ?? "");
      const lines = [
        `# Diff ${diffCount}`,
        "",
        `- Message Index: ${messageIndex + 1}`,
        `- Message ID: ${message.info?.id ?? "unknown"}`,
        `- Role: ${message.info?.role ?? "unknown"}`,
        `- File: ${diff.file}`,
        `- Status: ${diff.status ?? "unknown"}`,
        `- Additions: ${diff.additions ?? 0}`,
        `- Deletions: ${diff.deletions ?? 0}`,
        "",
        "## Unified Diff",
        "```diff",
        diffText.length > 0 ? diffText : "(no content)",
        "```",
        "",
      ];

      await writeFile(targetPath, lines.join("\n"), "utf8");
    }
  }
}

function renderSessionMarkdown(
  exported: SessionExport,
  context: {
    maxToolOutputChars: number;
    relativeSessionDir: string;
    childTaskSessionIds: string[];
  },
): string {
  const lines: string[] = [];
  lines.push(`# Session ${exported.info.id}`);
  lines.push("");
  lines.push(`- Title: ${exported.info.title ?? "Untitled"}`);
  lines.push(`- Directory: ${exported.info.directory ?? "unknown"}`);
  lines.push(`- Export Directory: ${context.relativeSessionDir}`);
  lines.push(`- Version: ${exported.info.version ?? "unknown"}`);
  lines.push(`- Messages: ${exported.messages.length}`);
  lines.push(`- Subagent Sessions: ${context.childTaskSessionIds.length}`);
  lines.push(`- Created: ${formatUnixMs(exported.info.time?.created)}`);
  lines.push(`- Updated: ${formatUnixMs(exported.info.time?.updated)}`);
  lines.push("");

  if (context.childTaskSessionIds.length > 0) {
    lines.push("## Subagent Session IDs");
    lines.push("");
    for (const id of context.childTaskSessionIds) {
      lines.push(`- ${id}`);
    }
    lines.push("");
  }

  lines.push("## Messages");
  lines.push("");

  for (const [messageIndex, message] of exported.messages.entries()) {
    const role = message.info?.role ?? "unknown";
    const messageID = message.info?.id ?? "unknown";
    lines.push(`### ${String(messageIndex + 1).padStart(3, "0")} ${role}`);
    lines.push("");
    lines.push(`- Message ID: ${messageID}`);
    lines.push(`- Session ID: ${message.info?.sessionID ?? exported.info.id}`);
    lines.push(`- Created: ${formatUnixMs(message.info?.time?.created)}`);
    if (message.info?.time?.completed) {
      lines.push(`- Completed: ${formatUnixMs(message.info.time.completed)}`);
    }
    if (message.info?.agent) {
      lines.push(`- Agent: ${message.info.agent}`);
    }
    if (message.info?.modelID || message.info?.providerID) {
      lines.push(
        `- Model: ${message.info?.providerID ?? "unknown"}/${message.info?.modelID ?? "unknown"}`,
      );
    }

    const messageDiffs = message.info?.summary?.diffs ?? [];
    if (messageDiffs.length > 0) {
      lines.push(`- Diffs: ${messageDiffs.length} (see ./diffs)`);
    }
    lines.push("");

    for (const [partIndex, part] of (message.parts ?? []).entries()) {
      lines.push(`#### Part ${partIndex + 1}: ${part.type}`);
      lines.push("");
      if (part.type === "text") {
        lines.push(part.text?.trim() ? part.text.trim() : "(empty)");
        lines.push("");
        continue;
      }

      if (part.type === "reasoning") {
        lines.push("```text");
        lines.push(part.text?.trim() ? part.text.trim() : "(empty)");
        lines.push("```");
        lines.push("");
        continue;
      }

      if (part.type === "tool") {
        lines.push(`- Tool: ${part.tool ?? "unknown"}`);
        lines.push(`- Status: ${part.state?.status ?? "unknown"}`);
        if (part.callID) {
          lines.push(`- Call ID: ${part.callID}`);
        }
        lines.push("");
        lines.push("Input:");
        lines.push("```json");
        lines.push(stringifyMaybe(part.state?.input));
        lines.push("```");
        lines.push("");
        lines.push("Output:");
        lines.push("```text");
        lines.push(
          clipText(
            typeof part.state?.output === "string"
              ? part.state.output
              : stringifyMaybe(part.state?.output),
            context.maxToolOutputChars,
          ),
        );
        lines.push("```");
        lines.push("");
        continue;
      }

      if (part.type === "patch") {
        lines.push(`- Hash: ${part.hash ?? "unknown"}`);
        if (part.files && part.files.length > 0) {
          lines.push("- Files:");
          for (const file of part.files) {
            lines.push(`  - ${file}`);
          }
        }
        lines.push("");
        continue;
      }

      if (part.type === "agent") {
        lines.push(`- Agent Name: ${part.name ?? "unknown"}`);
        lines.push(`- Source: ${part.source?.value ?? "unknown"}`);
        lines.push("");
        continue;
      }

      if (part.type === "file") {
        lines.push(`- File Name: ${part.filename ?? "unknown"}`);
        lines.push(`- Mime: ${part.mime ?? "unknown"}`);
        lines.push(`- URL: ${part.url ?? "unknown"}`);
        lines.push("");
        continue;
      }

      lines.push("```json");
      lines.push(stringifyMaybe(part));
      lines.push("```");
      lines.push("");
    }
  }

  return `${lines.join("\n")}\n`;
}

function renderSessionReadme(tree: ExportTreeNode, exportedAt: string): string {
  const lines: string[] = [];
  lines.push(`# OpenCode Session Export ${tree.sessionId}`);
  lines.push("");
  lines.push(`- Exported At: ${exportedAt}`);
  lines.push(`- Root Session: ${tree.sessionId}`);
  lines.push(`- Root Title: ${tree.title}`);
  lines.push(`- Root Role: ${tree.role}`);
  lines.push(`- Root Transcript: ${tree.relativeDir}/session.md`);
  lines.push("- Root JSON: ./session.export.json");
  lines.push("- Root Diffs: ./diffs");
  lines.push("");
  lines.push("## Session Tree");
  lines.push("");
  renderTreeLines(tree, lines, 0);
  lines.push("");
  return `${lines.join("\n")}`;
}

function renderTreeLines(node: ExportTreeNode, lines: string[], depth: number): void {
  const indent = "  ".repeat(depth);
  lines.push(`${indent}- ${node.role}: ${node.sessionId} (${node.title})`);
  for (const child of node.children) {
    renderTreeLines(child, lines, depth + 1);
  }
}

async function renderUnifiedDiff(filePath: string, before: string, after: string): Promise<string> {
  if (before === after) {
    return "";
  }

  const tempDir = join(Bun.env.TMPDIR ?? "/tmp", `opencode-session-export-${randomUUID()}`);
  const beforePath = join(tempDir, "before.txt");
  const afterPath = join(tempDir, "after.txt");

  await mkdir(tempDir, { recursive: true });
  await writeFile(beforePath, before, "utf8");
  await writeFile(afterPath, after, "utf8");

  try {
    const result = await runCapture("git", [
      "--no-pager",
      "diff",
      "--no-index",
      "--",
      beforePath,
      afterPath,
    ]);

    if (result.exitCode !== 0 && result.exitCode !== 1) {
      return [
        `--- ${filePath} (before)`,
        `+++ ${filePath} (after)`,
        `@@ error @@`,
        `git diff failed (exit=${result.exitCode})`,
      ].join("\n");
    }

    const sanitized = result.stdout
      .replaceAll(beforePath, `${filePath}.before`)
      .replaceAll(afterPath, `${filePath}.after`)
      .trim();

    return sanitized;
  } finally {
    await rm(tempDir, { recursive: true, force: true });
  }
}

async function runCapture(
  command: string,
  args: string[],
): Promise<{ exitCode: number; stdout: string; stderr: string }> {
  const process = Bun.spawn([command, ...args], {
    cwd: repoRoot,
    stdout: "pipe",
    stderr: "pipe",
  });

  const stdout = process.stdout ? await new Response(process.stdout).text() : "";
  const stderr = process.stderr ? await new Response(process.stderr).text() : "";
  const exitCode = await process.exited;

  return { exitCode, stdout, stderr };
}

function stringifyMaybe(value: unknown): string {
  if (typeof value === "string") {
    return value;
  }

  if (value === undefined) {
    return "undefined";
  }

  try {
    return JSON.stringify(value, null, 2);
  } catch {
    return "[unserializable]";
  }
}

function clipText(value: string, maxChars: number): string {
  if (value.length <= maxChars) {
    return value;
  }

  return `${value.slice(0, maxChars)}\n... [truncated ${value.length - maxChars} chars]`;
}

function sanitizeForFile(value: string): string {
  const trimmed = value.trim().toLowerCase();
  const replaced = trimmed.replace(/[^a-z0-9._-]+/g, "-");
  const normalized = replaced.replace(/-+/g, "-").replace(/^-|-$/g, "");
  return normalized.length > 0 ? normalized.slice(0, 80) : "item";
}

function buildTopLevelFolderName(
  index: number,
  sessionId: string,
  title: string | undefined,
  updatedAt: number | undefined,
): string {
  const rank = String(index).padStart(2, "0");
  const safeDate = formatDateForPath(updatedAt ?? Date.now());
  const safeTitle = sanitizeForFile(title ?? "session");
  return `${rank}_${safeDate}_${safeTitle}_${sessionId}`;
}

function buildChildFolderName(index: number, sessionId: string, title: string | undefined): string {
  const rank = String(index).padStart(2, "0");
  const safeTitle = sanitizeForFile(title ?? "subagent");
  return `${rank}_${safeTitle}_${sessionId}`;
}

function formatDateForPath(unixMs: number): string {
  const iso = new Date(unixMs).toISOString();
  return iso.replace(/[-:]/g, "").replace(/\.\d{3}Z$/, "Z");
}

function formatUnixMs(unixMs: number | undefined): string {
  if (!unixMs) {
    return "unknown";
  }
  return new Date(unixMs).toISOString();
}
