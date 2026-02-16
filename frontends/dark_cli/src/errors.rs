use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DarkCliError {
    #[error("Dark CLI // HTTP // Request failed (method={method},path={path},error={source})")]
    Http {
        method: String,
        path: String,
        #[source]
        source: reqwest::Error,
    },

    #[error("Dark CLI // API // Request returned failure status (status={status},path={path},body={body})")]
    ApiStatus {
        status: u16,
        path: String,
        body: Value,
    },

    #[error("Dark CLI // Output // JSON serialization failed (error={0})")]
    JsonSerialization(#[from] serde_json::Error),

    #[error("Dark CLI // Output // TOML serialization failed (error={0})")]
    TomlSerialization(#[from] toml::ser::Error),
}
