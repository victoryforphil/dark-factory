use std::fs;
use std::path::Path;

use dark_tui_components::{next_index, previous_index};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutocompleteMode {
    Slash,
    File,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutocompleteItem {
    pub label: String,
    pub insert: String,
    pub tag: String,
}

pub const DEFAULT_SLASH_COMMANDS: &[(&str, &str)] = &[
    ("help", "toggle help"),
    ("refresh", "refresh snapshot"),
    ("new", "create session"),
    ("clear", "clear messages"),
    ("sessions", "session summary"),
    ("agent", "set agent"),
    ("model", "set model"),
    ("grep", "search workspace"),
];

#[derive(Debug, Clone)]
pub struct ChatAutocomplete {
    pub open: bool,
    pub mode: Option<AutocompleteMode>,
    pub query: String,
    pub selected: usize,
    pub token_start: usize,
    pub items: Vec<AutocompleteItem>,
    workspace_file_cache: Vec<String>,
    workspace_file_cache_loaded: bool,
}

impl ChatAutocomplete {
    pub fn new() -> Self {
        Self {
            open: false,
            mode: None,
            query: String::new(),
            selected: 0,
            token_start: 0,
            items: Vec::new(),
            workspace_file_cache: Vec::new(),
            workspace_file_cache_loaded: false,
        }
    }

    pub fn close(&mut self) {
        self.open = false;
        self.mode = None;
        self.query.clear();
        self.selected = 0;
        self.token_start = 0;
        self.items.clear();
    }

    pub fn move_up(&mut self) {
        if self.items.is_empty() {
            self.selected = 0;
            return;
        }

        self.selected = previous_index(self.selected, self.items.len());
    }

    pub fn move_down(&mut self) {
        if self.items.is_empty() {
            self.selected = 0;
            return;
        }

        self.selected = next_index(self.selected, self.items.len());
    }

    pub fn set_selected(&mut self, index: usize) {
        if self.items.is_empty() {
            self.selected = 0;
            return;
        }

        self.selected = index.min(self.items.len().saturating_sub(1));
    }

    pub fn apply_selection(&mut self, draft: &mut String, cursor: &mut usize) -> Option<String> {
        let index = self.selected.min(self.items.len().saturating_sub(1));
        let selected = self.items.get(index)?.clone();

        let mut chars = draft.chars().collect::<Vec<_>>();
        let start = self.token_start.min(chars.len());
        let end = (*cursor).min(chars.len());
        chars.splice(start..end, selected.insert.chars());
        *draft = chars.into_iter().collect();
        *cursor = start + selected.insert.chars().count();
        self.close();

        Some(selected.label)
    }

    pub fn refresh(
        &mut self,
        draft: &str,
        cursor: usize,
        composing: bool,
        extra_slash_commands: &[(&str, &str)],
    ) {
        if !composing {
            self.close();
            return;
        }

        let Some((trigger, query, token_start)) = current_trigger(draft, cursor) else {
            self.close();
            return;
        };

        let items = match trigger {
            '/' => slash_items(&query, extra_slash_commands),
            '@' => file_items(&self.workspace_file_cache, &query),
            _ => Vec::new(),
        };

        if items.is_empty() {
            self.close();
            return;
        }

        self.open = true;
        self.mode = match trigger {
            '/' => Some(AutocompleteMode::Slash),
            '@' => Some(AutocompleteMode::File),
            _ => None,
        };
        self.query = query;
        self.token_start = token_start;
        self.items = items;
        self.selected = self.selected.min(self.items.len().saturating_sub(1));
    }

    pub fn ensure_workspace_cache(&mut self, directory: &str) {
        if self.workspace_file_cache_loaded {
            return;
        }

        self.workspace_file_cache = collect_workspace_files(directory, 2000, 6);
        self.workspace_file_cache_loaded = true;
    }

    pub fn anchor_position(&self, draft: &str, cursor: usize) -> Option<(usize, usize)> {
        if !self.open {
            return None;
        }

        Some(row_col_from_cursor_index(
            draft,
            self.token_start.min(cursor.max(self.token_start)),
        ))
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn mode(&self) -> Option<AutocompleteMode> {
        self.mode
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn items(&self) -> &[AutocompleteItem] {
        &self.items
    }
}

fn current_trigger(value: &str, cursor_index: usize) -> Option<(char, String, usize)> {
    let mut cursor_byte = value.len();
    if cursor_index < value.chars().count() {
        cursor_byte = value
            .char_indices()
            .nth(cursor_index)
            .map(|(byte, _)| byte)
            .unwrap_or(value.len());
    }

    let prefix = &value[..cursor_byte];
    let token_start_byte = prefix
        .rfind(|ch: char| ch.is_whitespace())
        .map(|index| index + 1)
        .unwrap_or(0);
    let token = &prefix[token_start_byte..];

    let (trigger, query) = if let Some(query) = token.strip_prefix('/') {
        ('/', query)
    } else if let Some(query) = token.strip_prefix('@') {
        ('@', query)
    } else {
        return None;
    };

    let token_start = value[..token_start_byte].chars().count();
    Some((trigger, query.to_string(), token_start))
}

fn slash_items(query: &str, extra: &[(&str, &str)]) -> Vec<AutocompleteItem> {
    let mut commands = DEFAULT_SLASH_COMMANDS.to_vec();
    commands.extend_from_slice(extra);

    let needle = query.trim().to_ascii_lowercase();
    let mut items = commands
        .into_iter()
        .filter(|(name, _)| needle.is_empty() || name.contains(&needle))
        .map(|(name, desc)| AutocompleteItem {
            label: format!("/{name}"),
            insert: if matches!(name, "agent" | "model" | "grep") {
                format!("/{name} ")
            } else {
                format!("/{name}")
            },
            tag: desc.to_string(),
        })
        .collect::<Vec<_>>();

    items.sort_by(|left, right| left.label.cmp(&right.label));
    items
}

fn file_items(paths: &[String], query: &str) -> Vec<AutocompleteItem> {
    let needle = query.trim().to_ascii_lowercase();
    paths
        .iter()
        .filter(|path| needle.is_empty() || path.to_ascii_lowercase().contains(&needle))
        .take(80)
        .map(|path| AutocompleteItem {
            label: format!("@{path}"),
            insert: format!("@{path}"),
            tag: "file".to_string(),
        })
        .collect()
}

fn collect_workspace_files(root: &str, limit: usize, max_depth: usize) -> Vec<String> {
    let mut output = Vec::new();
    let root_path = Path::new(root);
    collect_workspace_files_recursive(root_path, root_path, 0, max_depth, limit, &mut output);
    output.sort();
    output
}

fn collect_workspace_files_recursive(
    root: &Path,
    current: &Path,
    depth: usize,
    max_depth: usize,
    limit: usize,
    output: &mut Vec<String>,
) {
    if output.len() >= limit || depth > max_depth {
        return;
    }

    let Ok(entries) = fs::read_dir(current) else {
        return;
    };

    for entry in entries.flatten() {
        if output.len() >= limit {
            break;
        }

        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();

        if name.starts_with('.') || matches!(name.as_ref(), "target" | "node_modules" | "generated")
        {
            continue;
        }

        if path.is_dir() {
            collect_workspace_files_recursive(root, &path, depth + 1, max_depth, limit, output);
            continue;
        }

        if !path.is_file() {
            continue;
        }

        if let Ok(relative) = path.strip_prefix(root) {
            output.push(relative.to_string_lossy().replace('\\', "/"));
        }
    }
}

fn row_col_from_cursor_index(value: &str, cursor_index: usize) -> (usize, usize) {
    let mut row = 0usize;
    let mut col = 0usize;

    for (index, ch) in value.chars().enumerate() {
        if index == cursor_index {
            break;
        }

        if ch == '\n' {
            row += 1;
            col = 0;
        } else {
            col += 1;
        }
    }

    (row, col)
}

#[cfg(test)]
mod tests {
    use super::{AutocompleteMode, ChatAutocomplete};

    #[test]
    fn refresh_opens_slash_items_for_matching_query() {
        let mut autocomplete = ChatAutocomplete::new();
        autocomplete.refresh("/he", 3, true, &[]);

        assert!(autocomplete.is_open());
        assert_eq!(autocomplete.mode(), Some(AutocompleteMode::Slash));
        assert!(autocomplete
            .items()
            .iter()
            .any(|item| item.label == "/help"));
    }

    #[test]
    fn apply_selection_replaces_active_token() {
        let mut autocomplete = ChatAutocomplete::new();
        let mut draft = "/mo".to_string();
        let mut cursor = draft.chars().count();

        autocomplete.refresh(&draft, cursor, true, &[]);
        let selected = autocomplete
            .apply_selection(&mut draft, &mut cursor)
            .expect("selection should apply");

        assert_eq!(selected, "/model");
        assert_eq!(draft, "/model ");
        assert_eq!(cursor, 7);
        assert!(!autocomplete.is_open());
    }

    #[test]
    fn anchor_position_tracks_token_start() {
        let mut autocomplete = ChatAutocomplete::new();
        let draft = "hello\n/mo";
        let cursor = draft.chars().count();

        autocomplete.refresh(draft, cursor, true, &[]);
        let anchor = autocomplete.anchor_position(draft, cursor);

        assert_eq!(anchor, Some((1, 0)));
    }
}
