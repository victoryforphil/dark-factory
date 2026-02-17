use anyhow::{Context, Result, bail};
use futures_util::StreamExt;
use reqwest::Method;
use serde_json::Value;
use tokio::sync::mpsc::UnboundedSender;

use crate::core::ChatRealtimeEvent;

use super::opencode_extract::{append_query, extract_session_id};

pub(crate) async fn stream_realtime_events(
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
