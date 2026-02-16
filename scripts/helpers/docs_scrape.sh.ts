#!/usr/bin/env bun

import { existsSync, mkdirSync, readdirSync, rmSync } from "node:fs";
import { dirname, resolve } from "node:path";

export type ScrapeMode = "jina" | "html";

export type ScrapeResult = {
  url: string;
  ok: boolean;
  markdown?: string;
  error?: string;
  mode?: ScrapeMode;
};

export type PageArtifact = ScrapeResult & {
  fileName: string;
  sourcePath: string;
};

export type DiscoveryResult<TMethod extends string = string> = {
  urls: string[];
  method: TMethod;
  detail: string;
};

const DEFAULT_USER_AGENT = "dark-factory-doc-scraper/1.0";

function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

export function decodeEntities(value: string): string {
  return value
    .replaceAll("&amp;", "&")
    .replaceAll("&quot;", '"')
    .replaceAll("&apos;", "'")
    .replaceAll("&#x27;", "'")
    .replaceAll("&#39;", "'")
    .replaceAll("&nbsp;", " ")
    .replaceAll("&lt;", "<")
    .replaceAll("&gt;", ">");
}

export function stripJinaEnvelope(markdown: string): string {
  return markdown.replace(/^Title:.*?\nURL Source:.*?\nMarkdown Content:\n/s, "").trim();
}

function stripTags(value: string): string {
  return value.replace(/<[^>]+>/g, "");
}

function plainTextFromHtml(value: string): string {
  return decodeEntities(stripTags(value).replace(/\s+/g, " ")).trim();
}

function extractMainHtml(html: string): string {
  const articleMatch = html.match(/<article[^>]*>([\s\S]*?)<\/article>/i);
  if (articleMatch?.[1]) {
    return articleMatch[1];
  }

  const mainMatch = html.match(/<main[^>]*>([\s\S]*?)<\/main>/i);
  if (mainMatch?.[1]) {
    return mainMatch[1];
  }

  return html;
}

export function htmlToMarkdown(html: string): string {
  let content = extractMainHtml(html);

  content = content
    .replace(/<script[\s\S]*?<\/script>/gi, "")
    .replace(/<style[\s\S]*?<\/style>/gi, "")
    .replace(/<noscript[\s\S]*?<\/noscript>/gi, "")
    .replace(/<svg[\s\S]*?<\/svg>/gi, "")
    .replace(/<button[^>]*>[\s\S]*?<\/button>/gi, "");

  content = content.replace(/<pre[^>]*><code[^>]*>([\s\S]*?)<\/code><\/pre>/gi, (_match, code) => {
    const normalizedCode = decodeEntities(
      stripTags(code)
        .replace(/<br\s*\/?>/gi, "\n")
        .replace(/\n{3,}/g, "\n\n")
        .trim(),
    );

    if (!normalizedCode) {
      return "";
    }

    return `\n\n\`\`\`\n${normalizedCode}\n\`\`\`\n\n`;
  });

  content = content.replace(/<code[^>]*>([\s\S]*?)<\/code>/gi, (_match, code) => {
    const normalizedCode = decodeEntities(stripTags(code)).trim();
    return normalizedCode ? `\`${normalizedCode}\`` : "";
  });

  content = content.replace(/<a[^>]*href="([^"]+)"[^>]*>([\s\S]*?)<\/a>/gi, (_match, href, label) => {
    const text = plainTextFromHtml(label);
    if (!text) {
      return "";
    }

    return `[${text}](${decodeEntities(href)})`;
  });

  content = content.replace(/<h([1-6])[^>]*>([\s\S]*?)<\/h\1>/gi, (_match, level, inner) => {
    const heading = plainTextFromHtml(inner);
    if (!heading) {
      return "";
    }

    return `\n\n${"#".repeat(Number(level))} ${heading}\n\n`;
  });

  content = content.replace(/<li[^>]*>([\s\S]*?)<\/li>/gi, (_match, inner) => {
    const line = plainTextFromHtml(inner);
    return line ? `- ${line}\n` : "";
  });

  content = content
    .replace(/<br\s*\/?>/gi, "\n")
    .replace(/<\/(p|div|section|article|header|ul|ol|table|tr)>/gi, "\n\n")
    .replace(/<(p|div|section|article|header|ul|ol|table|tr)[^>]*>/gi, "\n")
    .replace(/<t[hd][^>]*>([\s\S]*?)<\/t[hd]>/gi, (_match, inner) => {
      const cell = plainTextFromHtml(inner);
      return cell ? `${cell} ` : "";
    });

  return decodeEntities(content)
    .replace(/<[^>]+>/g, "")
    .replace(/[ \t]+\n/g, "\n")
    .replace(/\n{3,}/g, "\n\n")
    .trim();
}

export function sanitizeSegment(value: string, options?: { stripMarkdownExtension?: boolean }): string {
  const stripMarkdownExtension = options?.stripMarkdownExtension ?? false;
  let clean = decodeURIComponent(value).toLowerCase();
  if (stripMarkdownExtension) {
    clean = clean.replace(/\.md$/i, "");
  }

  clean = clean.replace(/[^a-z0-9]+/g, "-").replace(/^-+|-+$/g, "");

  return clean || "page";
}

export function sourcePathFromUrl(url: string, options?: { emptyPathFallback?: string }): string {
  const pathname = new URL(url).pathname.replace(/\/+$/, "");
  if (pathname) {
    return pathname;
  }

  return options?.emptyPathFallback ?? "/";
}

export function fileStemFromSourcePath(
  sourcePath: string,
  options?: {
    prefix?: string;
    rootPath?: string;
    rootStem?: string;
    trimPrefixPath?: string;
    stripMarkdownExtension?: boolean;
  },
): string {
  const prefix = options?.prefix ?? "docs";
  const rootPath = options?.rootPath?.replace(/\/+$/, "");
  const rootStem = options?.rootStem ?? prefix;

  if (rootPath && sourcePath === rootPath) {
    return rootStem;
  }

  let workingPath = sourcePath;
  const trimPrefixPath = options?.trimPrefixPath?.replace(/\/+$/, "");
  if (trimPrefixPath) {
    const pattern = new RegExp(`^${escapeRegExp(trimPrefixPath)}\\/?`);
    workingPath = workingPath.replace(pattern, "");
  }

  const segments = workingPath
    .split("/")
    .filter(Boolean)
    .map((segment) =>
      sanitizeSegment(segment, { stripMarkdownExtension: options?.stripMarkdownExtension ?? false }),
    );

  if (segments.length === 0) {
    return rootStem;
  }

  return [prefix, ...segments].join("__");
}

export function extractSummary(markdown: string, fallback: string, maxChars: number = 240): string {
  const lines = markdown.split("\n");
  let inCodeBlock = false;

  for (const rawLine of lines) {
    const line = rawLine.trim();

    if (line.startsWith("```") || line.startsWith("~~~")) {
      inCodeBlock = !inCodeBlock;
      continue;
    }

    if (inCodeBlock || !line || line.startsWith("#") || line.startsWith("- [")) {
      continue;
    }

    if (line.length <= 25) {
      continue;
    }

    if (line.length > maxChars) {
      return `${line.slice(0, maxChars - 3)}...`;
    }

    return line;
  }

  return fallback;
}

export function keywordsForPath(sourcePath: string, base: string, dropLeadingSegments: number = 0): string {
  const dynamic = sourcePath
    .split("/")
    .filter(Boolean)
    .slice(dropLeadingSegments)
    .map((part) => part.replace(/[^a-zA-Z0-9]+/g, " "))
    .filter(Boolean)
    .join(", ");

  return dynamic ? `${base}, ${dynamic}` : base;
}

export async function fetchText(url: string): Promise<string> {
  const response = await fetch(url, {
    headers: {
      "user-agent": DEFAULT_USER_AGENT,
    },
  });

  if (!response.ok) {
    throw new Error(`Request failed (${response.status}) for ${url}`);
  }

  return await response.text();
}

export function extractLocUrls(xml: string): string[] {
  return Array.from(xml.matchAll(/<loc>(.*?)<\/loc>/g), (match) => decodeEntities(match[1]?.trim() ?? "")).filter(
    Boolean,
  );
}

export function extractHrefUrls(html: string): string[] {
  return Array.from(html.matchAll(/<a[^>]*href="([^"]+)"[^>]*>/gi), (match) => match[1]?.trim() ?? "").filter(Boolean);
}

export function extractLinkedUrls(markdown: string): string[] {
  return Array.from(markdown.matchAll(/\((https?:\/\/[^)\s]+)\)/g), (match) => match[1]?.trim() ?? "").filter(Boolean);
}

export function dedupeSorted(urls: string[]): string[] {
  return Array.from(new Set(urls)).sort((a, b) => a.localeCompare(b));
}

export async function discoverUrlsFromSitemap(
  sitemapUrl: string,
  normalizeUrl: (url: string) => string | null,
): Promise<string[]> {
  const visited = new Set<string>();
  const pending: string[] = [sitemapUrl];
  const docsUrls = new Set<string>();

  while (pending.length > 0) {
    const current = pending.pop();
    if (!current || visited.has(current)) {
      continue;
    }

    visited.add(current);

    let xml: string;
    try {
      xml = await fetchText(current);
    } catch {
      continue;
    }

    const locUrls = extractLocUrls(xml);
    for (const locUrl of locUrls) {
      if (locUrl.endsWith(".xml")) {
        if (!visited.has(locUrl)) {
          pending.push(locUrl);
        }
        continue;
      }

      const normalized = normalizeUrl(locUrl);
      if (normalized) {
        docsUrls.add(normalized);
      }
    }
  }

  return Array.from(docsUrls).sort((a, b) => a.localeCompare(b));
}

export async function discoverUrlsFromRootPage(
  rootUrl: string,
  normalizeUrl: (url: string) => string | null,
): Promise<string[]> {
  try {
    const html = await fetchText(rootUrl);
    const linkedUrls = extractHrefUrls(html)
      .map((url) => normalizeUrl(url))
      .filter((url): url is string => Boolean(url));
    return dedupeSorted([rootUrl, ...linkedUrls]);
  } catch {
    return [rootUrl];
  }
}

export async function mapConcurrent<T, R>(
  values: T[],
  limit: number,
  map: (value: T, index: number) => Promise<R>,
): Promise<R[]> {
  const results = new Array<R>(values.length);
  let index = 0;

  async function worker(): Promise<void> {
    while (true) {
      const currentIndex = index;
      index += 1;

      if (currentIndex >= values.length) {
        return;
      }

      results[currentIndex] = await map(values[currentIndex], currentIndex);
    }
  }

  const workerCount = Math.min(limit, values.length);
  await Promise.all(Array.from({ length: workerCount }, () => worker()));
  return results;
}

export async function scrapeDocsPage(url: string, count: number, total: number): Promise<ScrapeResult> {
  const jinaUrl = `https://r.jina.ai/http://${url.replace(/^https?:\/\//, "")}`;

  try {
    console.log(`Docs // Scrape // ${count}/${total} ${url}`);

    try {
      const markdown = stripJinaEnvelope(await fetchText(jinaUrl));
      if (markdown) {
        return {
          url,
          ok: true,
          markdown,
          mode: "jina",
        };
      }
    } catch {
      // Fall through to direct HTML scrape.
    }

    const html = await fetchText(url);
    const markdown = htmlToMarkdown(html);

    if (!markdown) {
      return {
        url,
        ok: false,
        error: "No markdown content returned from HTML fallback",
      };
    }

    return {
      url,
      ok: true,
      markdown,
      mode: "html",
    };
  } catch (error) {
    return {
      url,
      ok: false,
      error: error instanceof Error ? error.message : "Unknown scrape error",
    };
  }
}

export function resolveOutputDirectory(repoRoot: string, defaultOutputDirRelative: string, outputArg?: string): string {
  if (!outputArg) {
    return resolve(repoRoot, defaultOutputDirRelative);
  }

  const resolved = resolve(repoRoot, outputArg);
  if (resolved.endsWith(".md")) {
    return dirname(resolved);
  }

  return resolved;
}

export function outputDirectoryRelative(repoRoot: string, outputDir: string): string {
  return outputDir.startsWith(repoRoot) ? outputDir.slice(repoRoot.length + 1) : outputDir;
}

export function clearExistingExtFiles(outputDir: string): void {
  if (!existsSync(outputDir)) {
    return;
  }

  for (const entry of readdirSync(outputDir, { withFileTypes: true })) {
    if (!entry.isFile() || !entry.name.endsWith(".ext.md")) {
      continue;
    }

    rmSync(resolve(outputDir, entry.name));
  }
}

export function prepareOutputDirectory(outputDir: string): void {
  mkdirSync(outputDir, { recursive: true });
  clearExistingExtFiles(outputDir);
}

export function buildPageArtifacts(
  scrapeResults: ScrapeResult[],
  getFileStem: (result: ScrapeResult) => string,
  getSourcePath: (result: ScrapeResult) => string,
): PageArtifact[] {
  const usedNames = new Map<string, number>();

  return scrapeResults.map((result) => {
    const stem = getFileStem(result);
    const seen = usedNames.get(stem) ?? 0;
    usedNames.set(stem, seen + 1);

    const suffix = seen === 0 ? "" : `__${seen + 1}`;
    const fileName = `${stem}${suffix}.ext.md`;

    return {
      ...result,
      fileName,
      sourcePath: getSourcePath(result),
    };
  });
}

export type ArtifactStats = {
  successCount: number;
  failureCount: number;
  blockedPages: string[];
};

export type SnapshotPageOptions = {
  sourceKey: string;
  sourceRoot: string;
  baseKeywords: string;
  summaryFallback: string;
  collectionMethodNote: string;
  dropKeywordPathSegments?: number;
};

export type SnapshotIndexOptions = {
  sourceKey: string;
  sourceRoot: string;
  outputDirRelative: string;
  indexKeywords: string;
  indexSummary: string;
  scopeText: string;
  notes: string[];
  discovery?: {
    method: string;
    detail: string;
  };
};

export function summarizeArtifacts(artifacts: PageArtifact[]): ArtifactStats {
  const successCount = artifacts.filter((artifact) => artifact.ok).length;
  const failureCount = artifacts.length - successCount;
  const blockedPages = artifacts.filter((artifact) => !artifact.ok).map((artifact) => artifact.sourcePath);

  return {
    successCount,
    failureCount,
    blockedPages,
  };
}

export function buildSnapshotPageMarkdown(
  artifact: PageArtifact,
  capturedAt: string,
  options: SnapshotPageOptions,
): string {
  const pageSummary = artifact.ok
    ? extractSummary(artifact.markdown ?? "", options.summaryFallback)
    : `Scrape failed: ${artifact.error ?? "unknown error"}`;

  const pageBody = artifact.ok ? artifact.markdown?.trim() ?? "" : `> Scrape failed: ${artifact.error ?? "unknown error"}`;

  return [
    "----",
    `## External Docs Snapshot // ${options.sourceKey}`,
    "",
    `- Captured: ${capturedAt}`,
    `- Source root: ${options.sourceRoot}`,
    `- Source page: ${artifact.sourcePath}`,
    `- Keywords: ${keywordsForPath(artifact.sourcePath, options.baseKeywords, options.dropKeywordPathSegments ?? 0)}`,
    `- Summary: ${pageSummary}`,
    "----",
    "",
    `Source: ${artifact.url}`,
    "",
    pageBody,
    "",
    "----",
    "## Notes / Comments / Lessons",
    "",
    `- Collection method: ${options.collectionMethodNote}`,
    `- Conversion path: ${artifact.mode === "html" ? "direct HTML fallback parser" : "r.jina.ai markdown proxy"}.`,
    "- This file is one page-level external snapshot in markdown `.ext.md` format.",
    "----",
    "",
  ].join("\n");
}

export function buildSnapshotIndexMarkdown(
  artifacts: PageArtifact[],
  capturedAt: string,
  options: SnapshotIndexOptions,
): string {
  const stats = summarizeArtifacts(artifacts);

  const pageList = artifacts
    .map((artifact) => {
      const status = artifact.ok ? "ok" : "failed";
      const mode = artifact.mode ?? "unknown";
      return `- [${artifact.fileName}](./${artifact.fileName}) - ${artifact.sourcePath} (${status},mode=${mode})`;
    })
    .join("\n");

  const discoveryLine = options.discovery
    ? [`- Discovery: ${options.discovery.method} (${options.discovery.detail})`]
    : [];

  return [
    "----",
    `## External Docs Index // ${options.sourceKey}`,
    "",
    `- Captured: ${capturedAt}`,
    `- Source root: ${options.sourceRoot}`,
    `- Output directory: ${options.outputDirRelative}`,
    ...discoveryLine,
    `- Scope: ${options.scopeText}`,
    `- Keywords: ${options.indexKeywords}`,
    `- Summary: ${options.indexSummary}`,
    "----",
    "",
    "## Pages",
    "",
    pageList,
    "",
    "----",
    "## Notes / Comments / Lessons",
    "",
    ...options.notes,
    `- Capture results: success=${stats.successCount}, failed=${stats.failureCount}.`,
    "----",
    "",
  ].join("\n");
}
