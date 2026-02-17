use serde_json::Value;

pub fn extract_message_text(parts: &[Value]) -> String {
    let joined = parts
        .iter()
        .filter_map(format_message_part)
        .collect::<Vec<_>>()
        .join("\n\n")
        .trim()
        .to_string();

    if joined.is_empty() {
        "(no text content)".to_string()
    } else {
        joined
    }
}

fn format_message_part(part: &Value) -> Option<String> {
    let map = part.as_object()?;
    let part_type = map
        .get("type")
        .and_then(Value::as_str)
        .map(|value| value.trim().to_ascii_lowercase())
        .unwrap_or_default();

    match part_type.as_str() {
        "text" | "assistant" | "message" | "" => map
            .get("text")
            .or_else(|| map.get("content"))
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToString::to_string),
        "thinking" | "reasoning" => map
            .get("text")
            .or_else(|| map.get("content"))
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|text| format!("### Thinking\n{text}")),
        "tool" | "tool_call" | "toolcall" => {
            let name = map
                .get("tool")
                .or_else(|| map.get("name"))
                .and_then(Value::as_str)
                .unwrap_or("tool");
            let mut sections = vec![format!("### Tool Call ({name})")];
            if let Some(input) = map.get("input") {
                sections.push(format!("Input:\n```json\n{}\n```", pretty(input)));
            }
            if let Some(output) = map.get("output") {
                sections.push(format!("Output:\n```json\n{}\n```", pretty(output)));
            }

            Some(sections.join("\n\n"))
        }
        _ => map
            .get("text")
            .or_else(|| map.get("content"))
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToString::to_string),
    }
}

fn pretty(value: &Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
}
