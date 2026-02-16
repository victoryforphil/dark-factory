#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import {
  buildPageArtifacts,
  buildSnapshotIndexMarkdown,
  buildSnapshotPageMarkdown,
  dedupeSorted,
  discoverUrlsFromSitemap,
  extractLinkedUrls,
  fetchText,
  fileStemFromSourcePath,
  mapConcurrent,
  outputDirectoryRelative,
  prepareOutputDirectory,
  resolveOutputDirectory,
  scrapeDocsPage,
  sourcePathFromUrl,
  summarizeArtifacts,
} from "../helpers/docs_scrape.sh.ts";
import { findRepoRoot } from "../helpers/run_root.sh.ts";

type DiscoveryMethod = "sitemap" | "llms";

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
  const sitemapUrls = await discoverUrlsFromSitemap(SITEMAP_URL, normalizeUrl);
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

function fileStemFromUrl(url: string): string {
  const sourcePath = sourcePathFromUrl(url);
  return fileStemFromSourcePath(sourcePath, {
    rootPath: "/",
    rootStem: "docs",
    stripMarkdownExtension: true,
  });
}

const repoRoot = findRepoRoot(import.meta.dir);
const outputArg = Bun.argv[2];
const outputDir = resolveOutputDirectory(repoRoot, DEFAULT_OUTPUT_DIR_RELATIVE, outputArg);
const outputDirRelative = outputDirectoryRelative(repoRoot, outputDir);

prepareOutputDirectory(outputDir);

const discovery = await discoverDocsUrls(repoRoot);
if (discovery.urls.length === 0) {
  throw new Error(`No docs URLs found for source ${SOURCE_KEY} (sitemap=${SITEMAP_URL},llms=${LLMS_URL})`);
}

const scrapeResults = await mapConcurrent(discovery.urls, CONCURRENCY, (url, index) =>
  scrapeDocsPage(url, index + 1, discovery.urls.length),
);

const artifacts = buildPageArtifacts(
  scrapeResults,
  (result) => fileStemFromUrl(result.url),
  (result) => sourcePathFromUrl(result.url),
);

const capturedAt = new Date().toISOString();

for (const artifact of artifacts) {
  await Bun.write(
    resolve(outputDir, artifact.fileName),
    buildSnapshotPageMarkdown(artifact, capturedAt, {
      sourceKey: SOURCE_KEY,
      sourceRoot: DOCS_ROOT,
      baseKeywords: "elysiajs, docs, bun, typescript",
      summaryFallback: "Elysia documentation page snapshot.",
      collectionMethodNote: "sitemap-first discovery with llms fallback support.",
    }),
  );
}

await Bun.write(
  resolve(outputDir, "index.ext.md"),
  buildSnapshotIndexMarkdown(artifacts, capturedAt, {
    sourceKey: SOURCE_KEY,
    sourceRoot: DOCS_ROOT,
    outputDirRelative,
    discovery,
    scopeText: `${artifacts.length} pages`,
    indexKeywords: "elysiajs, docs index, bun, typescript",
    indexSummary: "This index links one `.ext.md` file per docs page snapshot for ElysiaJS.",
    notes: [
      "- Per-page files are flattened in this directory and prefixed with `docs` in the filename stem.",
      "- Regenerate by re-running the scraper script; old `.ext.md` files in this directory are replaced.",
    ],
  }),
);

const stats = summarizeArtifacts(artifacts);
console.log(
  `Docs // Scrape // Wrote split external docs (source=${SOURCE_KEY},discovery=${discovery.method},pages=${artifacts.length},ok=${stats.successCount},failed=${stats.failureCount},blocked=${stats.blockedPages.join(",") || "none"},dir=${outputDir})`,
);
