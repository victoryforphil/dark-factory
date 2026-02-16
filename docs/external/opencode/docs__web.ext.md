----
## External Docs Snapshot // opencode

- Captured: 2026-02-16T07:29:35.337Z
- Source root: https://opencode.ai/docs
- Source page: /docs/web
- Keywords: opencode, docs, ai coding assistant, cli, web
- Summary: Using OpenCode in your browser.
----

Source: https://opencode.ai/docs/web

# Web

Using OpenCode in your browser.

OpenCode can run as a web application in your browser, providing the same powerful AI coding experience without needing a terminal.

## [Getting Started](#getting-started)

Start the web interface by running:

Terminal window

```
opencode web
```

This starts a local server on `127.0.0.1` with a random available port and automatically opens OpenCode in your default browser.

Caution

If `OPENCODE_SERVER_PASSWORD` is not set, the server will be unsecured. This is fine for local use but should be set for network access.

Windows Users

For the best experience, run `opencode web` from [WSL](/docs/windows-wsl) rather than PowerShell. This ensures proper file system access and terminal integration.

## [Configuration](#configuration)

You can configure the web server using command line flags or in your [config file](/docs/config).

### [Port](#port)

By default, OpenCode picks an available port. You can specify a port:

Terminal window

```
opencode web --port 4096
```

### [Hostname](#hostname)

By default, the server binds to `127.0.0.1` (localhost only). To make OpenCode accessible on your network:

Terminal window

```
opencode web --hostname 0.0.0.0
```

When using `0.0.0.0`, OpenCode will display both local and network addresses:

```
Local access:       http://localhost:4096  Network access:     http://192.168.1.100:4096
```

### [mDNS Discovery](#mdns-discovery)

Enable mDNS to make your server discoverable on the local network:

Terminal window

```
opencode web --mdns
```

This automatically sets the hostname to `0.0.0.0` and advertises the server as `opencode.local`.

You can customize the mDNS domain name to run multiple instances on the same network:

Terminal window

```
opencode web --mdns --mdns-domain myproject.local
```

### [CORS](#cors)

To allow additional domains for CORS (useful for custom frontends):

Terminal window

```
opencode web --cors https://example.com
```

### [Authentication](#authentication)

To protect access, set a password using the `OPENCODE_SERVER_PASSWORD` environment variable:

Terminal window

```
OPENCODE_SERVER_PASSWORD=secret opencode web
```

The username defaults to `opencode` but can be changed with `OPENCODE_SERVER_USERNAME`.

## [Using the Web Interface](#using-the-web-interface)

Once started, the web interface provides access to your OpenCode sessions.

### [Sessions](#sessions)

View and manage your sessions from the homepage. You can see active sessions and start new ones.

### [Server Status](#server-status)

Click “See Servers” to view connected servers and their status.

## [Attaching a Terminal](#attaching-a-terminal)

You can attach a terminal TUI to a running web server:

Terminal window

```
# Start the web serveropencode web --port 4096
# In another terminal, attach the TUIopencode attach http://localhost:4096
```

This allows you to use both the web interface and terminal simultaneously, sharing the same sessions and state.

## [Config File](#config-file)

You can also configure server settings in your `opencode.json` config file:

```
{  "server": {    "port": 4096,    "hostname": "0.0.0.0",    "mdns": true,    "cors": ["https://example.com"]  }}
```

Command line flags take precedence over config file settings.

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/web.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord) Select language   EnglishالعربيةBosanskiDanskDeutschEspañolFrançaisItaliano日本語한국어Norsk BokmålPolskiPortuguês (Brasil)РусскийไทยTürkçe简体中文繁體中文

&copy; [Anomaly](https://anoma.ly)

Last updated: Feb 15, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
