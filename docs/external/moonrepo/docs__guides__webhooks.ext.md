----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/webhooks
- Keywords: moon, moonrepo, docs, monorepo, build, guides, webhooks
- Summary: Looking to gather metrics for your pipelines? Gain insight into run durations and failures? Maybe
----

Source: https://moonrepo.dev/docs/guides/webhooks

# Webhooks (experimental)

Looking to gather metrics for your pipelines? Gain insight into run durations and failures? Maybe
you want to send Slack or Discord notifications? With our webhooks, all of these are possible!

When the [`notifier.webhookUrl`](/docs/config/workspace#webhookurl) setting is configured with an HTTPS
URL, and moon is running in a CI environment, moon will POST a payload to this endpoint for every
event in our pipeline.

## Payload structure

Every webhook event is posted with the following request body, known as a payload.

- `type` (`string`) - The type of [event](#events).

- `environment` (`object | null`) - Information about the current CI/CD pipeline environment.

- `event` (`object`) - The event specific payload. View each event for an example of their structure.

- `createdAt` (`string`) - When the event was created, as a UTC timestamp in ISO 8601 (RFC 3339) format.

- `uuid` (`string`) - A unique identifier for all webhooks in the current run batch.

- `trace` (`string`) - A unique identifier for all webhooks in the overall run batch. Can be defined via `MOON_TRACE_ID` environment variable.

```
{  "type": "...",  "environment": "...",  "event": {    // ...  },  "createdAt": "...",  "uuid": "...",  "trace": "..."}
```

The `uuid` field can be used to differentiate concurrently running pipelines!

### Pipeline environment

When webhooks are sent from a CI/CD pipeline, we attempt to include information about the
environment under the `environment` field. If information could not be detected, this field is null,
otherwise it contains these fields.

- `baseBranch` (`string | null`) - When a merge/pull request, the target (base) branch, otherwise null.

- `branch` (`string`) - When a merge/pull request, the source (head) branch, otherwise the triggering branch.

- `id` (`string`) - ID of the current pipeline instance.

- `provider` (`string`) - Name of your CI/CD provider. GitHub Actions, GitLab, CircleCI, etc.

- `requestId` (`string | null`) - The ID of the merge/pull request.

- `requestUrl` (`string | null`) - Link to the merge/pull request.

- `revision` (`string`) - The HEAD commit, revision, tag, ref, etc, that triggered the pipeline.

- `url` (`string | null`) - Link to the current pipeline, when available.

## Events

### Pipeline

Runs actions within moon using a robust dependency graph. Is triggered when using
[`moon run`](/docs/commands/run).

### `pipeline.started`

Triggered when the pipeline has been created but before actions have started to run.

This event includes the number of actions registered within the pipeline, but does not provide
detailed information about the actions. Use the [`action.*`](#actionstarted) events for this.

```
{  "type": "pipeline.started",  "createdAt": "...",  "environment": "...",  "event": {    "actionsCount": 15  },  "uuid": "..."}
```

### `pipeline.finished`

Triggered when the pipeline has finished running all actions, with aggregated counts based on final
status.

This event is not triggered if the pipeline crashes (this does not include actions that have
failed, as those are legitimate runs). Use the [`pipeline.aborted`](#pipelineaborted) event if you
want to also catch crashes.

```
{  "type": "pipeline.finished",  "createdAt": "...",  "environment": "...",  "event": {    "cachedCount": 10,    "baselineDuration": {      "secs": 60,      "nanos": 3591693    },    "duration": {      "secs": 120,      "nanos": 3591693    },    "estimatedSavings": {      "secs": 60,      "nanos": 0    },    "failedCount": 1,    "passedCount": 4  },  "uuid": "..."}
```

### `pipeline.aborted`

Triggered when the pipeline has crashed for unknown reasons, or had to abort as a result of a
critical action failing.

```
{  "type": "pipeline.aborted",  "createdAt": "...",  "environment": "...",  "event": {    "error": "..."  },  "uuid": "..."}
```

### Actions

Actions are "jobs" within the pipeline that are executed topologically.

### `action.started`

Triggered when an action within the pipeline has started to run.

```
{  "type": "action.started",  "createdAt": "...",  "environment": "...",  "event": {    "action": {      "attempts": null,      "createdAt": "...",      "duration": {        "secs": 0,        "nanos": 3591693      },      "error": null,      "label": "InstallWorkspaceDeps(node:18.0.0)",      "nodeIndex": 5,      "status": "passed"    },    "node": {      "action": "InstallDeps",      "params": [        {          "toolchain": "Node",          "version": "18.0.0"        }      ]    }  },  "uuid": "..."}
```

### `action.finished`

Triggered when an action within the pipeline has finished running, either with a success or failure.
If the action failed, the `error` field will be set with the error message.

```
{  "type": "action.finished",  "createdAt": "...",  "environment": "...",  "event": {    "action": {      "attempts": null,      "createdAt": "...",      "duration": {        "secs": 0,        "nanos": 3591693      },      "error": null,      "label": "InstallWorkspaceDeps(node:18.0.0)",      "nodeIndex": 5,      "status": "passed"    },    "error": null,    "node": {      "action": "InstallDeps",      "params": {        "toolchain": "Node",        "version": "18.0.0"      }    }  },  "uuid": "..."}
```

### `dependencies.installing`

Triggered when dependencies for a workspace or project have started to install. When targeting a
project, the `project` field will be set, otherwise `null` for the entire workspace.

```
{  "type": "dependencies.installing",  "createdAt": "...",  "environment": "...",  "event": {    "project": {      "id": "server"      // ...    },    "runtime": {      "toolchain": "Node",      "version": "18.0.0"    },    "root": ".",    "toolchain": "node"  },  "uuid": "..."}
```

### `dependencies.installed`

Triggered when dependencies for a workspace or project have finished installing. When targeting a
project, the `project` field will be set, otherwise `null` for the entire workspace. If the install
failed, the `error` field will be set with the error message.

For more information about the action, refer to the [`action.finished`](#actionfinished) event.
Installed deps can be scoped with the `InstallDeps(...)` labels.

```
{  "type": "dependencies.installed",  "createdAt": "...",  "environment": "...",  "event": {    "error": null,    "project": null,    "runtime": {      "toolchain": "Node",      "version": "18.0.0"    },    "root": ".",    "toolchain": "node"  },  "uuid": "..."}
```

### `environment.initializing`v1.37.0

Triggered when an environment is being setup for a toolchain. When targeting a project, the
`project` field will be set, otherwise `null` for the entire workspace.

```
{  "type": "environment.initializing",  "createdAt": "...",  "environment": "...",  "event": {    "project": {      "id": "server"      // ...    },    "root": ".",    "toolchain": "node"  },  "uuid": "..."}
```

### `environment.initialized`v1.37.0

Triggered when an environment has been setup for a toolchain. When targeting a project, the
`project` field will be set, otherwise `null` for the entire workspace. If setup failed, the `error`
field will be set with the error message.

For more information about the action, refer to the [`action.finished`](#actionfinished) event.
Installed deps can be scoped with the `SetupEnvironment(...)` labels.

```
{  "type": "environment.initialized",  "createdAt": "...",  "environment": "...",  "event": {    "error": null,    "project": null,    "root": ".",    "toolchain": "node"  },  "uuid": "..."}
```

### `project.syncing`

Triggered when an affected project has started syncing its workspace state. This occurs
automatically before a project's task is ran.

```
{  "type": "project.syncing",  "createdAt": "...",  "environment": "...",  "event": {    "project": {      "id": "client"      // ...    },    "runtime": {      "toolchain": "Node",      "version": "18.0.0"    }  },  "uuid": "..."}
```

### `project.synced`

Triggered when an affected project has finished syncing. If the sync failed, the `error` field will
be set with the error message.

For more information about the action, refer to the [`action.finished`](#actionfinished) event.
Synced projects can be scoped with the `SyncProject(...)` labels.

```
{  "type": "project.synced",  "createdAt": "...",  "environment": "...",  "event": {    "error": null,    "project": {      "id": "client"      // ...    },    "runtime": {      "toolchain": "Node",      "version": "18.0.0"    }  },  "uuid": "..."}
```

### `tool.installing`

Triggered when a tool within the toolchain has started downloading and installing.

This event is always triggered, regardless of whether the tool has already been installed or not.
For an accurate state, use the [`action.finished`](#actionfinished) event. If the `status` is
"skipped", then the tool was already installed.

```
{  "type": "tool.installing",  "createdAt": "...",  "environment": "...",  "event": {    "runtime": {      "toolchain": "Node",      "version": "18.0.0"    }  },  "uuid": "..."}
```

### `tool.installed`

Triggered when a tool within the toolchain has finished installing. If the install failed, the
`error` field will be set with the error message.

For more information about the action, refer to the [`action.finished`](#actionfinished) event.
Tools can be scoped with the `SetupToolchain(...)` labels.

```
{  "type": "tool.installed",  "createdAt": "...",  "environment": "...",  "event": {    "error": null,    "runtime": {      "toolchain": "Node",      "version": "18.0.0"    }  },  "uuid": "..."}
```

### `toolchain.installing`

Triggered when a toolchain plugin has started downloading and installing.

This event is always triggered, regardless of whether the tool has already been installed or not.
For an accurate state, use the [`action.finished`](#actionfinished) event. If the `status` is
"skipped", then the tool was already installed.

```
{  "type": "toolchain.installing",  "createdAt": "...",  "environment": "...",  "event": {    "spec": {      "id": "node",      "req": "18.0.0"    }  },  "uuid": "..."}
```

### `toolchain.installed`

Triggered when a toolchain plugin has finished installing. If the install failed, the `error` field
will be set with the error message.

For more information about the action, refer to the [`action.finished`](#actionfinished) event.
Tools can be scoped with the `SetupToolchain(...)` labels.

```
{  "type": "toolchain.installed",  "createdAt": "...",  "environment": "...",  "event": {    "error": null,    "spec": {      "id": "node",      "req": "18.0.0"    }  },  "uuid": "..."}
```

### `task.running`

Triggered when a [task](/docs/concepts/task) has started to run (via [`moon run`](/docs/commands/run) or
similar command).

```
{  "type": "task.running",  "createdAt": "...",  "environment": "...",  "event": {    "target": "app:build"  },  "uuid": "..."}
```

### `task.ran`

Triggered when a [task](/docs/concepts/task) has finished running. If the run failed, the `error` field
will be set with the error message.

For more information about the action, refer to the [`action.finished`](#actionfinished) event. Ran
tasks can be scoped with the `RunTask(...)`, `RunInteractiveTask(...)`, and `RunPersistentTask(...)`
labels.

```
{  "type": "task.ran",  "createdAt": "...",  "environment": "...",  "event": {    "error": null,    "target": "app:build"  },  "uuid": "..."}
```

### `workspace.syncing`

Triggered when the workspace is being synced.

```
{  "type": "workspace.syncing",  "createdAt": "...",  "environment": "...",  "event": {    "target": "app:build"  },  "uuid": "..."}
```

### `workspace.synced`

Triggered when the workspace has finished syncing. If the action failed, the `error` field will be
set with the error message.

```
{  "type": "workspace.synced",  "createdAt": "...",  "environment": "...",  "event": {    "error": null  },  "uuid": "..."}
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
