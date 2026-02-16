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

    #[error("Dark Rust // API // Request returned failure status (status={status},path={path},body={body})")]
    ApiStatus {
        status: u16,
        path: String,
        body: Value,
    },

    #[error("Dark Rust // JSON // Serialization failed (error={0})")]
    JsonSerialization(#[from] serde_json::Error),
}
