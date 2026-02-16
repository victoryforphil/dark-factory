---
name: external-docs-scraping
description: Create and run Bun scraper scripts that snapshot external docs into split `.ext.md` files under `docs/external/<source>/`.
---

## What I do

- Standardize how we add a new external docs snapshot source.
- Create or adapt a Bun script in `scripts/` using existing scraper conventions.
- Produce split per-page `*.ext.md` files plus an `index.ext.md` under `docs/external/<source>/`.
- This skill remains compatible, but prefer `.opencode/skills/docs-scraping/SKILL.md` for generic user-request driven scraping workflows.

## When to use me

Use this when the task is to add a new external docs source like the existing moonrepo snapshot flow.

## Output contract

- Script file: `scripts/scrape_<source>_docs.sh.ts`
- Output directory: `docs/external/<source>/`
- Output files:
  - One page-level `*.ext.md` file per source docs page
  - `index.ext.md` linking all generated page files

## Script conventions

- Use Bun shebang script format: `#!/usr/bin/env bun`.
- Reuse shared helpers from `scripts/helpers/` (for example `findRepoRoot`).
- Keep script deterministic and idempotent where practical.
- Default to sitemap or deterministic page discovery over fragile crawling.
- Clear prior `*.ext.md` files in the target output directory before regenerating.

## Page/index format conventions

- Keep each page file in `.ext.md` format with:
  - metadata block (captured timestamp, source URL/path, keywords, summary)
  - normalized markdown body
  - notes section describing capture/conversion method
- Keep `index.ext.md` as a full page inventory with status summary (`ok`/`failed`).

## Implementation workflow

1. Identify docs root and discovery method (`sitemap.xml` preferred).
2. Create script in `scripts/` following `scrape_moon_docs.sh.ts` structure.
3. Map URLs to stable flattened filenames (for example `docs__guides__intro.ext.md`).
4. Scrape each page with a resilient strategy:
   - Primary: `r.jina.ai` markdown proxy
   - Fallback: direct HTML fetch and conversion
5. Write all page snapshots and `index.ext.md` to `docs/external/<source>/`.
6. Run the script once and report page count + success/failure totals.
7. Update `README.md` and `AGENTS.md` when a new scraper entrypoint is added.

## Notes

- Prefer explicit logs using project format: `Docs // Scrape // Message (...)`.
- Do not include secrets, auth tokens, or private pages in snapshots.
- If scraping is partially blocked, still produce index entries marking failed pages.
