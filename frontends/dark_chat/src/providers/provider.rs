use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::core::{
    ChatMessage, ChatRealtimeEvent, ChatSession, ProviderHealth, ProviderRuntimeStatus,
};

#[async_trait]
pub trait ChatProvider: Send + Sync {
    fn provider_name(&self) -> &'static str;

    fn supports_realtime(&self) -> bool {
        false
    }

    fn start_realtime_stream(
        &self,
        _directory: String,
    ) -> Option<UnboundedReceiver<ChatRealtimeEvent>> {
        None
    }

    async fn health(&self) -> Result<ProviderHealth>;

    async fn list_sessions(&self, directory: &str) -> Result<Vec<ChatSession>>;

    async fn create_session(&self, directory: &str, title: Option<&str>) -> Result<ChatSession>;

    async fn list_messages(
        &self,
        directory: &str,
        session_id: &str,
        limit: Option<u32>,
    ) -> Result<Vec<ChatMessage>>;

    async fn list_agents(&self, _directory: &str) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    async fn list_models(&self, _directory: &str) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    async fn fetch_runtime_status(&self, _directory: &str) -> Result<ProviderRuntimeStatus> {
        Ok(ProviderRuntimeStatus::default())
    }

    async fn send_prompt(
        &self,
        directory: &str,
        session_id: &str,
        prompt: &str,
        model: Option<&str>,
        agent: Option<&str>,
    ) -> Result<()>;

    async fn run_command(&self, _directory: &str, _session_id: &str, _command: &str) -> Result<()> {
        Ok(())
    }
}
