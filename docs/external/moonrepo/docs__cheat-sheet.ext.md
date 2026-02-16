----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/cheat-sheet
- Keywords: moon, moonrepo, docs, monorepo, build, cheat sheet
- Summary: Don't have time to read the docs? Here's a quick cheat sheet to get you started.
----

Source: https://moonrepo.dev/docs/cheat-sheet

Don't have time to read the docs? Here's a quick cheat sheet to get you started.

Tasks[​](http://moonrepo.dev/docs/cheat-sheet#tasks "Direct link to Tasks")
---------------------------------------------------------------------------

Learn more about [tasks](https://moonrepo.dev/docs/concepts/task) and [targets](https://moonrepo.dev/docs/concepts/target).

#### Run all build and test tasks for all projects[​](http://moonrepo.dev/docs/cheat-sheet#run-all-build-and-test-tasks-for-all-projects "Direct link to Run all build and test tasks for all projects")

`moon check --all`

#### Run all build and test tasks in a project[​](http://moonrepo.dev/docs/cheat-sheet#run-all-build-and-test-tasks-in-a-project "Direct link to Run all build and test tasks in a project")

`moon check project`

#### Run all build and test tasks for closest project based on working directory[​](http://moonrepo.dev/docs/cheat-sheet#run-all-build-and-test-tasks-for-closest-project-based-on-working-directory "Direct link to Run all build and test tasks for closest project based on working directory")

`moon check`

#### Run a task in all projects[​](http://moonrepo.dev/docs/cheat-sheet#run-a-task-in-all-projects "Direct link to Run a task in all projects")

`moon run :task`

#### Run a task in all projects with a tag[​](http://moonrepo.dev/docs/cheat-sheet#run-a-task-in-all-projects-with-a-tag "Direct link to Run a task in all projects with a tag")

`moon run '#tag:task'# ORmoon run \#tag:task# ORmoon run :task --query "tag=tag"`

#### Run a task in a project[​](http://moonrepo.dev/docs/cheat-sheet#run-a-task-in-a-project "Direct link to Run a task in a project")

`moon run project:task`

#### Run multiple tasks in all projects[​](http://moonrepo.dev/docs/cheat-sheet#run-multiple-tasks-in-all-projects "Direct link to Run multiple tasks in all projects")

`moon run :task1 :task2`

#### Run multiple tasks in any project[​](http://moonrepo.dev/docs/cheat-sheet#run-multiple-tasks-in-any-project "Direct link to Run multiple tasks in any project")

`moon run projecta:task1 projectb:task2`

#### Run a task in applications, libraries, or tools[​](http://moonrepo.dev/docs/cheat-sheet#run-a-task-in-applications-libraries-or-tools "Direct link to Run a task in applications, libraries, or tools")

`moon run :task --query "projectLayer=application"`

#### Run a task in projects of a specific language[​](http://moonrepo.dev/docs/cheat-sheet#run-a-task-in-projects-of-a-specific-language "Direct link to Run a task in projects of a specific language")

`moon run :task --query "language=typescript"`

#### Run a task in projects matching a keyword[​](http://moonrepo.dev/docs/cheat-sheet#run-a-task-in-projects-matching-a-keyword "Direct link to Run a task in projects matching a keyword")

`moon run :task --query "project~react-*"`

#### Run a task in projects based on file path[​](http://moonrepo.dev/docs/cheat-sheet#run-a-task-in-projects-based-on-file-path "Direct link to Run a task in projects based on file path")

`moon run :task --query "projectSource~packages/*"`

Task configuration[​](http://moonrepo.dev/docs/cheat-sheet#task-configuration "Direct link to Task configuration")
------------------------------------------------------------------------------------------------------------------

Learn more about [available options](https://moonrepo.dev/docs/config/project#tasks).

#### Disable caching[​](http://moonrepo.dev/docs/cheat-sheet#disable-caching "Direct link to Disable caching")

moon.yml

`tasks:  example:    # ...    options:      cache: false`

#### Re-run flaky tasks[​](http://moonrepo.dev/docs/cheat-sheet#re-run-flaky-tasks "Direct link to Re-run flaky tasks")

moon.yml

`tasks:  example:    # ...    options:      retryCount: 3`

#### Depend on tasks from parent project's dependencies[​](http://moonrepo.dev/docs/cheat-sheet#depend-on-tasks-from-parent-projects-dependencies "Direct link to Depend on tasks from parent project's dependencies")

moon.yml

`# Also inferred from the languagedependsOn:  - 'project-a'  - 'project-b'tasks:  example:    # ...    deps:      - '^:build'`

#### Depend on tasks from arbitrary projects[​](http://moonrepo.dev/docs/cheat-sheet#depend-on-tasks-from-arbitrary-projects "Direct link to Depend on tasks from arbitrary projects")

moon.yml

`tasks:  example:    # ...    deps:      - 'other-project:task'`

#### Run dependencies serially[​](http://moonrepo.dev/docs/cheat-sheet#run-dependencies-serially "Direct link to Run dependencies serially")

moon.yml

`tasks:  example:    # ...    deps:      - 'first'      - 'second'      - 'third'    options:      runDepsInParallel: false`

moon.yml

`tasks:  example:    command: 'noop'    deps:      - 'app:watch'      - 'backend:start'      - 'tailwind:watch'    preset: 'server'`

> The `local` or `persistent` settings are required for this to work.

Languages[​](http://moonrepo.dev/docs/cheat-sheet#languages "Direct link to Languages")
---------------------------------------------------------------------------------------

#### Run system binaries available on `PATH`[​](http://moonrepo.dev/docs/cheat-sheet#run-system-binaries-available-on-path "Direct link to run-system-binaries-available-on-path")

moon.yml

`language: 'bash' # batch, etctasks:  example:    command: 'printenv'`

moon.yml

`tasks:  example:    command: 'printenv'    toolchain: 'system'`

#### Run language binaries not supported in moon's toolchain[​](http://moonrepo.dev/docs/cheat-sheet#run-language-binaries-not-supported-in-moons-toolchain "Direct link to Run language binaries not supported in moon's toolchain")

moon.yml

`language: 'ruby'tasks:  example:    command: 'rubocop'    toolchain: 'system'`

#### Run npm binaries (Node.js)[​](http://moonrepo.dev/docs/cheat-sheet#run-npm-binaries-nodejs "Direct link to Run npm binaries (Node.js)")

moon.yml

`language: 'javascript' # typescripttasks:  example:    command: 'eslint'`

moon.yml

`tasks:  example:    command: 'eslint'    toolchain: 'node'`

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
