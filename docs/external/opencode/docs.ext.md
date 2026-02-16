----
## External Docs Snapshot // opencode

- Captured: 2026-02-16T07:29:35.337Z
- Source root: https://opencode.ai/docs
- Source page: /docs
- Keywords: opencode, docs, ai coding assistant, cli
- Summary: Intro | AI coding agent built for the terminal
----

Source: https://opencode.ai/docs

Intro | AI coding agent built for the terminal
===============
Intro | OpenCode
===============
[Skip to content](http://opencode.ai/docs#_top)

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

*   [Overview](http://opencode.ai/docs#_top)
*   [Install](http://opencode.ai/docs#install)
*   [Configure](http://opencode.ai/docs#configure)
*   [Initialize](http://opencode.ai/docs#initialize)
*   [Usage](http://opencode.ai/docs#usage)
    *   [Ask questions](http://opencode.ai/docs#ask-questions)
    *   [Add features](http://opencode.ai/docs#add-features)
    *   [Make changes](http://opencode.ai/docs#make-changes)
    *   [Undo changes](http://opencode.ai/docs#undo-changes)

*   [Share](http://opencode.ai/docs#share)
*   [Customize](http://opencode.ai/docs#customize)

On this page
------------

*   [Overview](http://opencode.ai/docs#_top)
*   [Install](http://opencode.ai/docs#install)
*   [Configure](http://opencode.ai/docs#configure)
*   [Initialize](http://opencode.ai/docs#initialize)
*   [Usage](http://opencode.ai/docs#usage)
    *   [Ask questions](http://opencode.ai/docs#ask-questions)
    *   [Add features](http://opencode.ai/docs#add-features)
    *   [Make changes](http://opencode.ai/docs#make-changes)
    *   [Undo changes](http://opencode.ai/docs#undo-changes)

*   [Share](http://opencode.ai/docs#share)
*   [Customize](http://opencode.ai/docs#customize)

Intro
=====

Get started with OpenCode.

[**OpenCode**](http://opencode.ai/) is an open source AI coding agent. It’s available as a terminal-based interface, desktop app, or IDE extension.

![Image 3: OpenCode TUI with the opencode theme](http://opencode.ai/docs/_astro/screenshot.CQjBbRyJ_1dLadc.webp)

Let’s get started.

* * *

#### [Prerequisites](http://opencode.ai/docs#prerequisites)

To use OpenCode in your terminal, you’ll need:

1.   A modern terminal emulator like:

    *   [WezTerm](https://wezterm.org/), cross-platform
    *   [Alacritty](https://alacritty.org/), cross-platform
    *   [Ghostty](https://ghostty.org/), Linux and macOS
    *   [Kitty](https://sw.kovidgoyal.net/kitty/), Linux and macOS

2.   API keys for the LLM providers you want to use.

* * *

[Install](http://opencode.ai/docs#install)
------------------------------------------

The easiest way to install OpenCode is through the install script.

Terminal window

`curl -fsSL https://opencode.ai/install | bash`

You can also install it with the following commands:

*   **Using Node.js**

    *   [npm](http://opencode.ai/docs#tab-panel-0)
    *   [Bun](http://opencode.ai/docs#tab-panel-1)
    *   [pnpm](http://opencode.ai/docs#tab-panel-2)
    *   [Yarn](http://opencode.ai/docs#tab-panel-3)

Terminal window

`npm install -g opencode-ai`

Terminal window

`bun install -g opencode-ai`

Terminal window

`pnpm install -g opencode-ai`

Terminal window

`yarn global add opencode-ai`

*   **Using Homebrew on macOS and Linux**

Terminal window

`brew install anomalyco/tap/opencode`    
> We recommend using the OpenCode tap for the most up to date releases. The official `brew install opencode` formula is maintained by the Homebrew team and is updated less frequently.

*   **Installing on Arch Linux**

Terminal window

`sudo pacman -S opencode           # Arch Linux (Stable)paru -S opencode-bin              # Arch Linux (Latest from AUR)`    

#### [Windows](http://opencode.ai/docs#windows)

Recommended: Use WSL

For the best experience on Windows, we recommend using [Windows Subsystem for Linux (WSL)](http://opencode.ai/docs/windows-wsl). It provides better performance and full compatibility with OpenCode’s features.

*   **Using Chocolatey**

Terminal window

`choco install opencode`    
*   **Using Scoop**

Terminal window

`scoop install opencode`    
*   **Using NPM**

Terminal window

`npm install -g opencode-ai`    
*   **Using Mise**

Terminal window

`mise use -g github:anomalyco/opencode`    
*   **Using Docker**

Terminal window

`docker run -it --rm ghcr.io/anomalyco/opencode`    

Support for installing OpenCode on Windows using Bun is currently in progress.

You can also grab the binary from the [Releases](https://github.com/anomalyco/opencode/releases).

* * *

[Configure](http://opencode.ai/docs#configure)
----------------------------------------------

With OpenCode you can use any LLM provider by configuring their API keys.

If you are new to using LLM providers, we recommend using [OpenCode Zen](http://opencode.ai/docs/zen). It’s a curated list of models that have been tested and verified by the OpenCode team.

1.   Run the `/connect` command in the TUI, select opencode, and head to [opencode.ai/auth](https://opencode.ai/auth).

`/connect`    
2.   Sign in, add your billing details, and copy your API key.

3.   Paste your API key.

`┌ API key││└ enter`    

Alternatively, you can select one of the other providers. [Learn more](http://opencode.ai/docs/providers#directory).

* * *

[Initialize](http://opencode.ai/docs#initialize)
------------------------------------------------

Now that you’ve configured a provider, you can navigate to a project that you want to work on.

Terminal window

`cd /path/to/project`

And run OpenCode.

Terminal window

`opencode`

Next, initialize OpenCode for the project by running the following command.

`/init`

This will get OpenCode to analyze your project and create an `AGENTS.md` file in the project root.

Tip

You should commit your project’s `AGENTS.md` file to Git.

This helps OpenCode understand the project structure and the coding patterns used.

* * *

[Usage](http://opencode.ai/docs#usage)
--------------------------------------

You are now ready to use OpenCode to work on your project. Feel free to ask it anything!

If you are new to using an AI coding agent, here are some examples that might help.

* * *

### [Ask questions](http://opencode.ai/docs#ask-questions)

You can ask OpenCode to explain the codebase to you.

Tip

Use the `@` key to fuzzy search for files in the project.

`How is authentication handled in @packages/functions/src/api/index.ts`

This is helpful if there’s a part of the codebase that you didn’t work on.

* * *

### [Add features](http://opencode.ai/docs#add-features)

You can ask OpenCode to add new features to your project. Though we first recommend asking it to create a plan.

1.   **Create a plan**

OpenCode has a _Plan mode_ that disables its ability to make changes and instead suggest _how_ it’ll implement the feature.

Switch to it using the **Tab** key. You’ll see an indicator for this in the lower right corner.

`<TAB>`    
Now let’s describe what we want it to do.

`When a user deletes a note, we'd like to flag it as deleted in the database.Then create a screen that shows all the recently deleted notes.From this screen, the user can undelete a note or permanently delete it.`    
You want to give OpenCode enough details to understand what you want. It helps to talk to it like you are talking to a junior developer on your team.

Tip

Give OpenCode plenty of context and examples to help it understand what you want. 
2.   **Iterate on the plan**

Once it gives you a plan, you can give it feedback or add more details.

`We'd like to design this new screen using a design I've used before.[Image #1] Take a look at this image and use it as a reference.`    
Tip

Drag and drop images into the terminal to add them to the prompt. 
OpenCode can scan any images you give it and add them to the prompt. You can do this by dragging and dropping an image into the terminal.

3.   **Build the feature**

Once you feel comfortable with the plan, switch back to _Build mode_ by hitting the **Tab** key again.

`<TAB>`    
And asking it to make the changes.

`Sounds good! Go ahead and make the changes.`    

* * *

### [Make changes](http://opencode.ai/docs#make-changes)

For more straightforward changes, you can ask OpenCode to directly build it without having to review the plan first.

`We need to add authentication to the /settings route. Take a look at how this ishandled in the /notes route in @packages/functions/src/notes.ts and implementthe same logic in @packages/functions/src/settings.ts`

You want to make sure you provide a good amount of detail so OpenCode makes the right changes.

* * *

### [Undo changes](http://opencode.ai/docs#undo-changes)

Let’s say you ask OpenCode to make some changes.

`Can you refactor the function in @packages/functions/src/api/index.ts?`

But you realize that it is not what you wanted. You **can undo** the changes using the `/undo` command.

`/undo`

OpenCode will now revert the changes you made and show your original message again.

`Can you refactor the function in @packages/functions/src/api/index.ts?`

From here you can tweak the prompt and ask OpenCode to try again.

Tip

You can run `/undo` multiple times to undo multiple changes.

Or you **can redo** the changes using the `/redo` command.

`/redo`

* * *

[Share](http://opencode.ai/docs#share)
--------------------------------------

The conversations that you have with OpenCode can be [shared with your team](http://opencode.ai/docs/share).

`/share`

This will create a link to the current conversation and copy it to your clipboard.

Note

Conversations are not shared by default.

Here’s an [example conversation](https://opencode.ai/s/4XP1fce5) with OpenCode.

* * *

[Customize](http://opencode.ai/docs#customize)
----------------------------------------------

And that’s it! You are now a pro at using OpenCode.

To make it your own, we recommend [picking a theme](http://opencode.ai/docs/themes), [customizing the keybinds](http://opencode.ai/docs/keybinds), [configuring code formatters](http://opencode.ai/docs/formatters), [creating custom commands](http://opencode.ai/docs/commands), or playing around with the [OpenCode config](http://opencode.ai/docs/config).

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/index.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord)Select language 

© [Anomaly](https://anoma.ly/)

Last updated: Feb 15, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with direct HTML fallback support.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
