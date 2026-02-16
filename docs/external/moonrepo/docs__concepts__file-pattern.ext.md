----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/concepts/file-pattern
- Keywords: moon, moonrepo, docs, monorepo, build, concepts, file pattern
- Summary: Globs in moon are [Rust-based globs](https://github.com/olson-sean-k/wax), not JavaScript-based.
----

Source: https://moonrepo.dev/docs/concepts/file-pattern

# File patterns

## Globs

Globs in moon are [Rust-based globs](https://github.com/olson-sean-k/wax), not JavaScript-based.
This may result in different or unexpected results. The following guidelines must be met when using
globs:

- Must use forward slashes (`/`) for path separators, even on Windows.

- Must not start with or use any relative path parts, `.` or `..`.

### Supported syntax

- `*` - Matches zero or more characters, but does not match the `/` character. Will attempt to match the longest possible text (eager).

- `$` - Like `*`, but will attempt to match the shortest possible text (lazy).

- `**` - Matches zero or more directories.

- `?` - Matches exactly one character, but not `/`.

- `[abc]` - Matches one case-sensitive character listed in the brackets.

- `[!xyz]` - Like the above, but will match any character not listed.

- `[a-z]` - Matches one case-sensitive character in range in the brackets.

- `[!x-z]` - Like the above, but will match any character not in range.

- `{glob,glob}` - Matches one or more comma separated list of sub-glob patterns.

- `` - Matches a sub-glob within a defined bounds.

- `!` - At the start of a pattern, will negate previous positive patterns.

### Examples

```
README.{md,mdx,txt}src/**/*tests/**/*.?js!**/__tests__/**/*logs/--.log
```

## Project relative

When configuring [`fileGroups`](/docs/config/project#filegroups), [`inputs`](/docs/config/project#inputs),
and [`outputs`](/docs/config/project#outputs), all listed file paths and globs are relative from the
project root they will be ran in. They must not traverse upwards with `..`.

```
# Validsrc/**/*./src/**/*package.json# Invalid../utils
```

## Workspace relative

When configuring [`fileGroups`](/docs/config/project#filegroups), [`inputs`](/docs/config/project#inputs),
and [`outputs`](/docs/config/project#outputs), a listed file path or glob can be prefixed with `/` to
resolve relative from the workspace root, and not the project root.

```
# In projectpackage.json# In workspace/package.json
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
