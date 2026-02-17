mod autocomplete;
mod composer;
mod conversation_panel;
mod message_renderer;
mod message_types;
mod model_selector;
mod session_tree;

pub use autocomplete::{
    AutocompleteItem, AutocompleteMode, ChatAutocomplete, DEFAULT_SLASH_COMMANDS,
};
pub use composer::ComposerState;
pub use conversation_panel::{
    ConversationComposer, ConversationHeader, ConversationMessage, ConversationPalette,
    ConversationPanelProps, ConversationStatusTone, render_conversation_panel,
    status_tone_for_status,
};
pub use message_renderer::extract_message_text;
pub use message_types::{AgentMessage, AgentMessagePart, AgentMessageRole};
pub use model_selector::{ItemSelector, SelectorKind};
pub use session_tree::{SessionLike, SessionTreeRow, tree_prefix, walk_session_tree};
