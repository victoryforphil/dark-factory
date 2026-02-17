# Phase 3: Split Monolithic app.rs in Both Crates

**Risk**: Medium | **Impact**: Medium | **Dependencies**: Phase 2 (trait informs split boundaries)

## Goal

Break the monolithic `App` structs in both frontends into focused sub-modules. The `App` struct remains the single state holder but delegates to sub-state structs. This makes the codebase navigable, testable, and ready for Component extraction in Phase 4-5.

---

## Part A: Split `frontends/dark_chat/src/tui/app.rs` (1,285 lines)

### Current field groupings (from analysis)

| Group | Fields | Lines |
|-------|--------|-------|
| Core/Config | base_url, directory, provider_name, preferences_path, refresh_seconds | 42-46 |
| Theme/UI | theme, focus, status_message, show_help, last_synced | 47,59,60,93,94 |
| Chat State | health, sessions, selected_session, messages, runtime_status, scrolls | 48-50,57-58,83-85 |
| Agents/Models | agents, selected_agent, models, selected_model, active_model_override | 51-55 |
| Composer | draft, draft_cursor, composer, composing | 61-64 |
| Autocomplete | composer_autocomplete_* (6 fields), workspace_file_cache* | 65-72 |
| Selectors | model_selector_* (6), agent_selector_* (4) | 73-82 |
| Flight/Realtime | *_in_flight (3), realtime_* (4) | 86-92 |

### Target structure

```
frontends/dark_chat/src/tui/
├── app/
│   ├── mod.rs            # App struct (thin), re-exports
│   ├── state.rs          # Core config, theme, UI globals, chat state, flight/realtime
│   ├── composer.rs       # ComposerState: draft, cursor, textarea, composing
│   ├── autocomplete.rs   # AutocompleteState: open, mode, query, selected, items, workspace cache
│   ├── selectors.rs      # SelectorState: model_selector_*, agent_selector_*
│   └── persistence.rs    # persist/load selection logic, preferences path
├── input.rs              # (unchanged)
├── mod.rs                # (unchanged)
...
```

### Step 1: Create `app/composer.rs`

Extract composer state into its own struct:

```rust
use tui_textarea::TextArea;

pub struct ComposerState {
    pub(crate) draft: String,
    pub(crate) draft_cursor: usize,
    pub(crate) composer: TextArea<'static>,
    pub(crate) composing: bool,
}

impl ComposerState {
    pub fn new() -> Self { /* current init values */ }

    // Move these methods from App:
    pub fn open(&mut self) { /* lines 729-739 */ }
    pub fn cancel(&mut self) { /* lines 741-745 */ }
    pub fn insert_char(&mut self, ch: char) { /* lines 747-755 */ }
    pub fn backspace(&mut self) { /* lines 757-769 */ }
    pub fn delete_char(&mut self) { /* lines 771-778 */ }
    pub fn move_cursor_left(&mut self) { /* lines 780-782 */ }
    pub fn move_cursor_right(&mut self) { /* lines 784-790 */ }
    pub fn move_cursor_start(&mut self) { /* lines 792-796 */ }
    pub fn move_cursor_end(&mut self) { /* lines 798-802 */ }
    pub fn move_cursor_word_left(&mut self) { /* lines 804-808 */ }
    pub fn move_cursor_word_right(&mut self) { /* lines 810-815 */ }
    pub fn clear_draft(&mut self) { /* lines 817-821 */ }
    pub fn sync_from_draft(&mut self) { /* lines 949-952 */ }
    pub fn take_prompt(&self) -> Option<String> { /* lines 865-876 */ }
    pub fn clear_after_send(&mut self) { /* lines 878-886 */ }

    // Getters
    pub fn draft(&self) -> &str { &self.draft }
    pub fn draft_cursor(&self) -> usize { self.draft_cursor }
    pub fn composer(&self) -> &TextArea<'static> { &self.composer }
    pub fn composing(&self) -> bool { self.composing }
}
```

**Methods moved**: Lines 729-821, 865-886, 949-952 (~160 lines)

### Step 2: Create `app/autocomplete.rs`

```rust
pub struct AutocompleteState {
    pub(crate) open: bool,
    pub(crate) mode: Option<ComposerAutocompleteMode>,
    pub(crate) query: String,
    pub(crate) selected: usize,
    pub(crate) token_start: usize,
    pub(crate) items: Vec<ComposerAutocompleteItem>,
    pub(crate) workspace_file_cache: Vec<String>,
    pub(crate) workspace_file_cache_loaded: bool,
}

// Types:
pub struct ComposerAutocompleteItem { pub label: String, pub insert: String, pub tag: String }
pub enum ComposerAutocompleteMode { Slash, File }

impl AutocompleteState {
    pub fn new() -> Self { /* current init values */ }

    // Move these methods from App:
    pub fn close(&mut self) { /* lines 668-675 */ }
    pub fn move_up(&mut self) { /* lines 677-686 */ }
    pub fn move_down(&mut self) { /* lines 688-697 */ }
    pub fn set_selected(&mut self, index: usize) { /* lines 698-706 */ }
    pub fn apply_selection(&mut self, draft: &mut String, cursor: &mut usize) -> Option<String> { /* lines 708-723 */ }
    pub fn refresh(&mut self, draft: &str, cursor: usize, composing: bool) { /* lines 954-993 */ }
    pub fn ensure_workspace_cache(&mut self, directory: &str) { /* lines 995-1015 */ }
    pub fn anchor_position(&self, draft: &str, cursor: usize) -> Option<(usize, usize)> { /* lines 257-266 */ }

    // Getters
    pub fn is_open(&self) -> bool { self.open }
    pub fn mode(&self) -> Option<ComposerAutocompleteMode> { self.mode }
    pub fn query(&self) -> &str { &self.query }
    pub fn selected(&self) -> usize { self.selected }
    pub fn items(&self) -> &[ComposerAutocompleteItem] { &self.items }
}

// Move free functions here:
fn current_composer_trigger(draft: &str, cursor: usize) -> Option<(char, String, usize)>
fn slash_autocomplete_items(query: &str) -> Vec<ComposerAutocompleteItem>
fn file_autocomplete_items(paths: &[String], query: &str) -> Vec<ComposerAutocompleteItem>
```

**Methods moved**: Lines 668-723, 954-1015, 1046-1175 (~200 lines)

**Coupling note**: `apply_selection` needs mutable access to `draft` and `cursor` from ComposerState. Solution: pass them as `&mut` params, or have App orchestrate: `let label = self.autocomplete.apply_selection(&mut self.composer.draft, &mut self.composer.draft_cursor)`.

### Step 3: Create `app/selectors.rs`

```rust
pub struct ModelSelectorState {
    pub(crate) open: bool,
    pub(crate) raw_mode: bool,
    pub(crate) query: String,
    pub(crate) raw_input: String,
    pub(crate) selected: usize,
    pub(crate) anchor_col: Option<u16>,
}

pub struct AgentSelectorState {
    pub(crate) open: bool,
    pub(crate) query: String,
    pub(crate) selected: usize,
    pub(crate) anchor_col: Option<u16>,
}

impl ModelSelectorState {
    // Move all model_selector_* methods (~120 lines)
    pub fn open(&mut self) { ... }
    pub fn close(&mut self) { ... }
    pub fn toggle_raw_mode(&mut self) { ... }
    pub fn insert_query_char(&mut self, ch: char) { ... }
    pub fn backspace_query(&mut self) { ... }
    pub fn clear_query(&mut self) { ... }
    pub fn move_up(&mut self) { ... }
    pub fn move_down(&mut self) { ... }
    pub fn set_selected(&mut self, index: usize) { ... }
    pub fn filtered_items<'a>(&self, models: &'a [String]) -> Vec<&'a str> { ... }
    pub fn confirm(&mut self, models: &[String]) -> Option<String> { ... }
}

impl AgentSelectorState {
    // Move all agent_selector_* methods (~90 lines)
    // Similar structure
}
```

**Methods moved**: Lines 440-661 (~220 lines)

**Coupling note**: `confirm` methods need the models/agents lists from core state. Pass them as params.

### Step 4: Create `app/persistence.rs`

```rust
pub fn persist_selection(path: &Path, model: Option<&str>, agent: Option<&str>) -> io::Result<()> { ... }
pub fn load_selection(path: &Path) -> io::Result<(Option<String>, Option<String>)> { ... }
```

**Methods moved**: Lines 1017-1044 (~30 lines)

### Step 5: Create `app/mod.rs` (thin aggregator)

```rust
mod autocomplete;
mod composer;
mod persistence;
mod selectors;
mod state;

pub use autocomplete::{AutocompleteState, ComposerAutocompleteItem, ComposerAutocompleteMode};
pub use composer::ComposerState;
pub use selectors::{ModelSelectorState, AgentSelectorState};

// Re-export types that existed before
pub use state::{App, FocusPane};
```

### Step 6: Refactor App struct in `app/state.rs`

```rust
pub struct App {
    // Core/Config
    base_url: String,
    directory: String,
    provider_name: String,
    preferences_path: PathBuf,
    refresh_seconds: u64,

    // Theme/UI
    theme: ComponentTheme,
    focus: FocusPane,
    status_message: String,
    show_help: bool,
    last_synced: String,

    // Chat state
    health: ProviderHealth,
    sessions: Vec<ChatSession>,
    selected_session: usize,
    messages: Vec<ChatMessage>,
    runtime_status: ProviderRuntimeStatus,
    agents: Vec<String>,
    selected_agent: usize,
    models: Vec<String>,
    selected_model: usize,
    active_model_override: Option<String>,

    // Scrolls
    sessions_scroll_index: usize,
    chat_scroll_lines: u16,
    runtime_scroll_lines: u16,

    // Flight/Realtime
    refresh_in_flight: bool,
    send_in_flight: bool,
    create_in_flight: bool,
    realtime_supported: bool,
    realtime_connected: bool,
    realtime_last_event: Option<String>,
    realtime_event_count: u64,

    // Sub-states (new)
    pub(crate) composer: ComposerState,
    pub(crate) autocomplete: AutocompleteState,
    pub(crate) model_selector: ModelSelectorState,
    pub(crate) agent_selector: AgentSelectorState,
}
```

The existing getters/methods stay on `App` but delegate:
```rust
impl App {
    // Composer delegation
    pub fn composer(&self) -> &TextArea<'static> { self.composer.composer() }
    pub fn composing(&self) -> bool { self.composer.composing() }
    pub fn open_composer(&mut self) { self.composer.open() }
    // ... etc

    // Autocomplete delegation
    pub fn composer_autocomplete_open(&self) -> bool { self.autocomplete.is_open() }
    // ... etc

    // Selector delegation
    pub fn is_model_selector_open(&self) -> bool { self.model_selector.open }
    // ... etc
}
```

**This preserves the existing public API** so `input.rs`, `mod.rs`, and panels don't need changes yet.

### Expected line reduction

| File | Before | After |
|------|--------|-------|
| `app/state.rs` (was `app.rs`) | 1,285 | ~500 |
| `app/composer.rs` | 0 | ~170 |
| `app/autocomplete.rs` | 0 | ~220 |
| `app/selectors.rs` | 0 | ~230 |
| `app/persistence.rs` | 0 | ~35 |
| `app/mod.rs` | 0 | ~20 |

---

## Part B: Split `frontends/dark_tui/src/app.rs` (~1,700 lines)

### Current field groupings

| Group | Fields | Lines |
|-------|--------|-------|
| Core/Config | directory, chat_preferences_path, refresh_seconds | 117-119 |
| Theme/UI | theme, status_message, command_message, runtime_status, last_updated | 131-134,141 |
| Dashboard | focus, results_view_mode, filter_variants_to_product, products/variants/actors, selected_* | 120-128 |
| Viz | viz_selection, viz_offset_x/y, drag_anchor | 130,136-138 |
| Spawn | spawn_form | 142 |
| Chat Core | chat_visible, chat_actor_id, chat_messages, chat_needs_refresh | 143-145,164 |
| Chat Composer | chat_draft, chat_composing | 146-147 |
| Chat Options | chat_model/agent_options, chat_selected/preferred_model/agent | 148-153 |
| Chat Pickers | chat_picker_*, | 154-156 |
| Chat Autocomplete | chat_autocomplete_* | 157-161 |
| Chat Workspace | chat_workspace_file_cache* | 162-163 |
| Flight | snapshot/chat_*_refresh_in_flight, chat_send_in_flight, action_requests_in_flight | 165-168 |

### Target structure

```
frontends/dark_tui/src/
├── app/
│   ├── mod.rs              # App struct (thin), re-exports
│   ├── state.rs            # Core config, theme, UI globals, dashboard data, selections, flight
│   ├── viz.rs              # VizState: viz_selection, offset, drag, catalog_nodes, pan/scroll
│   ├── spawn.rs            # SpawnFormState + SpawnRequest (already partially extracted)
│   ├── chat.rs             # ChatState: visibility, actor_id, messages, composer, options
│   ├── chat_picker.rs      # ChatPickerState: picker open/query/selected/kind
│   ├── chat_autocomplete.rs # ChatAutocompleteState: open/mode/query/selected/items/workspace
│   └── persistence.rs      # Chat selection persistence
├── service.rs              # (unchanged)
├── ui/                     # (unchanged)
...
```

### Step 1: Create `app/viz.rs`

```rust
pub struct VizState {
    pub(crate) selection: Option<VizSelection>,
    pub(crate) offset_x: i32,
    pub(crate) offset_y: i32,
    pub(crate) drag_anchor: Option<DragAnchor>,
}

pub struct VizSelection { pub entity: String, pub kind: VizEntityKind }
pub struct DragAnchor { pub start_col: u16, pub start_row: u16, pub start_ox: i32, pub start_oy: i32 }

impl VizState {
    // Move methods: viz_select_next/prev, set_viz_selection (partial),
    // start/end_drag, apply_drag, viz_scroll, reset_offset
    // Lines ~1102-1206
}
```

**Coupling note**: `set_viz_selection` also updates `selected_product/variant/actor` and `chat_actor_id`. This orchestration stays in `App` but calls `viz.set_selection(...)` for the viz-specific part.

### Step 2: Create `app/chat.rs`

```rust
pub struct ChatState {
    pub(crate) visible: bool,
    pub(crate) actor_id: Option<String>,
    pub(crate) messages: Vec<ActorChatMessageRow>,
    pub(crate) draft: String,
    pub(crate) composing: bool,
    pub(crate) needs_refresh: bool,
    pub(crate) model_options: Vec<String>,
    pub(crate) agent_options: Vec<String>,
    pub(crate) selected_model: Option<String>,
    pub(crate) selected_agent: Option<String>,
    pub(crate) preferred_model: Option<String>,
    pub(crate) preferred_agent: Option<String>,
}

impl ChatState {
    // Move: toggle_visibility, open/cancel_composer, insert/backspace,
    // commit_sent, current_prompt, set_options, apply_messages
    // Lines ~396-552, 889-897
}
```

### Step 3: Create `app/chat_picker.rs`

```rust
pub struct ChatPickerState {
    pub(crate) open: Option<ChatPickerKind>,
    pub(crate) query: String,
    pub(crate) selected: usize,
}

pub enum ChatPickerKind { Model, Agent }

impl ChatPickerState {
    // Move: open/close/move_up/move_down/set_selected/apply/filtered_items
    // Lines ~568-714
}
```

### Step 4: Create `app/chat_autocomplete.rs`

Similar to Phase 3A, but simpler (uses `Vec<String>` instead of `ComposerAutocompleteItem`):

```rust
pub struct ChatAutocompleteState {
    pub(crate) open: bool,
    pub(crate) mode: Option<char>,
    pub(crate) query: String,
    pub(crate) selected: usize,
    pub(crate) items: Vec<String>,
    pub(crate) workspace_file_cache: Vec<String>,
    pub(crate) workspace_file_cache_loaded: bool,
}

impl ChatAutocompleteState {
    // Move: close, move_up/down, set_selected, apply_selection, refresh, ensure_cache
    // Lines ~725-816
    // Move free fns: slash_suggestions, file_suggestions, current_chat_trigger, chat_token_start
    // Lines ~1625-1700
}
```

### Step 5: Refactor App struct in `app/state.rs`

Embed sub-states:
```rust
pub struct App {
    // Core/Config + Theme/UI + Dashboard + Selections + Flight (stay here)
    // ...

    // Sub-states
    pub(crate) viz: VizState,
    pub(crate) spawn: Option<SpawnFormState>,
    pub(crate) chat: ChatState,
    pub(crate) chat_picker: ChatPickerState,
    pub(crate) chat_autocomplete: ChatAutocompleteState,
}
```

Delegate with the same public-API-preserving pattern as Part A.

### Expected line reduction

| File | Before | After |
|------|--------|-------|
| `app/state.rs` (was `app.rs`) | ~1,700 | ~600 |
| `app/viz.rs` | 0 | ~200 |
| `app/chat.rs` | 0 | ~200 |
| `app/chat_picker.rs` | 0 | ~160 |
| `app/chat_autocomplete.rs` | 0 | ~200 |
| `app/spawn.rs` | 0 | ~100 |
| `app/persistence.rs` | 0 | ~40 |
| `app/mod.rs` | 0 | ~30 |

---

## Verification

```bash
cargo check -p dark_chat
cargo check -p dark_tui
cargo test -p dark_chat    # (if any tests)
cargo test -p dark_tui     # (if any tests)
```

The key constraint: **public API must not change**. All existing call sites in `input.rs`, `tui/mod.rs`, panels, and views should continue to work via delegation methods on `App`.

## Migration Strategy

For each sub-module:
1. Create the new file with the sub-state struct
2. Move fields from App to sub-state struct
3. Move methods from App impl to sub-state impl
4. Add sub-state field to App
5. Add delegation methods on App
6. `cargo check` after each sub-module

Do NOT try to move all sub-modules at once. One at a time, verify compilation.
