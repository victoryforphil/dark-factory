----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/migrate-to-moon
- Keywords: moon, moonrepo, docs, monorepo, build, migrate to moon
- Summary: Now that we've talked about the workspace, projects, tasks, and more, we must talk about something
----

Source: https://moonrepo.dev/docs/migrate-to-moon

# Migrate to moon

Now that we've talked about the workspace, projects, tasks, and more, we must talk about something
important... Should you embrace moon tasks? Or keep using language/ecosystem specific scripts? Or
both (incremental adoption)?

## Migrate to moon tasks

We suggest using moon tasks (of course), as they provide far more granular control and configurable
options than scripts, and a `moon.yml` is a better
[source of truth](/docs/faq#what-should-be-considered-the-source-of-truth). Scripts aren't powerful
enough to scale for large codebases.

An example of what this may look like can be found below. This may look like a lot, but it pays
dividends in the long run.

/moon.yml

```
language: 'javascript'fileGroups:  sources:    - 'src/**/*'  tests:    - 'tests/**/*'tasks:  build:    command: 'webpack build --output-path @out(0)'    inputs:      - '@globs(sources)'      - 'webpack.config.js'    outputs:      - 'build'  dev:    command: 'webpack server'    inputs:      - '@globs(sources)'      - 'webpack.config.js'    preset: 'server'  format:    command: 'prettier --check .'    inputs:      - '@globs(sources)'      - '@globs(tests)'      - '/prettier.config.js'  lint:    command: 'eslint .'    inputs:      - '@globs(sources)'      - '@globs(tests)'      - '.eslintignore'      - '.eslintrc.js'      - '/.eslintrc.js'  test:    command: 'jest .'    inputs:      - '@globs(sources)'      - '@globs(tests)'      - 'jest.config.js'  typecheck:    command: 'tsc --build'    inputs:      - '@globs(sources)'      - '@globs(tests)'      - 'tsconfig.json'      - '/tsconfig.json'
```

## Continue using scripts

As a frontend developer you're already familiar with the Node.js ecosystem, specifically around
defining and using `package.json` scripts, and you may not want to deviate from this. Don't worry,
simply enable the [`node.inferTasksFromScripts`](/docs/config/toolchain#infertasksfromscripts) setting
to automatically create moon tasks from a project's scripts! These can then be ran with
[`moon run`](/docs/commands/run).

This implementation is a simple abstraction that runs `npm run ` (or pnpm/yarn) in the
project directory as a child process. While this works, relying on `package.json` scripts incurs the
following risks and disadvantages:

- [Inputs](/docs/config/project#inputs) default to `**/*`: A change to every project relative file will mark the task as affected, even those not necessary for the task. Granular input control is lost.

- A change to workspace relative files will not mark the task as affected. For example, a change to `/prettier.config.js` would not be detected for a `npm run format` script.

- [Outputs](/docs/config/project#outputs) default to an empty list unless: moon will attempt to extract outputs from arguments, by looking for variations of `--out`, `--outFile`, `--dist-dir`, etc.

- If no output could be determined, builds will not be cached and hydrated.

- Tasks will always [run in CI](/docs/config/project#runinci) unless: moon will attempt to determine invalid CI tasks by looking for popular command usage, for example: `webpack serve`, `next dev`, `--watch` usage, and more. This is not an exhaustive check.

- The script name contains variations of `dev`, `start`, or `serve`.

## Next steps

By this point, you should have a better understanding behind moon's fundamentals! Why not adopt
incrementally next? Jump into [guides](/docs/guides/ci) for advanced use cases or [concepts](/docs/concepts)
for a deeper understanding.

[Community help & support](https://discord.gg/qCh9MEynv2)[Releases & updates](https://twitter.com/tothemoonrepo)

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
