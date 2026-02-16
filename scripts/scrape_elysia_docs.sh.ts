#!/usr/bin/env bun

import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { findRepoRoot } from "./helpers/run_root.sh.ts";

type ScrapeMode = "jina" | "html";
type DiscoveryMethod = "sitemap" | "llms";

type ScrapeResult = {
  url: string;
  ok: boolean;
  markdown?: string;
  error?: string;
  mode?: ScrapeMode;
};

type PageArtifact = ScrapeResult & {
  fileName: string;
  sourcePath: string;
};

type DiscoveryResult = {
  urls: string[];
  method: DiscoveryMethod;
  detail: string;
};

const SOURCE_KEY = "elysia";
const DOCS_ROOT = "https://elysiajs.com/";
const ROOT_HOST = new URL(DOCS_ROOT).hostname;
const SITEMAP_URL = "https://elysiajs.com/sitemap.xml";
const LLMS_URL = "https://elysiajs.com/llms.txt";
const LOCAL_CONTEXT_RELATIVE = "docs/context/elysia-js.llms.txt";
const DEFAULT_OUTPUT_DIR_RELATIVE = "docs/external/elysia";
const CONCURRENCY = 4;
const SUMMARY_MAX_CHARS = 240;

function decodeEntities(value: string): string {
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

function stripJinaEnvelope(markdown: string): string {
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

function htmlToMarkdown(html: string): string {
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

function sanitizeSegment(value: string): string {
  const clean = decodeURIComponent(value)
    .toLowerCase()
    .replace(/\.md$/i, "")
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");

  return clean || "page";
}

function sourcePathFromUrl(url: string): string {
  const pathname = new URL(url).pathname.replace(/\/+$/, "");
  return pathname || "/";
}

function fileStemFromUrl(url: string): string {
  const sourcePath = sourcePathFromUrl(url);
  if (sourcePath === "/") {
    return "docs";
  }

  const segments = sourcePath
    .split("/")
    .filter(Boolean)
    .map(sanitizeSegment);

  if (segments.length === 0) {
    return "docs";
  }

  return ["docs", ...segments].join("__");
}

function extractSummary(markdown: string): string {
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

    if (line.length > SUMMARY_MAX_CHARS) {
      return `${line.slice(0, SUMMARY_MAX_CHARS - 3)}...`;
    }

    return line;
  }

  return "Elysia documentation page snapshot.";
}

function keywordsForPath(sourcePath: string): string {
  const dynamic = sourcePath
    .split("/")
    .filter(Boolean)
    .map((part) => part.replace(/[^a-zA-Z0-9]+/g, " "))
    .filter(Boolean)
    .join(", ");

  const base = "elysiajs, docs, bun, typescript";
  return dynamic ? `${base}, ${dynamic}` : base;
}

function buildPageMarkdown(result: PageArtifact, capturedAt: string): string {
  const pageSummary = result.ok
    ? extractSummary(result.markdown ?? "")
    : `Scrape failed: ${result.error ?? "unknown error"}`;

  const pageBody = result.ok
    ? result.markdown?.trim() ?? ""
    : `> Scrape failed: ${result.error ?? "unknown error"}`;

  return [
    "----",
    "## External Docs Snapshot // elysia",
    "",
    `- Captured: ${capturedAt}`,
    `- Source root: ${DOCS_ROOT}`,
    `- Source page: ${result.sourcePath}`,
    `- Keywords: ${keywordsForPath(result.sourcePath)}`,
    `- Summary: ${pageSummary}`,
    "----",
    "",
    `Source: ${result.url}`,
    "",
    pageBody,
    "",
    "----",
    "## Notes / Comments / Lessons",
    "",
    "- Collection method: sitemap-first discovery with llms fallback support.",
    `- Conversion path: ${result.mode === "html" ? "direct HTML fallback parser" : "r.jina.ai markdown proxy"}.`,
    "- This file is one page-level external snapshot in markdown `.ext.md` format.",
    "----",
    "",
  ].join("\n");
}

function buildIndexMarkdown(
  artifacts: PageArtifact[],
  capturedAt: string,
  outputDirRelative: string,
  discovery: DiscoveryResult,
): string {
  const successCount = artifacts.filter((artifact) => artifact.ok).length;
  const failureCount = artifacts.length - successCount;

  const pageList = artifacts
    .map((artifact) => {
      const status = artifact.ok ? "ok" : "failed";
      const mode = artifact.mode ?? "unknown";
      return `- [${artifact.fileName}](./${artifact.fileName}) - ${artifact.sourcePath} (${status},mode=${mode})`;
    })
    .join("\n");

  return [
    "----",
    "## External Docs Index // elysia",
    "",
    `- Captured: ${capturedAt}`,
    `- Source root: ${DOCS_ROOT}`,
    `- Output directory: ${outputDirRelative}`,
    `- Discovery: ${discovery.method} (${discovery.detail})`,
    `- Scope: ${artifacts.length} pages`,
    "- Keywords: elysiajs, docs index, bun, typescript",
    "- Summary: This index links one `.ext.md` file per docs page snapshot for ElysiaJS.",
    "----",
    "",
    "## Pages",
    "",
    pageList,
    "",
    "----",
    "## Notes / Comments / Lessons",
    "",
    "- Per-page files are flattened in this directory and prefixed with `docs` in the filename stem.",
    "- Regenerate by re-running the scraper script; old `.ext.md` files in this directory are replaced.",
    `- Capture results: success=${successCount}, failed=${failureCount}.`,
    "----",
    "",
  ].join("\n");
}

async function fetchText(url: string): Promise<string> {
  const response = await fetch(url, {
    headers: {
      "user-agent": "dark-factory-doc-scraper/1.0",
    },
  });

  if (!response.ok) {
    throw new Error(`Request failed (${response.status}) for ${url}`);
  }

  return await response.text();
}

function extractLocUrls(xml: string): string[] {
  return Array.from(xml.matchAll(/<loc>(.*?)<\/loc>/g), (match) => decodeEntities(match[1]?.trim() ?? "")).filter(Boolean);
}

function extractLinkedUrls(markdown: string): string[] {
  return Array.from(markdown.matchAll(/\((https?:\/\/[^)\s]+)\)/g), (match) => match[1]?.trim() ?? "").filter(Boolean);
}

function normalizeUrl(url: string): string | null {
  try {
    const parsed = new URL(url);
    if (parsed.hostname !== ROOT_HOST) {
      return null;
    }

    parsed.hash = "";
    parsed.search = "";

    if (parsed.pathname !== "/") {
      parsed.pathname = parsed.pathname.replace(/\/+$/, "") || "/";
    }

    return isLikelyDocsPath(parsed.pathname) ? parsed.toString() : null;
  } catch {
    return null;
  }
}

function isLikelyDocsPath(pathname: string): boolean {
  const lower = pathname.toLowerCase();

  if (lower === "/") {
    return true;
  }

  if (
    lower.startsWith("/assets/") ||
    lower.startsWith("/_astro/") ||
    lower.startsWith("/images/") ||
    lower.startsWith("/favicon")
  ) {
    return false;
  }

  if (/\.(png|jpg|jpeg|gif|svg|webp|ico|css|js|map|xml|json|txt|pdf|zip)$/i.test(lower)) {
    return false;
  }

  return true;
}

function dedupeSorted(urls: string[]): string[] {
  return Array.from(new Set(urls)).sort((a, b) => a.localeCompare(b));
}

async function discoverDocsUrlsFromSitemap(sitemapUrl: string): Promise<string[]> {
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

async function discoverDocsUrlsFromLlms(repoRoot: string): Promise<string[]> {
  const urls: string[] = [];

  try {
    const remote = await fetchText(LLMS_URL);
    urls.push(...extractLinkedUrls(remote));
  } catch {
    // Continue to local fallback.
  }

  const localContextPath = resolve(repoRoot, LOCAL_CONTEXT_RELATIVE);
  if (existsSync(localContextPath)) {
    const localContext = readFileSync(localContextPath, "utf-8");
    urls.push(...extractLinkedUrls(localContext));
  }

  return dedupeSorted(urls.map((url) => normalizeUrl(url)).filter((url): url is string => Boolean(url)));
}

async function discoverDocsUrls(repoRoot: string): Promise<DiscoveryResult> {
  const sitemapUrls = await discoverDocsUrlsFromSitemap(SITEMAP_URL);
  const llmsUrls = await discoverDocsUrlsFromLlms(repoRoot);

  if (sitemapUrls.length > 0) {
    const merged = dedupeSorted([...sitemapUrls, ...llmsUrls]);
    return {
      urls: merged,
      method: "sitemap",
      detail: `sitemap_primary=${sitemapUrls.length},llms_supplement=${llmsUrls.length}`,
    };
  }

  return {
    urls: llmsUrls,
    method: "llms",
    detail: `llms_fallback=${llmsUrls.length}`,
  };
}

async function mapConcurrent<T, R>(
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

async function scrapeDocsPage(url: string, count: number, total: number): Promise<ScrapeResult> {
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

function resolveOutputDirectory(repoRoot: string, outputArg?: string): string {
  if (!outputArg) {
    return resolve(repoRoot, DEFAULT_OUTPUT_DIR_RELATIVE);
  }

  const resolved = resolve(repoRoot, outputArg);
  if (resolved.endsWith(".md")) {
    return dirname(resolved);
  }

  return resolved;
}

function clearExistingExtFiles(outputDir: string): void {
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

const repoRoot = findRepoRoot(import.meta.dir);
const outputArg = Bun.argv[2];
const outputDir = resolveOutputDirectory(repoRoot, outputArg);
const outputDirRelative = outputDir.startsWith(repoRoot) ? outputDir.slice(repoRoot.length + 1) : outputDir;

mkdirSync(outputDir, { recursive: true });
clearExistingExtFiles(outputDir);

const discovery = await discoverDocsUrls(repoRoot);
if (discovery.urls.length === 0) {
  throw new Error(`No docs URLs found for source ${SOURCE_KEY} (sitemap=${SITEMAP_URL},llms=${LLMS_URL})`);
}

const scrapeResults = await mapConcurrent(discovery.urls, CONCURRENCY, (url, index) =>
  scrapeDocsPage(url, index + 1, discovery.urls.length),
);

const usedNames = new Map<string, number>();
const artifacts: PageArtifact[] = scrapeResults.map((result) => {
  const stem = fileStemFromUrl(result.url);
  const seen = usedNames.get(stem) ?? 0;
  usedNames.set(stem, seen + 1);

  const suffix = seen === 0 ? "" : `__${seen + 1}`;
  const fileName = `${stem}${suffix}.ext.md`;

  return {
    ...result,
    fileName,
    sourcePath: sourcePathFromUrl(result.url),
  };
});

const capturedAt = new Date().toISOString();

for (const artifact of artifacts) {
  await Bun.write(resolve(outputDir, artifact.fileName), buildPageMarkdown(artifact, capturedAt));
}

await Bun.write(resolve(outputDir, "index.ext.md"), buildIndexMarkdown(artifacts, capturedAt, outputDirRelative, discovery));

const successCount = artifacts.filter((artifact) => artifact.ok).length;
const failureCount = artifacts.length - successCount;
const blockedPages = artifacts
  .filter((artifact) => !artifact.ok)
  .map((artifact) => artifact.sourcePath)
  .join(",");

console.log(
  `Docs // Scrape // Wrote split external docs (source=${SOURCE_KEY},discovery=${discovery.method},pages=${artifacts.length},ok=${successCount},failed=${failureCount},blocked=${blockedPages || "none"},dir=${outputDir})`,
);
