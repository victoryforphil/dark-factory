use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

use crate::client::{RawApiResponse, normalize_path};
use crate::error::DarkRustError;

const WS_RPC_PATH: &str = "/ws";
const WS_REQUEST_TIMEOUT: Duration = Duration::from_secs(20);
const WS_DRAIN_TIMEOUT: Duration = Duration::from_millis(1);

#[derive(Debug, Clone, Deserialize)]
pub struct DarkCoreWsEvent {
    pub event: String,
    pub timestamp: String,
    #[serde(default)]
    pub payload: Value,
}

#[derive(Debug)]
struct DarkCoreWsState {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
    buffered_events: VecDeque<DarkCoreWsEvent>,
}

#[derive(Debug)]
struct DarkCoreWsInner {
    ws_url: String,
    state: Mutex<DarkCoreWsState>,
    request_sequence: AtomicU64,
}

#[derive(Debug, Clone)]
pub struct DarkCoreWsClient {
    inner: Arc<DarkCoreWsInner>,
}

#[derive(Debug, Serialize)]
struct WsRpcRequestEnvelope {
    #[serde(rename = "type")]
    message_type: &'static str,
    id: String,
    method: String,
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum WsServerEnvelope {
    RpcResponse {
        id: String,
        status: u16,
        path: String,
        body: Value,
    },
    ProtocolError {
        id: Option<String>,
        error: WsProtocolErrorBody,
    },
    Event {
        event: String,
        timestamp: String,
        #[serde(default)]
        payload: Value,
    },
}

#[derive(Debug, Deserialize)]
struct WsProtocolErrorBody {
    code: String,
    message: String,
}

impl DarkCoreWsClient {
    pub async fn connect(base_url: String) -> Result<Self, DarkRustError> {
        let ws_url = build_ws_url(&base_url)?;
        let (socket, _) = connect_async(ws_url.as_str()).await.map_err(|source| {
            DarkRustError::WebSocketConnect {
                url: ws_url.clone(),
                source,
            }
        })?;

        Ok(Self {
            inner: Arc::new(DarkCoreWsInner {
                ws_url,
                state: Mutex::new(DarkCoreWsState {
                    socket,
                    buffered_events: VecDeque::new(),
                }),
                request_sequence: AtomicU64::new(1),
            }),
        })
    }

    pub fn ws_url(&self) -> &str {
        &self.inner.ws_url
    }

    pub async fn request_raw(
        &self,
        method: &str,
        path: &str,
        query: Option<&[(String, String)]>,
        body: Option<Value>,
    ) -> Result<RawApiResponse, DarkRustError> {
        let request_id = format!(
            "rpc_{}",
            self.inner.request_sequence.fetch_add(1, Ordering::Relaxed)
        );
        let method = normalize_http_method(method)?;
        let path = normalize_path(path);
        let query_map = query
            .filter(|pairs| !pairs.is_empty())
            .map(|pairs| pairs.iter().cloned().collect::<BTreeMap<_, _>>());
        let request_body = body.filter(|value| !value.is_null());

        let envelope = WsRpcRequestEnvelope {
            message_type: "rpc_request",
            id: request_id.clone(),
            method,
            path: path.clone(),
            query: query_map,
            body: request_body,
        };

        let payload_text = serde_json::to_string(&envelope)?;
        let mut state = self.inner.state.lock().await;

        state
            .socket
            .send(Message::Text(payload_text.into()))
            .await
            .map_err(|source| DarkRustError::WebSocketIo {
                url: self.inner.ws_url.clone(),
                source,
            })?;

        loop {
            let next = tokio::time::timeout(WS_REQUEST_TIMEOUT, state.socket.next())
                .await
                .map_err(|_| DarkRustError::WebSocketTimeout { path: path.clone() })?;

            let Some(next) = next else {
                return Err(DarkRustError::WebSocketClosed {
                    url: self.inner.ws_url.clone(),
                });
            };

            let message = next.map_err(|source| DarkRustError::WebSocketIo {
                url: self.inner.ws_url.clone(),
                source,
            })?;

            let Some(text) = websocket_message_to_text(message)? else {
                continue;
            };

            let envelope = parse_ws_server_envelope(&text)?;
            match envelope {
                WsServerEnvelope::RpcResponse {
                    id,
                    status,
                    path,
                    body,
                } => {
                    if id != request_id {
                        continue;
                    }

                    return Ok(RawApiResponse { status, path, body });
                }
                WsServerEnvelope::ProtocolError { id, error } => {
                    let id_matches = id
                        .as_deref()
                        .map(|candidate| candidate == request_id)
                        .unwrap_or(true);

                    if !id_matches {
                        continue;
                    }

                    return Err(DarkRustError::WebSocketProtocol {
                        message: format!("{}: {}", error.code, error.message),
                    });
                }
                WsServerEnvelope::Event {
                    event,
                    timestamp,
                    payload,
                } => {
                    state.buffered_events.push_back(DarkCoreWsEvent {
                        event,
                        timestamp,
                        payload,
                    });
                }
            }
        }
    }

    pub async fn drain_events(&self) -> Result<Vec<DarkCoreWsEvent>, DarkRustError> {
        let mut state = self.inner.state.lock().await;
        let mut events = state.buffered_events.drain(..).collect::<Vec<_>>();

        loop {
            let next = match tokio::time::timeout(WS_DRAIN_TIMEOUT, state.socket.next()).await {
                Ok(value) => value,
                Err(_) => break,
            };

            let Some(next) = next else {
                break;
            };

            let message = next.map_err(|source| DarkRustError::WebSocketIo {
                url: self.inner.ws_url.clone(),
                source,
            })?;

            let Some(text) = websocket_message_to_text(message)? else {
                continue;
            };

            let envelope = parse_ws_server_envelope(&text)?;
            if let WsServerEnvelope::Event {
                event,
                timestamp,
                payload,
            } = envelope
            {
                events.push(DarkCoreWsEvent {
                    event,
                    timestamp,
                    payload,
                });
            }
        }

        Ok(events)
    }
}

fn normalize_http_method(method: &str) -> Result<String, DarkRustError> {
    let normalized = method.trim().to_uppercase();

    match normalized.as_str() {
        "GET" | "POST" | "PATCH" | "DELETE" => Ok(normalized),
        _ => Err(DarkRustError::InvalidHttpMethod {
            method: method.to_string(),
        }),
    }
}

fn websocket_message_to_text(message: Message) -> Result<Option<String>, DarkRustError> {
    match message {
        Message::Text(text) => Ok(Some(text.to_string())),
        Message::Binary(binary) => String::from_utf8(binary.to_vec())
            .map(Some)
            .map_err(|error| DarkRustError::WebSocketProtocol {
                message: format!("Binary message was not valid UTF-8 ({error})"),
            }),
        Message::Ping(_) | Message::Pong(_) => Ok(None),
        Message::Close(_) => Ok(None),
        _ => Ok(None),
    }
}

fn parse_ws_server_envelope(value: &str) -> Result<WsServerEnvelope, DarkRustError> {
    serde_json::from_str(value).map_err(|error| DarkRustError::WebSocketProtocol {
        message: format!("Invalid websocket payload ({error})"),
    })
}

fn build_ws_url(base_url: &str) -> Result<String, DarkRustError> {
    let mut url = Url::parse(base_url).map_err(|source| DarkRustError::InvalidWebSocketUrl {
        base_url: base_url.to_string(),
        message: source.to_string(),
    })?;

    let ws_scheme = match url.scheme() {
        "http" | "ws" => "ws",
        "https" | "wss" => "wss",
        _ => {
            return Err(DarkRustError::InvalidWebSocketUrl {
                base_url: base_url.to_string(),
                message: format!("Unsupported URL scheme: {}", url.scheme()),
            });
        }
    };

    url.set_scheme(ws_scheme)
        .map_err(|_| DarkRustError::InvalidWebSocketUrl {
            base_url: base_url.to_string(),
            message: "Unable to convert URL scheme to websocket.".to_string(),
        })?;

    let base_path = url.path().trim_end_matches('/');
    let ws_path = if base_path.is_empty() {
        WS_RPC_PATH.to_string()
    } else {
        format!("{base_path}{WS_RPC_PATH}")
    };

    url.set_path(&ws_path);
    url.set_query(None);
    url.set_fragment(None);

    Ok(url.to_string())
}

#[cfg(test)]
mod tests {
    use super::build_ws_url;

    #[test]
    fn builds_ws_url_from_http_base() {
        let ws_url = build_ws_url("http://localhost:4150").expect("ws url should be built");
        assert_eq!(ws_url, "ws://localhost:4150/ws");
    }

    #[test]
    fn builds_wss_url_from_https_base() {
        let ws_url = build_ws_url("https://example.test/api").expect("ws url should be built");
        assert_eq!(ws_url, "wss://example.test/api/ws");
    }
}
