----
## External Docs Snapshot // opencode

- Captured: 2026-02-16T04:13:51.889Z
- Source root: https://opencode.ai/docs
- Source page: /docs/network
- Keywords: opencode, docs, ai coding assistant, cli, network
- Summary: Configure proxies and custom certificates.
----

Source: https://opencode.ai/docs/network

# Network

Configure proxies and custom certificates.

OpenCode supports standard proxy environment variables and custom certificates for enterprise network environments.

## [Proxy](#proxy)

OpenCode respects standard proxy environment variables.

Terminal window

```
# HTTPS proxy (recommended)export HTTPS_PROXY=https://proxy.example.com:8080
# HTTP proxy (if HTTPS not available)export HTTP_PROXY=http://proxy.example.com:8080
# Bypass proxy for local server (required)export NO_PROXY=localhost,127.0.0.1
```

Caution

The TUI communicates with a local HTTP server. You must bypass the proxy for this connection to prevent routing loops.

You can configure the server’s port and hostname using [CLI flags](/docs/cli#run).

### [Authenticate](#authenticate)

If your proxy requires basic authentication, include credentials in the URL.

Terminal window

```
export HTTPS_PROXY=http://username:password@proxy.example.com:8080
```

Caution

Avoid hardcoding passwords. Use environment variables or secure credential storage.

For proxies requiring advanced authentication like NTLM or Kerberos, consider using an LLM Gateway that supports your authentication method.

## [Custom certificates](#custom-certificates)

If your enterprise uses custom CAs for HTTPS connections, configure OpenCode to trust them.

Terminal window

```
export NODE_EXTRA_CA_CERTS=/path/to/ca-cert.pem
```

This works for both proxy connections and direct API access.

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/network.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord) Select language   EnglishالعربيةBosanskiDanskDeutschEspañolFrançaisItaliano日本語한국어Norsk BokmålPolskiPortuguês (Brasil)РусскийไทยTürkçe简体中文繁體中文

&copy; [Anomaly](https://anoma.ly)

Last updated: Feb 15, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
