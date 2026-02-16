----
## External Docs Snapshot // opencode

- Captured: 2026-02-16T04:13:51.889Z
- Source root: https://opencode.ai/docs
- Source page: /docs/tools
- Keywords: opencode, docs, ai coding assistant, cli, tools
- Summary: Manage the tools an LLM can use.
----

Source: https://opencode.ai/docs/tools

# Tools

Manage the tools an LLM can use.

Tools allow the LLM to perform actions in your codebase. OpenCode comes with a set of built-in tools, but you can extend it with [custom tools](/docs/custom-tools) or [MCP servers](/docs/mcp-servers).

By default, all tools are enabled and don’t need permission to run. You can control tool behavior through [permissions](/docs/permissions).

## [Configure](#configure)

Use the `permission` field to control tool behavior. You can allow, deny, or require approval for each tool.

- opencode.json ``` { "$schema": "https://opencode.ai/config.json", "permission": { "edit": "deny", "bash": "ask", "webfetch": "allow" }} ``` You can also use wildcards to control multiple tools at once. For example, to require approval for all tools from an MCP server: opencode.json ``` { "$schema": "https://opencode.ai/config.json", "permission": { "mymcp_*": "ask" }} ``` [Learn more](/docs/permissions) about configuring permissions. ## [Built-in](#built-in) Here are all the built-in tools available in OpenCode. ### [bash](#bash) Execute shell commands in your project environment. opencode.json ``` { "$schema": "https://opencode.ai/config.json", "permission": { "bash": "allow" }} ``` This tool allows the LLM to run terminal commands like `npm install`, `git status`, or any other shell command. ### [edit](#edit) Modify existing files using exact string replacements. opencode.json ``` { "$schema": "https://opencode.ai/config.json", "permission": { "edit": "allow" }} ``` This tool performs precise edits to files by replacing exact text matches. It’s the primary way the LLM modifies code. ### [write](#write) Create new files or overwrite existing ones. opencode.json ``` { "$schema": "https://opencode.ai/config.json", "permission": { "edit": "allow" }} ``` Use this to allow the LLM to create new files. It will overwrite existing files if they already exist. NoteThe `write` tool is controlled by the `edit` permission, which covers all file modifications (`edit`, `write`, `patch`, `multiedit`). ### [read](#read) Read file contents from your codebase. opencode.json ``` { "$schema": "https://opencode.ai/config.json", "permission": { "read": "allow" }} ``` This tool reads files and returns their contents. It supports reading specific line ranges for large files. ### [grep](#grep) Search file contents using regular expressions. opencode.json ``` { "$schema": "https://opencode.ai/config.json", "permission": { "grep": "allow" }} ``` Fast content search across your codebase. Supports full regex syntax and file pattern filtering. ### [glob](#glob) Find files by pattern matching. opencode.json ``` { "$schema": "https://opencode.ai/config.json", "permission": { "glob": "allow" }} ``` Search for files using glob patterns like `**/*.js` or `src/**/*.ts`. Returns matching file paths sorted by modification time. ### [list](#list) List files and directories in a given path. opencode.json ``` { "$schema": "https://opencode.ai/config.json", "permission": { "list": "allow" }} ``` This tool lists directory contents. It accepts glob patterns to filter results. ### [lsp (experimental)](#lsp-experimental) Interact with your configured LSP servers to get code intelligence features like definitions, references, hover info, and call hierarchy. NoteThis tool is only available when `OPENCODE_EXPERIMENTAL_LSP_TOOL=true` (or `OPENCODE_EXPERIMENTAL=true`). opencode.json ``` { "$schema": "https://opencode.ai/config.json", "permission": { "lsp": "allow" }} ``` Supported operations include `goToDefinition`, `findReferences`, `hover`, `documentSymbol`, `workspaceSymbol`, `goToImplementation`, `prepareCallHierarchy`, `incomingCalls`, and `outgoingCalls`. To configure which LSP servers are available for your project, see [LSP Servers](/docs/lsp). ### [patch](#patch) Apply patches to files. opencode.json ``` { "$schema": "https://opencode.ai/config.json", "permission": { "edit": "allow" }} ``` This tool applies patch files to your codebase. Useful for applying diffs and patches from various sources. NoteThe `patch` tool is controlled by the `edit` permission, which covers all file modifications (`edit`, `write`, `patch`, `multiedit`). ### [skill](#skill) Load a [skill](/docs/skills) (a `SKILL.md` file) and return its content in the conversation. opencode.json ``` { "$schema": "https://opencode.ai/config.json", "permission": { "skill": "allow" }} ``` ### [todowrite](#todowrite) Manage todo lists during coding sessions. opencode.json ``` { "$schema": "https://opencode.ai/config.json", "permission": { "todowrite": "allow" }} ``` Creates and updates task lists to track progress during complex operations. The LLM uses this to organize multi-step tasks. NoteThis tool is disabled for subagents by default, but you can enable it manually. [Learn more](/docs/agents/#permissions) ### [todoread](#todoread) Read existing todo lists. opencode.json ``` { "$schema": "https://opencode.ai/config.json", "permission": { "todoread": "allow" }} ``` Reads the current todo list state. Used by the LLM to track what tasks are pending or completed. NoteThis tool is disabled for subagents by default, but you can enable it manually. [Learn more](/docs/agents/#permissions) ### [webfetch](#webfetch) Fetch web content. opencode.json ``` { "$schema": "https://opencode.ai/config.json", "permission": { "webfetch": "allow" }} ``` Allows the LLM to fetch and read web pages. Useful for looking up documentation or researching online resources. ### [websearch](#websearch) Search the web for information. NoteThis tool is only available when using the OpenCode provider or when the `OPENCODE_ENABLE_EXA` environment variable is set to any truthy value (e.g., `true` or `1`).To enable when launching OpenCode:Terminal window ``` OPENCODE_ENABLE_EXA=1 opencode ``` opencode.json ``` { "$schema": "https://opencode.ai/config.json", "permission": { "websearch": "allow" }} ``` Performs web searches using Exa AI to find relevant information online. Useful for researching topics, finding current events, or gathering information beyond the training data cutoff. No API key is required — the tool connects directly to Exa AI’s hosted MCP service without authentication. TipUse `websearch` when you need to find information (discovery), and `webfetch` when you need to retrieve content from a specific URL (retrieval). ### [question](#question) Ask the user questions during execution. opencode.json ``` { "$schema": "https://opencode.ai/config.json", "permission": { "question": "allow" }} ``` This tool allows the LLM to ask the user questions during a task. It’s useful for: Gathering user preferences or requirements

- Clarifying ambiguous instructions

- Getting decisions on implementation choices

- Offering choices about what direction to take

Each question includes a header, the question text, and a list of options. Users can select from the provided options or type a custom answer. When there are multiple questions, users can navigate between them before submitting all answers.

## [Custom tools](#custom-tools)

Custom tools let you define your own functions that the LLM can call. These are defined in your config file and can execute arbitrary code.

[Learn more](/docs/custom-tools) about creating custom tools.

## [MCP servers](#mcp-servers)

MCP (Model Context Protocol) servers allow you to integrate external tools and services. This includes database access, API integrations, and third-party services.

[Learn more](/docs/mcp-servers) about configuring MCP servers.

## [Internals](#internals)

Internally, tools like `grep`, `glob`, and `list` use [ripgrep](https://github.com/BurntSushi/ripgrep) under the hood. By default, ripgrep respects `.gitignore` patterns, which means files and directories listed in your `.gitignore` will be excluded from searches and listings.

### [Ignore patterns](#ignore-patterns)

To include files that would normally be ignored, create a `.ignore` file in your project root. This file can explicitly allow certain paths.

.ignore

```
!node_modules/!dist/!build/
```

For example, this `.ignore` file allows ripgrep to search within `node_modules/`, `dist/`, and `build/` directories even if they’re listed in `.gitignore`.

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/tools.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord) Select language   EnglishالعربيةBosanskiDanskDeutschEspañolFrançaisItaliano日本語한국어Norsk BokmålPolskiPortuguês (Brasil)РусскийไทยTürkçe简体中文繁體中文

&copy; [Anomaly](https://anoma.ly)

Last updated: Feb 15, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
