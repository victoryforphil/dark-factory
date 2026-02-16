#!/usr/bin/env bun

import { resolve } from "node:path";
import {
  buildPageArtifacts,
  buildSnapshotIndexMarkdown,
  buildSnapshotPageMarkdown,
  fileStemFromSourcePath,
  mapConcurrent,
  outputDirectoryRelative,
  prepareOutputDirectory,
  resolveOutputDirectory,
  scrapeDocsPage,
  sourcePathFromUrl,
  summarizeArtifacts,
} from "../helpers/docs_scrape.sh.ts";
import {
  discoverDocsRsUrls,
  docsRsPathPrefix,
  docsRsRootUrl,
} from "../helpers/docsrs_scrape.sh.ts";
import { findRepoRoot } from "../helpers/run_root.sh.ts";

const SOURCE_KEY = "ratatui_docs";
const DOCS_RS_TARGET = {
  crateName: "ratatui",
  version: "latest",
  modulePath: "ratatui",
};
const DOCS_ROOT = docsRsRootUrl(DOCS_RS_TARGET);
const ROOT_PATH = docsRsPathPrefix(DOCS_RS_TARGET);
const DEFAULT_OUTPUT_DIR_RELATIVE = "docs/external/ratatui_docs";
const CONCURRENCY = 4;

function fileStemFromUrl(url: string): string {
  const sourcePath = sourcePathFromUrl(url);
  return fileStemFromSourcePath(sourcePath, {
    rootPath: ROOT_PATH,
    rootStem: "docs",
    trimPrefixPath: ROOT_PATH,
    stripMarkdownExtension: true,
  });
}

const repoRoot = findRepoRoot(import.meta.dir);
const outputArg = Bun.argv[2];
const outputDir = resolveOutputDirectory(repoRoot, DEFAULT_OUTPUT_DIR_RELATIVE, outputArg);
const outputDirRelative = outputDirectoryRelative(repoRoot, outputDir);

prepareOutputDirectory(outputDir);

const discovery = await discoverDocsRsUrls(DOCS_RS_TARGET);
if (discovery.urls.length === 0) {
  throw new Error(`No docs URLs found for source ${SOURCE_KEY} (root=${DOCS_ROOT})`);
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
      baseKeywords: "ratatui, docs.rs, rustdoc, api docs",
      summaryFallback: "Ratatui docs.rs page snapshot.",
      collectionMethodNote: "docs.rs sitemap shard discovery scoped to crate/version/module path with HTML fallback.",
      dropKeywordPathSegments: 2,
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
    scopeText: `${artifacts.length} pages under ${ROOT_PATH}`,
    indexKeywords: "ratatui, docs.rs index, rustdoc, api",
    indexSummary: "This index links one `.ext.md` file per docs.rs page snapshot for ratatui API docs.",
    notes: [
      "- This scraper uses docs.rs shard sitemap discovery first and keeps pages under the crate/module path only.",
      "- Regenerate by re-running the scraper script; old `.ext.md` files in this directory are replaced.",
    ],
  }),
);

const stats = summarizeArtifacts(artifacts);
console.log(
  `Docs // Scrape // Wrote split external docs (source=${SOURCE_KEY},discovery=${discovery.method},pages=${artifacts.length},ok=${stats.successCount},failed=${stats.failureCount},blocked=${stats.blockedPages.join(",") || "none"},dir=${outputDir})`,
);
