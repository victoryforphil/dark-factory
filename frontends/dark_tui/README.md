# Dark Factory - `dark_tui`

Ratatui-based TUI frontend for monitoring and operating `dark_core`.

## Current Status

- Grid-style dashboard is active for products and variants with actor/runtime telemetry in the header.
- Sidebar panel is metadata-focused and follows the actively highlighted product/variant selection.
- Viz mode supports click-to-select product/variant cards plus drag/scroll panning.
- `n` opens a `Spawn in TUI` popup with provider selection and an initial prompt field.
- Selecting an actor opens a dedicated chat panel between catalog and details panes.
- Chat panel supports compose/send and can be toggled visible/hidden.
- Action keys support refresh, variant poll, product init, spawn, and attach command generation.
- Dashboard reuses shared HTTP client/types from `lib/dark_rust`.

## Scope (Current)

- Provide an at-a-glance terminal dashboard for product/variant state and actor runtime health.
- Keep keyboard-first controls human-friendly and discoverable.
- Keep implementation modular so adding panes/actions stays straightforward.

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
| `--refresh-seconds <n>` | `DARK_TUI_REFRESH_SECONDS` | `8` | Auto-refresh cadence |
| `--poll-variants <true\|false>` | `DARK_TUI_POLL_VARIANTS` | `true` | Poll variant git metadata while listing |

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

## Run

From repo root:

```bash
cargo run --manifest-path frontends/dark_tui/Cargo.toml -- --help
```

Or with helper script:

```bash
./scripts/dtui.sh.ts --help
```
