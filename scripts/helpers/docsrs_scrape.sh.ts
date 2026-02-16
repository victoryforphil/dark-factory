#!/usr/bin/env bun

import {
  dedupeSorted,
  discoverUrlsFromRootPage,
  discoverUrlsFromSitemap,
  DiscoveryResult,
  extractHrefUrls,
  fetchText,
} from "./docs_scrape.sh.ts";

export type DocsRsDiscoveryMethod = "sitemap" | "html";

export type DocsRsDiscoveryResult = DiscoveryResult<DocsRsDiscoveryMethod>;

export type DocsRsTarget = {
  crateName: string;
  version: string;
  modulePath: string;
};

export type DocsRsDiscoveryOptions = {
  maxPages?: number;
};

const DOCS_RS_ROOT = "https://docs.rs";
const DOCS_RS_HOST = "docs.rs";

export function docsRsRootUrl(target: DocsRsTarget): string {
  const modulePath = target.modulePath.replace(/^\/+|\/+$/g, "");
  return `${DOCS_RS_ROOT}/${target.crateName}/${target.version}/${modulePath}/`;
}

export function docsRsPathPrefix(target: DocsRsTarget): string {
  return `/${target.crateName}/${target.version}/${target.modulePath.replace(/^\/+|\/+$/g, "")}`;
}

export function docsRsSitemapUrlForCrate(crateName: string): string {
  const first = crateName.trim().charAt(0).toLowerCase();
  const shard = /^[a-z]$/.test(first) ? first : "a";
  return `${DOCS_RS_ROOT}/-/sitemap/${shard}/sitemap.xml`;
}

export function normalizeDocsRsUrl(url: string, pathPrefix: string): string | null {
  try {
    const parsed = new URL(url, DOCS_RS_ROOT);
    if (parsed.hostname !== DOCS_RS_HOST) {
      return null;
    }

    parsed.hash = "";
    parsed.search = "";
    parsed.pathname = parsed.pathname.replace(/\/+$/, "") || "/";

    if (parsed.pathname !== pathPrefix && !parsed.pathname.startsWith(`${pathPrefix}/`)) {
      return null;
    }

    if (/\.(png|jpg|jpeg|gif|svg|webp|ico|css|js|map|xml|json|txt|pdf|zip|wasm)$/i.test(parsed.pathname)) {
      return null;
    }

    return parsed.toString();
  } catch {
    return null;
  }
}

export async function discoverDocsRsUrls(target: DocsRsTarget): Promise<DocsRsDiscoveryResult> {
  const rootUrl = docsRsRootUrl(target);
  const pathPrefix = docsRsPathPrefix(target);
  const sitemapUrl = docsRsSitemapUrlForCrate(target.crateName);
  const normalize = (url: string) => normalizeDocsRsUrl(url, pathPrefix);

  const sitemapUrls = await discoverUrlsFromSitemap(sitemapUrl, normalize);
  if (sitemapUrls.length > 0) {
    const expanded = await expandDocsRsUrlsFromPageLinks(sitemapUrls, normalize);
    return {
      urls: expanded,
      method: "sitemap",
      detail: `sitemap_url=${sitemapUrl},matched=${sitemapUrls.length},expanded=${expanded.length}`,
    };
  }

  const htmlUrls = await discoverUrlsFromRootPage(rootUrl, normalize);
  const expanded = await expandDocsRsUrlsFromPageLinks(htmlUrls, normalize);
  return {
    urls: expanded,
    method: "html",
    detail: `root_fallback=${htmlUrls.length},expanded=${expanded.length}`,
  };
}

export async function expandDocsRsUrlsFromPageLinks(
  seedUrls: string[],
  normalize: (url: string) => string | null,
  options?: DocsRsDiscoveryOptions,
): Promise<string[]> {
  const maxPages = options?.maxPages ?? 200;
  const pending = [...seedUrls];
  const discovered = new Set<string>();

  while (pending.length > 0 && discovered.size < maxPages) {
    const current = pending.shift();
    if (!current || discovered.has(current)) {
      continue;
    }

    discovered.add(current);

    let html: string;
    try {
      html = await fetchText(current);
    } catch {
      continue;
    }

    const linked = extractHrefUrls(html)
      .map((url) => normalize(url))
      .filter((url): url is string => Boolean(url));

    for (const url of linked) {
      if (!discovered.has(url) && !pending.includes(url)) {
        pending.push(url);
      }
      if (discovered.size + pending.length >= maxPages) {
        break;
      }
    }
  }

  return dedupeSorted(Array.from(discovered));
}
