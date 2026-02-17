/// Normalized role labels used by shared chat components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatMessageRole {
    User,
    Assistant,
    System,
    Tool,
    Other(String),
}

impl ChatMessageRole {
    /// Maps arbitrary role text into the normalized enum.
    pub fn from_role(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "user" => Self::User,
            "assistant" => Self::Assistant,
            "system" => Self::System,
            "tool" => Self::Tool,
            other => Self::Other(other.to_string()),
        }
    }

    /// Returns a lowercase display label for this role.
    pub fn label(&self) -> &str {
        match self {
            Self::User => "user",
            Self::Assistant => "assistant",
            Self::System => "system",
            Self::Tool => "tool",
            Self::Other(value) => value.as_str(),
        }
    }
}

/// Renderable chat message row consumed by message-list components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMessageEntry {
    pub role: ChatMessageRole,
    pub text: String,
    pub created_at: Option<String>,
}

impl ChatMessageEntry {
    /// Creates a chat message entry.
    pub fn new(role: ChatMessageRole, text: impl Into<String>, created_at: Option<String>) -> Self {
        Self {
            role,
            text: text.into(),
            created_at,
        }
    }
}
