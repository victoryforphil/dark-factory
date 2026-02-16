----
## External Docs Snapshot // opencode

- Captured: 2026-02-16T07:29:35.337Z
- Source root: https://opencode.ai/docs
- Source page: /docs/network
- Keywords: opencode, docs, ai coding assistant, cli, network
- Summary: [Skip to content](http://opencode.ai/docs/network#_top)
----

Source: https://opencode.ai/docs/network

Network | OpenCode
===============
[Skip to content](http://opencode.ai/docs/network#_top)

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

*   [Overview](http://opencode.ai/docs/network#_top)
*   [Proxy](http://opencode.ai/docs/network#proxy)
    *   [Authenticate](http://opencode.ai/docs/network#authenticate)

*   [Custom certificates](http://opencode.ai/docs/network#custom-certificates)

On this page
------------

*   [Overview](http://opencode.ai/docs/network#_top)
*   [Proxy](http://opencode.ai/docs/network#proxy)
    *   [Authenticate](http://opencode.ai/docs/network#authenticate)

*   [Custom certificates](http://opencode.ai/docs/network#custom-certificates)

Network
=======

Configure proxies and custom certificates.

OpenCode supports standard proxy environment variables and custom certificates for enterprise network environments.

* * *

[Proxy](http://opencode.ai/docs/network#proxy)
----------------------------------------------

OpenCode respects standard proxy environment variables.

Terminal window

```
# HTTPS proxy (recommended)export HTTPS_PROXY=https://proxy.example.com:8080
# HTTP proxy (if HTTPS not available)export HTTP_PROXY=http://proxy.example.com:8080
# Bypass proxy for local server (required)export NO_PROXY=localhost,127.0.0.1
```

Caution

The TUI communicates with a local HTTP server. You must bypass the proxy for this connection to prevent routing loops.

You can configure the server’s port and hostname using [CLI flags](http://opencode.ai/docs/cli#run).

* * *

### [Authenticate](http://opencode.ai/docs/network#authenticate)

If your proxy requires basic authentication, include credentials in the URL.

Terminal window

`export HTTPS_PROXY=http://username:password@proxy.example.com:8080`

Caution

Avoid hardcoding passwords. Use environment variables or secure credential storage.

For proxies requiring advanced authentication like NTLM or Kerberos, consider using an LLM Gateway that supports your authentication method.

* * *

[Custom certificates](http://opencode.ai/docs/network#custom-certificates)
--------------------------------------------------------------------------

If your enterprise uses custom CAs for HTTPS connections, configure OpenCode to trust them.

Terminal window

`export NODE_EXTRA_CA_CERTS=/path/to/ca-cert.pem`

This works for both proxy connections and direct API access.

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/network.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord)Select language 

© [Anomaly](https://anoma.ly/)

Last updated: Feb 15, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with direct HTML fallback support.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
