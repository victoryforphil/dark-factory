use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, anyhow};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalSlashCommand {
    ToggleHelp,
    Refresh,
    CreateSession,
    ClearMessages,
    Sessions,
    SetAgent(String),
    SetModel(String),
    Grep(String),
    ToggleDetailExpansion,
}

pub fn parse_local_slash_command(prompt: &str) -> Option<LocalSlashCommand> {
    let trimmed = prompt.trim();
    let command = trimmed.strip_prefix('/')?.trim();
    if command.is_empty() {
        return None;
    }

    let mut parts = command.splitn(2, char::is_whitespace);
    let name = parts.next()?.trim().to_ascii_lowercase();
    let arg = parts.next().map(str::trim).unwrap_or("");

    match name.as_str() {
        "help" => Some(LocalSlashCommand::ToggleHelp),
        "refresh" => Some(LocalSlashCommand::Refresh),
        "new" | "new-session" => Some(LocalSlashCommand::CreateSession),
        "clear" => Some(LocalSlashCommand::ClearMessages),
        "sessions" => Some(LocalSlashCommand::Sessions),
        "agent" if !arg.is_empty() => Some(LocalSlashCommand::SetAgent(arg.to_string())),
        "model" if !arg.is_empty() => Some(LocalSlashCommand::SetModel(arg.to_string())),
        "grep" if !arg.is_empty() => Some(LocalSlashCommand::Grep(arg.to_string())),
        "expand" | "detail" | "details" => Some(LocalSlashCommand::ToggleDetailExpansion),
        _ => None,
    }
}

pub fn parse_remote_slash_command(prompt: &str) -> Option<String> {
    let trimmed = prompt.trim();
    let command = trimmed.strip_prefix('/')?.trim();
    if command.is_empty() {
        return None;
    }

    if parse_local_slash_command(trimmed).is_some() {
        return None;
    }

    Some(command.to_string())
}

pub fn run_local_grep_summary(directory: &str, pattern: &str) -> Result<String> {
    let trimmed = pattern.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("pattern cannot be empty"));
    }

    let output = Command::new("rg")
        .arg("--line-number")
        .arg("--max-count")
        .arg("12")
        .arg(trimmed)
        .arg(directory)
        .output()
        .context("failed to execute rg")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let matches = stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .take(3)
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    let status_code = output.status.code().unwrap_or_default();
    if status_code != 0 && status_code != 1 {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "rg exited with status {status_code}: {}",
            stderr.trim()
        ));
    }

    if matches.is_empty() {
        Ok(format!("/grep no matches for '{trimmed}'"))
    } else {
        Ok(format!("/grep {}", matches.join(" | ")))
    }
}

pub fn build_prompt_with_file_context(directory: &str, prompt: &str) -> (String, usize) {
    let file_refs = extract_file_references(prompt);
    if file_refs.is_empty() {
        return (prompt.to_string(), 0);
    }

    let mut sections = Vec::new();
    let mut included = 0usize;

    for file_ref in file_refs {
        if included >= 4 {
            break;
        }

        let Some(path) = resolve_reference_path(directory, &file_ref) else {
            continue;
        };

        let Ok(content) = std::fs::read_to_string(&path) else {
            continue;
        };

        included += 1;
        let clipped = clip_text(&content, 4000);
        sections.push(format!("[file:{}]\n{}", path.to_string_lossy(), clipped));
    }

    if sections.is_empty() {
        return (prompt.to_string(), 0);
    }

    (
        format!(
            "{prompt}\n\nReferenced file context:\n{}",
            sections.join("\n\n")
        ),
        included,
    )
}

fn extract_file_references(prompt: &str) -> Vec<String> {
    let mut refs = BTreeSet::<String>::new();

    for token in prompt.split_whitespace() {
        let trimmed = token.trim_matches(|char: char| {
            matches!(
                char,
                ',' | '.' | ';' | ':' | ')' | '(' | '[' | ']' | '{' | '}' | '"' | '\''
            )
        });

        if !trimmed.starts_with('@') {
            continue;
        }

        let path = trimmed.trim_start_matches('@').trim();
        if path.is_empty() {
            continue;
        }

        refs.insert(path.to_string());
    }

    refs.into_iter().collect()
}

fn resolve_reference_path(directory: &str, reference: &str) -> Option<PathBuf> {
    let candidate = PathBuf::from(reference);
    let resolved = if candidate.is_absolute() {
        candidate
    } else {
        Path::new(directory).join(candidate)
    };

    let canonical = resolved.canonicalize().ok()?;
    let workspace = Path::new(directory).canonicalize().ok()?;

    if !canonical.starts_with(&workspace) {
        return None;
    }

    if !canonical.is_file() {
        return None;
    }

    Some(canonical)
}

fn clip_text(value: &str, max_chars: usize) -> String {
    let trimmed = value.trim();
    if trimmed.chars().count() <= max_chars {
        return trimmed.to_string();
    }

    let clipped = trimmed.chars().take(max_chars).collect::<String>();
    format!("{clipped}\n...[truncated]")
}
