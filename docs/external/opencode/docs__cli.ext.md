----
## External Docs Snapshot // opencode

- Captured: 2026-02-16T07:29:35.337Z
- Source root: https://opencode.ai/docs
- Source page: /docs/cli
- Keywords: opencode, docs, ai coding assistant, cli, cli
- Summary: [Skip to content](http://opencode.ai/docs/cli#_top)
----

Source: https://opencode.ai/docs/cli

CLI | OpenCode
===============
[Skip to content](http://opencode.ai/docs/cli#_top)

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

*   [Overview](http://opencode.ai/docs/cli#_top)
    *   [tui](http://opencode.ai/docs/cli#tui)

*   [Commands](http://opencode.ai/docs/cli#commands)
    *   [agent](http://opencode.ai/docs/cli#agent)
    *   [attach](http://opencode.ai/docs/cli#attach)
    *   [auth](http://opencode.ai/docs/cli#auth)
    *   [github](http://opencode.ai/docs/cli#github)
    *   [mcp](http://opencode.ai/docs/cli#mcp)
    *   [models](http://opencode.ai/docs/cli#models)
    *   [run](http://opencode.ai/docs/cli#run-1)
    *   [serve](http://opencode.ai/docs/cli#serve)
    *   [session](http://opencode.ai/docs/cli#session)
    *   [stats](http://opencode.ai/docs/cli#stats)
    *   [export](http://opencode.ai/docs/cli#export)
    *   [import](http://opencode.ai/docs/cli#import)
    *   [web](http://opencode.ai/docs/cli#web)
    *   [acp](http://opencode.ai/docs/cli#acp)
    *   [uninstall](http://opencode.ai/docs/cli#uninstall)
    *   [upgrade](http://opencode.ai/docs/cli#upgrade)

*   [Global Flags](http://opencode.ai/docs/cli#global-flags)
*   [Environment variables](http://opencode.ai/docs/cli#environment-variables)
    *   [Experimental](http://opencode.ai/docs/cli#experimental)

On this page
------------

*   [Overview](http://opencode.ai/docs/cli#_top)
    *   [tui](http://opencode.ai/docs/cli#tui)

*   [Commands](http://opencode.ai/docs/cli#commands)
    *   [agent](http://opencode.ai/docs/cli#agent)
    *   [attach](http://opencode.ai/docs/cli#attach)
    *   [auth](http://opencode.ai/docs/cli#auth)
    *   [github](http://opencode.ai/docs/cli#github)
    *   [mcp](http://opencode.ai/docs/cli#mcp)
    *   [models](http://opencode.ai/docs/cli#models)
    *   [run](http://opencode.ai/docs/cli#run-1)
    *   [serve](http://opencode.ai/docs/cli#serve)
    *   [session](http://opencode.ai/docs/cli#session)
    *   [stats](http://opencode.ai/docs/cli#stats)
    *   [export](http://opencode.ai/docs/cli#export)
    *   [import](http://opencode.ai/docs/cli#import)
    *   [web](http://opencode.ai/docs/cli#web)
    *   [acp](http://opencode.ai/docs/cli#acp)
    *   [uninstall](http://opencode.ai/docs/cli#uninstall)
    *   [upgrade](http://opencode.ai/docs/cli#upgrade)

*   [Global Flags](http://opencode.ai/docs/cli#global-flags)
*   [Environment variables](http://opencode.ai/docs/cli#environment-variables)
    *   [Experimental](http://opencode.ai/docs/cli#experimental)

CLI
===

OpenCode CLI options and commands.

The OpenCode CLI by default starts the [TUI](http://opencode.ai/docs/tui) when run without any arguments.

Terminal window

`opencode`

But it also accepts commands as documented on this page. This allows you to interact with OpenCode programmatically.

Terminal window

`opencode run "Explain how closures work in JavaScript"`

* * *

### [tui](http://opencode.ai/docs/cli#tui)

Start the OpenCode terminal user interface.

Terminal window

`opencode [project]`

#### [Flags](http://opencode.ai/docs/cli#flags)

| Flag | Short | Description |
| --- | --- | --- |
| `--continue` | `-c` | Continue the last session |
| `--session` | `-s` | Session ID to continue |
| `--fork` |  | Fork the session when continuing (use with `--continue` or `--session`) |
| `--prompt` |  | Prompt to use |
| `--model` | `-m` | Model to use in the form of provider/model |
| `--agent` |  | Agent to use |
| `--port` |  | Port to listen on |
| `--hostname` |  | Hostname to listen on |

* * *

[Commands](http://opencode.ai/docs/cli#commands)
------------------------------------------------

The OpenCode CLI also has the following commands.

* * *

### [agent](http://opencode.ai/docs/cli#agent)

Manage agents for OpenCode.

Terminal window

`opencode agent [command]`

* * *

### [attach](http://opencode.ai/docs/cli#attach)

Attach a terminal to an already running OpenCode backend server started via `serve` or `web` commands.

Terminal window

`opencode attach [url]`

This allows using the TUI with a remote OpenCode backend. For example:

Terminal window

```
# Start the backend server for web/mobile accessopencode web --port 4096 --hostname 0.0.0.0
# In another terminal, attach the TUI to the running backendopencode attach http://10.20.30.40:4096
```

#### [Flags](http://opencode.ai/docs/cli#flags-1)

| Flag | Short | Description |
| --- | --- | --- |
| `--dir` |  | Working directory to start TUI in |
| `--session` | `-s` | Session ID to continue |

* * *

#### [create](http://opencode.ai/docs/cli#create)

Create a new agent with custom configuration.

Terminal window

`opencode agent create`

This command will guide you through creating a new agent with a custom system prompt and tool configuration.

* * *

#### [list](http://opencode.ai/docs/cli#list)

List all available agents.

Terminal window

`opencode agent list`

* * *

### [auth](http://opencode.ai/docs/cli#auth)

Command to manage credentials and login for providers.

Terminal window

`opencode auth [command]`

* * *

#### [login](http://opencode.ai/docs/cli#login)

OpenCode is powered by the provider list at [Models.dev](https://models.dev/), so you can use `opencode auth login` to configure API keys for any provider you’d like to use. This is stored in `~/.local/share/opencode/auth.json`.

Terminal window

`opencode auth login`

When OpenCode starts up it loads the providers from the credentials file. And if there are any keys defined in your environments or a `.env` file in your project.

* * *

#### [list](http://opencode.ai/docs/cli#list-1)

Lists all the authenticated providers as stored in the credentials file.

Terminal window

`opencode auth list`

Or the short version.

Terminal window

`opencode auth ls`

* * *

#### [logout](http://opencode.ai/docs/cli#logout)

Logs you out of a provider by clearing it from the credentials file.

Terminal window

`opencode auth logout`

* * *

### [github](http://opencode.ai/docs/cli#github)

Manage the GitHub agent for repository automation.

Terminal window

`opencode github [command]`

* * *

#### [install](http://opencode.ai/docs/cli#install)

Install the GitHub agent in your repository.

Terminal window

`opencode github install`

This sets up the necessary GitHub Actions workflow and guides you through the configuration process. [Learn more](http://opencode.ai/docs/github).

* * *

#### [run](http://opencode.ai/docs/cli#run)

Run the GitHub agent. This is typically used in GitHub Actions.

Terminal window

`opencode github run`

##### [Flags](http://opencode.ai/docs/cli#flags-2)

| Flag | Description |
| --- | --- |
| `--event` | GitHub mock event to run the agent for |
| `--token` | GitHub personal access token |

* * *

### [mcp](http://opencode.ai/docs/cli#mcp)

Manage Model Context Protocol servers.

Terminal window

`opencode mcp [command]`

* * *

#### [add](http://opencode.ai/docs/cli#add)

Add an MCP server to your configuration.

Terminal window

`opencode mcp add`

This command will guide you through adding either a local or remote MCP server.

* * *

#### [list](http://opencode.ai/docs/cli#list-2)

List all configured MCP servers and their connection status.

Terminal window

`opencode mcp list`

Or use the short version.

Terminal window

`opencode mcp ls`

* * *

#### [auth](http://opencode.ai/docs/cli#auth-1)

Authenticate with an OAuth-enabled MCP server.

Terminal window

`opencode mcp auth [name]`

If you don’t provide a server name, you’ll be prompted to select from available OAuth-capable servers.

You can also list OAuth-capable servers and their authentication status.

Terminal window

`opencode mcp auth list`

Or use the short version.

Terminal window

`opencode mcp auth ls`

* * *

#### [logout](http://opencode.ai/docs/cli#logout-1)

Remove OAuth credentials for an MCP server.

Terminal window

`opencode mcp logout [name]`

* * *

#### [debug](http://opencode.ai/docs/cli#debug)

Debug OAuth connection issues for an MCP server.

Terminal window

`opencode mcp debug <name>`

* * *

### [models](http://opencode.ai/docs/cli#models)

List all available models from configured providers.

Terminal window

`opencode models [provider]`

This command displays all models available across your configured providers in the format `provider/model`.

This is useful for figuring out the exact model name to use in [your config](http://opencode.ai/docs/config/).

You can optionally pass a provider ID to filter models by that provider.

Terminal window

`opencode models anthropic`

#### [Flags](http://opencode.ai/docs/cli#flags-3)

| Flag | Description |
| --- | --- |
| `--refresh` | Refresh the models cache from models.dev |
| `--verbose` | Use more verbose model output (includes metadata like costs) |

Use the `--refresh` flag to update the cached model list. This is useful when new models have been added to a provider and you want to see them in OpenCode.

Terminal window

`opencode models --refresh`

* * *

### [run](http://opencode.ai/docs/cli#run-1)

Run opencode in non-interactive mode by passing a prompt directly.

Terminal window

`opencode run [message..]`

This is useful for scripting, automation, or when you want a quick answer without launching the full TUI. For example.

Terminal window

`opencode run Explain the use of context in Go`

You can also attach to a running `opencode serve` instance to avoid MCP server cold boot times on every run:

Terminal window

```
# Start a headless server in one terminalopencode serve
# In another terminal, run commands that attach to itopencode run --attach http://localhost:4096 "Explain async/await in JavaScript"
```

#### [Flags](http://opencode.ai/docs/cli#flags-4)

| Flag | Short | Description |
| --- | --- | --- |
| `--command` |  | The command to run, use message for args |
| `--continue` | `-c` | Continue the last session |
| `--session` | `-s` | Session ID to continue |
| `--fork` |  | Fork the session when continuing (use with `--continue` or `--session`) |
| `--share` |  | Share the session |
| `--model` | `-m` | Model to use in the form of provider/model |
| `--agent` |  | Agent to use |
| `--file` | `-f` | File(s) to attach to message |
| `--format` |  | Format: default (formatted) or json (raw JSON events) |
| `--title` |  | Title for the session (uses truncated prompt if no value provided) |
| `--attach` |  | Attach to a running opencode server (e.g., [http://localhost:4096](http://localhost:4096/)) |
| `--port` |  | Port for the local server (defaults to random port) |

* * *

### [serve](http://opencode.ai/docs/cli#serve)

Start a headless OpenCode server for API access. Check out the [server docs](http://opencode.ai/docs/server) for the full HTTP interface.

Terminal window

`opencode serve`

This starts an HTTP server that provides API access to opencode functionality without the TUI interface. Set `OPENCODE_SERVER_PASSWORD` to enable HTTP basic auth (username defaults to `opencode`).

#### [Flags](http://opencode.ai/docs/cli#flags-5)

| Flag | Description |
| --- | --- |
| `--port` | Port to listen on |
| `--hostname` | Hostname to listen on |
| `--mdns` | Enable mDNS discovery |
| `--cors` | Additional browser origin(s) to allow CORS |

* * *

### [session](http://opencode.ai/docs/cli#session)

Manage OpenCode sessions.

Terminal window

`opencode session [command]`

* * *

#### [list](http://opencode.ai/docs/cli#list-3)

List all OpenCode sessions.

Terminal window

`opencode session list`

##### [Flags](http://opencode.ai/docs/cli#flags-6)

| Flag | Short | Description |
| --- | --- | --- |
| `--max-count` | `-n` | Limit to N most recent sessions |
| `--format` |  | Output format: table or json (table) |

* * *

### [stats](http://opencode.ai/docs/cli#stats)

Show token usage and cost statistics for your OpenCode sessions.

Terminal window

`opencode stats`

#### [Flags](http://opencode.ai/docs/cli#flags-7)

| Flag | Description |
| --- | --- |
| `--days` | Show stats for the last N days (all time) |
| `--tools` | Number of tools to show (all) |
| `--models` | Show model usage breakdown (hidden by default). Pass a number to show top N |
| `--project` | Filter by project (all projects, empty string: current project) |

* * *

### [export](http://opencode.ai/docs/cli#export)

Export session data as JSON.

Terminal window

`opencode export [sessionID]`

If you don’t provide a session ID, you’ll be prompted to select from available sessions.

* * *

### [import](http://opencode.ai/docs/cli#import)

Import session data from a JSON file or OpenCode share URL.

Terminal window

`opencode import <file>`

You can import from a local file or an OpenCode share URL.

Terminal window

`opencode import session.jsonopencode import https://opncd.ai/s/abc123`

* * *

### [web](http://opencode.ai/docs/cli#web)

Start a headless OpenCode server with a web interface.

Terminal window

`opencode web`

This starts an HTTP server and opens a web browser to access OpenCode through a web interface. Set `OPENCODE_SERVER_PASSWORD` to enable HTTP basic auth (username defaults to `opencode`).

#### [Flags](http://opencode.ai/docs/cli#flags-8)

| Flag | Description |
| --- | --- |
| `--port` | Port to listen on |
| `--hostname` | Hostname to listen on |
| `--mdns` | Enable mDNS discovery |
| `--cors` | Additional browser origin(s) to allow CORS |

* * *

### [acp](http://opencode.ai/docs/cli#acp)

Start an ACP (Agent Client Protocol) server.

Terminal window

`opencode acp`

This command starts an ACP server that communicates via stdin/stdout using nd-JSON.

#### [Flags](http://opencode.ai/docs/cli#flags-9)

| Flag | Description |
| --- | --- |
| `--cwd` | Working directory |
| `--port` | Port to listen on |
| `--hostname` | Hostname to listen on |

* * *

### [uninstall](http://opencode.ai/docs/cli#uninstall)

Uninstall OpenCode and remove all related files.

Terminal window

`opencode uninstall`

#### [Flags](http://opencode.ai/docs/cli#flags-10)

| Flag | Short | Description |
| --- | --- | --- |
| `--keep-config` | `-c` | Keep configuration files |
| `--keep-data` | `-d` | Keep session data and snapshots |
| `--dry-run` |  | Show what would be removed without removing |
| `--force` | `-f` | Skip confirmation prompts |

* * *

### [upgrade](http://opencode.ai/docs/cli#upgrade)

Updates opencode to the latest version or a specific version.

Terminal window

`opencode upgrade [target]`

To upgrade to the latest version.

Terminal window

`opencode upgrade`

To upgrade to a specific version.

Terminal window

`opencode upgrade v0.1.48`

#### [Flags](http://opencode.ai/docs/cli#flags-11)

| Flag | Short | Description |
| --- | --- | --- |
| `--method` | `-m` | The installation method that was used; curl, npm, pnpm, bun, brew |

* * *

[Global Flags](http://opencode.ai/docs/cli#global-flags)
--------------------------------------------------------

The opencode CLI takes the following global flags.

| Flag | Short | Description |
| --- | --- | --- |
| `--help` | `-h` | Display help |
| `--version` | `-v` | Print version number |
| `--print-logs` |  | Print logs to stderr |
| `--log-level` |  | Log level (DEBUG, INFO, WARN, ERROR) |

* * *

[Environment variables](http://opencode.ai/docs/cli#environment-variables)
--------------------------------------------------------------------------

OpenCode can be configured using environment variables.

| Variable | Type | Description |
| --- | --- | --- |
| `OPENCODE_AUTO_SHARE` | boolean | Automatically share sessions |
| `OPENCODE_GIT_BASH_PATH` | string | Path to Git Bash executable on Windows |
| `OPENCODE_CONFIG` | string | Path to config file |
| `OPENCODE_CONFIG_DIR` | string | Path to config directory |
| `OPENCODE_CONFIG_CONTENT` | string | Inline json config content |
| `OPENCODE_DISABLE_AUTOUPDATE` | boolean | Disable automatic update checks |
| `OPENCODE_DISABLE_PRUNE` | boolean | Disable pruning of old data |
| `OPENCODE_DISABLE_TERMINAL_TITLE` | boolean | Disable automatic terminal title updates |
| `OPENCODE_PERMISSION` | string | Inlined json permissions config |
| `OPENCODE_DISABLE_DEFAULT_PLUGINS` | boolean | Disable default plugins |
| `OPENCODE_DISABLE_LSP_DOWNLOAD` | boolean | Disable automatic LSP server downloads |
| `OPENCODE_ENABLE_EXPERIMENTAL_MODELS` | boolean | Enable experimental models |
| `OPENCODE_DISABLE_AUTOCOMPACT` | boolean | Disable automatic context compaction |
| `OPENCODE_DISABLE_CLAUDE_CODE` | boolean | Disable reading from `.claude` (prompt + skills) |
| `OPENCODE_DISABLE_CLAUDE_CODE_PROMPT` | boolean | Disable reading `~/.claude/CLAUDE.md` |
| `OPENCODE_DISABLE_CLAUDE_CODE_SKILLS` | boolean | Disable loading `.claude/skills` |
| `OPENCODE_DISABLE_MODELS_FETCH` | boolean | Disable fetching models from remote sources |
| `OPENCODE_FAKE_VCS` | string | Fake VCS provider for testing purposes |
| `OPENCODE_DISABLE_FILETIME_CHECK` | boolean | Disable file time checking for optimization |
| `OPENCODE_CLIENT` | string | Client identifier (defaults to `cli`) |
| `OPENCODE_ENABLE_EXA` | boolean | Enable Exa web search tools |
| `OPENCODE_SERVER_PASSWORD` | string | Enable basic auth for `serve`/`web` |
| `OPENCODE_SERVER_USERNAME` | string | Override basic auth username (default `opencode`) |
| `OPENCODE_MODELS_URL` | string | Custom URL for fetching models configuration |

* * *

### [Experimental](http://opencode.ai/docs/cli#experimental)

These environment variables enable experimental features that may change or be removed.

| Variable | Type | Description |
| --- | --- | --- |
| `OPENCODE_EXPERIMENTAL` | boolean | Enable all experimental features |
| `OPENCODE_EXPERIMENTAL_ICON_DISCOVERY` | boolean | Enable icon discovery |
| `OPENCODE_EXPERIMENTAL_DISABLE_COPY_ON_SELECT` | boolean | Disable copy on select in TUI |
| `OPENCODE_EXPERIMENTAL_BASH_DEFAULT_TIMEOUT_MS` | number | Default timeout for bash commands in ms |
| `OPENCODE_EXPERIMENTAL_OUTPUT_TOKEN_MAX` | number | Max output tokens for LLM responses |
| `OPENCODE_EXPERIMENTAL_FILEWATCHER` | boolean | Enable file watcher for entire dir |
| `OPENCODE_EXPERIMENTAL_OXFMT` | boolean | Enable oxfmt formatter |
| `OPENCODE_EXPERIMENTAL_LSP_TOOL` | boolean | Enable experimental LSP tool |
| `OPENCODE_EXPERIMENTAL_DISABLE_FILEWATCHER` | boolean | Disable file watcher |
| `OPENCODE_EXPERIMENTAL_EXA` | boolean | Enable experimental Exa features |
| `OPENCODE_EXPERIMENTAL_LSP_TY` | boolean | Enable experimental LSP type checking |
| `OPENCODE_EXPERIMENTAL_MARKDOWN` | boolean | Enable experimental markdown features |
| `OPENCODE_EXPERIMENTAL_PLAN_MODE` | boolean | Enable plan mode |

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/cli.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord)Select language 

© [Anomaly](https://anoma.ly/)

Last updated: Feb 15, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with direct HTML fallback support.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
