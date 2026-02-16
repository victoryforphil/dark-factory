#!/usr/bin/env bun

import { resolve } from "node:path";
import {
  buildPageArtifacts,
  buildSnapshotIndexMarkdown,
  buildSnapshotPageMarkdown,
  dedupeSorted,
  discoverUrlsFromRootPage,
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

type DiscoveryMethod = "sitemap" | "html";

type DiscoveryResult = {
  urls: string[];
  method: DiscoveryMethod;
  detail: string;
};

const SOURCE_KEY = "ratatui_web";
const DOCS_ROOT = "https://ratatui.rs/";
const ROOT_URL = new URL(DOCS_ROOT);
const ROOT_HOST = ROOT_URL.hostname;
const ROOT_PATH = ROOT_URL.pathname.replace(/\/+$/, "") || "/";
const SITEMAP_URL = "https://ratatui.rs/sitemap-index.xml";
const DEFAULT_OUTPUT_DIR_RELATIVE = "docs/external/ratatui_web";
const CONCURRENCY = 4;

function normalizeUrl(url: string): string | null {
  try {
    const parsed = new URL(url, DOCS_ROOT);
    if (parsed.hostname !== ROOT_HOST) {
      return null;
    }

    parsed.hash = "";
    parsed.search = "";
    parsed.pathname = parsed.pathname.replace(/\/+$/, "") || "/";

    if (ROOT_PATH !== "/" && parsed.pathname !== ROOT_PATH && !parsed.pathname.startsWith(`${ROOT_PATH}/`)) {
      return null;
    }

    if (/(\.(png|jpg|jpeg|gif|svg|webp|ico|css|js|map|xml|json|txt|pdf|zip|wasm)$)/i.test(parsed.pathname)) {
      return null;
    }

    return parsed.toString();
  } catch {
    return null;
  }
}

async function discoverDocsUrls(): Promise<DiscoveryResult> {
  const sitemapUrls = await discoverUrlsFromSitemap(SITEMAP_URL, normalizeUrl);
  if (sitemapUrls.length > 0) {
    return {
      urls: sitemapUrls,
      method: "sitemap",
      detail: `sitemap_url=${SITEMAP_URL},matched=${sitemapUrls.length}`,
    };
  }

  const htmlUrls = await discoverUrlsFromRootPage(DOCS_ROOT, normalizeUrl);
  return {
    urls: dedupeSorted(htmlUrls),
    method: "html",
    detail: `root_fallback=${htmlUrls.length}`,
  };
}

function fileStemFromUrl(url: string): string {
  const sourcePath = sourcePathFromUrl(url);
  return fileStemFromSourcePath(sourcePath, {
    rootPath: ROOT_PATH,
    rootStem: "docs",
    stripMarkdownExtension: true,
  });
}

const repoRoot = findRepoRoot(import.meta.dir);
const outputArg = Bun.argv[2];
const outputDir = resolveOutputDirectory(repoRoot, DEFAULT_OUTPUT_DIR_RELATIVE, outputArg);
const outputDirRelative = outputDirectoryRelative(repoRoot, outputDir);

prepareOutputDirectory(outputDir);

const discovery = await discoverDocsUrls();
if (discovery.urls.length === 0) {
  throw new Error(`No docs URLs found for source ${SOURCE_KEY} (sitemap=${SITEMAP_URL},root=${DOCS_ROOT})`);
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
      baseKeywords: "ratatui, rust, tui, terminal ui, docs",
      summaryFallback: "Ratatui website documentation page snapshot.",
      collectionMethodNote: "sitemap-index-first discovery with direct HTML fallback support.",
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
    scopeText: `${artifacts.length} pages under ${ROOT_HOST}`,
    indexKeywords: "ratatui, docs index, rust, terminal ui",
    indexSummary: "This index links one `.ext.md` file per docs page snapshot for the Ratatui website.",
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
