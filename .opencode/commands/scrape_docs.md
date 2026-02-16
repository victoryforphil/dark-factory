---
description: Add or refresh external docs snapshots from a user request
agent: build
---

Use the `docs-scraping` skill from `.opencode/skills/docs-scraping/SKILL.md`.

Goal:

- Implement or refresh a reusable scraper script for the requested docs source.
- Generate split page snapshots in `docs/external/<source>/` as `.ext.md` files plus `index.ext.md`.

Process:

1. Extract request metadata:
   - source key (for folder and script naming)
   - docs root URL
   - page discovery strategy (`sitemap.xml` preferred)
2. Create or update `scripts/scrape_<source>_docs.sh.ts`.
3. Reuse helpers under `scripts/helpers/` and keep script conventions (`#!/usr/bin/env bun`, `*.sh.ts`).
4. Implement resilient scraping:
   - primary: `r.jina.ai` markdown proxy
   - fallback: direct HTML conversion to markdown
5. Write split page outputs to `docs/external/<source>/` and include `index.ext.md`.
6. Report capture totals (`pages`, `ok`, `failed`) and blocked pages.
7. Update `README.md` and/or `AGENTS.md` if new entrypoints or conventions were added.

Example usage:

- Request example: "Scrape https://opencode.ai/docs into docs/external/opencode as split `.ext.md` pages with index."
- Expected script: `scripts/scrape_opencode_docs.sh.ts`
- Expected outputs:
  - `docs/external/opencode/index.ext.md`
  - `docs/external/opencode/docs.ext.md`
  - `docs/external/opencode/docs__introduction.ext.md`
  - additional page-level `*.ext.md` files

Return:

- Script path(s) added/updated
- Output directory generated
- Discovery strategy used
- Capture totals and notable failures
- Exact rerun command (`bun scripts/scrape_<source>_docs.sh.ts`)
