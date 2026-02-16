#[derive(Debug, Clone, Default)]
pub struct ProviderHealth {
    pub healthy: bool,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ChatSession {
    pub id: String,
    pub title: String,
    pub parent_id: Option<String>,
    pub status: String,
    pub updated_at: Option<String>,
    pub updated_unix: Option<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct ChatMessage {
    pub id: String,
    pub role: String,
    pub text: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ChatRealtimeEvent {
    pub event_type: String,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ProviderRuntimeStatus {
    pub mcp: Vec<String>,
    pub lsp: Vec<String>,
    pub formatter: Vec<String>,
    pub config_path: Option<String>,
}
