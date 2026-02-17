use anyhow::{Context, Result, bail};
use async_trait::async_trait;
use reqwest::Method;
use serde_json::{Value, json};
use tokio::sync::mpsc::{UnboundedReceiver, unbounded_channel};

use dark_tui_components::compact_timestamp;

use crate::core::{
    ChatMessage, ChatRealtimeEvent, ChatSession, ProviderHealth, ProviderRuntimeStatus,
};
use crate::framework::extract_message_text;
use crate::providers::provider::ChatProvider;

use super::opencode_extract::{
    extract_config_path, extract_mcp_status, extract_session_statuses, extract_status_list,
    extract_string_options, format_unix_timestamp, normalize_unix_timestamp, unwrap_data,
};
use super::opencode_realtime::stream_realtime_events;
use super::opencode_wire::{MessageWire, SessionWire};

#[derive(Debug, Clone)]
pub struct OpenCodeProvider {
    pub(crate) base_url: String,
    pub(crate) http: reqwest::Client,
    pub(crate) basic_auth: Option<(String, String)>,
}

impl OpenCodeProvider {
    pub fn new(base_url: String) -> Self {
        let password = std::env::var("OPENCODE_SERVER_PASSWORD").ok();
        let username = std::env::var("OPENCODE_SERVER_USERNAME")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "opencode".to_string());

        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            http: reqwest::Client::new(),
            basic_auth: password.map(|secret| (username, secret)),
        }
    }
}

#[async_trait]
impl ChatProvider for OpenCodeProvider {
    fn provider_name(&self) -> &'static str {
        "opencode/server"
    }

    fn supports_realtime(&self) -> bool {
        true
    }

    fn start_realtime_stream(
        &self,
        directory: String,
    ) -> Option<UnboundedReceiver<ChatRealtimeEvent>> {
        let (sender, receiver) = unbounded_channel();
        let client = self.http.clone();
        let base_url = self.base_url.clone();
        let basic_auth = self.basic_auth.clone();

        tokio::spawn(async move {
            if let Err(error) =
                stream_realtime_events(client, base_url, basic_auth, directory, sender.clone())
                    .await
            {
                let _ = sender.send(ChatRealtimeEvent {
                    event_type: format!("stream.error:{error}"),
                    session_id: None,
                });
            }
        });

        Some(receiver)
    }

    async fn health(&self) -> Result<ProviderHealth> {
        let body = self
            .request_json_with_fallback(Method::GET, &["/", "/health"], &[], None)
            .await?;

        let data = unwrap_data(body);
        let healthy = data.get("healthy").and_then(Value::as_bool).unwrap_or(true);
        let version = data
            .get("version")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        Ok(ProviderHealth { healthy, version })
    }

    async fn list_sessions(&self, directory: &str) -> Result<Vec<ChatSession>> {
        let query = vec![("directory".to_string(), directory.to_string())];

        let sessions_value = self
            .request_json_with_fallback(Method::GET, &["/session", "/session/"], &query, None)
            .await?;
        let sessions_data = unwrap_data(sessions_value);

        let mut sessions: Vec<SessionWire> = serde_json::from_value(sessions_data)
            .context("OpenCode // Session // failed to decode session list")?;

        sessions.sort_by(|left, right| right.id.cmp(&left.id));

        let statuses = self
            .request_json_with_fallback(
                Method::GET,
                &["/session/status", "/session/status/"],
                &query,
                None,
            )
            .await
            .ok()
            .map(unwrap_data)
            .map(|value| extract_session_statuses(&value))
            .unwrap_or_default();

        let mapped = sessions
            .into_iter()
            .map(|session| {
                let status = statuses
                    .get(&session.id)
                    .cloned()
                    .unwrap_or_else(|| "idle".to_string());

                ChatSession {
                    id: session.id,
                    title: session
                        .title
                        .filter(|value| !value.trim().is_empty())
                        .unwrap_or_else(|| "Untitled session".to_string()),
                    parent_id: session.parent_id,
                    status,
                    updated_at: session
                        .updated_at
                        .as_deref()
                        .map(compact_timestamp)
                        .or_else(|| session.time.updated.and_then(format_unix_timestamp)),
                    updated_unix: session.time.updated.map(normalize_unix_timestamp),
                }
            })
            .collect();

        Ok(mapped)
    }

    async fn create_session(&self, directory: &str, title: Option<&str>) -> Result<ChatSession> {
        let query = vec![("directory".to_string(), directory.to_string())];
        let body = json!({
            "title": title
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .unwrap_or("Dark Chat session"),
        });

        let created = self
            .request_json_with_fallback(
                Method::POST,
                &["/session", "/session/"],
                &query,
                Some(body),
            )
            .await?;
        let data = unwrap_data(created);
        let record: SessionWire = serde_json::from_value(data)
            .context("OpenCode // Session // failed to decode created session")?;

        Ok(ChatSession {
            id: record.id,
            title: record
                .title
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "Untitled session".to_string()),
            parent_id: record.parent_id,
            status: "idle".to_string(),
            updated_at: record
                .updated_at
                .as_deref()
                .map(compact_timestamp)
                .or_else(|| record.time.updated.and_then(format_unix_timestamp)),
            updated_unix: record.time.updated.map(normalize_unix_timestamp),
        })
    }

    async fn list_messages(
        &self,
        directory: &str,
        session_id: &str,
        limit: Option<u32>,
    ) -> Result<Vec<ChatMessage>> {
        let mut query = vec![("directory".to_string(), directory.to_string())];
        if let Some(limit) = limit {
            query.push(("limit".to_string(), limit.to_string()));
        }

        let path_message = format!("/session/{session_id}/message");
        let path_messages = format!("/session/{session_id}/messages");
        let payload = self
            .request_json_with_fallback(
                Method::GET,
                &[path_message.as_str(), path_messages.as_str()],
                &query,
                None,
            )
            .await?;

        let data = unwrap_data(payload);
        let records: Vec<MessageWire> = serde_json::from_value(data)
            .context("OpenCode // Session // failed to decode messages")?;

        let mut mapped = records
            .into_iter()
            .map(|record| ChatMessage {
                id: record.info.id,
                role: record.info.role,
                text: extract_message_text(&record.parts),
                created_at: record
                    .info
                    .time
                    .created
                    .and_then(format_unix_timestamp)
                    .or_else(|| record.info.created_at.as_deref().map(compact_timestamp)),
            })
            .collect::<Vec<_>>();

        mapped.sort_by(|left, right| left.created_at.cmp(&right.created_at));
        Ok(mapped)
    }

    async fn list_agents(&self, directory: &str) -> Result<Vec<String>> {
        let query = vec![("directory".to_string(), directory.to_string())];
        let payload = self
            .request_json_with_fallback(Method::GET, &["/agent", "/agent/"], &query, None)
            .await?;
        let data = unwrap_data(payload);

        let mut options =
            extract_string_options(&data, &["id", "name", "key", "slug", "identifier", "value"]);

        options.sort();
        options.dedup();
        Ok(options)
    }

    async fn list_models(&self, directory: &str) -> Result<Vec<String>> {
        let query = vec![("directory".to_string(), directory.to_string())];
        let payload = self
            .request_json_with_fallback(
                Method::GET,
                &["/config/providers", "/config/providers/"],
                &query,
                None,
            )
            .await?;
        let data = unwrap_data(payload);

        let mut models = Vec::new();

        if let Some(defaults) = data
            .get("default")
            .or_else(|| data.get("defaults"))
            .and_then(Value::as_object)
        {
            for value in defaults.values() {
                if let Some(model) = value.as_str() {
                    let trimmed = model.trim();
                    if !trimmed.is_empty() {
                        models.push(trimmed.to_string());
                    }
                }
            }
        }

        let extract_model_label = |model: &Value, fallback: Option<&str>| -> String {
            let from_value = model
                .as_str()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToString::to_string)
                .or_else(|| {
                    model
                        .get("id")
                        .or_else(|| model.get("model"))
                        .or_else(|| model.get("name"))
                        .and_then(Value::as_str)
                        .map(str::trim)
                        .filter(|value| !value.is_empty())
                        .map(ToString::to_string)
                });

            from_value
                .or_else(|| {
                    fallback
                        .map(str::trim)
                        .filter(|value| !value.is_empty())
                        .map(ToString::to_string)
                })
                .unwrap_or_default()
        };

        if let Some(providers) = data.get("providers").and_then(Value::as_array) {
            for provider in providers {
                let provider_id = provider
                    .get("id")
                    .or_else(|| provider.get("key"))
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .trim()
                    .to_string();

                if let Some(model_entries) = provider.get("models") {
                    let mut push_model = |model_label: String| {
                        if model_label.is_empty() {
                            return;
                        }

                        if model_label.contains('/') || provider_id.is_empty() {
                            models.push(model_label);
                        } else {
                            models.push(format!("{provider_id}/{model_label}"));
                        }
                    };

                    if let Some(entries) = model_entries.as_array() {
                        for model in entries {
                            push_model(extract_model_label(model, None));
                        }
                    } else if let Some(entries) = model_entries.as_object() {
                        for (model_key, model) in entries {
                            push_model(extract_model_label(model, Some(model_key)));
                        }
                    }
                }
            }
        }

        models.sort();
        models.dedup();
        Ok(models)
    }

    async fn fetch_runtime_status(&self, directory: &str) -> Result<ProviderRuntimeStatus> {
        let query = vec![("directory".to_string(), directory.to_string())];

        let lsp = self
            .request_json_with_fallback(Method::GET, &["/lsp", "/lsp/"], &query, None)
            .await
            .map(unwrap_data)
            .map(|value| extract_status_list(&value, "lsp"))
            .unwrap_or_default();

        let formatter = self
            .request_json_with_fallback(Method::GET, &["/formatter", "/formatter/"], &query, None)
            .await
            .map(unwrap_data)
            .map(|value| extract_status_list(&value, "formatter"))
            .unwrap_or_default();

        let mcp = self
            .request_json_with_fallback(Method::GET, &["/mcp", "/mcp/"], &query, None)
            .await
            .map(unwrap_data)
            .map(|value| extract_mcp_status(&value))
            .unwrap_or_default();

        let config_path = self
            .request_json_with_fallback(Method::GET, &["/config", "/config/"], &query, None)
            .await
            .ok()
            .map(unwrap_data)
            .and_then(|value| extract_config_path(&value));

        Ok(ProviderRuntimeStatus {
            mcp,
            lsp,
            formatter,
            config_path,
        })
    }

    async fn send_prompt(
        &self,
        directory: &str,
        session_id: &str,
        prompt: &str,
        model: Option<&str>,
        agent: Option<&str>,
    ) -> Result<()> {
        self.send_prompt_with_options(directory, session_id, prompt, model, agent, false)
            .await
    }

    async fn run_command(&self, directory: &str, session_id: &str, command: &str) -> Result<()> {
        let trimmed = command.trim();
        if trimmed.is_empty() {
            bail!("OpenCode // Session // command cannot be empty");
        }

        let query = vec![("directory".to_string(), directory.to_string())];
        let body = json!({
            "command": trimmed,
        });

        let path = format!("/session/{session_id}/command");
        let _ = self
            .request_json_with_fallback(Method::POST, &[path.as_str()], &query, Some(body))
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::framework::extract_message_text;

    #[test]
    fn extract_message_text_formats_thinking_and_tool_calls() {
        let parts = vec![
            serde_json::json!({
                "type": "thinking",
                "text": "Inspecting repository state"
            }),
            serde_json::json!({
                "type": "tool_call",
                "tool": "bash",
                "input": { "command": "git status" },
                "output": "clean"
            }),
            serde_json::json!({
                "type": "text",
                "text": "All set."
            }),
        ];

        let rendered = extract_message_text(&parts);

        assert!(rendered.contains("### Thinking"));
        assert!(rendered.contains("Inspecting repository state"));
        assert!(rendered.contains("### Tool Call (bash)"));
        assert!(rendered.contains("Input:"));
        assert!(rendered.contains("\"command\": \"git status\""));
        assert!(rendered.contains("Output:"));
        assert!(rendered.contains("All set."));
    }

    #[test]
    fn extract_message_text_falls_back_when_no_content_exists() {
        let parts = vec![serde_json::json!({
            "type": "tool_call",
            "tool": "bash"
        })];

        let rendered = extract_message_text(&parts);
        assert_eq!(rendered, "### Tool Call (bash)");
    }
}
