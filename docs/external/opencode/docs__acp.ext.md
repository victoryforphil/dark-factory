----
## External Docs Snapshot // opencode

- Captured: 2026-02-16T07:29:35.337Z
- Source root: https://opencode.ai/docs
- Source page: /docs/acp
- Keywords: opencode, docs, ai coding assistant, cli, acp
- Summary: [Skip to content](http://opencode.ai/docs/acp#_top)
----

Source: https://opencode.ai/docs/acp

ACP Support | OpenCode
===============
[Skip to content](http://opencode.ai/docs/acp#_top)

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

*   [Overview](http://opencode.ai/docs/acp#_top)
*   [Configure](http://opencode.ai/docs/acp#configure)
    *   [Zed](http://opencode.ai/docs/acp#zed)
    *   [JetBrains IDEs](http://opencode.ai/docs/acp#jetbrains-ides)
    *   [Avante.nvim](http://opencode.ai/docs/acp#avantenvim)
    *   [CodeCompanion.nvim](http://opencode.ai/docs/acp#codecompanionnvim)

*   [Support](http://opencode.ai/docs/acp#support)

On this page
------------

*   [Overview](http://opencode.ai/docs/acp#_top)
*   [Configure](http://opencode.ai/docs/acp#configure)
    *   [Zed](http://opencode.ai/docs/acp#zed)
    *   [JetBrains IDEs](http://opencode.ai/docs/acp#jetbrains-ides)
    *   [Avante.nvim](http://opencode.ai/docs/acp#avantenvim)
    *   [CodeCompanion.nvim](http://opencode.ai/docs/acp#codecompanionnvim)

*   [Support](http://opencode.ai/docs/acp#support)

ACP Support
===========

Use OpenCode in any ACP-compatible editor.

OpenCode supports the [Agent Client Protocol](https://agentclientprotocol.com/) or (ACP), allowing you to use it directly in compatible editors and IDEs.

Tip

For a list of editors and tools that support ACP, check out the [ACP progress report](https://zed.dev/blog/acp-progress-report#available-now).

ACP is an open protocol that standardizes communication between code editors and AI coding agents.

* * *

[Configure](http://opencode.ai/docs/acp#configure)
--------------------------------------------------

To use OpenCode via ACP, configure your editor to run the `opencode acp` command.

The command starts OpenCode as an ACP-compatible subprocess that communicates with your editor over JSON-RPC via stdio.

Below are examples for popular editors that support ACP.

* * *

### [Zed](http://opencode.ai/docs/acp#zed)

Add to your [Zed](https://zed.dev/) configuration (`~/.config/zed/settings.json`):

~/.config/zed/settings.json

`{  "agent_servers": {    "OpenCode": {      "command": "opencode",      "args": ["acp"]    }  }}`

To open it, use the `agent: new thread` action in the **Command Palette**.

You can also bind a keyboard shortcut by editing your `keymap.json`:

keymap.json

`[  {    "bindings": {      "cmd-alt-o": [        "agent::NewExternalAgentThread",        {          "agent": {            "custom": {              "name": "OpenCode",              "command": {                "command": "opencode",                "args": ["acp"]              }            }          }        }      ]    }  }]`

* * *

### [JetBrains IDEs](http://opencode.ai/docs/acp#jetbrains-ides)

Add to your [JetBrains IDE](https://www.jetbrains.com/) acp.json according to the [documentation](https://www.jetbrains.com/help/ai-assistant/acp.html):

acp.json

`{  "agent_servers": {    "OpenCode": {      "command": "/absolute/path/bin/opencode",      "args": ["acp"]    }  }}`

To open it, use the new ‘OpenCode’ agent in the AI Chat agent selector.

* * *

### [Avante.nvim](http://opencode.ai/docs/acp#avantenvim)

Add to your [Avante.nvim](https://github.com/yetone/avante.nvim) configuration:

`{  acp_providers = {    ["opencode"] = {      command = "opencode",      args = { "acp" }    }  }}`

If you need to pass environment variables:

`{  acp_providers = {    ["opencode"] = {      command = "opencode",      args = { "acp" },      env = {        OPENCODE_API_KEY = os.getenv("OPENCODE_API_KEY")      }    }  }}`

* * *

### [CodeCompanion.nvim](http://opencode.ai/docs/acp#codecompanionnvim)

To use OpenCode as an ACP agent in [CodeCompanion.nvim](https://github.com/olimorris/codecompanion.nvim), add the following to your Neovim config:

`require("codecompanion").setup({  interactions = {    chat = {      adapter = {        name = "opencode",        model = "claude-sonnet-4",      },    },  },})`

This config sets up CodeCompanion to use OpenCode as the ACP agent for chat.

If you need to pass environment variables (like `OPENCODE_API_KEY`), refer to [Configuring Adapters: Environment Variables](https://codecompanion.olimorris.dev/getting-started#setting-an-api-key) in the CodeCompanion.nvim documentation for full details.

[Support](http://opencode.ai/docs/acp#support)
----------------------------------------------

OpenCode works the same via ACP as it does in the terminal. All features are supported:

Note

Some built-in slash commands like `/undo` and `/redo` are currently unsupported.

*   Built-in tools (file operations, terminal commands, etc.)
*   Custom tools and slash commands
*   MCP servers configured in your OpenCode config
*   Project-specific rules from `AGENTS.md`
*   Custom formatters and linters
*   Agents and permissions system

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/acp.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord)Select language 

© [Anomaly](https://anoma.ly/)

Last updated: Feb 15, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with direct HTML fallback support.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
