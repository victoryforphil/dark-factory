# Dark Factory - `dark_chat`

Ratatui-based OpenCode chat frontend that also exports a reusable chat framework library.

`dark_chat` currently has two roles:

1. a standalone TUI binary (`frontends/dark_chat/src/main.rs`)
2. a library used by other frontends (notably `dark_tui`) for chat rendering and provider access

## Current Status

- Runnable TUI binary (`dark_chat`) plus library exports in `src/lib.rs`.
- Provider architecture supports multiple backends; `opencode/server` is implemented first.
- OpenCode provider internals are split into focused modules:
  - `providers/opencode_server.rs` (provider surface)
  - `providers/opencode_transport.rs` (HTTP fallback request helpers)
  - `providers/opencode_realtime.rs` (SSE event streaming)
  - `providers/opencode_extract.rs` (payload extraction helpers)
  - `providers/opencode_wire.rs` (wire DTOs)
- TUI is split into app/panels modules and consumes shared components from `lib/dark_tui_components`.
- `framework/` exports reusable chat building blocks used by both `dark_chat` and `dark_tui`.
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

## Library Exports (`dark_chat::framework`)

- `conversation_panel` shared render API (`render_conversation_panel`, message/header/composer props)
- `autocomplete` reusable slash/@ completion state (`ChatAutocomplete`)
- `model_selector` reusable selector state (`ItemSelector`)
- `session_tree` reusable parent/child session walker
- `message_renderer` extraction helper (`extract_message_text`)
- `message_types` rich message data types (`AgentMessage*`)

## Runtime Options

| Option | Env | Default | Description |
| --- | --- | --- | --- |
| `--base-url <URL>` | `DARK_CHAT_BASE_URL` | `http://127.0.0.1:4096` | OpenCode server URL |
| `--directory <path>` | `DARK_CHAT_DIRECTORY` | current directory | Workspace directory to target |
| `--refresh-seconds <n>` | `DARK_CHAT_REFRESH_SECONDS` | `3` | Auto-refresh cadence |
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

## Check/Test

```bash
cargo check -p dark_chat
cargo test -p dark_chat
```
