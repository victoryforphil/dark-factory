---
description: Add or run an external docs scraper from a user request
agent: build
---

Use the `docs-scraping` skill from `.opencode/skills/docs-scraping/SKILL.md`.

Note: This command is kept as a singular-name alias; prefer `.opencode/commands/scrape_docs.md` for new usage.

Goal:

- Convert a user scraping request into a reusable external docs workflow.
- Create or refresh split page snapshots in `docs/external/<source>/` as `*.ext.md` plus `index.ext.md`.

Process:

1. Parse source metadata from the user request:
   - source key (for `<source>` folder and script name)
   - docs root URL
   - discovery strategy (`sitemap.xml` preferred)
2. Create or update script: `scripts/scrape_<source>_docs.sh.ts`.
3. Reuse shared helpers (`scripts/helpers/`) and follow Bun shebang script conventions.
4. Implement resilient scrape flow:
   - primary: `r.jina.ai` markdown proxy
   - fallback: direct HTML fetch + conversion
5. Regenerate `docs/external/<source>/*.ext.md` and `docs/external/<source>/index.ext.md`.
6. Report capture totals (`pages`, `ok`, `failed`) and call out any blocked pages.
7. If a new scraper entrypoint was added, update `README.md` and `AGENTS.md` references.

Return:

- Script path(s) added/updated
- Output directory generated
- Discovery strategy used
- Capture result totals and notable failures
- Exact rerun command (for example `bun scripts/scrape_<source>_docs.sh.ts`)
