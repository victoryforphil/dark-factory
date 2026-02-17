use std::collections::HashMap;

use anyhow::{Result, bail};
use serde_json::Value;

pub(crate) fn extract_session_id(payload: &Value) -> Option<String> {
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

pub(crate) fn value_to_string(value: &Value) -> Option<String> {
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

pub(crate) fn normalize_path(path: &str) -> String {
    if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    }
}

pub(crate) fn append_query(url: &mut String, query: &[(String, String)]) {
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

pub(crate) fn parse_response_body(response_text: String) -> Value {
    if response_text.trim().is_empty() {
        return Value::Null;
    }

    serde_json::from_str(&response_text).unwrap_or(Value::String(response_text))
}

pub(crate) fn ensure_success(status: u16, path: &str, body: Value) -> Result<Value> {
    if (200..300).contains(&status) {
        return Ok(body);
    }

    bail!("OpenCode // API // status failure (status={status},path={path},body={body})")
}

pub(crate) fn unwrap_data(value: Value) -> Value {
    match value {
        Value::Object(map) => map
            .get("data")
            .cloned()
            .unwrap_or_else(|| Value::Object(map)),
        other => other,
    }
}

pub(crate) fn extract_session_statuses(value: &Value) -> HashMap<String, String> {
    let Some(entries) = value.as_object() else {
        return HashMap::new();
    };

    entries
        .iter()
        .filter_map(|(session_id, status_value)| {
            extract_status_type(status_value).map(|status| (session_id.clone(), status.to_string()))
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

pub(crate) fn extract_string_options(value: &Value, candidate_keys: &[&str]) -> Vec<String> {
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

pub(crate) fn parse_model_selector(value: &str) -> Option<(String, String)> {
    let trimmed = value.trim();
    let (provider_id, model_id) = trimmed.split_once('/')?;
    let provider_id = provider_id.trim();
    let model_id = model_id.trim();

    if provider_id.is_empty() || model_id.is_empty() {
        return None;
    }

    Some((provider_id.to_string(), model_id.to_string()))
}

pub(crate) fn extract_status_list(value: &Value, default_name: &str) -> Vec<String> {
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

pub(crate) fn extract_mcp_status(value: &Value) -> Vec<String> {
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

pub(crate) fn format_unix_timestamp(value: i64) -> Option<String> {
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

pub(crate) fn normalize_unix_timestamp(value: i64) -> i64 {
    if value <= 0 {
        return 0;
    }

    if value > 1_000_000_000_000 {
        value / 1_000
    } else {
        value
    }
}

pub(crate) fn extract_config_path(value: &Value) -> Option<String> {
    fn looks_like_config_path(candidate: &str) -> bool {
        let trimmed = candidate.trim();
        if trimmed.is_empty() {
            return false;
        }

        trimmed.contains('/')
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
