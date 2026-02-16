----
## External Docs Snapshot // opencode

- Captured: 2026-02-16T07:29:35.337Z
- Source root: https://opencode.ai/docs
- Source page: /docs/commands
- Keywords: opencode, docs, ai coding assistant, cli, commands
- Summary: [Skip to content](http://opencode.ai/docs/commands#_top)
----

Source: https://opencode.ai/docs/commands

Commands | OpenCode
===============
[Skip to content](http://opencode.ai/docs/commands#_top)

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

*   [Overview](http://opencode.ai/docs/commands#_top)
*   [Create command files](http://opencode.ai/docs/commands#create-command-files)
*   [Configure](http://opencode.ai/docs/commands#configure)
    *   [JSON](http://opencode.ai/docs/commands#json)
    *   [Markdown](http://opencode.ai/docs/commands#markdown)

*   [Prompt config](http://opencode.ai/docs/commands#prompt-config)
    *   [Arguments](http://opencode.ai/docs/commands#arguments)
    *   [Shell output](http://opencode.ai/docs/commands#shell-output)
    *   [File references](http://opencode.ai/docs/commands#file-references)

*   [Options](http://opencode.ai/docs/commands#options)
    *   [Template](http://opencode.ai/docs/commands#template)
    *   [Description](http://opencode.ai/docs/commands#description)
    *   [Agent](http://opencode.ai/docs/commands#agent)
    *   [Subtask](http://opencode.ai/docs/commands#subtask)
    *   [Model](http://opencode.ai/docs/commands#model)

*   [Built-in](http://opencode.ai/docs/commands#built-in)

On this page
------------

*   [Overview](http://opencode.ai/docs/commands#_top)
*   [Create command files](http://opencode.ai/docs/commands#create-command-files)
*   [Configure](http://opencode.ai/docs/commands#configure)
    *   [JSON](http://opencode.ai/docs/commands#json)
    *   [Markdown](http://opencode.ai/docs/commands#markdown)

*   [Prompt config](http://opencode.ai/docs/commands#prompt-config)
    *   [Arguments](http://opencode.ai/docs/commands#arguments)
    *   [Shell output](http://opencode.ai/docs/commands#shell-output)
    *   [File references](http://opencode.ai/docs/commands#file-references)

*   [Options](http://opencode.ai/docs/commands#options)
    *   [Template](http://opencode.ai/docs/commands#template)
    *   [Description](http://opencode.ai/docs/commands#description)
    *   [Agent](http://opencode.ai/docs/commands#agent)
    *   [Subtask](http://opencode.ai/docs/commands#subtask)
    *   [Model](http://opencode.ai/docs/commands#model)

*   [Built-in](http://opencode.ai/docs/commands#built-in)

Commands
========

Create custom commands for repetitive tasks.

Custom commands let you specify a prompt you want to run when that command is executed in the TUI.

`/my-command`

Custom commands are in addition to the built-in commands like `/init`, `/undo`, `/redo`, `/share`, `/help`. [Learn more](http://opencode.ai/docs/tui#commands).

* * *

[Create command files](http://opencode.ai/docs/commands#create-command-files)
-----------------------------------------------------------------------------

Create markdown files in the `commands/` directory to define custom commands.

Create `.opencode/commands/test.md`:

.opencode/commands/test.md

```
---description: Run tests with coverageagent: buildmodel: anthropic/claude-3-5-sonnet-20241022---
Run the full test suite with coverage report and show any failures.Focus on the failing tests and suggest fixes.
```

The frontmatter defines command properties. The content becomes the template.

Use the command by typing `/` followed by the command name.

`"/test"`

* * *

[Configure](http://opencode.ai/docs/commands#configure)
-------------------------------------------------------

You can add custom commands through the OpenCode config or by creating markdown files in the `commands/` directory.

* * *

### [JSON](http://opencode.ai/docs/commands#json)

Use the `command` option in your OpenCode [config](http://opencode.ai/docs/config):

opencode.jsonc

`{  "$schema": "https://opencode.ai/config.json",  "command": {    // This becomes the name of the command    "test": {      // This is the prompt that will be sent to the LLM      "template": "Run the full test suite with coverage report and show any failures.\nFocus on the failing tests and suggest fixes.",      // This is shown as the description in the TUI      "description": "Run tests with coverage",      "agent": "build",      "model": "anthropic/claude-3-5-sonnet-20241022"    }  }}`

Now you can run this command in the TUI:

`/test`

* * *

### [Markdown](http://opencode.ai/docs/commands#markdown)

You can also define commands using markdown files. Place them in:

*   Global: `~/.config/opencode/commands/`
*   Per-project: `.opencode/commands/`

~/.config/opencode/commands/test.md

```
---description: Run tests with coverageagent: buildmodel: anthropic/claude-3-5-sonnet-20241022---
Run the full test suite with coverage report and show any failures.Focus on the failing tests and suggest fixes.
```

The markdown file name becomes the command name. For example, `test.md` lets you run:

`/test`

* * *

[Prompt config](http://opencode.ai/docs/commands#prompt-config)
---------------------------------------------------------------

The prompts for the custom commands support several special placeholders and syntax.

* * *

### [Arguments](http://opencode.ai/docs/commands#arguments)

Pass arguments to commands using the `$ARGUMENTS` placeholder.

.opencode/commands/component.md

```
---description: Create a new component---
Create a new React component named $ARGUMENTS with TypeScript support.Include proper typing and basic structure.
```

Run the command with arguments:

`/component Button`

And `$ARGUMENTS` will be replaced with `Button`.

You can also access individual arguments using positional parameters:

*   `$1` - First argument
*   `$2` - Second argument
*   `$3` - Third argument
*   And so on…

For example:

.opencode/commands/create-file.md

```
---description: Create a new file with content---
Create a file named $1 in the directory $2with the following content: $3
```

Run the command:

`/create-file config.json src "{ \"key\": \"value\" }"`

This replaces:

*   `$1` with `config.json`
*   `$2` with `src`
*   `$3` with `{ "key": "value" }`

* * *

### [Shell output](http://opencode.ai/docs/commands#shell-output)

Use _!`command`_ to inject [bash command](http://opencode.ai/docs/tui#bash-commands) output into your prompt.

For example, to create a custom command that analyzes test coverage:

.opencode/commands/analyze-coverage.md

```
---description: Analyze test coverage---
Here are the current test results:!`npm test`
Based on these results, suggest improvements to increase coverage.
```

Or to review recent changes:

.opencode/commands/review-changes.md

```
---description: Review recent changes---
Recent git commits:!`git log --oneline -10`
Review these changes and suggest any improvements.
```

Commands run in your project’s root directory and their output becomes part of the prompt.

* * *

### [File references](http://opencode.ai/docs/commands#file-references)

Include files in your command using `@` followed by the filename.

.opencode/commands/review-component.md

```
---description: Review component---
Review the component in @src/components/Button.tsx.Check for performance issues and suggest improvements.
```

The file content gets included in the prompt automatically.

* * *

[Options](http://opencode.ai/docs/commands#options)
---------------------------------------------------

Let’s look at the configuration options in detail.

* * *

### [Template](http://opencode.ai/docs/commands#template)

The `template` option defines the prompt that will be sent to the LLM when the command is executed.

opencode.json

`{  "command": {    "test": {      "template": "Run the full test suite with coverage report and show any failures.\nFocus on the failing tests and suggest fixes."    }  }}`

This is a **required** config option.

* * *

### [Description](http://opencode.ai/docs/commands#description)

Use the `description` option to provide a brief description of what the command does.

opencode.json

`{  "command": {    "test": {      "description": "Run tests with coverage"    }  }}`

This is shown as the description in the TUI when you type in the command.

* * *

### [Agent](http://opencode.ai/docs/commands#agent)

Use the `agent` config to optionally specify which [agent](http://opencode.ai/docs/agents) should execute this command. If this is a [subagent](http://opencode.ai/docs/agents/#subagents) the command will trigger a subagent invocation by default. To disable this behavior, set `subtask` to `false`.

opencode.json

`{  "command": {    "review": {      "agent": "plan"    }  }}`

This is an **optional** config option. If not specified, defaults to your current agent.

* * *

### [Subtask](http://opencode.ai/docs/commands#subtask)

Use the `subtask` boolean to force the command to trigger a [subagent](http://opencode.ai/docs/agents/#subagents) invocation. This is useful if you want the command to not pollute your primary context and will **force** the agent to act as a subagent, even if `mode` is set to `primary` on the [agent](http://opencode.ai/docs/agents) configuration.

opencode.json

`{  "command": {    "analyze": {      "subtask": true    }  }}`

This is an **optional** config option.

* * *

### [Model](http://opencode.ai/docs/commands#model)

Use the `model` config to override the default model for this command.

opencode.json

`{  "command": {    "analyze": {      "model": "anthropic/claude-3-5-sonnet-20241022"    }  }}`

This is an **optional** config option.

* * *

[Built-in](http://opencode.ai/docs/commands#built-in)
-----------------------------------------------------

opencode includes several built-in commands like `/init`, `/undo`, `/redo`, `/share`, `/help`; [learn more](http://opencode.ai/docs/tui#commands).

Note

Custom commands can override built-in commands.

If you define a custom command with the same name, it will override the built-in command.

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/commands.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord)Select language 

© [Anomaly](https://anoma.ly/)

Last updated: Feb 15, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with direct HTML fallback support.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
