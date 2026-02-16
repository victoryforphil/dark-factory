use dark_tui_components::{ChatMessageEntry, ChatMessageRole};

use crate::core::ChatMessage;

pub fn to_component_messages(messages: &[ChatMessage]) -> Vec<ChatMessageEntry> {
    messages
        .iter()
        .map(|message| {
            ChatMessageEntry::new(
                ChatMessageRole::from_role(&message.role),
                message.text.clone(),
                message.created_at.clone(),
            )
        })
        .collect()
}
