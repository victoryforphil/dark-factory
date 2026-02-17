#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentMessage {
    pub role: AgentMessageRole,
    pub parts: Vec<AgentMessagePart>,
    pub created_at: Option<String>,
    pub model: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentMessageRole {
    User,
    Assistant,
    System,
    Tool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentMessagePart {
    Text(String),
    ThinkingBlock {
        content: String,
        collapsed: bool,
    },
    ToolCall {
        name: String,
        args: Option<String>,
        result: Option<String>,
    },
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::{AgentMessage, AgentMessagePart, AgentMessageRole};

    #[test]
    fn agent_message_preserves_rich_parts() {
        let message = AgentMessage {
            role: AgentMessageRole::Assistant,
            parts: vec![
                AgentMessagePart::ThinkingBlock {
                    content: "Inspecting state".to_string(),
                    collapsed: false,
                },
                AgentMessagePart::ToolCall {
                    name: "bash".to_string(),
                    args: Some("git status".to_string()),
                    result: Some("clean".to_string()),
                },
            ],
            created_at: Some("unix:1".to_string()),
            model: Some("openai/gpt-5".to_string()),
            status: Some("ok".to_string()),
        };

        assert_eq!(message.role, AgentMessageRole::Assistant);
        assert_eq!(message.parts.len(), 2);
        assert_eq!(message.model.as_deref(), Some("openai/gpt-5"));
    }

    #[test]
    fn agent_message_parts_support_error_and_code_blocks() {
        let parts = vec![
            AgentMessagePart::CodeBlock {
                language: Some("rust".to_string()),
                code: "fn main() {}".to_string(),
            },
            AgentMessagePart::Error("failed to run".to_string()),
        ];

        assert!(matches!(
            parts[0],
            AgentMessagePart::CodeBlock {
                language: Some(_),
                ..
            }
        ));
        assert!(matches!(parts[1], AgentMessagePart::Error(_)));
    }
}
