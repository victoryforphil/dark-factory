#!/usr/bin/env bun

import { mkdirSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { findRepoRoot } from "./helpers/run_root.sh.ts";

type ScrapeResult = {
  url: string;
  ok: boolean;
  markdown?: string;
  error?: string;
};

const SITEMAP_URL = "https://moonrepo.dev/sitemap.xml";
const DOCS_ROOT = "https://moonrepo.dev/docs";
const DEFAULT_OUTPUT_RELATIVE =
  "docs/external/moonrepo/moonrepo_docs.ext.md";
const CONCURRENCY = 4;

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
  return markdown
    .replace(/^Title:.*?\nURL Source:.*?\nMarkdown Content:\n/s, "")
    .trim();
}

function stripTags(value: string): string {
  return value.replace(/<[^>]+>/g, "");
}

function plainTextFromHtml(value: string): string {
  return decodeEntities(stripTags(value).replace(/\s+/g, " ")).trim();
}

function extractMainHtml(html: string): string {
  const markdownBlockMatch = html.match(
    /<div class="theme-doc-markdown markdown">([\s\S]*?)<footer class="theme-doc-footer/s,
  );

  if (markdownBlockMatch?.[1]) {
    return markdownBlockMatch[1];
  }

  const articleMatch = html.match(/<article>([\s\S]*?)<\/article>/s);
  if (articleMatch?.[1]) {
    return articleMatch[1];
  }

  const mainMatch = html.match(/<main[^>]*>([\s\S]*?)<\/main>/s);
  return mainMatch?.[1] ?? html;
}

function htmlToMarkdown(html: string): string {
  let content = extractMainHtml(html);

  content = content
    .replace(/<script[\s\S]*?<\/script>/gi, "")
    .replace(/<style[\s\S]*?<\/style>/gi, "")
    .replace(/<a[^>]*class="hash-link"[^>]*>[\s\S]*?<\/a>/gi, "")
    .replace(/<span[^>]*aria-hidden="true"[^>]*>[\s\S]*?<\/span>/gi, "");

  content = content.replace(
    /<pre[^>]*><code[^>]*>([\s\S]*?)<\/code><\/pre>/gi,
    (_match, code) => {
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
    },
  );

  content = content.replace(/<code[^>]*>([\s\S]*?)<\/code>/gi, (_match, code) => {
    const normalizedCode = decodeEntities(stripTags(code)).trim();
    return normalizedCode ? `\`${normalizedCode}\`` : "";
  });

  content = content.replace(/<a[^>]*href="([^"]+)"[^>]*>([\s\S]*?)<\/a>/gi, (_m, href, label) => {
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

  content = decodeEntities(content)
    .replace(/<[^>]+>/g, "")
    .replace(/[ \t]+\n/g, "\n")
    .replace(/\n{3,}/g, "\n\n")
    .trim();

  return content;
}

function extractDocsUrls(sitemapXml: string): string[] {
  const urls = Array.from(sitemapXml.matchAll(/<loc>(.*?)<\/loc>/g), (match) =>
    decodeEntities(match[1]?.trim() ?? ""),
  )
    .filter((url) => url.startsWith(DOCS_ROOT))
    .filter((url) => !url.includes("/docs/tags"))
    .filter((url) => !url.includes("#"));

  return Array.from(new Set(urls)).sort((a, b) => a.localeCompare(b));
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
    };
  } catch (error) {
    return {
      url,
      ok: false,
      error: error instanceof Error ? error.message : "Unknown scrape error",
    };
  }
}

function buildHeader(urlCount: number, capturedAt: string): string {
  return [
    "----",
    "## External Docs Snapshot // moonrepo",
    "",
    `- Captured: ${capturedAt}`,
    `- Source root: ${DOCS_ROOT}`,
    `- Scope: ${urlCount} pages under /docs (excluding /docs/tags)`,
    "- Keywords: moon, monorepo, task graph, hashing, cache, toolchain, proto, workspace, codegen, query, migration, CI",
    "- Summary: moonrepo documentation centers on deterministic task orchestration, project/dependency graphs, and reproducible toolchains with proto-backed version pinning.",
    "----",
    "",
  ].join("\n");
}

function buildFooter(successCount: number, failureCount: number): string {
  return [
    "",
    "----",
    "## Notes / Comments / Lessons",
    "",
    "- Collection method: sitemap URL discovery + markdown conversion (r.jina.ai primary, direct HTML fallback parser secondary).",
    "- Most docs pages are content-rich and command-oriented; command reference and config schemas are the highest-density sections.",
    "- For ongoing updates, re-run this script to refresh snapshots and compare diffs over time.",
    `- Capture results: success=${successCount}, failed=${failureCount}.`,
    "----",
    "",
  ].join("\n");
}

function buildPageSection(result: ScrapeResult): string {
  if (!result.ok) {
    return [
      `## ${result.url.replace("https://moonrepo.dev", "")}`,
      "",
      `Source: ${result.url}`,
      "",
      `> Scrape failed: ${result.error ?? "unknown error"}`,
      "",
    ].join("\n");
  }

  return [
    `## ${result.url.replace("https://moonrepo.dev", "")}`,
    "",
    `Source: ${result.url}`,
    "",
    result.markdown?.trim() ?? "",
    "",
  ].join("\n");
}

const repoRoot = findRepoRoot(import.meta.dir);
const outputArg = Bun.argv[2];
const outputPath = resolve(repoRoot, outputArg ?? DEFAULT_OUTPUT_RELATIVE);

const sitemapXml = await fetchText(SITEMAP_URL);
const docsUrls = extractDocsUrls(sitemapXml);

if (docsUrls.length === 0) {
  throw new Error("No docs URLs found in moonrepo sitemap");
}

const results = await mapConcurrent(docsUrls, CONCURRENCY, (url, index) =>
  scrapeDocsPage(url, index + 1, docsUrls.length),
);

const successCount = results.filter((result) => result.ok).length;
const failureCount = results.length - successCount;
const capturedAt = new Date().toISOString();

const content = [
  buildHeader(docsUrls.length, capturedAt),
  ...results.map(buildPageSection),
  buildFooter(successCount, failureCount),
].join("\n");

mkdirSync(dirname(outputPath), { recursive: true });
await Bun.write(outputPath, content);

console.log(
  `Docs // Scrape // Wrote external docs snapshot (pages=${docsUrls.length},ok=${successCount},failed=${failureCount},path=${outputPath})`,
);
