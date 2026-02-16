----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/mcp
- Keywords: moon, moonrepo, docs, monorepo, build, guides, mcp
- Summary: [Model Context Protocol](https://modelcontextprotocol.io) (MCP) is an open standard that enables AI
----

Source: https://moonrepo.dev/docs/guides/mcp

# MCP integration

v1.37.0

[Model Context Protocol](https://modelcontextprotocol.io) (MCP) is an open standard that enables AI
models to interact with external tools and services through a unified interface. The moon CLI
contains an MCP server that you can register with your code editor to allow LLMs to use moon
directly.

## Setup

### Claude Code

To use [MCP servers in Claude Code](https://docs.anthropic.com/en/docs/claude-code/mcp), run the
following command in your terminal:

```
claude mcp add moon -s project -e MOON_WORKSPACE_ROOT=/absolute/path/to/your/moon/workspace -- moon mcp
```

Or create an `.mcp.json` file in your project directory.

```
{  "mcpServers": {    "moon": {      "command": "moon",      "args": ["mcp"],      "env": {        "MOON_WORKSPACE_ROOT": "/absolute/path/to/your/moon/workspace"      }    }  }}
```

### Cursor

To use [MCP servers in Cursor](https://docs.cursor.com/context/model-context-protocol), create a
`.cursor/mcp.json` file in your project directory, or `~/.cursor/mcp.json` globally, with the
following content:

.cursor/mcp.json

```
{  "mcpServers": {    "moon": {      "command": "moon",      "args": ["mcp"],      "env": {        "MOON_WORKSPACE_ROOT": "/absolute/path/to/your/moon/workspace"      }    }  }}
```

Once configured, the moon MCP server should appear in the "Available Tools" section on the MCP
settings page in Cursor.

### VS Code

To use MCP servers in VS Code, you must have the
[Copilot Chat](https://code.visualstudio.com/docs/copilot/chat/copilot-chat) extension installed.
Once installed, create a `.vscode/mcp.json` file with the following content:

.vscode/mcp.json

```
{  "servers": {    "moon": {      "type": "stdio",      "command": "moon",      "args": ["mcp"],      // >= 1.102 (June 2025)      "cwd": "${workspaceFolder}",      // Older versions      "env": {        "MOON_WORKSPACE_ROOT": "${workspaceFolder}"      }    }  }}
```

Once your MCP server is configured, you can use it with
[GitHub Copilot’s agent mode](https://code.visualstudio.com/docs/copilot/chat/chat-agent-mode):

- Open the Copilot Chat view in VS Code

- Enable agent mode using the mode select dropdown

- Toggle on moon's MCP tools using the "Tools" button

### Zed

To use [MCP servers in Zed](https://zed.dev/docs/ai/mcp), create a `.zed/settings.json` file in your
project directory, or `~/.config/zed/settings.json` globally, with the following content:

.zed/settings.json

```
{  "context_servers": {    "moon": {      "command": {        "path": "moon",        "args": ["mcp"],        "env": {          "MOON_WORKSPACE_ROOT": "/absolute/path/to/your/moon/workspace"        }      }    }  }}
```

Once your MCP server is configured, you'll need to enable the tools using the following steps:

- Open the Agent panel in Zed

- Click the Write/Ask toggle button and go to "Configure Profiles"

- Click "Customize" in the Ask section

- Click "Configure MCP Tools"

- Enable each tool under the "moon" section

## Available tools

The following tools are available in the moon MCP server and can be executed by LLMs using agent
mode.

- `get_project` - Get a project and its tasks by `id`.

- `get_projects` - Get all projects.

- `get_task` - Get a task by `target`.

- `get_tasks` - Get all tasks.

- `get_touched_files` - Gets touched files between base and head revisions. v1.38.0

- `sync_projects` - Runs the `SyncProject` action for one or many projects by `id`. v1.38.0

- `sync_workspace` - Runs the `SyncWorkspace` action. v1.38.0

info

The
[request and response shapes](https://github.com/moonrepo/moon/blob/master/packages/types/src/mcp.ts)
for these tools are defined as TypeScript types in the
[`@moonrepo/types`](https://www.npmjs.com/package/@moonrepo/types) package.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
