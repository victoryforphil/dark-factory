----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/query/changed-files
- Keywords: moon, moonrepo, docs, monorepo, build, commands, query, changed files
- Summary: Use the `moon query changed-files` sub-command to query for a list of changed files (added,
----

Source: https://moonrepo.dev/docs/commands/query/changed-files

# query changed-files

Use the `moon query changed-files` sub-command to query for a list of changed files (added,
modified, deleted, etc) using the current VCS state. These are the same queries that
[`moon ci`](/docs/commands/ci) and [`moon run`](/docs/commands/run) use under the hood.

Touches files are determined using the following logic:

- If `--defaultBranch` is provided, and the current branch is the [`vcs.defaultBranch`](/docs/config/workspace#defaultbranch), then compare against the previous revision of the default branch (`HEAD~1`). This is what [continuous integration](/docs/guides/ci) uses.

- If `--local` is provided, changed files are based on your local index only (`git status`).

- Otherwise, then compare the defined base (`--base`) against head (`--head`).

```
# Return all files$ moon query changed-files# Return deleted files$ moon query changed-files --status deleted# Return all files between 2 revisions$ moon query changed-files --base  --head
```

This will output a list of workspace relative files as JSON. The output has the following structure:

```
{	files: string[],	options: QueryOptions,}
```

### Options

- `--default-branch` - When on the default branch, compare against the previous revision.

- `--base ` - Base branch, commit, or revision to compare against. Defaults to [`vcs.defaultBranch`](/docs/config/workspace#defaultbranch).

- `--head ` - Current branch, commit, or revision to compare with. Defaults to `HEAD`.

- `--local` - Gather files from the local state instead of remote.

- `--remote` - Gather files from the remote state instead of local.

- `--status ` - Filter files based on a changed status. Can be passed multiple times. Types: `all` (default), `added`, `deleted`, `modified`, `staged`, `unstaged`, `untracked`

### Configuration

- [`vcs`](/docs/config/workspace#vcs) in `.moon/workspace.yml`

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
