use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpencodeSessionCreateInput {
    pub directory: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct OpencodeSessionStateQuery {
    pub directory: String,
    pub include_messages: bool,
}

#[derive(Debug, Clone, Default)]
pub struct OpencodeAttachQuery {
    pub directory: String,
    pub model: Option<String>,
    pub agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpencodeSessionCommandInput {
    pub directory: String,
    pub command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpencodeSessionPromptInput {
    pub directory: String,
    pub prompt: String,
    #[serde(rename = "noReply", skip_serializing_if = "Option::is_none")]
    pub no_reply: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpencodeSessionDirectoryInput {
    pub directory: String,
}
