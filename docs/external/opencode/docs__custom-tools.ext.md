----
## External Docs Snapshot // opencode

- Captured: 2026-02-16T07:29:35.337Z
- Source root: https://opencode.ai/docs
- Source page: /docs/custom-tools
- Keywords: opencode, docs, ai coding assistant, cli, custom tools
- Summary: [Skip to content](http://opencode.ai/docs/custom-tools#_top)
----

Source: https://opencode.ai/docs/custom-tools

Custom Tools | OpenCode
===============
[Skip to content](http://opencode.ai/docs/custom-tools#_top)

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

*   [Overview](http://opencode.ai/docs/custom-tools#_top)
*   [Creating a tool](http://opencode.ai/docs/custom-tools#creating-a-tool)
    *   [Location](http://opencode.ai/docs/custom-tools#location)
    *   [Structure](http://opencode.ai/docs/custom-tools#structure)
    *   [Arguments](http://opencode.ai/docs/custom-tools#arguments)
    *   [Context](http://opencode.ai/docs/custom-tools#context)

*   [Examples](http://opencode.ai/docs/custom-tools#examples)
    *   [Write a tool in Python](http://opencode.ai/docs/custom-tools#write-a-tool-in-python)

On this page
------------

*   [Overview](http://opencode.ai/docs/custom-tools#_top)
*   [Creating a tool](http://opencode.ai/docs/custom-tools#creating-a-tool)
    *   [Location](http://opencode.ai/docs/custom-tools#location)
    *   [Structure](http://opencode.ai/docs/custom-tools#structure)
    *   [Arguments](http://opencode.ai/docs/custom-tools#arguments)
    *   [Context](http://opencode.ai/docs/custom-tools#context)

*   [Examples](http://opencode.ai/docs/custom-tools#examples)
    *   [Write a tool in Python](http://opencode.ai/docs/custom-tools#write-a-tool-in-python)

Custom Tools
============

Create tools the LLM can call in opencode.

Custom tools are functions you create that the LLM can call during conversations. They work alongside opencode’s [built-in tools](http://opencode.ai/docs/tools) like `read`, `write`, and `bash`.

* * *

[Creating a tool](http://opencode.ai/docs/custom-tools#creating-a-tool)
-----------------------------------------------------------------------

Tools are defined as **TypeScript** or **JavaScript** files. However, the tool definition can invoke scripts written in **any language** — TypeScript or JavaScript is only used for the tool definition itself.

* * *

### [Location](http://opencode.ai/docs/custom-tools#location)

They can be defined:

*   Locally by placing them in the `.opencode/tools/` directory of your project.
*   Or globally, by placing them in `~/.config/opencode/tools/`.

* * *

### [Structure](http://opencode.ai/docs/custom-tools#structure)

The easiest way to create tools is using the `tool()` helper which provides type-safety and validation.

.opencode/tools/database.ts

```
import { tool } from "@opencode-ai/plugin"
export default tool({  description: "Query the project database",  args: {    query: tool.schema.string().describe("SQL query to execute"),  },  async execute(args) {    // Your database logic here    return `Executed query: ${args.query}`  },})
```

The **filename** becomes the **tool name**. The above creates a `database` tool.

* * *

#### [Multiple tools per file](http://opencode.ai/docs/custom-tools#multiple-tools-per-file)

You can also export multiple tools from a single file. Each export becomes **a separate tool** with the name **`<filename>_<exportname>`**:

.opencode/tools/math.ts

```
import { tool } from "@opencode-ai/plugin"
export const add = tool({  description: "Add two numbers",  args: {    a: tool.schema.number().describe("First number"),    b: tool.schema.number().describe("Second number"),  },  async execute(args) {    return args.a + args.b  },})
export const multiply = tool({  description: "Multiply two numbers",  args: {    a: tool.schema.number().describe("First number"),    b: tool.schema.number().describe("Second number"),  },  async execute(args) {    return args.a * args.b  },})
```

This creates two tools: `math_add` and `math_multiply`.

* * *

### [Arguments](http://opencode.ai/docs/custom-tools#arguments)

You can use `tool.schema`, which is just [Zod](https://zod.dev/), to define argument types.

`args: {  query: tool.schema.string().describe("SQL query to execute")}`

You can also import [Zod](https://zod.dev/) directly and return a plain object:

```
import { z } from "zod"
export default {  description: "Tool description",  args: {    param: z.string().describe("Parameter description"),  },  async execute(args, context) {    // Tool implementation    return "result"  },}
```

* * *

### [Context](http://opencode.ai/docs/custom-tools#context)

Tools receive context about the current session:

.opencode/tools/project.ts

```
import { tool } from "@opencode-ai/plugin"
export default tool({  description: "Get project information",  args: {},  async execute(args, context) {    // Access context information    const { agent, sessionID, messageID, directory, worktree } = context    return `Agent: ${agent}, Session: ${sessionID}, Message: ${messageID}, Directory: ${directory}, Worktree: ${worktree}`  },})
```

Use `context.directory` for the session working directory. Use `context.worktree` for the git worktree root.

* * *

[Examples](http://opencode.ai/docs/custom-tools#examples)
---------------------------------------------------------

### [Write a tool in Python](http://opencode.ai/docs/custom-tools#write-a-tool-in-python)

You can write your tools in any language you want. Here’s an example that adds two numbers using Python.

First, create the tool as a Python script:

.opencode/tools/add.py

```
import sys
a = int(sys.argv[1])b = int(sys.argv[2])print(a + b)
```

Then create the tool definition that invokes it:

.opencode/tools/python-add.ts

```
import { tool } from "@opencode-ai/plugin"import path from "path"
export default tool({  description: "Add two numbers using Python",  args: {    a: tool.schema.number().describe("First number"),    b: tool.schema.number().describe("Second number"),  },  async execute(args, context) {    const script = path.join(context.worktree, ".opencode/tools/add.py")    const result = await Bun.$`python3 ${script} ${args.a} ${args.b}`.text()    return result.trim()  },})
```

Here we are using the [`Bun.$`](https://bun.com/docs/runtime/shell) utility to run the Python script.

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/custom-tools.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord)Select language 

© [Anomaly](https://anoma.ly/)

Last updated: Feb 15, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with direct HTML fallback support.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
