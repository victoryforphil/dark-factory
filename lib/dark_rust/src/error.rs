use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DarkRustError {
    #[error("Dark Rust // HTTP // Request failed (method={method},path={path},error={source})")]
    Http {
        method: String,
        path: String,
        #[source]
        source: reqwest::Error,
    },

    #[error(
        "Dark Rust // API // Request returned failure status (status={status},path={path},body={body})"
    )]
    ApiStatus {
        status: u16,
        path: String,
        body: Value,
    },

    #[error("Dark Rust // JSON // Serialization failed (error={0})")]
    JsonSerialization(#[from] serde_json::Error),

    #[error("Dark Rust // HTTP // Unsupported method (method={method})")]
    InvalidHttpMethod { method: String },

    #[error("Dark Rust // WS // Invalid base URL (baseUrl={base_url},message={message})")]
    InvalidWebSocketUrl { base_url: String, message: String },

    #[error("Dark Rust // WS // Connect failed (url={url},error={source})")]
    WebSocketConnect {
        url: String,
        #[source]
        source: tokio_tungstenite::tungstenite::Error,
    },

    #[error("Dark Rust // WS // IO failed (url={url},error={source})")]
    WebSocketIo {
        url: String,
        #[source]
        source: tokio_tungstenite::tungstenite::Error,
    },

    #[error("Dark Rust // WS // Connection closed (url={url})")]
    WebSocketClosed { url: String },

    #[error("Dark Rust // WS // Request timed out (path={path})")]
    WebSocketTimeout { path: String },

    #[error("Dark Rust // WS // Protocol violation (message={message})")]
    WebSocketProtocol { message: String },

    #[error("Dark Rust // Locator // Invalid locator value (message={message})")]
    InvalidLocator { message: String },
}
