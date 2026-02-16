use serde_json::Value;

use crate::cli::OutputFormat;

pub fn render(format: OutputFormat, body: &Value) -> Result<String, anyhow::Error> {
    match format {
        OutputFormat::Pretty | OutputFormat::Json => Ok(serde_json::to_string_pretty(body)?),
        OutputFormat::Toml => Ok(toml::to_string_pretty(body)?),
    }
}
