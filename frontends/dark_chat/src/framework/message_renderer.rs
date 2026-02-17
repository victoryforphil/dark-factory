use serde_json::{Map, Value};

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
            .map(format_thinking),
        "tool" | "tool_call" | "toolcall" => Some(format_tool_call(map)),
        "command" | "shell_command" => map
            .get("command")
            .or_else(|| map.get("text"))
            .or_else(|| map.get("content"))
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|command| format!("### Shell Command\n```bash\n{command}\n```")),
        "command_output" | "shell_output" => {
            let output = map
                .get("output")
                .or_else(|| map.get("text"))
                .or_else(|| map.get("content"));
            output.map(|value| {
                let body = normalize_block(value, 36);
                format!("### Shell Output\n```text\n{body}\n```")
            })
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

fn format_thinking(content: &str) -> String {
    let block = truncate_block(content, 12);
    format!("### Thinking\n> {}", block.replace('\n', "\n> "))
}

fn format_tool_call(map: &serde_json::Map<String, Value>) -> String {
    let name = map
        .get("tool")
        .or_else(|| map.get("name"))
        .and_then(Value::as_str)
        .unwrap_or("tool");

    let input = tool_input_value(map);
    let output = tool_output_value(map);
    let mut sections = vec![format!("### Tool // {name}")];

    // For todowrite, produce a compact semantic summary instead of raw JSON.
    if name.eq_ignore_ascii_case("todowrite") {
        let summary = format_todo_summary(input.as_ref(), output.as_ref(), Some(map));
        sections.push(format!("summary: {summary}"));

        if let Some(items) = input
            .as_ref()
            .and_then(extract_todo_items)
            .or_else(|| output.as_ref().and_then(extract_todo_items))
            .or_else(|| extract_todo_items(&Value::Object(map.clone())))
        {
            let max_items = 12usize;
            let mut lines = items
                .iter()
                .take(max_items)
                .map(|item| {
                    let checked = if is_done_status(&item.status) {
                        "x"
                    } else {
                        " "
                    };
                    let mut suffix = String::new();
                    if !item.priority.is_empty() {
                        suffix = format!(" ({})", item.priority);
                    }
                    format!("- [{checked}] {}{suffix}", item.content)
                })
                .collect::<Vec<_>>();
            if items.len() > max_items {
                lines.push(format!("- [ ] ... +{} more", items.len() - max_items));
            }
            sections.push(format!("#### TODOS\n{}", lines.join("\n")));
        }

        return sections.join("\n\n");
    }

    // Task/sub-agent calls: render metadata + compact result preview.
    if name.eq_ignore_ascii_case("task") {
        if let Some(summary) = format_task_summary(input.as_ref(), output.as_ref(), map) {
            sections.push(format!("summary: {summary}"));
        }

        if let Some(task_block) = format_task_input_block(input.as_ref()) {
            sections.push(format!("#### TASK\n{task_block}"));
        }

        if let Some(output_block) = format_task_output_block(output.as_ref(), map) {
            sections.push(format!("#### OUT\n{output_block}"));
        }

        return sections.join("\n\n");
    }

    if let Some(summary) = tool_summary_line(name, input.as_ref(), output.as_ref(), Some(map)) {
        sections.push(format!("summary: {summary}"));
    }

    if let Some(value) = input.as_ref() {
        let body = normalize_block(value, 28);
        sections.push(format!("#### IN\n```json\n{body}\n```"));
    }

    if let Some(value) = output.as_ref() {
        let body = normalize_block(value, 36);
        sections.push(format!("#### OUT\n```json\n{body}\n```"));
    }

    sections.join("\n\n")
}

fn tool_summary_line(
    name: &str,
    input: Option<&Value>,
    output: Option<&Value>,
    raw: Option<&Map<String, Value>>,
) -> Option<String> {
    if name.eq_ignore_ascii_case("todowrite") {
        if let Some(count) = input
            .and_then(todos_count)
            .or_else(|| output.and_then(todos_count))
            .or_else(|| raw.and_then(todos_count_from_map))
        {
            return Some(format!("{count} todo item(s) updated"));
        }
    }

    if let Some(command) = input
        .and_then(command_line)
        .map(str::to_owned)
        .or_else(|| output.and_then(command_line).map(str::to_owned))
        .or_else(|| raw.and_then(command_line_from_map_owned))
    {
        return Some(command);
    }

    input
        .and_then(first_meaningful_line)
        .map(str::to_owned)
        .or_else(|| output.and_then(first_meaningful_line).map(str::to_owned))
        .or_else(|| raw.and_then(first_meaningful_line_from_map_owned))
}

/// Produces a concise human-readable todo summary from the tool payload.
///
/// Examples:
///   "5 todos (3 done, 2 active)"
///   "2 todo item(s) updated"
fn format_todo_summary(
    input: Option<&Value>,
    output: Option<&Value>,
    raw: Option<&Map<String, Value>>,
) -> String {
    // Try to extract per-status counts from the todos array.
    if let Some(stats) = input
        .and_then(extract_todo_stats)
        .or_else(|| output.and_then(extract_todo_stats))
        .or_else(|| {
            raw.map(|m| Value::Object(m.clone()))
                .as_ref()
                .and_then(extract_todo_stats)
        })
    {
        let total = stats.done + stats.active + stats.other;
        let mut parts = Vec::new();
        if stats.done > 0 {
            parts.push(format!("{} done", stats.done));
        }
        if stats.active > 0 {
            parts.push(format!("{} active", stats.active));
        }
        if stats.other > 0 {
            parts.push(format!("{} pending", stats.other));
        }
        if parts.is_empty() {
            return format!("{total} todos updated");
        }
        return format!("{total} todos ({})", parts.join(", "));
    }

    // Fallback: simple count.
    if let Some(count) = input
        .and_then(todos_count)
        .or_else(|| output.and_then(todos_count))
        .or_else(|| raw.and_then(todos_count_from_map))
    {
        return format!("{count} todo item(s) updated");
    }

    "todos updated".to_string()
}

struct TodoStats {
    done: usize,
    active: usize,
    other: usize,
}

#[derive(Debug, Clone)]
struct TodoItem {
    content: String,
    status: String,
    priority: String,
}

fn extract_todo_stats(value: &Value) -> Option<TodoStats> {
    let items = match value {
        Value::Array(items) => Some(items.as_slice()),
        Value::Object(map) => map
            .get("todos")
            .or_else(|| map.get("items"))
            .and_then(Value::as_array)
            .map(|v| v.as_slice()),
        Value::String(text) => {
            return serde_json::from_str::<Value>(text)
                .ok()
                .as_ref()
                .and_then(extract_todo_stats);
        }
        _ => None,
    }?;

    if items.is_empty() {
        return None;
    }

    let mut done = 0usize;
    let mut active = 0usize;
    let mut other = 0usize;

    for item in items {
        let status = item
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase();
        match status.as_str() {
            "completed" | "done" | "finished" => done += 1,
            "in_progress" | "active" | "in-progress" | "started" => active += 1,
            _ => other += 1,
        }
    }

    Some(TodoStats {
        done,
        active,
        other,
    })
}

fn extract_todo_items(value: &Value) -> Option<Vec<TodoItem>> {
    match value {
        Value::Array(items) => {
            let parsed = parse_todo_items_array(items);
            if parsed.is_empty() {
                None
            } else {
                Some(parsed)
            }
        }
        Value::Object(map) => map
            .get("todos")
            .or_else(|| map.get("items"))
            .and_then(Value::as_array)
            .map(|items| parse_todo_items_array(items.as_slice()))
            .filter(|items: &Vec<TodoItem>| !items.is_empty())
            .or_else(|| map.values().find_map(extract_todo_items)),
        Value::String(text) => serde_json::from_str::<Value>(text)
            .ok()
            .as_ref()
            .and_then(extract_todo_items),
        _ => None,
    }
}

fn parse_todo_items_array(items: &[Value]) -> Vec<TodoItem> {
    let mut parsed = Vec::new();
    for entry in items {
        let Some(map) = entry.as_object() else {
            continue;
        };

        let content = map
            .get("content")
            .or_else(|| map.get("text"))
            .or_else(|| map.get("label"))
            .and_then(Value::as_str)
            .unwrap_or("")
            .trim()
            .to_string();
        if content.is_empty() {
            continue;
        }

        let status = map
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or("pending")
            .trim()
            .to_string();
        let priority = map
            .get("priority")
            .and_then(Value::as_str)
            .unwrap_or("")
            .trim()
            .to_string();

        parsed.push(TodoItem {
            content,
            status,
            priority,
        });
    }
    parsed
}

fn is_done_status(status: &str) -> bool {
    matches!(
        status.trim().to_ascii_lowercase().as_str(),
        "completed" | "done" | "finished"
    )
}

fn format_task_summary(
    input: Option<&Value>,
    output: Option<&Value>,
    raw: &Map<String, Value>,
) -> Option<String> {
    let input_obj = input.and_then(Value::as_object);
    let subagent = input_obj
        .and_then(|map| str_from_map(map, &["subagent_type", "subagent", "agent"]))
        .or_else(|| str_from_map(raw, &["subagent_type", "subagent", "agent"]));
    let description = input_obj
        .and_then(|map| str_from_map(map, &["description", "title"]))
        .or_else(|| str_from_map(raw, &["description", "title"]));

    match (subagent, description) {
        (Some(agent), Some(desc)) => Some(format!("{} // {}", agent, truncate_inline(desc, 72))),
        (Some(agent), None) => Some(format!("{} sub-agent task", agent)),
        (None, Some(desc)) => Some(truncate_inline(desc, 84)),
        _ => output
            .and_then(extract_task_id)
            .map(|id| format!("task_id: {id}")),
    }
}

fn format_task_input_block(input: Option<&Value>) -> Option<String> {
    let map = input.and_then(Value::as_object)?;
    let mut rows = Vec::new();

    if let Some(agent) = str_from_map(map, &["subagent_type", "subagent", "agent"]) {
        rows.push(format!("- subagent: {agent}"));
    }
    if let Some(description) = str_from_map(map, &["description", "title"]) {
        rows.push(format!(
            "- description: {}",
            truncate_inline(description, 96)
        ));
    }
    if let Some(prompt) = str_from_map(map, &["prompt"]) {
        rows.push(format!("- prompt: {}", truncate_inline(prompt, 120)));
    }

    if rows.is_empty() {
        None
    } else {
        Some(rows.join("\n"))
    }
}

fn format_task_output_block(output: Option<&Value>, raw: &Map<String, Value>) -> Option<String> {
    let mut rows = Vec::new();

    let task_id = output
        .and_then(extract_task_id)
        .or_else(|| str_from_map(raw, &["task_id", "taskId"]).map(str::to_string));
    if let Some(task_id) = task_id {
        rows.push(format!("- task_id: {task_id}"));
    }

    if let Some(result_preview) = output.and_then(extract_task_result_preview) {
        rows.push("- result:".to_string());
        rows.extend(result_preview.lines().map(|line| format!("  {line}")));
    }

    if rows.is_empty() {
        None
    } else {
        Some(rows.join("\n"))
    }
}

fn extract_task_id(value: &Value) -> Option<String> {
    match value {
        Value::Object(map) => str_from_map(map, &["task_id", "taskId"]).map(str::to_string),
        Value::String(text) => text.lines().find_map(|line| {
            let trimmed = line.trim();
            trimmed
                .strip_prefix("task_id:")
                .or_else(|| trimmed.strip_prefix("taskId:"))
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .map(ToString::to_string)
        }),
        _ => None,
    }
}

fn extract_task_result_preview(value: &Value) -> Option<String> {
    let text = value.as_str()?;
    let body = extract_tag_block(text, "<task_result>", "</task_result>")?;
    let mut lines = Vec::new();
    let mut consumed = 0usize;

    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        lines.push(truncate_inline(trimmed, 120));
        consumed += 1;
        if consumed >= 8 {
            break;
        }
    }

    if lines.is_empty() {
        None
    } else {
        let total_nonempty = body.lines().filter(|line| !line.trim().is_empty()).count();
        if total_nonempty > lines.len() {
            lines.push(format!("... +{} more", total_nonempty - lines.len()));
        }
        Some(lines.join("\n"))
    }
}

fn extract_tag_block<'a>(text: &'a str, start_tag: &str, end_tag: &str) -> Option<&'a str> {
    let start = text.find(start_tag)? + start_tag.len();
    let end = text[start..].find(end_tag)? + start;
    Some(text[start..end].trim())
}

fn str_from_map<'a>(map: &'a Map<String, Value>, keys: &[&str]) -> Option<&'a str> {
    keys.iter()
        .find_map(|key| map.get(*key))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn truncate_inline(value: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    if value.chars().count() <= max_chars {
        return value.to_string();
    }

    if max_chars <= 3 {
        return "...".to_string();
    }

    let clipped = value.chars().take(max_chars - 3).collect::<String>();
    format!("{clipped}...")
}

fn todos_count(value: &Value) -> Option<usize> {
    match value {
        Value::Array(items) => Some(items.len()),
        Value::Object(map) => map
            .get("todos")
            .or_else(|| map.get("items"))
            .and_then(Value::as_array)
            .map(|items| items.len())
            .or_else(|| map.values().find_map(todos_count)),
        Value::String(text) => serde_json::from_str::<Value>(text).ok().and_then(|parsed| {
            let parsed_ref = &parsed;
            todos_count(parsed_ref)
        }),
        _ => None,
    }
}

fn first_meaningful_line(value: &Value) -> Option<&str> {
    match value {
        Value::String(text) => text.lines().map(str::trim).find(|line| {
            !line.is_empty() && *line != "{}" && *line != "[]" && !is_noise_summary_line(line)
        }),
        _ => None,
    }
}

fn is_noise_summary_line(line: &str) -> bool {
    let compact = line.trim().to_ascii_lowercase();
    compact.starts_with('"')
        && (compact.starts_with("\"callid\"")
            || compact.starts_with("\"messageid\"")
            || compact.starts_with("\"sessionid\"")
            || compact.starts_with("\"metadata\"")
            || compact.starts_with("\"openai\"")
            || compact.starts_with("\"state\""))
}

fn command_line(value: &Value) -> Option<&str> {
    match value {
        Value::String(text) => text
            .lines()
            .map(str::trim)
            .find(|line| !line.is_empty() && !line.starts_with('{') && !line.starts_with('[')),
        Value::Object(map) => map
            .get("command")
            .or_else(|| map.get("cmd"))
            .or_else(|| map.get("script"))
            .or_else(|| map.get("shell"))
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|line| !line.is_empty()),
        _ => None,
    }
}

fn tool_input_value(map: &Map<String, Value>) -> Option<Value> {
    first_present_value(
        map,
        &[
            "input",
            "arguments",
            "args",
            "params",
            "request",
            "body",
            "payload",
            "call",
        ],
    )
    .map(|value| canonicalize_tool_block(&value))
    .or_else(|| {
        map.get("state")
            .and_then(Value::as_object)
            .and_then(|state| state.get("input"))
            .map(canonicalize_tool_block)
    })
    .or_else(|| {
        let extras = tool_extra_fields(map);
        if extras.is_empty() {
            None
        } else {
            Some(canonicalize_tool_block(&Value::Object(extras)))
        }
    })
}

fn tool_output_value(map: &Map<String, Value>) -> Option<Value> {
    first_present_value(
        map,
        &[
            "output", "result", "response", "data", "error", "stderr", "stdout",
        ],
    )
    .map(|value| canonicalize_tool_block(&value))
    .or_else(|| {
        map.get("state")
            .and_then(Value::as_object)
            .and_then(|state| {
                state
                    .get("output")
                    .or_else(|| state.get("result"))
                    .or_else(|| state.get("error"))
            })
            .map(canonicalize_tool_block)
    })
}

fn first_present_value(map: &Map<String, Value>, keys: &[&str]) -> Option<Value> {
    keys.iter()
        .find_map(|key| map.get(*key))
        .filter(|value| !value.is_null())
        .cloned()
}

fn tool_extra_fields(map: &Map<String, Value>) -> Map<String, Value> {
    let mut extra = Map::new();
    for (key, value) in map {
        if matches!(
            key.as_str(),
            "type"
                | "tool"
                | "name"
                | "id"
                | "toolCallId"
                | "tool_call_id"
                | "status"
                | "time"
                | "createdAt"
                | "created_at"
                | "input"
                | "arguments"
                | "args"
                | "params"
                | "request"
                | "body"
                | "payload"
                | "call"
                | "output"
                | "result"
                | "response"
                | "data"
                | "error"
                | "stderr"
                | "stdout"
        ) {
            continue;
        }

        extra.insert(key.clone(), value.clone());
    }
    extra
}

fn todos_count_from_map(map: &Map<String, Value>) -> Option<usize> {
    let value = Value::Object(map.clone());
    todos_count(&value)
}

fn command_line_from_map_owned(map: &Map<String, Value>) -> Option<String> {
    map.get("command")
        .or_else(|| map.get("cmd"))
        .or_else(|| map.get("script"))
        .or_else(|| map.get("shell"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
}

fn first_meaningful_line_from_map_owned(map: &Map<String, Value>) -> Option<String> {
    map.values()
        .find_map(first_meaningful_line)
        .map(ToString::to_string)
}

fn normalize_block(value: &Value, max_lines: usize) -> String {
    let canonical = canonicalize_tool_block(value);
    let raw = match canonical {
        Value::String(text) => text.trim().to_string(),
        other => pretty(&other),
    };
    truncate_block(&raw, max_lines)
}

fn canonicalize_tool_block(value: &Value) -> Value {
    if let Value::String(text) = value {
        if let Ok(parsed) = serde_json::from_str::<Value>(text) {
            return canonicalize_tool_block(&parsed);
        }

        return Value::String(text.trim().to_string());
    }

    if let Value::Object(map) = value {
        if let Some(state_payload) = map
            .get("state")
            .and_then(Value::as_object)
            .and_then(|state| {
                state
                    .get("input")
                    .or_else(|| state.get("output"))
                    .or_else(|| state.get("result"))
            })
        {
            return canonicalize_tool_block(state_payload);
        }

        let mut reduced = Map::new();
        if let Some(command) = map.get("command").or_else(|| map.get("cmd")) {
            reduced.insert("command".to_string(), command.clone());
        }
        if let Some(todos) = map.get("todos").or_else(|| map.get("items")) {
            reduced.insert("todos".to_string(), todos.clone());
        }
        if let Some(error) = map.get("error") {
            reduced.insert("error".to_string(), error.clone());
        }
        if let Some(stdout) = map.get("stdout") {
            reduced.insert("stdout".to_string(), stdout.clone());
        }
        if let Some(stderr) = map.get("stderr") {
            reduced.insert("stderr".to_string(), stderr.clone());
        }

        if !reduced.is_empty() {
            return Value::Object(reduced);
        }
    }

    value.clone()
}

fn truncate_block(raw: &str, max_lines: usize) -> String {
    let max = max_lines.max(1);
    let mut lines = raw.lines();
    let mut kept = Vec::new();

    for _ in 0..max {
        let Some(line) = lines.next() else {
            break;
        };
        kept.push(line);
    }

    if lines.next().is_some() {
        kept.push("... (truncated)");
    }

    if kept.is_empty() {
        "(empty)".to_string()
    } else {
        kept.join("\n")
    }
}

fn pretty(value: &Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
}

#[cfg(test)]
mod tests {
    use super::extract_message_text;

    #[test]
    fn formats_tool_call_with_summary_and_sections() {
        let parts = vec![serde_json::json!({
            "type": "tool_call",
            "tool": "bash",
            "input": "git status --short",
            "output": " M file.rs"
        })];

        let rendered = extract_message_text(&parts);

        assert!(rendered.contains("### Tool // bash"));
        assert!(rendered.contains("summary: git status --short"));
        assert!(rendered.contains("#### IN"));
        assert!(rendered.contains("#### OUT"));
    }

    #[test]
    fn formats_todowrite_summary_from_todos_count() {
        let parts = vec![serde_json::json!({
            "type": "tool_call",
            "tool": "todowrite",
            "input": {
                "todos": [
                    {"content": "a", "status": "pending", "priority": "low"},
                    {"content": "b", "status": "completed", "priority": "medium"}
                ]
            }
        })];

        let rendered = extract_message_text(&parts);
        // New compact summary: shows per-status breakdown.
        assert!(rendered.contains("summary: 2 todos (1 done, 1 pending)"));
        // Todowrite calls should NOT emit raw IN/OUT JSON sections.
        assert!(!rendered.contains("#### IN"));
        assert!(!rendered.contains("#### OUT"));
        // Checklist rows should be present for detail popup readability.
        assert!(rendered.contains("- [ ] a (low)"));
        assert!(rendered.contains("- [x] b (medium)"));
    }

    #[test]
    fn unwraps_nested_state_input_for_tool_preview() {
        let parts = vec![serde_json::json!({
            "type": "tool_call",
            "tool": "pty_read",
            "input": {
                "callID": "call_abc",
                "messageID": "msg_123",
                "metadata": {"openai": {"itemId": "fc_1"}},
                "state": {
                    "input": {
                        "id": "pty_4b364890",
                        "limit": 120,
                        "offset": 40
                    }
                }
            }
        })];

        let rendered = extract_message_text(&parts);
        assert!(rendered.contains("### Tool // pty_read"));
        assert!(rendered.contains("\"id\": \"pty_4b364890\""));
        assert!(!rendered.contains("\"callID\""));
        assert!(!rendered.contains("\"messageID\""));
    }
}
