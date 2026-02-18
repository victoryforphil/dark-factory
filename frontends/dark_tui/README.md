# Dark Factory - `dark_tui`

Ratatui-based TUI frontend for monitoring and operating `dark_core`.

## Current Status

- Grid-style dashboard is active for products and variants with actor/runtime telemetry in the header.
- Sidebar panel is metadata-focused and follows the actively highlighted product/variant selection.
- Viz mode supports click-to-select product/variant cards plus drag/scroll panning.
- `n` opens a `Spawn in TUI` popup with provider selection and an initial prompt field.
- Selecting an actor opens a dedicated chat panel between catalog and details panes.
- Chat panel supports compose/send, can be toggled visible/hidden, and is rendered via shared `dark_chat` framework components.
- Action keys support refresh, variant poll, product init, spawn, and attach command generation.
- Dashboard prefers shared websocket RPC transport from `lib/dark_rust` and falls back to REST when websocket is unavailable.
- Realtime route mutation events from `dark_core` trigger immediate refreshes between interval ticks.
- Service code is split into focused modules:
  - `src/service.rs` (service API/orchestration)
  - `src/service_wire.rs` (wire DTOs)
  - `src/service_convert.rs` (row conversion and helper functions)
- Unified catalog rendering is split across:
  - `src/ui/render/views/unified_catalog_view.rs` (layout + interaction)
  - `src/ui/render/views/catalog_cards.rs` (card + connector render helpers)

## Scope (Current)

- Provide an at-a-glance terminal dashboard for product/variant state and actor runtime health.
- Keep keyboard-first controls human-friendly and discoverable.
- Keep implementation modular so adding panes/actions stays straightforward.

## Shared Dependencies

- `lib/dark_rust` for `dark_core` REST/WebSocket clients and API envelopes.
- `lib/dark_tui_components` for reusable pane/chat/status widgets and utilities.
- `dark_chat::framework` for shared conversation panel rendering and chat-focused UI types.

## Ratatui Docs

- `dark_tui` uses `ratatui` for terminal UI rendering.
- Local external snapshots for reference:
  - `docs/external/ratatui_web/index.ext.md` (Ratatui website guides/tutorials/examples)
  - `docs/external/ratatui_docs/index.ext.md` (docs.rs API snapshots)

## Runtime Options

| Option | Env | Default | Description |
| --- | --- | --- | --- |
| `--base-url <URL>` | `DARK_CORE_BASE_URL` | `http://localhost:4150` | Base URL for `dark_core` |
| `--directory <path>` | `DARK_TUI_DIRECTORY` | current directory | Directory used for local product/variant actor workflows |
| `--refresh-seconds <n>` | `DARK_TUI_REFRESH_SECONDS` | `2` | Base auto-refresh cadence (boosts to 1s while actors/sub-agents are busy) |
| `--actor-auto-poll-seconds <n>` | `DARK_TUI_ACTOR_AUTO_POLL_SECONDS` | `5` | Base actor status polling cadence (boosts to 2s while actors/sub-agents are busy) |
| `--poll-variants <true\|false>` | `DARK_TUI_POLL_VARIANTS` | `true` | Poll variant git metadata while listing |

Runtime behavior:

- For local base URLs (`localhost`/`127.0.0.1`), `dark_tui` now ensures `dark_core` is running in a tmux session before launching the TUI.
- If the `dark_core` executable is missing, `dark_tui` auto-runs `bun run build:exec` in `dark_core` first.
- Disable this behavior with `DARK_TUI_AUTO_START_DARK_CORE=false`.

## Keybindings

- `q` or `Ctrl+C`: quit
- `Tab` / `Shift+Tab`: cycle focus across products and variants
- `j`/`Down`, `k`/`Up`: move selection inside focused pane
- `r`: refresh all panes
- `f`: toggle variant filter (selected product only vs all variants)
- `p`: poll selected variant
- `i`: run product init for the configured directory
- `n`: open spawn popup (provider + initial prompt)
- `a`: build attach command for selected actor
- `l`: toggle embedded `dark_core` tmux log panel
- `t`: toggle chat panel visibility
- `c`: start composing a chat prompt for selected actor

Chat compose controls:

- `Enter`: send prompt
- `Esc`: cancel compose mode
- `Backspace`: delete prompt text

Spawn popup controls:

- `j`/`Down`, `k`/`Up`: select provider
- type text: edit initial prompt
- `Backspace`: delete prompt text
- `Enter`: spawn with selected provider and prompt
- `Esc`: close popup

Provider source of truth:

- `dark_tui` queries `GET /system/providers` for enabled/default providers when opening the spawn popup.

## Check/Test

```bash
cargo check -p dark_tui
cargo test -p dark_tui
```

## Run

From repo root:

```bash
cargo run --manifest-path frontends/dark_tui/Cargo.toml -- --help
```

Or with helper script:

```bash
./scripts/dtui.sh.ts --help
```
