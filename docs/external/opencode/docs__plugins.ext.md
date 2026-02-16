----
## External Docs Snapshot // opencode

- Captured: 2026-02-16T07:29:35.337Z
- Source root: https://opencode.ai/docs
- Source page: /docs/plugins
- Keywords: opencode, docs, ai coding assistant, cli, plugins
- Summary: [Skip to content](http://opencode.ai/docs/plugins#_top)
----

Source: https://opencode.ai/docs/plugins

Plugins | OpenCode
===============
[Skip to content](http://opencode.ai/docs/plugins#_top)

[![Image 1](http://opencode.ai/docs/_astro/logo-dark.DOStV66V.svg)![Image 2](http://opencode.ai/docs/_astro/logo-light.B0yzR0O5.svg) OpenCode](http://opencode.ai/docs/)

[app.header.home](http://opencode.ai/)[app.header.docs](http://opencode.ai/docs/)

[](https://github.com/anomalyco/opencode)[](https://opencode.ai/discord)

Search Ctrl K

 Cancel 

*   [Intro](http://opencode.ai/docs/)
*   [Config](http://opencode.ai/docs/config/)
*   [Providers](http://opencode.ai/docs/providers/)
*   [Network](http://opencode.ai/docs/network/)
*   [Enterprise](http://opencode.ai/docs/enterprise/)
*   [Troubleshooting](http://opencode.ai/docs/troubleshooting/)
*   [Windows (WSL)](http://opencode.ai/docs/windows-wsl/)
*   
Usage 
    *   [TUI](http://opencode.ai/docs/tui/)
    *   [CLI](http://opencode.ai/docs/cli/)
    *   [Web](http://opencode.ai/docs/web/)
    *   [IDE](http://opencode.ai/docs/ide/)
    *   [Zen](http://opencode.ai/docs/zen/)
    *   [Share](http://opencode.ai/docs/share/)
    *   [GitHub](http://opencode.ai/docs/github/)
    *   [GitLab](http://opencode.ai/docs/gitlab/)

*   
Configure 
    *   [Tools](http://opencode.ai/docs/tools/)
    *   [Rules](http://opencode.ai/docs/rules/)
    *   [Agents](http://opencode.ai/docs/agents/)
    *   [Models](http://opencode.ai/docs/models/)
    *   [Themes](http://opencode.ai/docs/themes/)
    *   [Keybinds](http://opencode.ai/docs/keybinds/)
    *   [Commands](http://opencode.ai/docs/commands/)
    *   [Formatters](http://opencode.ai/docs/formatters/)
    *   [Permissions](http://opencode.ai/docs/permissions/)
    *   [LSP Servers](http://opencode.ai/docs/lsp/)
    *   [MCP servers](http://opencode.ai/docs/mcp-servers/)
    *   [ACP Support](http://opencode.ai/docs/acp/)
    *   [Agent Skills](http://opencode.ai/docs/skills/)
    *   [Custom Tools](http://opencode.ai/docs/custom-tools/)

*   
Develop 
    *   [SDK](http://opencode.ai/docs/sdk/)
    *   [Server](http://opencode.ai/docs/server/)
    *   [Plugins](http://opencode.ai/docs/plugins/)
    *   [Ecosystem](http://opencode.ai/docs/ecosystem/)

[GitHub](https://github.com/anomalyco/opencode)[Discord](https://opencode.ai/discord)

Select theme Select language 

On this page

*   [Overview](http://opencode.ai/docs/plugins#_top)
*   [Use a plugin](http://opencode.ai/docs/plugins#use-a-plugin)
    *   [From local files](http://opencode.ai/docs/plugins#from-local-files)
    *   [From npm](http://opencode.ai/docs/plugins#from-npm)
    *   [How plugins are installed](http://opencode.ai/docs/plugins#how-plugins-are-installed)
    *   [Load order](http://opencode.ai/docs/plugins#load-order)

*   [Create a plugin](http://opencode.ai/docs/plugins#create-a-plugin)
    *   [Dependencies](http://opencode.ai/docs/plugins#dependencies)
    *   [Basic structure](http://opencode.ai/docs/plugins#basic-structure)
    *   [TypeScript support](http://opencode.ai/docs/plugins#typescript-support)
    *   [Events](http://opencode.ai/docs/plugins#events)

*   [Examples](http://opencode.ai/docs/plugins#examples)
    *   [Send notifications](http://opencode.ai/docs/plugins#send-notifications)
    *   [.env protection](http://opencode.ai/docs/plugins#env-protection)
    *   [Inject environment variables](http://opencode.ai/docs/plugins#inject-environment-variables)
    *   [Custom tools](http://opencode.ai/docs/plugins#custom-tools)
    *   [Logging](http://opencode.ai/docs/plugins#logging)
    *   [Compaction hooks](http://opencode.ai/docs/plugins#compaction-hooks)

On this page
------------

*   [Overview](http://opencode.ai/docs/plugins#_top)
*   [Use a plugin](http://opencode.ai/docs/plugins#use-a-plugin)
    *   [From local files](http://opencode.ai/docs/plugins#from-local-files)
    *   [From npm](http://opencode.ai/docs/plugins#from-npm)
    *   [How plugins are installed](http://opencode.ai/docs/plugins#how-plugins-are-installed)
    *   [Load order](http://opencode.ai/docs/plugins#load-order)

*   [Create a plugin](http://opencode.ai/docs/plugins#create-a-plugin)
    *   [Dependencies](http://opencode.ai/docs/plugins#dependencies)
    *   [Basic structure](http://opencode.ai/docs/plugins#basic-structure)
    *   [TypeScript support](http://opencode.ai/docs/plugins#typescript-support)
    *   [Events](http://opencode.ai/docs/plugins#events)

*   [Examples](http://opencode.ai/docs/plugins#examples)
    *   [Send notifications](http://opencode.ai/docs/plugins#send-notifications)
    *   [.env protection](http://opencode.ai/docs/plugins#env-protection)
    *   [Inject environment variables](http://opencode.ai/docs/plugins#inject-environment-variables)
    *   [Custom tools](http://opencode.ai/docs/plugins#custom-tools)
    *   [Logging](http://opencode.ai/docs/plugins#logging)
    *   [Compaction hooks](http://opencode.ai/docs/plugins#compaction-hooks)

Plugins
=======

Write your own plugins to extend OpenCode.

Plugins allow you to extend OpenCode by hooking into various events and customizing behavior. You can create plugins to add new features, integrate with external services, or modify OpenCode’s default behavior.

For examples, check out the [plugins](http://opencode.ai/docs/ecosystem#plugins) created by the community.

* * *

[Use a plugin](http://opencode.ai/docs/plugins#use-a-plugin)
------------------------------------------------------------

There are two ways to load plugins.

* * *

### [From local files](http://opencode.ai/docs/plugins#from-local-files)

Place JavaScript or TypeScript files in the plugin directory.

*   `.opencode/plugins/` - Project-level plugins
*   `~/.config/opencode/plugins/` - Global plugins

Files in these directories are automatically loaded at startup.

* * *

### [From npm](http://opencode.ai/docs/plugins#from-npm)

Specify npm packages in your config file.

opencode.json

`{  "$schema": "https://opencode.ai/config.json",  "plugin": ["opencode-helicone-session", "opencode-wakatime", "@my-org/custom-plugin"]}`

Both regular and scoped npm packages are supported.

Browse available plugins in the [ecosystem](http://opencode.ai/docs/ecosystem#plugins).

* * *

### [How plugins are installed](http://opencode.ai/docs/plugins#how-plugins-are-installed)

**npm plugins** are installed automatically using Bun at startup. Packages and their dependencies are cached in `~/.cache/opencode/node_modules/`.

**Local plugins** are loaded directly from the plugin directory. To use external packages, you must create a `package.json` within your config directory (see [Dependencies](http://opencode.ai/docs/plugins#dependencies)), or publish the plugin to npm and [add it to your config](http://opencode.ai/docs/config#plugins).

* * *

### [Load order](http://opencode.ai/docs/plugins#load-order)

Plugins are loaded from all sources and all hooks run in sequence. The load order is:

1.   Global config (`~/.config/opencode/opencode.json`)
2.   Project config (`opencode.json`)
3.   Global plugin directory (`~/.config/opencode/plugins/`)
4.   Project plugin directory (`.opencode/plugins/`)

Duplicate npm packages with the same name and version are loaded once. However, a local plugin and an npm plugin with similar names are both loaded separately.

* * *

[Create a plugin](http://opencode.ai/docs/plugins#create-a-plugin)
------------------------------------------------------------------

A plugin is a **JavaScript/TypeScript module** that exports one or more plugin functions. Each function receives a context object and returns a hooks object.

* * *

### [Dependencies](http://opencode.ai/docs/plugins#dependencies)

Local plugins and custom tools can use external npm packages. Add a `package.json` to your config directory with the dependencies you need.

.opencode/package.json

`{  "dependencies": {    "shescape": "^2.1.0"  }}`

OpenCode runs `bun install` at startup to install these. Your plugins and tools can then import them.

.opencode/plugins/my-plugin.ts

```
import { escape } from "shescape"
export const MyPlugin = async (ctx) => {  return {    "tool.execute.before": async (input, output) => {      if (input.tool === "bash") {        output.args.command = escape(output.args.command)      }    },  }}
```

* * *

### [Basic structure](http://opencode.ai/docs/plugins#basic-structure)

.opencode/plugins/example.js

```
export const MyPlugin = async ({ project, client, $, directory, worktree }) => {  console.log("Plugin initialized!")
  return {    // Hook implementations go here  }}
```

The plugin function receives:

*   `project`: The current project information.
*   `directory`: The current working directory.
*   `worktree`: The git worktree path.
*   `client`: An opencode SDK client for interacting with the AI.
*   `$`: Bun’s [shell API](https://bun.com/docs/runtime/shell) for executing commands.

* * *

### [TypeScript support](http://opencode.ai/docs/plugins#typescript-support)

For TypeScript plugins, you can import types from the plugin package:

my-plugin.ts

```
import type { Plugin } from "@opencode-ai/plugin"
export const MyPlugin: Plugin = async ({ project, client, $, directory, worktree }) => {  return {    // Type-safe hook implementations  }}
```

* * *

### [Events](http://opencode.ai/docs/plugins#events)

Plugins can subscribe to events as seen below in the Examples section. Here is a list of the different events available.

#### [Command Events](http://opencode.ai/docs/plugins#command-events)

*   `command.executed`

#### [File Events](http://opencode.ai/docs/plugins#file-events)

*   `file.edited`
*   `file.watcher.updated`

#### [Installation Events](http://opencode.ai/docs/plugins#installation-events)

*   `installation.updated`

#### [LSP Events](http://opencode.ai/docs/plugins#lsp-events)

*   `lsp.client.diagnostics`
*   `lsp.updated`

#### [Message Events](http://opencode.ai/docs/plugins#message-events)

*   `message.part.removed`
*   `message.part.updated`
*   `message.removed`
*   `message.updated`

#### [Permission Events](http://opencode.ai/docs/plugins#permission-events)

*   `permission.asked`
*   `permission.replied`

#### [Server Events](http://opencode.ai/docs/plugins#server-events)

*   `server.connected`

#### [Session Events](http://opencode.ai/docs/plugins#session-events)

*   `session.created`
*   `session.compacted`
*   `session.deleted`
*   `session.diff`
*   `session.error`
*   `session.idle`
*   `session.status`
*   `session.updated`

#### [Todo Events](http://opencode.ai/docs/plugins#todo-events)

*   `todo.updated`

#### [Shell Events](http://opencode.ai/docs/plugins#shell-events)

*   `shell.env`

#### [Tool Events](http://opencode.ai/docs/plugins#tool-events)

*   `tool.execute.after`
*   `tool.execute.before`

#### [TUI Events](http://opencode.ai/docs/plugins#tui-events)

*   `tui.prompt.append`
*   `tui.command.execute`
*   `tui.toast.show`

* * *

[Examples](http://opencode.ai/docs/plugins#examples)
----------------------------------------------------

Here are some examples of plugins you can use to extend opencode.

* * *

### [Send notifications](http://opencode.ai/docs/plugins#send-notifications)

Send notifications when certain events occur:

.opencode/plugins/notification.js

`export const NotificationPlugin = async ({ project, client, $, directory, worktree }) => {  return {    event: async ({ event }) => {      // Send notification on session completion      if (event.type === "session.idle") {        await $`osascript -e 'display notification "Session completed!" with title "opencode"'`      }    },  }}`

We are using `osascript` to run AppleScript on macOS. Here we are using it to send notifications.

Note

If you’re using the OpenCode desktop app, it can send system notifications automatically when a response is ready or when a session errors.

* * *

### [.env protection](http://opencode.ai/docs/plugins#env-protection)

Prevent opencode from reading `.env` files:

.opencode/plugins/env-protection.js

`export const EnvProtection = async ({ project, client, $, directory, worktree }) => {  return {    "tool.execute.before": async (input, output) => {      if (input.tool === "read" && output.args.filePath.includes(".env")) {        throw new Error("Do not read .env files")      }    },  }}`

* * *

### [Inject environment variables](http://opencode.ai/docs/plugins#inject-environment-variables)

Inject environment variables into all shell execution (AI tools and user terminals):

.opencode/plugins/inject-env.js

`export const InjectEnvPlugin = async () => {  return {    "shell.env": async (input, output) => {      output.env.MY_API_KEY = "secret"      output.env.PROJECT_ROOT = input.cwd    },  }}`

* * *

### [Custom tools](http://opencode.ai/docs/plugins#custom-tools)

Plugins can also add custom tools to opencode:

.opencode/plugins/custom-tools.ts

```
import { type Plugin, tool } from "@opencode-ai/plugin"
export const CustomToolsPlugin: Plugin = async (ctx) => {  return {    tool: {      mytool: tool({        description: "This is a custom tool",        args: {          foo: tool.schema.string(),        },        async execute(args, context) {          const { directory, worktree } = context          return `Hello ${args.foo} from ${directory} (worktree: ${worktree})`        },      }),    },  }}
```

The `tool` helper creates a custom tool that opencode can call. It takes a Zod schema function and returns a tool definition with:

*   `description`: What the tool does
*   `args`: Zod schema for the tool’s arguments
*   `execute`: Function that runs when the tool is called

Your custom tools will be available to opencode alongside built-in tools.

* * *

### [Logging](http://opencode.ai/docs/plugins#logging)

Use `client.app.log()` instead of `console.log` for structured logging:

.opencode/plugins/my-plugin.ts

`export const MyPlugin = async ({ client }) => {  await client.app.log({    body: {      service: "my-plugin",      level: "info",      message: "Plugin initialized",      extra: { foo: "bar" },    },  })}`

Levels: `debug`, `info`, `warn`, `error`. See [SDK documentation](https://opencode.ai/docs/sdk) for details.

* * *

### [Compaction hooks](http://opencode.ai/docs/plugins#compaction-hooks)

Customize the context included when a session is compacted:

.opencode/plugins/compaction.ts

```
import type { Plugin } from "@opencode-ai/plugin"
export const CompactionPlugin: Plugin = async (ctx) => {  return {    "experimental.session.compacting": async (input, output) => {      // Inject additional context into the compaction prompt      output.context.push(`## Custom Context
Include any state that should persist across compaction:- Current task status- Important decisions made- Files being actively worked on`)    },  }}
```

The `experimental.session.compacting` hook fires before the LLM generates a continuation summary. Use it to inject domain-specific context that the default compaction prompt would miss.

You can also replace the compaction prompt entirely by setting `output.prompt`:

.opencode/plugins/custom-compaction.ts

```
import type { Plugin } from "@opencode-ai/plugin"
export const CustomCompactionPlugin: Plugin = async (ctx) => {  return {    "experimental.session.compacting": async (input, output) => {      // Replace the entire compaction prompt      output.prompt = `You are generating a continuation prompt for a multi-agent swarm session.
Summarize:1. The current task and its status2. Which files are being modified and by whom3. Any blockers or dependencies between agents4. The next steps to complete the work
Format as a structured prompt that a new agent can use to resume work.`    },  }}
```

When `output.prompt` is set, it completely replaces the default compaction prompt. The `output.context` array is ignored in this case.

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/plugins.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord)Select language 

© [Anomaly](https://anoma.ly/)

Last updated: Feb 15, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with direct HTML fallback support.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
