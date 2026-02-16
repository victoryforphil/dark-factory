use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::core::systems::default_session_title;
use crate::core::types::{
    ChatMessage, ChatRealtimeEvent, ChatSession, ProviderHealth, ProviderRuntimeStatus,
};
use crate::providers::ChatProvider;

#[derive(Debug, Clone)]
pub struct ChatSnapshot {
    pub health: ProviderHealth,
    pub sessions: Vec<ChatSession>,
    pub active_session_id: Option<String>,
    pub messages: Vec<ChatMessage>,
    pub agents: Vec<String>,
    pub models: Vec<String>,
    pub runtime_status: ProviderRuntimeStatus,
}

#[derive(Clone)]
pub struct ChatBackend {
    provider: Arc<dyn ChatProvider>,
    directory: String,
}

impl ChatBackend {
    pub fn new(provider: Arc<dyn ChatProvider>, directory: String) -> Self {
        Self {
            provider,
            directory,
        }
    }

    pub fn provider_name(&self) -> &'static str {
        self.provider.provider_name()
    }

    pub fn supports_realtime(&self) -> bool {
        self.provider.supports_realtime()
    }

    pub fn start_realtime_stream(&self) -> Option<UnboundedReceiver<ChatRealtimeEvent>> {
        self.provider.start_realtime_stream(self.directory.clone())
    }

    pub fn directory(&self) -> &str {
        &self.directory
    }

    pub async fn bootstrap(
        &self,
        preferred_session_id: Option<&str>,
        preferred_title: Option<&str>,
    ) -> Result<ChatSnapshot> {
        let mut sessions = self.provider.list_sessions(&self.directory).await?;
        if sessions.is_empty() {
            let title = preferred_title
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToString::to_string)
                .unwrap_or_else(|| default_session_title(&self.directory));

            let created = self
                .provider
                .create_session(&self.directory, Some(&title))
                .await?;
            sessions.push(created);
        }

        let health = self.provider.health().await?;
        let active_session_id = pick_active_session_id(&sessions, preferred_session_id);
        let messages = match active_session_id.as_deref() {
            Some(session_id) => {
                self.provider
                    .list_messages(&self.directory, session_id, Some(120))
                    .await?
            }
            None => Vec::new(),
        };
        let agents = self
            .provider
            .list_agents(&self.directory)
            .await
            .unwrap_or_default();
        let models = self
            .provider
            .list_models(&self.directory)
            .await
            .unwrap_or_default();
        let runtime_status = self
            .provider
            .fetch_runtime_status(&self.directory)
            .await
            .unwrap_or_default();

        Ok(ChatSnapshot {
            health,
            sessions,
            active_session_id,
            messages,
            agents,
            models,
            runtime_status,
        })
    }

    pub async fn refresh(&self, active_session_id: Option<&str>) -> Result<ChatSnapshot> {
        let health = self.provider.health().await?;
        let sessions = self.provider.list_sessions(&self.directory).await?;
        let selected = pick_active_session_id(&sessions, active_session_id);
        let messages = match selected.as_deref() {
            Some(session_id) => {
                self.provider
                    .list_messages(&self.directory, session_id, Some(120))
                    .await?
            }
            None => Vec::new(),
        };
        let agents = self
            .provider
            .list_agents(&self.directory)
            .await
            .unwrap_or_default();
        let models = self
            .provider
            .list_models(&self.directory)
            .await
            .unwrap_or_default();
        let runtime_status = self
            .provider
            .fetch_runtime_status(&self.directory)
            .await
            .unwrap_or_default();

        Ok(ChatSnapshot {
            health,
            sessions,
            active_session_id: selected,
            messages,
            agents,
            models,
            runtime_status,
        })
    }

    pub async fn create_session(&self, title: Option<&str>) -> Result<ChatSession> {
        let final_title = title
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToString::to_string)
            .unwrap_or_else(|| default_session_title(&self.directory));

        self.provider
            .create_session(&self.directory, Some(&final_title))
            .await
    }

    pub async fn send_prompt(
        &self,
        session_id: &str,
        prompt: &str,
        model: Option<&str>,
        agent: Option<&str>,
    ) -> Result<()> {
        self.provider
            .send_prompt(&self.directory, session_id, prompt, model, agent)
            .await
    }

    pub async fn run_command(&self, session_id: &str, command: &str) -> Result<()> {
        self.provider
            .run_command(&self.directory, session_id, command)
            .await
    }
}

fn pick_active_session_id(sessions: &[ChatSession], preferred: Option<&str>) -> Option<String> {
    if sessions.is_empty() {
        return None;
    }

    if let Some(preferred) = preferred {
        if sessions.iter().any(|session| session.id == preferred) {
            return Some(preferred.to_string());
        }
    }

    sessions.first().map(|session| session.id.clone())
}
