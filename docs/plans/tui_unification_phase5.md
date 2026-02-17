# Phase 5: Mature dark_chat as the Chat Framework Lib

**Risk**: Medium | **Impact**: High | **Dependencies**: Phase 1, 2, 3

## Goal

Restructure `dark_chat` so it serves as both:
1. A **standalone binary** (the OpenCode chat TUI)
2. A **framework library** that `dark_tui` imports for chat functionality

Currently `dark_tui` depends on `dark_chat` but only uses its provider/backend layer plus a thin `framework::conversation_panel`. This phase promotes `dark_chat` into a full chat framework with:
- Rich agent message types (tool calls, thinking blocks, markdown)
- Complete conversation panel with composer, autocomplete, model/agent pickers
- Session management components
- Provider abstraction

---

## Current State

### What dark_tui currently imports from dark_chat

From `frontends/dark_tui/src/service.rs`:
- `dark_chat::providers::opencode_server::OpenCodeServerProvider` — to send messages and commands to actors
- `dark_chat::core::types::ChatMessage` — for message types
- `dark_chat::providers::provider::ChatProvider` — trait

From `frontends/dark_tui/src/ui/render/panels/chat_panel.rs`:
- `dark_chat::framework::conversation_panel::render_conversation_panel` — the reusable chat render fn
- `dark_chat::tui::app::App` — imported but only used to convert messages

### What dark_chat currently exports (lib.rs)

```rust
pub mod cli;
pub mod core;
pub mod framework;
pub mod providers;
pub mod tui;
```

Everything is public, but the useful framework API is minimal.

---

## Step 1: Restructure dark_chat module hierarchy

### Target structure

```
frontends/dark_chat/src/
├── lib.rs                    # Public API: framework, providers, core
├── main.rs                   # Binary entry (thin)
├── cli.rs                    # CLI args (binary-only)
│
├── core/                     # Shared types and backend
│   ├── mod.rs
│   ├── types.rs              # ChatSession, ChatMessage, etc.
│   ├── backend.rs            # ChatBackend orchestration
│   └── systems.rs            # Utility functions
│
├── providers/                # Provider implementations
│   ├── mod.rs
│   ├── provider.rs           # ChatProvider trait
│   └── opencode_server.rs    # OpenCode server implementation
│
├── framework/                # Reusable chat UI framework (THE KEY EXPORT)
│   ├── mod.rs                # Public API surface
│   ├── conversation_panel.rs # Full conversation panel component
│   ├── message_types.rs      # AgentMessage, ToolCall, ThinkingBlock, etc.
│   ├── message_renderer.rs   # Message formatting/extraction (from opencode_server)
│   ├── autocomplete.rs       # Chat autocomplete state + rendering
│   ├── model_selector.rs     # Model/agent selector state + rendering
│   ├── session_tree.rs       # Session tree walker + renderer
│   └── composer.rs           # Composer state (wraps tui-textarea)
│
├── tui/                      # Binary-specific TUI app
│   ├── mod.rs                # Main loop
│   ├── app/                  # App state (split per Phase 3A)
│   │   ├── mod.rs
│   │   ├── state.rs
│   │   ├── composer.rs       # Uses framework::composer
│   │   ├── autocomplete.rs   # Uses framework::autocomplete
│   │   ├── selectors.rs      # Uses framework::model_selector
│   │   └── persistence.rs
│   ├── input.rs              # Key handling
│   ├── realtime.rs           # SSE event filtering
│   ├── commands.rs           # Slash command parsing
│   ├── views/
│   │   └── main_view.rs
│   └── panels/
│       ├── chat_panel.rs     # Uses framework::conversation_panel
│       ├── header_panel.rs
│       ├── footer_panel.rs
│       ├── key_bar_panel.rs
│       ├── sessions_panel.rs # Uses framework::session_tree
│       └── status_panel.rs
```

---

## Step 2: Create `framework/message_types.rs`

### Why

`opencode_server.rs` (1,386 lines) contains ~400 lines of message parsing/extraction that converts wire format into displayable text. This logic should be in the framework so `dark_tui` can also use it for rich message rendering.

### What to extract

From `frontends/dark_chat/src/providers/opencode_server.rs`:

```rust
/// Rich message representation for agent chat UIs.
pub struct AgentMessage {
    pub role: AgentMessageRole,
    pub parts: Vec<AgentMessagePart>,
    pub created_at: Option<String>,
    pub model: Option<String>,
    pub status: Option<String>,
}

pub enum AgentMessageRole {
    User,
    Assistant,
    System,
    Tool,
}

pub enum AgentMessagePart {
    Text(String),
    ThinkingBlock { content: String, collapsed: bool },
    ToolCall { name: String, args: Option<String>, result: Option<String> },
    CodeBlock { language: Option<String>, code: String },
    Error(String),
}
```

### What to extract from opencode_server

The following functions move to `framework/message_renderer.rs`:
- `extract_message_text(parts: &[serde_json::Value]) -> String` (currently ~lines 920-1036)
- Tool/thinking block rendering helpers
- `compact_timestamp` (already moved to components in Phase 1)

---

## Step 3: Create `framework/autocomplete.rs`

### Why

Autocomplete logic is duplicated between dark_chat (`app.rs` lines 668-723, 954-1015) and dark_tui (`app.rs` lines 725-816). The framework should own the canonical implementation.

### What to create

```rust
/// Reusable chat autocomplete state.
/// Both dark_chat and dark_tui embed this in their app state.
pub struct ChatAutocomplete {
    pub open: bool,
    pub mode: Option<AutocompleteMode>,
    pub query: String,
    pub selected: usize,
    pub token_start: usize,
    pub items: Vec<AutocompleteItem>,
    workspace_file_cache: Vec<String>,
    workspace_file_cache_loaded: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutocompleteMode {
    Slash,
    File,
}

#[derive(Debug, Clone)]
pub struct AutocompleteItem {
    pub label: String,
    pub insert: String,
    pub tag: String,
}

impl ChatAutocomplete {
    pub fn new() -> Self { ... }
    pub fn close(&mut self) { ... }
    pub fn move_up(&mut self) { ... }
    pub fn move_down(&mut self) { ... }
    pub fn set_selected(&mut self, index: usize) { ... }
    pub fn apply_selection(&mut self, draft: &mut String, cursor: &mut usize) -> Option<String> { ... }
    pub fn refresh(&mut self, draft: &str, cursor: usize, composing: bool, extra_slash_commands: &[(&str, &str)]) { ... }
    pub fn ensure_workspace_cache(&mut self, directory: &str) { ... }
    pub fn anchor_position(&self, draft: &str, cursor: usize) -> Option<(usize, usize)> { ... }

    // Getters
    pub fn is_open(&self) -> bool { ... }
    pub fn mode(&self) -> Option<AutocompleteMode> { ... }
    pub fn query(&self) -> &str { ... }
    pub fn selected(&self) -> usize { ... }
    pub fn items(&self) -> &[AutocompleteItem] { ... }
}

// Built-in slash commands
pub const DEFAULT_SLASH_COMMANDS: &[(&str, &str)] = &[
    ("help", "toggle help"),
    ("refresh", "refresh snapshot"),
    ("new", "create session"),
    ("clear", "clear messages"),
    ("sessions", "session summary"),
    ("agent", "set agent"),
    ("model", "set model"),
    ("grep", "search workspace"),
];
```

### Migration

1. dark_chat `app/autocomplete.rs` wraps `ChatAutocomplete` (thin delegation)
2. dark_tui `app/chat_autocomplete.rs` wraps `ChatAutocomplete`
3. Both frontends remove their local implementations

---

## Step 4: Create `framework/model_selector.rs`

### Why

Model/agent selector logic is in dark_chat's selectors (lines 440-661) and dark_tui's pickers (lines 568-714). Similar structure, different naming.

### What to create

```rust
/// Reusable model/agent picker state.
pub struct ItemSelector {
    pub open: bool,
    pub query: String,
    pub selected: usize,
    pub raw_mode: bool,       // Optional (dark_chat uses this)
    pub raw_input: String,    // Optional
    pub anchor_col: Option<u16>,
}

pub enum SelectorKind {
    Model,
    Agent,
}

impl ItemSelector {
    pub fn new() -> Self { ... }
    pub fn open(&mut self) { ... }
    pub fn close(&mut self) { ... }
    pub fn insert_query_char(&mut self, ch: char) { ... }
    pub fn backspace_query(&mut self) { ... }
    pub fn clear_query(&mut self) { ... }
    pub fn move_up(&mut self) { ... }
    pub fn move_down(&mut self) { ... }
    pub fn set_selected(&mut self, index: usize) { ... }
    pub fn filtered_items<'a>(&self, items: &'a [String]) -> Vec<&'a str> { ... }
    pub fn confirm(&self, items: &[String]) -> Option<String> { ... }
    pub fn toggle_raw_mode(&mut self) { ... }
}
```

### Migration

1. dark_chat replaces `ModelSelectorState` and `AgentSelectorState` with `ItemSelector`
2. dark_tui replaces `ChatPickerState` with two `ItemSelector` instances

---

## Step 5: Create `framework/session_tree.rs`

### Why

dark_chat's `sessions_panel.rs` (372 lines) has a tree walker that builds parent/child session relationships. This is reusable for any hierarchical session display.

### What to create

```rust
/// Session tree node for hierarchical display.
pub struct SessionTreeRow {
    pub session_id: String,
    pub title: String,
    pub status: String,
    pub is_active: bool,
    pub depth: usize,
    pub is_last: bool,
    pub ancestors_are_last: Vec<bool>,
    pub child_count: usize,
    pub created_at: Option<String>,
}

/// Walk a flat session list and produce tree rows.
pub fn walk_session_tree(
    sessions: &[impl SessionLike],
    active_session_id: Option<&str>,
) -> Vec<SessionTreeRow> {
    // Build parent_id -> children map
    // DFS from roots (no parent_id)
    // Compute depth, is_last, ancestors_are_last
    todo!()
}

/// Trait for session-like types.
pub trait SessionLike {
    fn id(&self) -> &str;
    fn parent_id(&self) -> Option<&str>;
    fn title(&self) -> &str;
    fn status(&self) -> &str;
    fn created_at(&self) -> Option<&str>;
}

/// Render a tree prefix ("│  ├─ " etc.)
pub fn tree_prefix(depth: usize, is_last: bool, ancestors_are_last: &[bool]) -> String {
    // Build the "│  │  ├─ " style prefix
    todo!()
}
```

### Migration

dark_chat's `sessions_panel.rs` replaces its local tree walker with the shared version.

---

## Step 6: Enhance `framework/conversation_panel.rs`

### Why

The existing `render_conversation_panel` (226 lines) is a good start but doesn't include popups, autocomplete, or hit-testing. After Phase 4's `PopupOverlay` component, this panel can compose popups for model/agent selection.

### What to enhance

```rust
/// Full chat conversation panel with integrated popups.
pub struct ConversationPanelProps<'a> {
    // Existing props
    pub title: &'a str,
    pub messages: Vec<ChatMessageEntry>,
    pub composer_text: &'a str,
    pub composing: bool,
    pub status: Option<(&'a str, ChatStatusTone)>,

    // New: autocomplete
    pub autocomplete: Option<&'a ChatAutocomplete>,

    // New: model/agent selectors
    pub model_selector: Option<&'a ItemSelector>,
    pub agent_selector: Option<&'a ItemSelector>,
    pub model_items: &'a [String],
    pub agent_items: &'a [String],

    // New: meta labels
    pub active_model: Option<&'a str>,
    pub active_agent: Option<&'a str>,
}

/// Render the full conversation panel including popups.
pub fn render_conversation_panel(
    frame: &mut Frame,
    area: Rect,
    props: &ConversationPanelProps,
    theme: &impl ComponentThemeLike,
) {
    // 1. Render header (ChatConversationHeaderComponent)
    // 2. Render messages (ChatMessageListComponent)
    // 3. Render composer (ChatComposerComponent) with meta labels
    // 4. Render popups (PopupOverlay) for selectors/autocomplete
}

/// Hit-test for the full conversation panel.
pub fn conversation_panel_hit_test(
    area: Rect,
    props: &ConversationPanelProps,
    col: u16,
    row: u16,
) -> ConversationHit {
    // Check popups first (top layer)
    // Then composer meta labels
    // Then message list
    // Then header
}

pub enum ConversationHit {
    Outside,
    ModelSelectorItem(usize),
    AgentSelectorItem(usize),
    AutocompleteItem(usize),
    ModelLabel,
    AgentLabel,
    Message(usize),
    Composer,
}
```

### Migration

1. dark_chat's `chat_panel.rs` (1,012 lines) shrinks dramatically — popup rendering, hit-testing, and area calculation move to the framework. The panel becomes a thin adapter that constructs `ConversationPanelProps` from `App` state.

2. dark_tui's `chat_panel.rs` (544 lines) replaces its entire popup/hit-test implementation with the shared version. It already calls `render_conversation_panel` — now it gets popups for free.

---

## Step 7: Update lib.rs exports

### File: `frontends/dark_chat/src/lib.rs`

```rust
pub mod core;
pub mod framework;
pub mod providers;
pub mod tui;
```

Ensure `framework` re-exports the key types:
```rust
// framework/mod.rs
pub mod autocomplete;
pub mod conversation_panel;
pub mod message_renderer;
pub mod message_types;
pub mod model_selector;
pub mod session_tree;
pub mod composer;

pub use autocomplete::{ChatAutocomplete, AutocompleteItem, AutocompleteMode, DEFAULT_SLASH_COMMANDS};
pub use conversation_panel::{render_conversation_panel, ConversationPanelProps, ConversationHit};
pub use message_types::{AgentMessage, AgentMessageRole, AgentMessagePart};
pub use model_selector::{ItemSelector, SelectorKind};
pub use session_tree::{walk_session_tree, SessionTreeRow, SessionLike, tree_prefix};
```

---

## Verification

```bash
cargo check -p dark_chat
cargo check -p dark_tui
cargo test -p dark_chat
```

## Estimated Impact

- `dark_chat/providers/opencode_server.rs`: 1,386 -> ~1,000 lines (message extraction moved)
- `dark_chat/tui/panels/chat_panel.rs`: 1,012 -> ~400 lines (popups/hits moved to framework)
- `dark_tui/ui/render/panels/chat_panel.rs`: 544 -> ~150 lines (thin adapter to framework)
- `dark_chat/framework/`: 226 -> ~800 lines (comprehensive chat framework)
- dark_tui no longer needs to duplicate any chat UI logic
- Autocomplete, selectors, and session tree become shared single-source implementations
