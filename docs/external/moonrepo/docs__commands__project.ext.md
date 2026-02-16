----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/project
- Keywords: moon, moonrepo, docs, monorepo, build, commands, project
- Summary: The `moon project [id]` (or `moon p`) command will display all available information about a project
----

Source: https://moonrepo.dev/docs/commands/project

# project

The `moon project [id]` (or `moon p`) command will display all available information about a project
that has been configured and exists within the graph. If a project does not exist, the program will
return with a 1 exit code.

```
$ moon project web
```

### Arguments

- `[id]` - ID or alias of a project, as defined in [`projects`](/docs/config/workspace#projects).

### Options

- `--json` - Print the project and its configuration as JSON.

- `--no-tasks` - Do not list tasks for the project.

## Example output

The following output is an example of what this command prints, using our very own
`@moonrepo/runtime` package.

```
RUNTIMEProject: runtimeAlias: @moonrepo/runtimeSource: packages/runtimeRoot: ~/Projects/moon/packages/runtimeToolchain: nodeLanguage: typescriptStack: unknownType: libraryDEPENDS ON  - types (implicit, production)INHERITS FROM  - .moon/tasks/node.ymlTASKSbuild:  › packemon build --addFiles --addExports --declarationformat:  › prettier --check --config ../../prettier.config.js --ignore-path ../../.prettierignore --no-error-on-unmatched-pattern .lint:  › eslint --cache --cache-location ./.eslintcache --color --ext .js,.ts,.tsx --ignore-path ../../.eslintignore --exit-on-fatal-error --no-error-on-unmatched-pattern --report-unused-disable-directives .lint-fix:  › eslint --cache --cache-location ./.eslintcache --color --ext .js,.ts,.tsx --ignore-path ../../.eslintignore --exit-on-fatal-error --no-error-on-unmatched-pattern --report-unused-disable-directives . --fixtest:  › jest --cache --color --preset jest-preset-moon --passWithNoTeststypecheck:  › tsc --buildFILE GROUPSconfigs:  - packages/runtime/*.{js,json}sources:  - packages/runtime/src/**/*  - packages/runtime/types/**/*tests:  - packages/runtime/tests/**/*
```

### Configuration

- [`projects`](/docs/config/workspace#projects) in `.moon/workspace.yml`

- [`project`](/docs/config/project#project) in `moon.yml`

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
