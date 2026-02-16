---
name: docs-scraping
description: Add or refresh external documentation snapshots from user requests into split `.ext.md` pages and `index.ext.md`.
---

## What I do

- Turn a user docs scraping request into a reusable script and generated docs snapshot output.
- Standardize output under `docs/external/<source>/` with one page-level `.ext.md` file per discovered page.
- Ensure each generated page has a metadata header and notes footer, and create an `index.ext.md` manifest.

## Accepted request patterns

Use this skill when the request includes one or more of:

- A docs root URL (for example `https://example.dev/docs`)
- A source/domain name (for example `moonrepo`, `astro`, `pnpm`)
- A request to "scrape", "snapshot", "mirror", or "refresh" external docs

## Output contract

- Script: `scripts/scrapes/scrape_<source>_docs.sh.ts`
- Output directory: `docs/external/<source>/`
- Output files:
  - Per-page files: `<stable-page-stem>.ext.md`
  - Index file: `index.ext.md`

## Behavior requirements

1. Resolve source metadata from the user request:
   - `source` key (safe folder/script slug)
   - docs root URL
   - discovery method (`sitemap.xml` preferred)
2. Reuse project script conventions:
   - Bun shebang (`#!/usr/bin/env bun`)
   - `*.sh.ts` naming
   - helpers under `scripts/helpers/`
3. Use resilient scraping strategy:
   - Primary: `r.jina.ai` markdown proxy
   - Fallback: direct HTML fetch + conversion to markdown
4. Normalize filenames from docs paths:
   - deterministic flattening (for example `docs__guides__intro.ext.md`)
5. Regenerate output cleanly:
   - remove old `*.ext.md` in target source directory
   - write fresh per-page files and `index.ext.md`

## Generated page format

- Top `----` section with summary metadata:
  - captured timestamp
  - source root
  - source page URL/path
  - keywords
  - concise summary
- Middle body with markdown content snapshot
- Bottom `----` notes/comments/lessons section

## Index format

- Top `----` section with source/capture metadata and summary totals
- Full page inventory with links to local page files and capture status
- Bottom `----` notes/comments/lessons section

## Implementation workflow

1. Inspect existing scraper scripts for reuse patterns (`scripts/scrapes/scrape_*.sh.ts`).
2. Create or update `scripts/scrapes/scrape_<source>_docs.sh.ts`.
3. Run the script once to generate docs output.
4. Report totals (`pages`, `ok`, `failed`) and notable blocked pages.
5. If a new script entrypoint was introduced, update `README.md` and `AGENTS.md`.

## Safety constraints

- Never embed secrets, auth headers, or private tokens in script or output files.
- Skip private/authenticated docs pages unless explicit credentials handling is requested and safe.
- Keep scripts idempotent and deterministic where practical.

## Return checklist

- Script path(s) added/updated
- Output directory generated/refreshed
- Discovery strategy used
- Capture totals and notable failures
- Exact rerun command (`bun scripts/scrapes/scrape_<source>_docs.sh.ts`)
