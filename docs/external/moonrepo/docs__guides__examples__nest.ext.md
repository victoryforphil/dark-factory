----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/examples/nest
- Keywords: moon, moonrepo, docs, monorepo, build, guides, examples, nest
- Summary: In this guide, you'll learn how to integrate [NestJS](https://nestjs.com/) into moon.
----

Source: https://moonrepo.dev/docs/guides/examples/nest

# Nest example

In this guide, you'll learn how to integrate [NestJS](https://nestjs.com/) into moon.

Begin by creating a new NestJS project in the root of an existing moon project (this should not be
created in the workspace root, unless a polyrepo).

```
npx @nestjs/cli@latest new nestjs-app --skip-git
```

View the [official NestJS docs](https://docs.nestjs.com/first-steps) for a more in-depth guide to
getting started!

## Setup

Since NestJS is per-project, the associated moon tasks should be defined in each project's
[`moon.yml`](/docs/config/project) file.

/moon.yml

```
layer: 'application'fileGroups:  app:    - 'nest-cli.*'tasks:  dev:    command: 'nest start --watch'    preset: 'server'  build:    command: 'nest build'    inputs:      - '@group(app)'      - '@group(sources)'
```

### TypeScript integration

NestJS has [built-in support for TypeScript](https://NestJS.io/guide/typescript-configuration), so
there is no need for additional configuration to enable TypeScript support.

At this point we'll assume that a `tsconfig.json` has been created in the application, and
typechecking works. From here we suggest utilizing a [global `typecheck` task](/docs/guides/examples/typescript) for
consistency across all projects within the repository.

## Configuration

### Root-level

We suggest against root-level configuration, as NestJS should be installed per-project, and the
`nest` command expects the configuration to live relative to the project root.

### Project-level

When creating a new NestJS project, a [`nest-cli.json`](https://docs.nestjs.com/cli/monorepo) is
created, and must exist in the project root. This allows each project to configure NestJS for
their needs.

/nest-cli.json

```
{  "$schema": "https://json.schemastore.org/nest-cli",  "collection": "@nestjs/schematics",  "type": "application",  "root": "./",  "sourceRoot": "src",  "compilerOptions": {    "tsConfigPath": "tsconfig.build.json"  }}
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
