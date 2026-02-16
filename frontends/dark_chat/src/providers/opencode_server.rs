use std::collections::HashMap;

use anyhow::{Context, Result, bail};
use async_trait::async_trait;
use futures_util::StreamExt;
use reqwest::Method;
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

use crate::core::{
    ChatMessage, ChatRealtimeEvent, ChatSession, ProviderHealth, ProviderRuntimeStatus,
};
use crate::providers::provider::ChatProvider;

#[derive(Debug, Clone)]
pub struct OpenCodeProvider {
    base_url: String,
    http: reqwest::Client,
    basic_auth: Option<(String, String)>,
}

#[derive(Debug)]
struct RawResponse {
    status: u16,
    path: String,
    body: Value,
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

    async fn raw_request(
        &self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
    ) -> Result<RawResponse> {
        let normalized_path = normalize_path(path);
        let mut url = format!("{}{}", self.base_url, normalized_path);
        append_query(&mut url, query);

        let mut request = self.http.request(method, url);

        if let Some((username, password)) = self.basic_auth.as_ref() {
            request = request.basic_auth(username, Some(password));
        }

        if let Some(body_value) = body {
            request = request.json(&body_value);
        }

        let response = request.send().await.with_context(|| {
            format!("OpenCode // HTTP // request failed (path={normalized_path})")
        })?;

        let status = response.status().as_u16();
        let response_text = response.text().await.with_context(|| {
            format!("OpenCode // HTTP // response read failed (path={normalized_path})")
        })?;

        let body = parse_response_body(response_text);
        Ok(RawResponse {
            status,
            path: normalized_path,
            body,
        })
    }

    async fn request_json_with_fallback(
        &self,
        method: Method,
        paths: &[&str],
        query: &[(String, String)],
        body: Option<Value>,
    ) -> Result<Value> {
        let mut first_non_404_error: Option<anyhow::Error> = None;

        for path in paths {
            let raw = self
                .raw_request(method.clone(), path, query, body.clone())
                .await?;

            if raw.status == 404 {
                continue;
            }

            match ensure_success(raw) {
                Ok(value) => return Ok(value),
                Err(error) => {
                    if first_non_404_error.is_none() {
                        first_non_404_error = Some(error);
                    }
                }
            }
        }

        if let Some(error) = first_non_404_error {
            return Err(error);
        }

        bail!("OpenCode // HTTP // all fallback paths returned 404 (paths={paths:?})")
    }

    pub async fn send_prompt_with_options(
        &self,
        directory: &str,
        session_id: &str,
        prompt: &str,
        model: Option<&str>,
        agent: Option<&str>,
        no_reply: bool,
    ) -> Result<()> {
        let trimmed = prompt.trim();
        if trimmed.is_empty() {
            bail!("OpenCode // Session // prompt cannot be empty");
        }

        let query = vec![("directory".to_string(), directory.to_string())];
        let mut body = json!({
            "noReply": no_reply,
            "parts": [{
                "type": "text",
                "text": trimmed,
            }],
        });

        if let Some(model) = model.and_then(parse_model_selector) {
            body["model"] = json!({
                "providerID": model.0,
                "modelID": model.1,
            });
        }

        if let Some(agent) = agent
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToString::to_string)
        {
            body["agent"] = Value::String(agent);
        }

        let path_message = format!("/session/{session_id}/message");
        let path_prompt = format!("/session/{session_id}/prompt");
        let _ = self
            .request_json_with_fallback(
                Method::POST,
                &[path_message.as_str(), path_prompt.as_str()],
                &query,
                Some(body),
            )
            .await?;

        Ok(())
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SessionWire {
    id: String,
    #[serde(default)]
    title: Option<String>,
    #[serde(default, alias = "parentID", alias = "parent_id")]
    parent_id: Option<String>,
    #[serde(default)]
    updated_at: Option<String>,
    #[serde(default)]
    time: SessionTimeWire,
}

#[cfg(test)]
mod tests {
    use super::SessionWire;

    #[test]
    fn session_wire_reads_parent_id_from_parent_id_key() {
        let payload = serde_json::json!({
            "id": "ses_child",
            "parent_id": "ses_parent"
        });
        let parsed: SessionWire = serde_json::from_value(payload).expect("session wire should parse");

        assert_eq!(parsed.parent_id.as_deref(), Some("ses_parent"));
    }

    #[test]
    fn session_wire_reads_parent_id_from_parent_id_caps_key() {
        let payload = serde_json::json!({
            "id": "ses_child",
            "parentID": "ses_parent"
        });
        let parsed: SessionWire = serde_json::from_value(payload).expect("session wire should parse");

        assert_eq!(parsed.parent_id.as_deref(), Some("ses_parent"));
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct SessionTimeWire {
    #[serde(default)]
    updated: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct MessageWire {
    info: MessageInfoWire,
    #[serde(default)]
    parts: Vec<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageInfoWire {
    id: String,
    #[serde(default)]
    role: String,
    #[serde(default)]
    created_at: Option<String>,
    #[serde(default)]
    time: MessageTimeWire,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct MessageTimeWire {
    #[serde(default)]
    created: Option<i64>,
}

async fn stream_realtime_events(
    client: reqwest::Client,
    base_url: String,
    basic_auth: Option<(String, String)>,
    directory: String,
    sender: UnboundedSender<ChatRealtimeEvent>,
) -> Result<()> {
    let mut url = format!("{base_url}/event");
    append_query(&mut url, &[("directory".to_string(), directory)]);

    let mut request = client
        .request(Method::GET, url)
        .header(reqwest::header::ACCEPT, "text/event-stream");

    if let Some((username, password)) = basic_auth.as_ref() {
        request = request.basic_auth(username, Some(password));
    }

    let response = request
        .send()
        .await
        .context("OpenCode // Realtime // event stream request failed")?;

    if !response.status().is_success() {
        bail!(
            "OpenCode // Realtime // event stream status failure (status={})",
            response.status()
        );
    }

    let _ = sender.send(ChatRealtimeEvent {
        event_type: "stream.connected".to_string(),
        session_id: None,
    });

    let mut stream = response.bytes_stream();
    let mut buffer = String::new();
    let mut current_event_type: Option<String> = None;
    let mut current_data = String::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.context("OpenCode // Realtime // stream chunk read failed")?;
        buffer.push_str(&String::from_utf8_lossy(&chunk));

        while let Some(index) = buffer.find('\n') {
            let mut line = buffer[..index].to_string();
            buffer.drain(..=index);

            if line.ends_with('\r') {
                let _ = line.pop();
            }

            if line.is_empty() {
                dispatch_sse_event(
                    &sender,
                    current_event_type.take(),
                    std::mem::take(&mut current_data),
                );
                continue;
            }

            if line.starts_with(':') {
                continue;
            }

            if let Some(value) = line.strip_prefix("event:") {
                current_event_type = Some(value.trim().to_string());
                continue;
            }

            if let Some(value) = line.strip_prefix("data:") {
                if !current_data.is_empty() {
                    current_data.push('\n');
                }
                current_data.push_str(value.trim_start());
            }
        }
    }

    if !current_data.trim().is_empty() || current_event_type.is_some() {
        dispatch_sse_event(&sender, current_event_type.take(), current_data);
    }

    let _ = sender.send(ChatRealtimeEvent {
        event_type: "stream.disconnected".to_string(),
        session_id: None,
    });

    Ok(())
}

fn dispatch_sse_event(
    sender: &UnboundedSender<ChatRealtimeEvent>,
    current_event_type: Option<String>,
    current_data: String,
) {
    if current_event_type.is_none() && current_data.trim().is_empty() {
        return;
    }

    let payload = serde_json::from_str::<Value>(&current_data).ok();
    let event_type = payload
        .as_ref()
        .and_then(|value| value.get("type"))
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .or_else(|| {
            current_event_type
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToString::to_string)
        })
        .unwrap_or_else(|| "event.unknown".to_string());

    let session_id = payload.as_ref().and_then(extract_session_id);

    let _ = sender.send(ChatRealtimeEvent {
        event_type,
        session_id,
    });
}

fn extract_session_id(payload: &Value) -> Option<String> {
    [
        "/sessionID",
        "/sessionId",
        "/id",
        "/properties/sessionID",
        "/properties/sessionId",
        "/properties/id",
        "/data/sessionID",
        "/data/sessionId",
        "/data/id",
    ]
    .iter()
    .find_map(|pointer| payload.pointer(pointer).and_then(value_to_string))
}

fn value_to_string(value: &Value) -> Option<String> {
    if let Some(value) = value.as_str() {
        return Some(value.to_string());
    }
    if let Some(value) = value.as_i64() {
        return Some(value.to_string());
    }
    if let Some(value) = value.as_u64() {
        return Some(value.to_string());
    }

    None
}

fn normalize_path(path: &str) -> String {
    if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    }
}

fn append_query(url: &mut String, query: &[(String, String)]) {
    if query.is_empty() {
        return;
    }

    let mut first = true;

    for (key, value) in query {
        if first {
            url.push('?');
            first = false;
        } else {
            url.push('&');
        }

        url.push_str(&url_encode(key));
        url.push('=');
        url.push_str(&url_encode(value));
    }
}

fn url_encode(value: &str) -> String {
    let mut encoded = String::new();

    for byte in value.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(*byte as char)
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }

    encoded
}

fn parse_response_body(response_text: String) -> Value {
    if response_text.trim().is_empty() {
        return Value::Null;
    }

    serde_json::from_str(&response_text).unwrap_or(Value::String(response_text))
}

fn ensure_success(response: RawResponse) -> Result<Value> {
    if (200..300).contains(&response.status) {
        return Ok(response.body);
    }

    bail!(
        "OpenCode // API // status failure (status={},path={},body={})",
        response.status,
        response.path,
        response.body
    )
}

fn unwrap_data(value: Value) -> Value {
    match value {
        Value::Object(map) => map
            .get("data")
            .cloned()
            .unwrap_or_else(|| Value::Object(map)),
        other => other,
    }
}

fn extract_session_statuses(value: &Value) -> HashMap<String, String> {
    let Some(entries) = value.as_object() else {
        return HashMap::new();
    };

    entries
        .iter()
        .filter_map(|(session_id, status_value)| {
            extract_status_type(status_value)
                .map(|status| (session_id.clone(), status.to_string()))
        })
        .collect()
}

fn extract_status_type(value: &Value) -> Option<&str> {
    if let Some(status) = value.as_str() {
        let trimmed = status.trim();
        return if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        };
    }

    let map = value.as_object()?;

    for key in ["type", "status", "state"] {
        let Some(entry) = map.get(key) else {
            continue;
        };

        if let Some(status) = extract_status_type(entry) {
            return Some(status);
        }
    }

    None
}

fn extract_string_options(value: &Value, candidate_keys: &[&str]) -> Vec<String> {
    let mut result = Vec::new();

    let Some(entries) = value.as_array() else {
        return result;
    };

    for entry in entries {
        if let Some(value) = entry.as_str() {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                result.push(trimmed.to_string());
            }
            continue;
        }

        for key in candidate_keys {
            let Some(value) = entry.get(*key).and_then(Value::as_str) else {
                continue;
            };

            let trimmed = value.trim();
            if !trimmed.is_empty() {
                result.push(trimmed.to_string());
                break;
            }
        }
    }

    result
}

fn parse_model_selector(value: &str) -> Option<(String, String)> {
    let trimmed = value.trim();
    let (provider_id, model_id) = trimmed.split_once('/')?;
    let provider_id = provider_id.trim();
    let model_id = model_id.trim();

    if provider_id.is_empty() || model_id.is_empty() {
        return None;
    }

    Some((provider_id.to_string(), model_id.to_string()))
}

fn extract_status_list(value: &Value, default_name: &str) -> Vec<String> {
    let Some(entries) = value.as_array() else {
        return Vec::new();
    };

    let mut results = Vec::new();

    for entry in entries {
        let name = entry
            .get("name")
            .or_else(|| entry.get("id"))
            .or_else(|| entry.get("key"))
            .or_else(|| entry.get("provider"))
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or(default_name);

        let status = entry
            .get("status")
            .or_else(|| entry.get("state"))
            .or_else(|| entry.get("type"))
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or("unknown");

        results.push(format!("{name}:{status}"));
    }

    results.sort();
    results.dedup();
    results
}

fn extract_mcp_status(value: &Value) -> Vec<String> {
    let Some(map) = value.as_object() else {
        return Vec::new();
    };

    let mut results = Vec::new();
    for (name, entry) in map {
        let status = entry
            .get("status")
            .or_else(|| entry.get("state"))
            .or_else(|| entry.get("type"))
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| {
                if entry
                    .get("connected")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
                {
                    "connected"
                } else {
                    "unknown"
                }
            });

        results.push(format!("{name}:{status}"));
    }

    results.sort();
    results.dedup();
    results
}

fn compact_timestamp(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return "-".to_string();
    }

    if let Some((date, rest)) = trimmed.split_once('T') {
        let time = rest.trim_end_matches('Z').split('.').next().unwrap_or(rest);
        return format!("{date} {time}");
    }

    trimmed.to_string()
}

fn format_unix_timestamp(value: i64) -> Option<String> {
    if value <= 0 {
        return None;
    }

    let seconds = if value > 1_000_000_000_000 {
        value / 1_000
    } else {
        value
    };

    Some(format!("unix:{seconds}"))
}

fn normalize_unix_timestamp(value: i64) -> i64 {
    if value <= 0 {
        return 0;
    }

    if value > 1_000_000_000_000 {
        value / 1_000
    } else {
        value
    }
}

fn extract_config_path(value: &Value) -> Option<String> {
    fn looks_like_config_path(candidate: &str) -> bool {
        let trimmed = candidate.trim();
        if trimmed.is_empty() {
            return false;
        }

        trimmed.contains("/")
            && (trimmed.contains("config")
                || trimmed.ends_with(".json")
                || trimmed.ends_with(".toml")
                || trimmed.ends_with(".yaml")
                || trimmed.ends_with(".yml"))
    }

    fn walk(value: &Value, depth: usize) -> Option<String> {
        if depth > 6 {
            return None;
        }

        match value {
            Value::String(text) => looks_like_config_path(text).then(|| text.trim().to_string()),
            Value::Object(map) => {
                for key in [
                    "path",
                    "configPath",
                    "config_path",
                    "file",
                    "configFile",
                    "config_file",
                    "location",
                ] {
                    if let Some(found) = map.get(key).and_then(|entry| walk(entry, depth + 1)) {
                        return Some(found);
                    }
                }

                map.values().find_map(|entry| walk(entry, depth + 1))
            }
            Value::Array(items) => items.iter().find_map(|entry| walk(entry, depth + 1)),
            _ => None,
        }
    }

    walk(value, 0)
}

fn extract_message_text(parts: &[Value]) -> String {
    let joined = parts
        .iter()
        .flat_map(|part| collect_text(part, 0))
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();

    if joined.is_empty() {
        "(no text content)".to_string()
    } else {
        joined
    }
}

fn collect_text(value: &Value, depth: usize) -> Vec<String> {
    if depth > 4 {
        return Vec::new();
    }

    match value {
        Value::Null => Vec::new(),
        Value::String(text) => {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                Vec::new()
            } else {
                vec![trimmed.to_string()]
            }
        }
        Value::Array(items) => items
            .iter()
            .flat_map(|item| collect_text(item, depth + 1))
            .collect(),
        Value::Object(map) => ["text", "content", "value", "message"]
            .iter()
            .flat_map(|key| map.get(*key).into_iter())
            .flat_map(|entry| collect_text(entry, depth + 1))
            .collect(),
        _ => Vec::new(),
    }
}
