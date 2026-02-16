----
## External Docs Snapshot // opencode

- Captured: 2026-02-16T07:29:35.337Z
- Source root: https://opencode.ai/docs
- Source page: /docs/windows-wsl
- Keywords: opencode, docs, ai coding assistant, cli, windows wsl
- Summary: Run OpenCode on Windows using WSL for the best experience.
----

Source: https://opencode.ai/docs/windows-wsl

# Windows (WSL)

Run OpenCode on Windows using WSL for the best experience.

While OpenCode can run directly on Windows, we recommend using [Windows Subsystem for Linux (WSL)](https://learn.microsoft.com/en-us/windows/wsl/install) for the best experience. WSL provides a Linux environment that works seamlessly with OpenCode’s features.

Why WSL?

WSL offers better file system performance, full terminal support, and compatibility with development tools that OpenCode relies on.

## [Setup](#setup)

- Install WSL If you haven’t already, [install WSL](https://learn.microsoft.com/en-us/windows/wsl/install) using the official Microsoft guide.

- Install OpenCode in WSL Once WSL is set up, open your WSL terminal and install OpenCode using one of the [installation methods](/docs/). Terminal window ``` curl -fsSL https://opencode.ai/install | bash ```

- Use OpenCode from WSL Navigate to your project directory (access Windows files via `/mnt/c/`, `/mnt/d/`, etc.) and run OpenCode. Terminal window ``` cd /mnt/c/Users/YourName/projectopencode ```

## [Desktop App + WSL Server](#desktop-app--wsl-server)

If you prefer using the OpenCode Desktop app but want to run the server in WSL:

- Start the server in WSL with `--hostname 0.0.0.0` to allow external connections: Terminal window ``` opencode serve --hostname 0.0.0.0 --port 4096 ```

- Connect the Desktop app to `http://localhost:4096`

Note

If `localhost` does not work in your setup, connect using the WSL IP address instead (from WSL: `hostname -I`) and use `http://&#x3C;wsl-ip>:4096`.

Caution

When using `--hostname 0.0.0.0`, set `OPENCODE_SERVER_PASSWORD` to secure the server.

Terminal window

```
OPENCODE_SERVER_PASSWORD=your-password opencode serve --hostname 0.0.0.0
```

## [Web Client + WSL](#web-client--wsl)

For the best web experience on Windows:

- Run `opencode web` in the WSL terminal rather than PowerShell: Terminal window ``` opencode web --hostname 0.0.0.0 ```

- Access from your Windows browser at `http://localhost:&#x3C;port>` (OpenCode prints the URL)

Running `opencode web` from WSL ensures proper file system access and terminal integration while still being accessible from your Windows browser.

## [Accessing Windows Files](#accessing-windows-files)

WSL can access all your Windows files through the `/mnt/` directory:

- `C:` drive → `/mnt/c/`

- `D:` drive → `/mnt/d/`

- And so on…

Example:

Terminal window

```
cd /mnt/c/Users/YourName/Documents/projectopencode
```

Tip

For the smoothest experience, consider cloning/copying your repo into the WSL filesystem (for example under `~/code/`) and running OpenCode there.

## [Tips](#tips)

- Keep OpenCode running in WSL for projects stored on Windows drives - file access is seamless

- Use VS Code’s [WSL extension](https://code.visualstudio.com/docs/remote/wsl) alongside OpenCode for an integrated development workflow

- Your OpenCode config and sessions are stored within the WSL environment at `~/.local/share/opencode/`

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/windows-wsl.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord) Select language   EnglishالعربيةBosanskiDanskDeutschEspañolFrançaisItaliano日本語한국어Norsk BokmålPolskiPortuguês (Brasil)РусскийไทยTürkçe简体中文繁體中文

&copy; [Anomaly](https://anoma.ly)

Last updated: Feb 15, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
