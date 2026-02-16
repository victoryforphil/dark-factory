#!/usr/bin/env bun

import { existsSync, mkdirSync, readdirSync, rmSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { findRepoRoot } from "./helpers/run_root.sh.ts";

type SessionPart = {
  type?: string;
  text?: string;
  synthetic?: boolean;
};

type SessionMessage = {
  info?: {
    role?: string;
    time?: {
      created?: number;
      completed?: number;
    };
  };
  parts?: SessionPart[];
};

type ExportedSession = {
  info?: {
    id?: string;
    slug?: string;
    title?: string;
    directory?: string;
    version?: string;
    time?: {
      created?: number;
      updated?: number;
    };
  };
  messages?: SessionMessage[];
};

type SessionArtifact = {
  id: string;
  title: string;
  slug: string;
  createdAt: string;
  updatedAt: string;
  messageCount: number;
  markdownFile: string;
  jsonFile: string;
};

const SOURCE_KEY = "opencode";
const DEFAULT_OUTPUT_DIR_RELATIVE = "docs/chats/opencode";

function sanitizeSegment(value: string): string {
  const clean = value
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");

  return clean || "session";
}

function toIsoTime(value?: number): string {
  if (!value) {
    return "unknown";
  }

  try {
    return new Date(value).toISOString();
  } catch {
    return "unknown";
  }
}

function resolveOutputDirectory(repoRoot: string, outputArg?: string): string {
  if (!outputArg) {
    return resolve(repoRoot, DEFAULT_OUTPUT_DIR_RELATIVE);
  }

  const resolved = resolve(repoRoot, outputArg);
  if (resolved.endsWith(".md") || resolved.endsWith(".json")) {
    return dirname(resolved);
  }

  return resolved;
}

function clearExistingExports(outputDir: string): void {
  if (!existsSync(outputDir)) {
    return;
  }

  for (const entry of readdirSync(outputDir, { withFileTypes: true })) {
    if (!entry.isFile()) {
      continue;
    }

    if (!entry.name.startsWith("session__") && entry.name !== "index.md") {
      continue;
    }

    if (!entry.name.endsWith(".md") && !entry.name.endsWith(".json")) {
      continue;
    }

    rmSync(resolve(outputDir, entry.name));
  }
}

async function runCommand(command: string[], cwd: string): Promise<{ stdout: string; stderr: string; exitCode: number }> {
  const process = Bun.spawn(command, {
    cwd,
    stdout: "pipe",
    stderr: "pipe",
  });

  const [stdout, stderr, exitCode] = await Promise.all([
    new Response(process.stdout).text(),
    new Response(process.stderr).text(),
    process.exited,
  ]);

  return {
    stdout,
    stderr,
    exitCode,
  };
}

function extractSessionIds(listOutput: string): string[] {
  const ids = Array.from(listOutput.matchAll(/\bses_[A-Za-z0-9]+\b/g), (match) => match[0]);
  return Array.from(new Set(ids));
}

function parseExportedSession(rawOutput: string, sessionID: string): ExportedSession {
  const start = rawOutput.indexOf("{");
  if (start < 0) {
    throw new Error(`Chat // Export // Missing JSON payload (session=${sessionID})`);
  }

  const payload = rawOutput.slice(start);

  try {
    return JSON.parse(payload) as ExportedSession;
  } catch (error) {
    throw new Error(
      `Chat // Export // Invalid JSON payload (session=${sessionID},error=${error instanceof Error ? error.message : "unknown"})`,
    );
  }
}

function buildSessionMarkdown(data: ExportedSession): string {
  const sessionID = data.info?.id ?? "unknown";
  const sessionTitle = data.info?.title ?? "Untitled session";
  const sessionSlug = data.info?.slug ?? "unknown";
  const sessionDirectory = data.info?.directory ?? "unknown";
  const sessionVersion = data.info?.version ?? "unknown";
  const createdAt = toIsoTime(data.info?.time?.created);
  const updatedAt = toIsoTime(data.info?.time?.updated);
  const messages = data.messages ?? [];

  const lines: string[] = [
    "----",
    "## OpenCode Chat Export",
    "",
    `- Session ID: ${sessionID}`,
    `- Title: ${sessionTitle}`,
    `- Slug: ${sessionSlug}`,
    `- Project directory: ${sessionDirectory}`,
    `- OpenCode version: ${sessionVersion}`,
    `- Created: ${createdAt}`,
    `- Updated: ${updatedAt}`,
    `- Message count: ${messages.length}`,
    "----",
    "",
    "## Conversation",
    "",
  ];

  for (const [index, message] of messages.entries()) {
    const role = message.info?.role ?? "unknown";
    const messageCreatedAt = toIsoTime(message.info?.time?.created);
    const textParts = (message.parts ?? [])
      .filter((part) => part.type === "text")
      .map((part) => part.text?.trim() ?? "")
      .filter(Boolean);

    lines.push(`### ${index + 1}. ${role} (${messageCreatedAt})`);
    lines.push("");

    if (textParts.length === 0) {
      lines.push("_No text content captured for this message._");
      lines.push("");
      continue;
    }

    for (const text of textParts) {
      lines.push(text);
      lines.push("");
    }
  }

  lines.push("----");
  lines.push("## Notes");
  lines.push("");
  lines.push("- Export source: `opencode export <sessionID>` JSON payload.");
  lines.push("- This markdown keeps text parts only; use the sibling `.json` file for full structured data.");
  lines.push("----");
  lines.push("");

  return lines.join("\n");
}

function buildIndexMarkdown(
  artifacts: SessionArtifact[],
  capturedAt: string,
  outputDirRelative: string,
): string {
  const pageList = artifacts
    .map(
      (artifact) =>
        `- [${artifact.markdownFile}](./${artifact.markdownFile}) | [json](./${artifact.jsonFile}) - ${artifact.title} (id=${artifact.id},messages=${artifact.messageCount},updated=${artifact.updatedAt})`,
    )
    .join("\n");

  return [
    "----",
    "## OpenCode Chat Export Index",
    "",
    `- Captured: ${capturedAt}`,
    `- Source tool: ${SOURCE_KEY}`,
    `- Output directory: ${outputDirRelative}`,
    `- Scope: ${artifacts.length} project sessions`,
    "----",
    "",
    "## Sessions",
    "",
    pageList || "- No sessions found for this project.",
    "",
    "----",
    "## Notes",
    "",
    "- Re-run this script to refresh all exported session files.",
    "- Existing `session__*.md` and `session__*.json` files in this directory are replaced.",
    "----",
    "",
  ].join("\n");
}

const repoRoot = findRepoRoot(import.meta.dir);
const outputArg = Bun.argv[2];
const outputDir = resolveOutputDirectory(repoRoot, outputArg);
const outputDirRelative = outputDir.startsWith(repoRoot)
  ? outputDir.slice(repoRoot.length + 1)
  : outputDir;

mkdirSync(outputDir, { recursive: true });
clearExistingExports(outputDir);

const listResult = await runCommand(["opencode", "session", "list"], repoRoot);
if (listResult.exitCode !== 0) {
  throw new Error(
    `Chat // Export // Failed to list sessions (exit=${listResult.exitCode},stderr=${listResult.stderr.trim() || "none"})`,
  );
}

const sessionIDs = extractSessionIds(listResult.stdout);

if (sessionIDs.length === 0) {
  const emptyIndex = buildIndexMarkdown([], new Date().toISOString(), outputDirRelative);
  await Bun.write(resolve(outputDir, "index.md"), emptyIndex);
  console.log(
    `Chat // Export // No sessions found for project (dir=${repoRoot},out=${outputDir})`,
  );
  process.exit(0);
}

const artifacts: SessionArtifact[] = [];
for (const [index, sessionID] of sessionIDs.entries()) {
  console.log(`Chat // Export // ${index + 1}/${sessionIDs.length} ${sessionID}`);

  const exportResult = await runCommand(["opencode", "export", sessionID], repoRoot);
  if (exportResult.exitCode !== 0) {
    throw new Error(
      `Chat // Export // Failed to export session (id=${sessionID},exit=${exportResult.exitCode},stderr=${exportResult.stderr.trim() || "none"})`,
    );
  }

  const exported = parseExportedSession(exportResult.stdout, sessionID);
  const title = exported.info?.title?.trim() || `Session ${sessionID}`;
  const slug = exported.info?.slug?.trim() || "unknown";
  const createdAt = toIsoTime(exported.info?.time?.created);
  const updatedAt = toIsoTime(exported.info?.time?.updated);
  const safeSlug = sanitizeSegment(slug);
  const fileStem = `session__${safeSlug}__${sessionID}`;
  const markdownFile = `${fileStem}.md`;
  const jsonFile = `${fileStem}.json`;
  const markdownContent = buildSessionMarkdown(exported);

  await Bun.write(resolve(outputDir, jsonFile), `${JSON.stringify(exported, null, 2)}\n`);
  await Bun.write(resolve(outputDir, markdownFile), markdownContent);

  artifacts.push({
    id: sessionID,
    title,
    slug,
    createdAt,
    updatedAt,
    messageCount: exported.messages?.length ?? 0,
    markdownFile,
    jsonFile,
  });
}

const capturedAt = new Date().toISOString();
const indexContent = buildIndexMarkdown(artifacts, capturedAt, outputDirRelative);
await Bun.write(resolve(outputDir, "index.md"), indexContent);

console.log(
  `Chat // Export // Wrote project session exports (sessions=${artifacts.length},dir=${outputDir})`,
);
