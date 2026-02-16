----
## External Docs Snapshot // opencode

- Captured: 2026-02-16T04:13:51.889Z
- Source root: https://opencode.ai/docs
- Source page: /docs/server
- Keywords: opencode, docs, ai coding assistant, cli, server
- Summary: Interact with opencode server over HTTP.
----

Source: https://opencode.ai/docs/server

# Server

Interact with opencode server over HTTP.

The `opencode serve` command runs a headless HTTP server that exposes an OpenAPI endpoint that an opencode client can use.

### [Usage](#usage)

- Terminal window ``` opencode serve [--port &#x3C;number>] [--hostname &#x3C;string>] [--cors &#x3C;origin>] ``` #### [Options](#options) FlagDescriptionDefault`--port`Port to listen on`4096``--hostname`Hostname to listen on`127.0.0.1``--mdns`Enable mDNS discovery`false``--mdns-domain`Custom domain name for mDNS service`opencode.local``--cors`Additional browser origins to allow`[]` `--cors` can be passed multiple times: Terminal window ``` opencode serve --cors http://localhost:5173 --cors https://app.example.com ``` ### [Authentication](#authentication) Set `OPENCODE_SERVER_PASSWORD` to protect the server with HTTP basic auth. The username defaults to `opencode`, or set `OPENCODE_SERVER_USERNAME` to override it. This applies to both `opencode serve` and `opencode web`. Terminal window ``` OPENCODE_SERVER_PASSWORD=your-password opencode serve ``` ### [How it works](#how-it-works) When you run `opencode` it starts a TUI and a server. Where the TUI is the client that talks to the server. The server exposes an OpenAPI 3.1 spec endpoint. This endpoint is also used to generate an [SDK](/docs/sdk). TipUse the opencode server to interact with opencode programmatically. This architecture lets opencode support multiple clients and allows you to interact with opencode programmatically. You can run `opencode serve` to start a standalone server. If you have the opencode TUI running, `opencode serve` will start a new server. #### [Connect to an existing server](#connect-to-an-existing-server) When you start the TUI it randomly assigns a port and hostname. You can instead pass in the `--hostname` and `--port` [flags](/docs/cli). Then use this to connect to its server. The [`/tui`](#tui) endpoint can be used to drive the TUI through the server. For example, you can prefill or run a prompt. This setup is used by the OpenCode [IDE](/docs/ide) plugins. ## [Spec](#spec) The server publishes an OpenAPI 3.1 spec that can be viewed at: ``` http://&#x3C;hostname>:&#x3C;port>/doc ``` For example, `http://localhost:4096/doc`. Use the spec to generate clients or inspect request and response types. Or view it in a Swagger explorer. ## [APIs](#apis) The opencode server exposes the following APIs. ### [Global](#global) MethodPathDescriptionResponse`GET``/global/health`Get server health and version`{ healthy: true, version: string }``GET``/global/event`Get global events (SSE stream)Event stream ### [Project](#project) MethodPathDescriptionResponse`GET``/project`List all projects[`Project[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`GET``/project/current`Get the current project[`Project`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts) ### [Path &#x26; VCS](#path--vcs) MethodPathDescriptionResponse`GET``/path`Get the current path[`Path`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`GET``/vcs`Get VCS info for the current project[`VcsInfo`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts) ### [Instance](#instance) MethodPathDescriptionResponse`POST``/instance/dispose`Dispose the current instance`boolean` ### [Config](#config) MethodPathDescriptionResponse`GET``/config`Get config info[`Config`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`PATCH``/config`Update config[`Config`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`GET``/config/providers`List providers and default models`{ providers:`[Provider[]](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`, default: { [key: string]: string } }` ### [Provider](#provider) MethodPathDescriptionResponse`GET``/provider`List all providers`{ all:`[Provider[]](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`, default: {...}, connected: string[] }``GET``/provider/auth`Get provider authentication methods`{ [providerID: string]:`[ProviderAuthMethod[]](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`}``POST``/provider/{id}/oauth/authorize`Authorize a provider using OAuth[`ProviderAuthAuthorization`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`POST``/provider/{id}/oauth/callback`Handle OAuth callback for a provider`boolean` ### [Sessions](#sessions) MethodPathDescriptionNotes`GET``/session`List all sessionsReturns [`Session[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`POST``/session`Create a new sessionbody: `{ parentID?, title? }`, returns [`Session`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`GET``/session/status`Get session status for all sessionsReturns `{ [sessionID: string]:`[SessionStatus](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`}``GET``/session/:id`Get session detailsReturns [`Session`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`DELETE``/session/:id`Delete a session and all its dataReturns `boolean``PATCH``/session/:id`Update session propertiesbody: `{ title? }`, returns [`Session`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`GET``/session/:id/children`Get a session’s child sessionsReturns [`Session[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`GET``/session/:id/todo`Get the todo list for a sessionReturns [`Todo[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`POST``/session/:id/init`Analyze app and create `AGENTS.md`body: `{ messageID, providerID, modelID }`, returns `boolean``POST``/session/:id/fork`Fork an existing session at a messagebody: `{ messageID? }`, returns [`Session`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`POST``/session/:id/abort`Abort a running sessionReturns `boolean``POST``/session/:id/share`Share a sessionReturns [`Session`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`DELETE``/session/:id/share`Unshare a sessionReturns [`Session`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`GET``/session/:id/diff`Get the diff for this sessionquery: `messageID?`, returns [`FileDiff[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`POST``/session/:id/summarize`Summarize the sessionbody: `{ providerID, modelID }`, returns `boolean``POST``/session/:id/revert`Revert a messagebody: `{ messageID, partID? }`, returns `boolean``POST``/session/:id/unrevert`Restore all reverted messagesReturns `boolean``POST``/session/:id/permissions/:permissionID`Respond to a permission requestbody: `{ response, remember? }`, returns `boolean` ### [Messages](#messages) MethodPathDescriptionNotes`GET``/session/:id/message`List messages in a sessionquery: `limit?`, returns `{ info:`[Message](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`, parts:`[Part[]](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`}[]``POST``/session/:id/message`Send a message and wait for responsebody: `{ messageID?, model?, agent?, noReply?, system?, tools?, parts }`, returns `{ info:`[Message](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`, parts:`[Part[]](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`}``GET``/session/:id/message/:messageID`Get message detailsReturns `{ info:`[Message](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`, parts:`[Part[]](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`}``POST``/session/:id/prompt_async`Send a message asynchronously (no wait)body: same as `/session/:id/message`, returns `204 No Content``POST``/session/:id/command`Execute a slash commandbody: `{ messageID?, agent?, model?, command, arguments }`, returns `{ info:`[Message](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`, parts:`[Part[]](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`}``POST``/session/:id/shell`Run a shell commandbody: `{ agent, model?, command }`, returns `{ info:`[Message](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`, parts:`[Part[]](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`}` ### [Commands](#commands) MethodPathDescriptionResponse`GET``/command`List all commands[`Command[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts) ### [Files](#files) MethodPathDescriptionResponse`GET``/find?pattern=`Search for text in filesArray of match objects with `path`, `lines`, `line_number`, `absolute_offset`, `submatches``GET``/find/file?query=`Find files and directories by name`string[]` (paths)`GET``/find/symbol?query=`Find workspace symbols[`Symbol[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`GET``/file?path=`List files and directories[`FileNode[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`GET``/file/content?path=`Read a file[`FileContent`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`GET``/file/status`Get status for tracked files[`File[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts) #### [`/find/file` query parameters](#findfile-query-parameters) `query` (required) — search string (fuzzy match)

- `type` (optional) — limit results to `"file"` or `"directory"`

- `directory` (optional) — override the project root for the search

- `limit` (optional) — max results (1–200)

- `dirs` (optional) — legacy flag (`"false"` returns only files)

### [Tools (Experimental)](#tools-experimental)

Method Path Description Response

`GET` `/experimental/tool/ids` List all tool IDs [`ToolIDs`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)

`GET` `/experimental/tool?provider= &model=` List tools with JSON schemas for a model [`ToolList`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)

### [LSP, Formatters &#x26; MCP](#lsp-formatters--mcp)

Method Path Description Response

`GET` `/lsp` Get LSP server status [`LSPStatus[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)

`GET` `/formatter` Get formatter status [`FormatterStatus[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)

`GET` `/mcp` Get MCP server status `{ [name: string]:`[MCPStatus](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`}`

`POST` `/mcp` Add MCP server dynamically body: `{ name, config }`, returns MCP status object

### [Agents](#agents)

Method Path Description Response

`GET` `/agent` List all available agents [`Agent[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)

### [Logging](#logging)

Method Path Description Response

`POST` `/log` Write log entry. Body: `{ service, level, message, extra? }` `boolean`

### [TUI](#tui)

Method Path Description Response

`POST` `/tui/append-prompt` Append text to the prompt `boolean`

`POST` `/tui/open-help` Open the help dialog `boolean`

`POST` `/tui/open-sessions` Open the session selector `boolean`

`POST` `/tui/open-themes` Open the theme selector `boolean`

`POST` `/tui/open-models` Open the model selector `boolean`

`POST` `/tui/submit-prompt` Submit the current prompt `boolean`

`POST` `/tui/clear-prompt` Clear the prompt `boolean`

`POST` `/tui/execute-command` Execute a command (`{ command }`) `boolean`

`POST` `/tui/show-toast` Show toast (`{ title?, message, variant }`) `boolean`

`GET` `/tui/control/next` Wait for the next control request Control request object

`POST` `/tui/control/response` Respond to a control request (`{ body }`) `boolean`

### [Auth](#auth)

Method Path Description Response

`PUT` `/auth/:id` Set authentication credentials. Body must match provider schema `boolean`

### [Events](#events)

Method Path Description Response

`GET` `/event` Server-sent events stream. First event is `server.connected`, then bus events Server-sent events stream

### [Docs](#docs)

Method Path Description Response

`GET` `/doc` OpenAPI 3.1 specification HTML page with OpenAPI spec

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/server.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord) Select language   EnglishالعربيةBosanskiDanskDeutschEspañolFrançaisItaliano日本語한국어Norsk BokmålPolskiPortuguês (Brasil)РусскийไทยTürkçe简体中文繁體中文

&copy; [Anomaly](https://anoma.ly)

Last updated: Feb 15, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
