----
## OpenCode Chat Export

- Session ID: ses_39b2f9630ffewQksq2Ld8n4axU
- Title: Design OpenCode client wrapper in core with TOML config integration
- Slug: kind-wizard
- Project directory: /Users/alex/repos/vfp/dark-factory
- OpenCode version: 1.2.5
- Created: 2026-02-16T04:58:26.127Z
- Updated: 2026-02-16T05:02:33.620Z
- Message count: 24
----

## Conversation

### 1. user (2026-02-16T04:58:26.141Z)

Design a client/ in @core/ to interface / wrap the OpenCode server API @docs/external/opencode/docs__server.ext.md OR (probably better now I look at it) the https://opencode.ai/docs/sdk/ @docs/external/opencode/docs__sdk.ext.md - use our @core/src/helpers/toml-serde.ts @scripts/helpers/ to add / read in any new config settigns we need for opencode.

Called the Read tool with the following input: {"filePath":"/Users/alex/repos/vfp/dark-factory/core/src/helpers/toml-serde.ts"}

Called the Read tool with the following input: {"filePath":"/Users/alex/repos/vfp/dark-factory/docs/external/opencode/docs__sdk.ext.md"}

Called the Read tool with the following input: {"filePath":"/Users/alex/repos/vfp/dark-factory/docs/external/opencode/docs__server.ext.md"}

Called the Read tool with the following input: {"filePath":"/Users/alex/repos/vfp/dark-factory/scripts/helpers/"}

<path>/Users/alex/repos/vfp/dark-factory/scripts/helpers/</path>
<type>directory</type>
<entries>
run_root.sh.ts
run_steps.sh.ts

(2 entries)
</entries>

Called the Read tool with the following input: {"filePath":"/Users/alex/repos/vfp/dark-factory/core/"}

<path>/Users/alex/repos/vfp/dark-factory/core/</path>
<type>directory</type>
<entries>
.gitignore
bun.lock
moon.yml
node_modules/
package.json
README.md
src/
tsconfig.json

(8 entries)
</entries>

<path>/Users/alex/repos/vfp/dark-factory/core/src/helpers/toml-serde.ts</path>
<type>file</type>
<content>1: import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
2: import { dirname } from "node:path";
3: import TOML from "@iarna/toml";
4: 
5: export interface TomlSerde<T> {
6:   decode(value: unknown): T;
7:   encode(value: T): Record<string, unknown>;
8: }
9: 
10: const ensureParentDirectory = (filePath: string) => {
11:   mkdirSync(dirname(filePath), { recursive: true });
12: };
13: 
14: const parseTomlDocument = (filePath: string, text: string): unknown => {
15:   try {
16:     return TOML.parse(text);
17:   } catch (error) {
18:     const message = error instanceof Error ? error.message : "Unable to parse TOML";
19:     throw new Error(`Core // TOML // Parse failed (path=${filePath},reason=${message})`);
20:   }
21: };
22: 
23: const serializeTomlDocument = (filePath: string, document: Record<string, unknown>): string => {
24:   try {
25:     const serialized = TOML.stringify(document);
26:     return serialized.endsWith("\n") ? serialized : `${serialized}\n`;
27:   } catch (error) {
28:     const message = error instanceof Error ? error.message : "Unable to serialize TOML";
29:     throw new Error(`Core // TOML // Serialize failed (path=${filePath},reason=${message})`);
30:   }
31: };
32: 
33: export const readTomlConfig = <T>(filePath: string, serde: TomlSerde<T>): T => {
34:   if (!existsSync(filePath)) {
35:     throw new Error(`Core // TOML // Config not found (path=${filePath})`);
36:   }
37: 
38:   const text = readFileSync(filePath, "utf8");
39:   const document = parseTomlDocument(filePath, text);
40: 
41:   try {
42:     return serde.decode(document);
43:   } catch (error) {
44:     const message = error instanceof Error ? error.message : "Decode failed";
45:     throw new Error(`Core // TOML // Decode failed (path=${filePath},reason=${message})`);
46:   }
47: };
48: 
49: export const readTomlConfigIfExists = <T>(
50:   filePath: string,
51:   serde: TomlSerde<T>,
52: ): T | undefined => {
53:   if (!existsSync(filePath)) {
54:     return undefined;
55:   }
56: 
57:   return readTomlConfig(filePath, serde);
58: };
59: 
60: export const writeTomlConfig = <T>(
61:   filePath: string,
62:   serde: TomlSerde<T>,
63:   value: T,
64: ): void => {
65:   ensureParentDirectory(filePath);
66: 
67:   const document = serde.encode(value);
68:   const serialized = serializeTomlDocument(filePath, document);
69: 
70:   writeFileSync(filePath, serialized, "utf8");
71: };
72: 
73: export const updateTomlConfig = <T>(
74:   filePath: string,
75:   serde: TomlSerde<T>,
76:   mutator: (current: T) => T,
77: ): T => {
78:   const current = readTomlConfig(filePath, serde);
79:   const next = mutator(current);
80:   writeTomlConfig(filePath, serde, next);
81:   return next;
82: };
83: 
84: export const expectTomlTable = (
85:   value: unknown,
86:   path: string,
87: ): Record<string, unknown> => {
88:   if (typeof value !== "object" || value === null || Array.isArray(value)) {
89:     throw new Error(`Expected TOML table at ${path}`);
90:   }
91: 
92:   return value as Record<string, unknown>;
93: };
94: 
95: export function readTomlString(
96:   table: Record<string, unknown>,
97:   key: string,
98: ): string;
99: export function readTomlString(
100:   table: Record<string, unknown>,
101:   key: string,
102:   options: { optional: true },
103: ): string | undefined;
104: export function readTomlString(
105:   table: Record<string, unknown>,
106:   key: string,
107:   options?: { optional?: boolean },
108: ): string | undefined {
109:   const value = table[key];
110: 
111:   if (value === undefined || value === null) {
112:     if (options?.optional) {
113:       return undefined;
114:     }
115: 
116:     throw new Error(`Expected TOML string at ${key}`);
117:   }
118: 
119:   if (typeof value !== "string") {
120:     throw new Error(`Expected TOML string at ${key}`);
121:   }
122: 
123:   return value;
124: }
125: 
126: export function readTomlNumber(
127:   table: Record<string, unknown>,
128:   key: string,
129: ): number;
130: export function readTomlNumber(
131:   table: Record<string, unknown>,
132:   key: string,
133:   options: { optional: true },
134: ): number | undefined;
135: export function readTomlNumber(
136:   table: Record<string, unknown>,
137:   key: string,
138:   options?: { optional?: boolean },
139: ): number | undefined {
140:   const value = table[key];
141: 
142:   if (value === undefined || value === null) {
143:     if (options?.optional) {
144:       return undefined;
145:     }
146: 
147:     throw new Error(`Expected TOML number at ${key}`);
148:   }
149: 
150:   if (typeof value !== "number" || Number.isNaN(value)) {
151:     throw new Error(`Expected TOML number at ${key}`);
152:   }
153: 
154:   return value;
155: }
156: 
157: export function readTomlBoolean(
158:   table: Record<string, unknown>,
159:   key: string,
160: ): boolean;
161: export function readTomlBoolean(
162:   table: Record<string, unknown>,
163:   key: string,
164:   options: { optional: true },
165: ): boolean | undefined;
166: export function readTomlBoolean(
167:   table: Record<string, unknown>,
168:   key: string,
169:   options?: { optional?: boolean },
170: ): boolean | undefined {
171:   const value = table[key];
172: 
173:   if (value === undefined || value === null) {
174:     if (options?.optional) {
175:       return undefined;
176:     }
177: 
178:     throw new Error(`Expected TOML boolean at ${key}`);
179:   }
180: 
181:   if (typeof value !== "boolean") {
182:     throw new Error(`Expected TOML boolean at ${key}`);
183:   }
184: 
185:   return value;
186: }
187: 

(End of file - total 187 lines)
</content>

<path>/Users/alex/repos/vfp/dark-factory/docs/external/opencode/docs__sdk.ext.md</path>
<type>file</type>
<content>1: ----
2: ## External Docs Snapshot // opencode
3: 
4: - Captured: 2026-02-16T04:13:51.889Z
5: - Source root: https://opencode.ai/docs
6: - Source page: /docs/sdk
7: - Keywords: opencode, docs, ai coding assistant, cli, sdk
8: - Summary: Type-safe JS client for opencode server.
9: ----
10: 
11: Source: https://opencode.ai/docs/sdk
12: 
13: # SDK
14: 
15: Type-safe JS client for opencode server.
16: 
17: The opencode JS/TS SDK provides a type-safe client for interacting with the server.
18: Use it to build integrations and control opencode programmatically.
19: 
20: [Learn more](/docs/server) about how the server works. For examples, check out the [projects](/docs/ecosystem#projects) built by the community.
21: 
22: ## [Install](#install)
23: 
24: Install the SDK from npm:
25: 
26: - Terminal window ``` npm install @opencode-ai/sdk ``` ## [Create client](#create-client) Create an instance of opencode: ``` import { createOpencode } from "@opencode-ai/sdk" const { client } = await createOpencode() ``` This starts both a server and a client #### [Options](#options) OptionTypeDescriptionDefault`hostname``string`Server hostname`127.0.0.1``port``number`Server port`4096``signal``AbortSignal`Abort signal for cancellation`undefined``timeout``number`Timeout in ms for server start`5000``config``Config`Configuration object`{}` ## [Config](#config) You can pass a configuration object to customize behavior. The instance still picks up your `opencode.json`, but you can override or add configuration inline: ``` import { createOpencode } from "@opencode-ai/sdk" const opencode = await createOpencode({ hostname: "127.0.0.1", port: 4096, config: { model: "anthropic/claude-3-5-sonnet-20241022", },}) console.log(`Server running at ${opencode.server.url}`) opencode.server.close() ``` ## [Client only](#client-only) If you already have a running instance of opencode, you can create a client instance to connect to it: ``` import { createOpencodeClient } from "@opencode-ai/sdk" const client = createOpencodeClient({ baseUrl: "http://localhost:4096",}) ``` #### [Options](#options-1) OptionTypeDescriptionDefault`baseUrl``string`URL of the server`http://localhost:4096``fetch``function`Custom fetch implementation`globalThis.fetch``parseAs``string`Response parsing method`auto``responseStyle``string`Return style: `data` or `fields``fields``throwOnError``boolean`Throw errors instead of return`false` ## [Types](#types) The SDK includes TypeScript definitions for all API types. Import them directly: ``` import type { Session, Message, Part } from "@opencode-ai/sdk" ``` All types are generated from the server’s OpenAPI specification and available in the [types file](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts). ## [Errors](#errors) The SDK c...
27: 
28: - Use `required` to specify which fields must be present
29: 
30: - Keep schemas focused - complex nested schemas may be harder for the model to fill correctly
31: 
32: - Set appropriate `retryCount` - increase for complex schemas, decrease for simple ones
33: 
34: ## [APIs](#apis)
35: 
36: The SDK exposes all server APIs through a type-safe client.
37: 
38: ### [Global](#global)
39: 
40: Method Description Response
41: 
42: `global.health()` Check server health and version `{ healthy: true, version: string }`
43: 
44: #### [Examples](#examples)
45: 
46: ```
47: const health = await client.global.health()console.log(health.data.version)
48: ```
49: 
50: ### [App](#app)
51: 
52: Method Description Response
53: 
54: `app.log()` Write a log entry `boolean`
55: 
56: `app.agents()` List all available agents [`Agent[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
57: 
58: #### [Examples](#examples-1)
59: 
60: ```
61: // Write a log entryawait client.app.log({  body: {    service: "my-app",    level: "info",    message: "Operation completed",  },})
62: // List available agentsconst agents = await client.app.agents()
63: ```
64: 
65: ### [Project](#project)
66: 
67: Method Description Response
68: 
69: `project.list()` List all projects [`Project[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
70: 
71: `project.current()` Get current project [`Project`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
72: 
73: #### [Examples](#examples-2)
74: 
75: ```
76: // List all projectsconst projects = await client.project.list()
77: // Get current projectconst currentProject = await client.project.current()
78: ```
79: 
80: ### [Path](#path)
81: 
82: Method Description Response
83: 
84: `path.get()` Get current path [`Path`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
85: 
86: #### [Examples](#examples-3)
87: 
88: ```
89: // Get current path informationconst pathInfo = await client.path.get()
90: ```
91: 
92: ### [Config](#config-1)
93: 
94: Method Description Response
95: 
96: `config.get()` Get config info [`Config`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
97: 
98: `config.providers()` List providers and default models `{ providers:`[`Provider[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`, default: { [key: string]: string } }`
99: 
100: #### [Examples](#examples-4)
101: 
102: ```
103: const config = await client.config.get()
104: const { providers, default: defaults } = await client.config.providers()
105: ```
106: 
107: ### [Sessions](#sessions)
108: 
109: Method Description Notes
110: 
111: `session.list()` List sessions Returns [`Session[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
112: 
113: `session.get({ path })` Get session Returns [`Session`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
114: 
115: `session.children({ path })` List child sessions Returns [`Session[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
116: 
117: `session.create({ body })` Create session Returns [`Session`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
118: 
119: `session.delete({ path })` Delete session Returns `boolean`
120: 
121: `session.update({ path, body })` Update session properties Returns [`Session`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
122: 
123: `session.init({ path, body })` Analyze app and create `AGENTS.md` Returns `boolean`
124: 
125: `session.abort({ path })` Abort a running session Returns `boolean`
126: 
127: `session.share({ path })` Share session Returns [`Session`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
128: 
129: `session.unshare({ path })` Unshare session Returns [`Session`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
130: 
131: `session.summarize({ path, body })` Summarize session Returns `boolean`
132: 
133: `session.messages({ path })` List messages in a session Returns `{ info:`[`Message`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`, parts:`[`Part[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`}[]`
134: 
135: `session.message({ path })` Get message details Returns `{ info:`[`Message`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`, parts:`[`Part[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`}`
136: 
137: `session.prompt({ path, body })` Send prompt message `body.noReply: true` returns UserMessage (context only). Default returns [`AssistantMessage`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts) with AI response. Supports `body.outputFormat` for [structured output](#structured-output)
138: 
139: `session.command({ path, body })` Send command to session Returns `{ info:`[`AssistantMessage`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`, parts:`[`Part[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`}`
140: 
141: `session.shell({ path, body })` Run a shell command Returns [`AssistantMessage`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
142: 
143: `session.revert({ path, body })` Revert a message Returns [`Session`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
144: 
145: `session.unrevert({ path })` Restore reverted messages Returns [`Session`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
146: 
147: `postSessionByIdPermissionsByPermissionId({ path, body })` Respond to a permission request Returns `boolean`
148: 
149: #### [Examples](#examples-5)
150: 
151: ```
152: // Create and manage sessionsconst session = await client.session.create({  body: { title: "My session" },})
153: const sessions = await client.session.list()
154: // Send a prompt messageconst result = await client.session.prompt({  path: { id: session.id },  body: {    model: { providerID: "anthropic", modelID: "claude-3-5-sonnet-20241022" },    parts: [{ type: "text", text: "Hello!" }],  },})
155: // Inject context without triggering AI response (useful for plugins)await client.session.prompt({  path: { id: session.id },  body: {    noReply: true,    parts: [{ type: "text", text: "You are a helpful assistant." }],  },})
156: ```
157: 
158: ### [Files](#files)
159: 
160: Method Description Response
161: 
162: `find.text({ query })` Search for text in files Array of match objects with `path`, `lines`, `line_number`, `absolute_offset`, `submatches`
163: 
164: `find.files({ query })` Find files and directories by name `string[]` (paths)
165: 
166: `find.symbols({ query })` Find workspace symbols [`Symbol[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
167: 
168: `file.read({ query })` Read a file `{ type: "raw" | "patch", content: string }`
169: 
170: `file.status({ query? })` Get status for tracked files [`File[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
171: 
172: `find.files` supports a few optional query fields:
173: 
174: - `type`: `"file"` or `"directory"`
175: 
176: - `directory`: override the project root for the search
177: 
178: - `limit`: max results (1–200)
179: 
180: #### [Examples](#examples-6)
181: 
182: ```
183: // Search and read filesconst textResults = await client.find.text({  query: { pattern: "function.*opencode" },})
184: const files = await client.find.files({  query: { query: "*.ts", type: "file" },})
185: const directories = await client.find.files({  query: { query: "packages", type: "directory", limit: 20 },})
186: const content = await client.file.read({  query: { path: "src/index.ts" },})
187: ```
188: 
189: ### [TUI](#tui)
190: 
191: Method Description Response
192: 
193: `tui.appendPrompt({ body })` Append text to the prompt `boolean`
194: 
195: `tui.openHelp()` Open the help dialog `boolean`
196: 
197: `tui.openSessions()` Open the session selector `boolean`
198: 
199: `tui.openThemes()` Open the theme selector `boolean`
200: 
201: `tui.openModels()` Open the model selector `boolean`
202: 
203: `tui.submitPrompt()` Submit the current prompt `boolean`
204: 
205: `tui.clearPrompt()` Clear the prompt `boolean`
206: 
207: `tui.executeCommand({ body })` Execute a command `boolean`
208: 
209: `tui.showToast({ body })` Show toast notification `boolean`
210: 
211: #### [Examples](#examples-7)
212: 
213: ```
214: // Control TUI interfaceawait client.tui.appendPrompt({  body: { text: "Add this to prompt" },})
215: await client.tui.showToast({  body: { message: "Task completed", variant: "success" },})
216: ```
217: 
218: ### [Auth](#auth)
219: 
220: Method Description Response
221: 
222: `auth.set({ ... })` Set authentication credentials `boolean`
223: 
224: #### [Examples](#examples-8)
225: 
226: ```
227: await client.auth.set({  path: { id: "anthropic" },  body: { type: "api", key: "your-api-key" },})
228: ```
229: 
230: ### [Events](#events)
231: 
232: Method Description Response
233: 
234: `event.subscribe()` Server-sent events stream Server-sent events stream
235: 
236: #### [Examples](#examples-9)
237: 
238: ```
239: // Listen to real-time eventsconst events = await client.event.subscribe()for await (const event of events.stream) {  console.log("Event:", event.type, event.properties)}
240: ```
241: 
242: [Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/sdk.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord) Select language   EnglishالعربيةBosanskiDanskDeutschEspañolFrançaisItaliano日本語한국어Norsk BokmålPolskiPortuguês (Brasil)РусскийไทยTürkçe简体中文繁體中文
243: 
244: &copy; [Anomaly](https://anoma.ly)
245: 
246: Last updated: Feb 15, 2026
247: 
248: ----
249: ## Notes / Comments / Lessons
250: 
251: - Collection method: sitemap discovery + markdown conversion.
252: - Conversion path: direct HTML fallback parser.
253: - This file is one page-level external snapshot in markdown `.ext.md` format.
254: ----
255: 

(End of file - total 255 lines)
</content>

<path>/Users/alex/repos/vfp/dark-factory/docs/external/opencode/docs__server.ext.md</path>
<type>file</type>
<content>1: ----
2: ## External Docs Snapshot // opencode
3: 
4: - Captured: 2026-02-16T04:13:51.889Z
5: - Source root: https://opencode.ai/docs
6: - Source page: /docs/server
7: - Keywords: opencode, docs, ai coding assistant, cli, server
8: - Summary: Interact with opencode server over HTTP.
9: ----
10: 
11: Source: https://opencode.ai/docs/server
12: 
13: # Server
14: 
15: Interact with opencode server over HTTP.
16: 
17: The `opencode serve` command runs a headless HTTP server that exposes an OpenAPI endpoint that an opencode client can use.
18: 
19: ### [Usage](#usage)
20: 
21: - Terminal window ``` opencode serve [--port &#x3C;number>] [--hostname &#x3C;string>] [--cors &#x3C;origin>] ``` #### [Options](#options) FlagDescriptionDefault`--port`Port to listen on`4096``--hostname`Hostname to listen on`127.0.0.1``--mdns`Enable mDNS discovery`false``--mdns-domain`Custom domain name for mDNS service`opencode.local``--cors`Additional browser origins to allow`[]` `--cors` can be passed multiple times: Terminal window ``` opencode serve --cors http://localhost:5173 --cors https://app.example.com ``` ### [Authentication](#authentication) Set `OPENCODE_SERVER_PASSWORD` to protect the server with HTTP basic auth. The username defaults to `opencode`, or set `OPENCODE_SERVER_USERNAME` to override it. This applies to both `opencode serve` and `opencode web`. Terminal window ``` OPENCODE_SERVER_PASSWORD=your-password opencode serve ``` ### [How it works](#how-it-works) When you run `opencode` it starts a TUI and a server. Where the TUI is the client that talks to the server. The server exposes an OpenAPI 3.1 spec endpoint. This endpoint is also used to generate an [SDK](/docs/sdk). TipUse the opencode server to interact with opencode programmatically. This architecture lets opencode support multiple clients and allows you to interact with opencode programmatically. You can run `opencode serve` to start a standalone server. If you have the opencode TUI running, `opencode serve` will start a new server. #### [Connect to an existing server](#connect-to-an-existing-server) When you start the TUI it randomly assigns a port and hostname. You can instead pass in the `--hostname` and `--port` [flags](/docs/cli). Then use this to connect to its server. The [`/tui`](#tui) endpoint can be used to drive the TUI through the server. For example, you can prefill or run a prompt. This setup is used by the OpenCode [IDE](/docs/ide) plugins. ## [Spec](#spec) The server publishes an OpenAPI 3.1 spec that can be viewed at: ``` http://&#x3C;hostname>:&#x3C;port>/doc ``` For ...
22: 
23: - `type` (optional) — limit results to `"file"` or `"directory"`
24: 
25: - `directory` (optional) — override the project root for the search
26: 
27: - `limit` (optional) — max results (1–200)
28: 
29: - `dirs` (optional) — legacy flag (`"false"` returns only files)
30: 
31: ### [Tools (Experimental)](#tools-experimental)
32: 
33: Method Path Description Response
34: 
35: `GET` `/experimental/tool/ids` List all tool IDs [`ToolIDs`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
36: 
37: `GET` `/experimental/tool?provider= &model=` List tools with JSON schemas for a model [`ToolList`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
38: 
39: ### [LSP, Formatters &#x26; MCP](#lsp-formatters--mcp)
40: 
41: Method Path Description Response
42: 
43: `GET` `/lsp` Get LSP server status [`LSPStatus[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
44: 
45: `GET` `/formatter` Get formatter status [`FormatterStatus[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
46: 
47: `GET` `/mcp` Get MCP server status `{ [name: string]:`[MCPStatus](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)`}`
48: 
49: `POST` `/mcp` Add MCP server dynamically body: `{ name, config }`, returns MCP status object
50: 
51: ### [Agents](#agents)
52: 
53: Method Path Description Response
54: 
55: `GET` `/agent` List all available agents [`Agent[]`](https://github.com/anomalyco/opencode/blob/dev/packages/sdk/js/src/gen/types.gen.ts)
56: 
57: ### [Logging](#logging)
58: 
59: Method Path Description Response
60: 
61: `POST` `/log` Write log entry. Body: `{ service, level, message, extra? }` `boolean`
62: 
63: ### [TUI](#tui)
64: 
65: Method Path Description Response
66: 
67: `POST` `/tui/append-prompt` Append text to the prompt `boolean`
68: 
69: `POST` `/tui/open-help` Open the help dialog `boolean`
70: 
71: `POST` `/tui/open-sessions` Open the session selector `boolean`
72: 
73: `POST` `/tui/open-themes` Open the theme selector `boolean`
74: 
75: `POST` `/tui/open-models` Open the model selector `boolean`
76: 
77: `POST` `/tui/submit-prompt` Submit the current prompt `boolean`
78: 
79: `POST` `/tui/clear-prompt` Clear the prompt `boolean`
80: 
81: `POST` `/tui/execute-command` Execute a command (`{ command }`) `boolean`
82: 
83: `POST` `/tui/show-toast` Show toast (`{ title?, message, variant }`) `boolean`
84: 
85: `GET` `/tui/control/next` Wait for the next control request Control request object
86: 
87: `POST` `/tui/control/response` Respond to a control request (`{ body }`) `boolean`
88: 
89: ### [Auth](#auth)
90: 
91: Method Path Description Response
92: 
93: `PUT` `/auth/:id` Set authentication credentials. Body must match provider schema `boolean`
94: 
95: ### [Events](#events)
96: 
97: Method Path Description Response
98: 
99: `GET` `/event` Server-sent events stream. First event is `server.connected`, then bus events Server-sent events stream
100: 
101: ### [Docs](#docs)
102: 
103: Method Path Description Response
104: 
105: `GET` `/doc` OpenAPI 3.1 specification HTML page with OpenAPI spec
106: 
107: [Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/server.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord) Select language   EnglishالعربيةBosanskiDanskDeutschEspañolFrançaisItaliano日本語한국어Norsk BokmålPolskiPortuguês (Brasil)РусскийไทยTürkçe简体中文繁體中文
108: 
109: &copy; [Anomaly](https://anoma.ly)
110: 
111: Last updated: Feb 15, 2026
112: 
113: ----
114: ## Notes / Comments / Lessons
115: 
116: - Collection method: sitemap discovery + markdown conversion.
117: - Conversion path: direct HTML fallback parser.
118: - This file is one page-level external snapshot in markdown `.ext.md` format.
119: ----
120: 

(End of file - total 120 lines)
</content>

### 2. assistant (2026-02-16T04:58:26.202Z)

_No text content captured for this message._

### 3. assistant (2026-02-16T04:58:36.685Z)

_No text content captured for this message._

### 4. assistant (2026-02-16T04:58:44.150Z)

_No text content captured for this message._

### 5. assistant (2026-02-16T04:58:50.671Z)

_No text content captured for this message._

### 6. assistant (2026-02-16T04:58:55.031Z)

_No text content captured for this message._

### 7. assistant (2026-02-16T04:58:58.778Z)

_No text content captured for this message._

### 8. assistant (2026-02-16T04:59:05.794Z)

_No text content captured for this message._

### 9. assistant (2026-02-16T04:59:10.881Z)

_No text content captured for this message._

### 10. assistant (2026-02-16T04:59:15.058Z)

_No text content captured for this message._

### 11. assistant (2026-02-16T04:59:26.593Z)

_No text content captured for this message._

### 12. assistant (2026-02-16T04:59:32.286Z)

_No text content captured for this message._

### 13. assistant (2026-02-16T04:59:36.025Z)

_No text content captured for this message._

### 14. assistant (2026-02-16T04:59:40.650Z)

_No text content captured for this message._

### 15. assistant (2026-02-16T04:59:45.482Z)

_No text content captured for this message._

### 16. assistant (2026-02-16T04:59:52.428Z)

_No text content captured for this message._

### 17. assistant (2026-02-16T04:59:56.277Z)

_No text content captured for this message._

### 18. assistant (2026-02-16T05:00:01.805Z)

_No text content captured for this message._

### 19. assistant (2026-02-16T05:00:07.705Z)

_No text content captured for this message._

### 20. assistant (2026-02-16T05:00:17.333Z)

_No text content captured for this message._

### 21. assistant (2026-02-16T05:00:24.568Z)

_No text content captured for this message._

### 22. assistant (2026-02-16T05:01:02.531Z)

_No text content captured for this message._

### 23. assistant (2026-02-16T05:02:24.281Z)

_No text content captured for this message._

### 24. assistant (2026-02-16T05:02:33.530Z)

_No text content captured for this message._

----
## Notes

- Export source: `opencode export <sessionID>` JSON payload.
- This markdown keeps text parts only; use the sibling `.json` file for full structured data.
----
