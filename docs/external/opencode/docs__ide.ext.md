----
## External Docs Snapshot // opencode

- Captured: 2026-02-16T04:13:51.889Z
- Source root: https://opencode.ai/docs
- Source page: /docs/ide
- Keywords: opencode, docs, ai coding assistant, cli, ide
- Summary: The OpenCode extension for VS Code, Cursor, and other IDEs
----

Source: https://opencode.ai/docs/ide

# IDE

The OpenCode extension for VS Code, Cursor, and other IDEs

OpenCode integrates with VS Code, Cursor, or any IDE that supports a terminal. Just run `opencode` in the terminal to get started.

## [Usage](#usage)

- Quick Launch: Use `Cmd+Esc` (Mac) or `Ctrl+Esc` (Windows/Linux) to open OpenCode in a split terminal view, or focus an existing terminal session if one is already running.

- New Session: Use `Cmd+Shift+Esc` (Mac) or `Ctrl+Shift+Esc` (Windows/Linux) to start a new OpenCode terminal session, even if one is already open. You can also click the OpenCode button in the UI.

- Context Awareness: Automatically share your current selection or tab with OpenCode.

- File Reference Shortcuts: Use `Cmd+Option+K` (Mac) or `Alt+Ctrl+K` (Linux/Windows) to insert file references. For example, `@File#L37-42`.

## [Installation](#installation)

To install OpenCode on VS Code and popular forks like Cursor, Windsurf, VSCodium:

- Open VS Code

- Open the integrated terminal

- Run `opencode` - the extension installs automatically

If on the other hand you want to use your own IDE when you run `/editor` or `/export` from the TUI, you’ll need to set `export EDITOR="code --wait"`. [Learn more](/docs/tui/#editor-setup).

### [Manual Install](#manual-install)

Search for OpenCode in the Extension Marketplace and click Install.

### [Troubleshooting](#troubleshooting)

If the extension fails to install automatically:

- Ensure you’re running `opencode` in the integrated terminal.

- Confirm the CLI for your IDE is installed: For VS Code: `code` command

- For Cursor: `cursor` command

- For Windsurf: `windsurf` command

- For VSCodium: `codium` command

- If not, run `Cmd+Shift+P` (Mac) or `Ctrl+Shift+P` (Windows/Linux) and search for “Shell Command: Install ‘code’ command in PATH” (or the equivalent for your IDE)

- Ensure VS Code has permission to install extensions

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/ide.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord) Select language   EnglishالعربيةBosanskiDanskDeutschEspañolFrançaisItaliano日本語한국어Norsk BokmålPolskiPortuguês (Brasil)РусскийไทยTürkçe简体中文繁體中文

&copy; [Anomaly](https://anoma.ly)

Last updated: Feb 15, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
