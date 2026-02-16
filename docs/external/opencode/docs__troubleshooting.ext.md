----
## External Docs Snapshot // opencode

- Captured: 2026-02-16T04:13:51.889Z
- Source root: https://opencode.ai/docs
- Source page: /docs/troubleshooting
- Keywords: opencode, docs, ai coding assistant, cli, troubleshooting
- Summary: Common issues and how to resolve them.
----

Source: https://opencode.ai/docs/troubleshooting

# Troubleshooting

Common issues and how to resolve them.

To debug issues with OpenCode, start by checking the logs and local data it stores on disk.

## [Logs](#logs)

Log files are written to:

- macOS/Linux: `~/.local/share/opencode/log/`

- Windows: Press `WIN+R` and paste `%USERPROFILE%\.local\share\opencode\log`

Log files are named with timestamps (e.g., `2025-01-09T123456.log`) and the most recent 10 log files are kept.

You can set the log level with the `--log-level` command-line option to get more detailed debug information. For example, `opencode --log-level DEBUG`.

## [Storage](#storage)

opencode stores session data and other application data on disk at:

- macOS/Linux: `~/.local/share/opencode/`

- Windows: Press `WIN+R` and paste `%USERPROFILE%\.local\share\opencode`

This directory contains:

- `auth.json` - Authentication data like API keys, OAuth tokens

- `log/` - Application logs

- `project/` - Project-specific data like session and message data If the project is within a Git repo, it is stored in `./&#x3C;project-slug>/storage/`

- If it is not a Git repo, it is stored in `./global/storage/`

## [Desktop app](#desktop-app)

OpenCode Desktop runs a local OpenCode server (the `opencode-cli` sidecar) in the background. Most issues are caused by a misbehaving plugin, a corrupted cache, or a bad server setting.

### [Quick checks](#quick-checks)

- Fully quit and relaunch the app.

- If the app shows an error screen, click Restart and copy the error details.

- macOS only: `OpenCode` menu -> Reload Webview (helps if the UI is blank/frozen).

### [Disable plugins](#disable-plugins)

If the desktop app is crashing on launch, hanging, or behaving strangely, start by disabling plugins.

#### [Check the global config](#check-the-global-config)

Open your global config file and look for a `plugin` key.

- macOS/Linux: `~/.config/opencode/opencode.jsonc` (or `~/.config/opencode/opencode.json`)

- macOS/Linux (older installs): `~/.local/share/opencode/opencode.jsonc`

- Windows: Press `WIN+R` and paste `%USERPROFILE%\.config\opencode\opencode.jsonc`

If you have plugins configured, temporarily disable them by removing the key or setting it to an empty array:

- ``` { "$schema": "https://opencode.ai/config.json", "plugin": [],} ``` #### [Check plugin directories](#check-plugin-directories) OpenCode can also load local plugins from disk. Temporarily move these out of the way (or rename the folder) and restart the desktop app: Global plugins macOS/Linux: `~/.config/opencode/plugins/`

- Windows: Press `WIN+R` and paste `%USERPROFILE%\.config\opencode\plugins`

- Project plugins (only if you use per-project config) `&#x3C;your-project>/.opencode/plugins/`

If the app starts working again, re-enable plugins one at a time to find which one is causing the issue.

### [Clear the cache](#clear-the-cache)

If disabling plugins doesn’t help (or a plugin install is stuck), clear the cache so OpenCode can rebuild it.

- Quit OpenCode Desktop completely.

- Delete the cache directory:

- macOS: Finder -> `Cmd+Shift+G` -> paste `~/.cache/opencode`

- Linux: delete `~/.cache/opencode` (or run `rm -rf ~/.cache/opencode`)

- Windows: Press `WIN+R` and paste `%USERPROFILE%\.cache\opencode`

- Restart OpenCode Desktop.

### [Fix server connection issues](#fix-server-connection-issues)

OpenCode Desktop can either start its own local server (default) or connect to a server URL you configured.

If you see a “Connection Failed” dialog (or the app never gets past the splash screen), check for a custom server URL.

#### [Clear the desktop default server URL](#clear-the-desktop-default-server-url)

From the Home screen, click the server name (with the status dot) to open the Server picker. In the Default server section, click Clear.

#### [Remove `server.port` / `server.hostname` from your config](#remove-serverport--serverhostname-from-your-config)

If your `opencode.json(c)` contains a `server` section, temporarily remove it and restart the desktop app.

#### [Check environment variables](#check-environment-variables)

If you have `OPENCODE_PORT` set in your environment, the desktop app will try to use that port for the local server.

- Unset `OPENCODE_PORT` (or pick a free port) and restart.

### [Linux: Wayland / X11 issues](#linux-wayland--x11-issues)

On Linux, some Wayland setups can cause blank windows or compositor errors.

- If you’re on Wayland and the app is blank/crashing, try launching with `OC_ALLOW_WAYLAND=1`.

- If that makes things worse, remove it and try launching under an X11 session instead.

### [Windows: WebView2 runtime](#windows-webview2-runtime)

On Windows, OpenCode Desktop requires the Microsoft Edge WebView2 Runtime. If the app opens to a blank window or won’t start, install/update WebView2 and try again.

### [Windows: General performance issues](#windows-general-performance-issues)

If you’re experiencing slow performance, file access issues, or terminal problems on Windows, try using [WSL (Windows Subsystem for Linux)](/docs/windows-wsl). WSL provides a Linux environment that works more seamlessly with OpenCode’s features.

### [Notifications not showing](#notifications-not-showing)

OpenCode Desktop only shows system notifications when:

- notifications are enabled for OpenCode in your OS settings, and

- the app window is not focused.

### [Reset desktop app storage (last resort)](#reset-desktop-app-storage-last-resort)

If the app won’t start and you can’t clear settings from inside the UI, reset the desktop app’s saved state.

- Quit OpenCode Desktop.

- Find and delete these files (they live in the OpenCode Desktop app data directory):

- `opencode.settings.dat` (desktop default server URL)

- `opencode.global.dat` and `opencode.workspace.*.dat` (UI state like recent servers/projects)

To find the directory quickly:

- macOS: Finder -> `Cmd+Shift+G` -> `~/Library/Application Support` (then search for the filenames above)

- Linux: search under `~/.local/share` for the filenames above

- Windows: Press `WIN+R` -> `%APPDATA%` (then search for the filenames above)

## [Getting help](#getting-help)

If you’re experiencing issues with OpenCode:

- Report issues on GitHub The best way to report bugs or request features is through our GitHub repository: [github.com/anomalyco/opencode/issues](https://github.com/anomalyco/opencode/issues) Before creating a new issue, search existing issues to see if your problem has already been reported.

- Join our Discord For real-time help and community discussion, join our Discord server: [opencode.ai/discord](https://opencode.ai/discord)

## [Common issues](#common-issues)

Here are some common issues and how to resolve them.

### [OpenCode won’t start](#opencode-wont-start)

- Check the logs for error messages

- Try running with `--print-logs` to see output in the terminal

- Ensure you have the latest version with `opencode upgrade`

### [Authentication issues](#authentication-issues)

- Try re-authenticating with the `/connect` command in the TUI

- Check that your API keys are valid

- Ensure your network allows connections to the provider’s API

### [Model not available](#model-not-available)

- Check that you’ve authenticated with the provider

- Verify the model name in your config is correct

- Some models may require specific access or subscriptions

If you encounter `ProviderModelNotFoundError` you are most likely incorrectly
referencing a model somewhere.
Models should be referenced like so: `&#x3C;providerId>/&#x3C;modelId>`

Examples:

- `openai/gpt-4.1`

- `openrouter/google/gemini-2.5-flash`

- `opencode/kimi-k2`

To figure out what models you have access to, run `opencode models`

### [ProviderInitError](#provideriniterror)

If you encounter a ProviderInitError, you likely have an invalid or corrupted configuration.

To resolve this:

- First, verify your provider is set up correctly by following the [providers guide](/docs/providers)

- If the issue persists, try clearing your stored configuration: Terminal window ``` rm -rf ~/.local/share/opencode ``` On Windows, press `WIN+R` and delete: `%USERPROFILE%\.local\share\opencode`

- Re-authenticate with your provider using the `/connect` command in the TUI.

### [AI_APICallError and provider package issues](#ai_apicallerror-and-provider-package-issues)

If you encounter API call errors, this may be due to outdated provider packages. opencode dynamically installs provider packages (OpenAI, Anthropic, Google, etc.) as needed and caches them locally.

To resolve provider package issues:

- Clear the provider package cache: Terminal window ``` rm -rf ~/.cache/opencode ``` On Windows, press `WIN+R` and delete: `%USERPROFILE%\.cache\opencode`

- Restart opencode to reinstall the latest provider packages

This will force opencode to download the most recent versions of provider packages, which often resolves compatibility issues with model parameters and API changes.

### [Copy/paste not working on Linux](#copypaste-not-working-on-linux)

Linux users need to have one of the following clipboard utilities installed for copy/paste functionality to work:

For X11 systems:

Terminal window

```
apt install -y xclip# orapt install -y xsel
```

For Wayland systems:

Terminal window

```
apt install -y wl-clipboard
```

For headless environments:

Terminal window

```
apt install -y xvfb# and run:Xvfb :99 -screen 0 1024x768x24 > /dev/null 2>&#x26;1 &#x26;export DISPLAY=:99.0
```

opencode will detect if you’re using Wayland and prefer `wl-clipboard`, otherwise it will try to find clipboard tools in order of: `xclip` and `xsel`.

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/troubleshooting.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord) Select language   EnglishالعربيةBosanskiDanskDeutschEspañolFrançaisItaliano日本語한국어Norsk BokmålPolskiPortuguês (Brasil)РусскийไทยTürkçe简体中文繁體中文

&copy; [Anomaly](https://anoma.ly)

Last updated: Feb 15, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
