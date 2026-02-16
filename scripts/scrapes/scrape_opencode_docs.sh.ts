#!/usr/bin/env bun

import { resolve } from "node:path";
import {
  buildPageArtifacts,
  buildSnapshotIndexMarkdown,
  buildSnapshotPageMarkdown,
  discoverUrlsFromSitemap,
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

const SOURCE_KEY = "opencode";
const DOCS_ROOT = "https://opencode.ai/docs";
const ROOT_HOST = new URL(DOCS_ROOT).hostname;
const SITEMAP_URL = "https://opencode.ai/sitemap.xml";
const DEFAULT_OUTPUT_DIR_RELATIVE = "docs/external/opencode";
const CONCURRENCY = 4;
const DEFAULT_LANGUAGE = "en";

function isEnglishDocsUrl(url: string): boolean {
  const pathname = new URL(url).pathname;
  if (!pathname.startsWith("/docs")) {
    return false;
  }

  const relative = pathname.replace(/^\/docs\/?/, "");
  if (!relative) {
    return true;
  }

  const firstSegment = relative.split("/")[0]?.toLowerCase() ?? "";
  if (!firstSegment) {
    return true;
  }

  if (firstSegment === "en" || firstSegment === "english") {
    return true;
  }

  if (/^[a-z]{2}(?:-[a-z]{2})?$/.test(firstSegment)) {
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
    parsed.pathname = parsed.pathname.replace(/\/+$/, "") || "/";

    if (!parsed.pathname.startsWith("/docs")) {
      return null;
    }

    return parsed.toString();
  } catch {
    return null;
  }
}

function fileStemFromUrl(url: string): string {
  const sourcePath = sourcePathFromUrl(url, { emptyPathFallback: "/docs" });
  return fileStemFromSourcePath(sourcePath, {
    rootPath: "/docs",
    rootStem: "docs",
    trimPrefixPath: "/docs",
  });
}

const repoRoot = findRepoRoot(import.meta.dir);
const outputArg = Bun.argv[2];
const outputDir = resolveOutputDirectory(repoRoot, DEFAULT_OUTPUT_DIR_RELATIVE, outputArg);
const outputDirRelative = outputDirectoryRelative(repoRoot, outputDir);

prepareOutputDirectory(outputDir);

const requestedLanguage = (Bun.env.DOCS_LANGUAGE ?? DEFAULT_LANGUAGE).toLowerCase();
const discoveredUrls = await discoverUrlsFromSitemap(SITEMAP_URL, normalizeUrl);
const docsUrls =
  requestedLanguage === "en" || requestedLanguage === "english"
    ? discoveredUrls.filter(isEnglishDocsUrl)
    : discoveredUrls;

if (docsUrls.length === 0) {
  throw new Error(`No docs URLs found from sitemap discovery (${SITEMAP_URL})`);
}

const scrapeResults = await mapConcurrent(docsUrls, CONCURRENCY, (url, index) =>
  scrapeDocsPage(url, index + 1, docsUrls.length),
);

const artifacts = buildPageArtifacts(
  scrapeResults,
  (result) => fileStemFromUrl(result.url),
  (result) => sourcePathFromUrl(result.url, { emptyPathFallback: "/docs" }),
);

const capturedAt = new Date().toISOString();

for (const artifact of artifacts) {
  await Bun.write(
    resolve(outputDir, artifact.fileName),
    buildSnapshotPageMarkdown(artifact, capturedAt, {
      sourceKey: SOURCE_KEY,
      sourceRoot: DOCS_ROOT,
      baseKeywords: "opencode, docs, ai coding assistant, cli",
      summaryFallback: "OpenCode documentation page snapshot.",
      collectionMethodNote: "sitemap discovery + markdown conversion.",
      dropKeywordPathSegments: 1,
    }),
  );
}

await Bun.write(
  resolve(outputDir, "index.ext.md"),
  buildSnapshotIndexMarkdown(artifacts, capturedAt, {
    sourceKey: SOURCE_KEY,
    sourceRoot: DOCS_ROOT,
    outputDirRelative,
    scopeText: `${artifacts.length} pages under /docs`,
    indexKeywords: "opencode, docs index, ai coding assistant, cli",
    indexSummary: "This index links one `.ext.md` file per docs page snapshot for OpenCode.",
    notes: [
      "- Per-page files are flattened in this directory and prefixed with `docs` in the filename stem.",
      "- Regenerate by re-running the scraper script; old `.ext.md` files in this directory are replaced.",
    ],
  }),
);

const stats = summarizeArtifacts(artifacts);
console.log(
  `Docs // Scrape // Wrote split external docs (source=${SOURCE_KEY},lang=${requestedLanguage},pages=${artifacts.length},ok=${stats.successCount},failed=${stats.failureCount},blocked=${stats.blockedPages.join(",") || "none"},dir=${outputDir})`,
);
