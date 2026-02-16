# Dark Factory - `dark_chat`

Ratatui-based OpenCode chat frontend with reusable provider/core layers.

## Current Status

- New frontend crate with a runnable TUI binary (`dark_chat`).
- Provider architecture supports multiple backends; `opencode/server` is implemented first.
- Core backend module handles bootstrap, refresh, session creation, and prompt send.
- TUI is split into views, panels, and components, and uses shared chat widgets from `lib/dark_tui_components`.
- Session list, conversation history, and composer are all keyboard-driven.
- Compose mode now uses `tui-textarea` for multiline editing with built-in cursor and undo behavior.
- Runtime panel now uses `tui-scrollview` for focused scrolling of status/help content.
- Conversation messages render Markdown structure (headings, emphasis, lists, blockquotes, and code fences).
- Realtime event syncing uses OpenCode `/event` stream with reconnect attempts.
- Agent/model options are loaded from OpenCode config routes and can be cycled at runtime.
- Local slash commands are available (`/help`, `/refresh`, `/new`, `/sessions`, `/agent`, `/model`, `/grep`, `/clear`).
- Non-local slash commands are forwarded to OpenCode session command execution.
- Prompt composer supports `@file/path` context injection from files inside the workspace directory.
- Runtime panel surfaces `mcp`, `lsp`, and formatter status snapshots when available.

## Runtime Options

| Option | Env | Default | Description |
| --- | --- | --- | --- |
| `--base-url <URL>` | `DARK_CHAT_BASE_URL` | `http://127.0.0.1:4096` | OpenCode server URL |
| `--directory <path>` | `DARK_CHAT_DIRECTORY` | current directory | Workspace directory to target |
| `--refresh-seconds <n>` | `DARK_CHAT_REFRESH_SECONDS` | `5` | Auto-refresh cadence |
| `--session <id>` | `DARK_CHAT_SESSION` | unset | Prefer session id on boot |
| `--session-title <title>` | `DARK_CHAT_SESSION_TITLE` | unset | Preferred title for bootstrap-created session |
| `--provider <provider>` | `DARK_CHAT_PROVIDER` | `opencode/server` | Chat provider backend |

## Keybindings

- `q` or `Ctrl+C`: quit
- `j`/`Down`, `k`/`Up`: switch selected session
- `j`/`k` also scroll focused chat/runtime panes
- `r`: refresh sessions and messages
- `n`: create a new session
- `a`: cycle selected agent
- `m`: open model picker (search + raw key)
- `c`: open compose mode
- `Enter` (compose mode): send prompt
- `Shift+Enter` (compose mode): insert newline
- `Esc` (compose mode): cancel compose
- `h`: toggle help in runtime panel

Compose extras:

- Prefix input with `/` for slash commands (`/help`, `/refresh`, `/agent <name>`, `/model <name>`, `/grep <pattern>`, etc.)
- Include `@relative/path.ext` tokens to inject file context into the prompt before submission

## Run

From repo root:

```bash
cargo run --manifest-path frontends/dark_chat/Cargo.toml -- --help
```
