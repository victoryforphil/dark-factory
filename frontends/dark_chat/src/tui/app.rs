use std::time::{SystemTime, UNIX_EPOCH};

use dark_tui_components::ComponentTheme;
use ratatui_code_editor::editor::Editor;
use ratatui_code_editor::theme::vesper;
use tui_textarea::{CursorMove, TextArea};

use crate::core::{ChatMessage, ChatSession, ChatSnapshot, ProviderHealth, ProviderRuntimeStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusPane {
    Sessions,
    Chat,
    Runtime,
    Composer,
}

pub struct App {
    base_url: String,
    directory: String,
    provider_name: String,
    refresh_seconds: u64,
    theme: ComponentTheme,
    health: ProviderHealth,
    sessions: Vec<ChatSession>,
    selected_session: usize,
    agents: Vec<String>,
    selected_agent: usize,
    models: Vec<String>,
    selected_model: usize,
    messages: Vec<ChatMessage>,
    runtime_status: ProviderRuntimeStatus,
    focus: FocusPane,
    status_message: String,
    draft: String,
    draft_cursor: usize,
    composer: TextArea<'static>,
    composing: bool,
    chat_scroll_lines: u16,
    runtime_scroll_lines: u16,
    code_preview_editor: Editor,
    code_preview_source: String,
    code_preview_last_content: String,
    refresh_in_flight: bool,
    send_in_flight: bool,
    create_in_flight: bool,
    realtime_supported: bool,
    realtime_connected: bool,
    realtime_last_event: Option<String>,
    realtime_event_count: u64,
    show_help: bool,
    last_synced: String,
}

impl App {
    pub fn new(
        base_url: String,
        directory: String,
        provider_name: String,
        refresh_seconds: u64,
    ) -> Self {
        let draft = String::new();
        let draft_cursor = 0;
        let composer = build_composer_textarea(&draft, draft_cursor);
        let code_preview_editor = Editor::new("markdown", "", vesper());

        Self {
            base_url,
            directory,
            provider_name,
            refresh_seconds,
            theme: ComponentTheme::default(),
            health: ProviderHealth {
                healthy: false,
                version: None,
            },
            sessions: Vec::new(),
            selected_session: 0,
            agents: Vec::new(),
            selected_agent: 0,
            models: Vec::new(),
            selected_model: 0,
            messages: Vec::new(),
            runtime_status: ProviderRuntimeStatus::default(),
            focus: FocusPane::Chat,
            status_message: "Booting dark_chat".to_string(),
            draft,
            draft_cursor,
            composer,
            composing: false,
            chat_scroll_lines: 0,
            runtime_scroll_lines: 0,
            code_preview_editor,
            code_preview_source: "none".to_string(),
            code_preview_last_content: String::new(),
            refresh_in_flight: false,
            send_in_flight: false,
            create_in_flight: false,
            realtime_supported: false,
            realtime_connected: false,
            realtime_last_event: None,
            realtime_event_count: 0,
            show_help: true,
            last_synced: "-".to_string(),
        }
    }

    pub fn refresh_seconds(&self) -> u64 {
        self.refresh_seconds
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn directory(&self) -> &str {
        &self.directory
    }

    pub fn provider_name(&self) -> &str {
        &self.provider_name
    }

    pub fn theme(&self) -> &ComponentTheme {
        &self.theme
    }

    pub fn health(&self) -> &ProviderHealth {
        &self.health
    }

    pub fn sessions(&self) -> &[ChatSession] {
        &self.sessions
    }

    pub fn selected_session_index(&self) -> usize {
        self.selected_session
    }

    pub fn messages(&self) -> &[ChatMessage] {
        &self.messages
    }

    pub fn runtime_status(&self) -> &ProviderRuntimeStatus {
        &self.runtime_status
    }

    pub fn focus(&self) -> FocusPane {
        self.focus
    }

    pub fn is_focus(&self, pane: FocusPane) -> bool {
        self.focus == pane
    }

    pub fn set_focus(&mut self, pane: FocusPane) {
        self.focus = pane;
    }

    pub fn chat_scroll_lines(&self) -> u16 {
        self.chat_scroll_lines
    }

    pub fn runtime_scroll_lines(&self) -> u16 {
        self.runtime_scroll_lines
    }

    pub fn composer(&self) -> &TextArea<'static> {
        &self.composer
    }

    pub fn code_preview_editor(&self) -> &Editor {
        &self.code_preview_editor
    }

    pub fn code_preview_source(&self) -> &str {
        &self.code_preview_source
    }

    pub fn active_agent(&self) -> Option<&str> {
        self.agents.get(self.selected_agent).map(String::as_str)
    }

    pub fn active_model(&self) -> Option<&str> {
        self.models.get(self.selected_model).map(String::as_str)
    }

    pub fn active_session(&self) -> Option<&ChatSession> {
        self.sessions.get(self.selected_session)
    }

    pub fn active_session_id(&self) -> Option<&str> {
        self.active_session().map(|session| session.id.as_str())
    }

    pub fn status_message(&self) -> &str {
        &self.status_message
    }

    pub fn set_status_message(&mut self, value: impl Into<String>) {
        self.status_message = value.into();
    }

    pub fn apply_snapshot(&mut self, snapshot: ChatSnapshot) {
        let previous_session_id = self.active_session_id().map(ToString::to_string);
        let previous_agent = self.active_agent().map(ToString::to_string);
        let previous_model = self.active_model().map(ToString::to_string);

        self.health = snapshot.health;
        self.sessions = snapshot.sessions;
        self.messages = snapshot.messages;
        self.runtime_status = snapshot.runtime_status;
        self.agents = normalize_options(snapshot.agents);
        self.models = normalize_options(snapshot.models);
        self.last_synced = now_label();

        self.selected_session = resolve_session_index(
            &self.sessions,
            snapshot
                .active_session_id
                .as_deref()
                .or(previous_session_id.as_deref()),
        );
        self.selected_agent = resolve_option_index(&self.agents, previous_agent.as_deref());
        self.selected_model = resolve_option_index(&self.models, previous_model.as_deref());
        self.sync_preview_editor();
    }

    pub fn select_next_session(&mut self) {
        self.selected_session = next_index(self.selected_session, self.sessions.len());
        self.chat_scroll_lines = 0;
    }

    pub fn select_previous_session(&mut self) {
        self.selected_session = previous_index(self.selected_session, self.sessions.len());
        self.chat_scroll_lines = 0;
    }

    pub fn set_active_session_id(&mut self, id: &str) {
        if let Some(index) = self.sessions.iter().position(|session| session.id == id) {
            self.selected_session = index;
            self.chat_scroll_lines = 0;
        }
    }

    pub fn select_next_agent(&mut self) {
        self.selected_agent = next_index(self.selected_agent, self.agents.len());
    }

    pub fn select_next_model(&mut self) {
        self.selected_model = next_index(self.selected_model, self.models.len());
    }

    pub fn set_active_agent_by_name(&mut self, value: &str) -> bool {
        let target = value.trim();
        if target.is_empty() {
            return false;
        }

        if let Some(index) = self.agents.iter().position(|agent| agent == target) {
            self.selected_agent = index;
            return true;
        }

        false
    }

    pub fn set_active_model_by_name(&mut self, value: &str) -> bool {
        let target = value.trim();
        if target.is_empty() {
            return false;
        }

        if let Some(index) = self.models.iter().position(|model| model == target) {
            self.selected_model = index;
            return true;
        }

        false
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
        self.chat_scroll_lines = 0;
        self.sync_preview_editor();
    }

    pub fn is_composing(&self) -> bool {
        self.composing
    }

    pub fn open_composer(&mut self) {
        if self.active_session().is_none() {
            return;
        }

        self.focus = FocusPane::Composer;
        self.composing = true;
        self.draft_cursor = self.draft.chars().count();
        self.sync_composer_from_draft();
        self.sync_preview_editor();
    }

    pub fn cancel_composer(&mut self) {
        self.composing = false;
        self.focus = FocusPane::Chat;
        self.sync_preview_editor();
    }

    pub fn draft(&self) -> &str {
        &self.draft
    }

    pub fn insert_draft_char(&mut self, value: char) {
        if !self.composing {
            return;
        }

        insert_char_at_cursor(&mut self.draft, self.draft_cursor, value);
        self.draft_cursor = self.draft_cursor.saturating_add(1);
        self.sync_composer_from_draft();
        self.sync_preview_editor();
    }

    pub fn backspace_draft(&mut self) {
        if !self.composing {
            return;
        }

        if self.draft_cursor == 0 {
            return;
        }

        self.draft_cursor = self.draft_cursor.saturating_sub(1);
        remove_char_at_cursor(&mut self.draft, self.draft_cursor);
        self.sync_composer_from_draft();
        self.sync_preview_editor();
    }

    pub fn delete_draft_char(&mut self) {
        if !self.composing {
            return;
        }

        remove_char_at_cursor(&mut self.draft, self.draft_cursor);
        self.sync_composer_from_draft();
        self.sync_preview_editor();
    }

    pub fn move_draft_cursor_left(&mut self) {
        if !self.composing {
            return;
        }

        self.draft_cursor = self.draft_cursor.saturating_sub(1);
        self.sync_composer_from_draft();
    }

    pub fn move_draft_cursor_right(&mut self) {
        if !self.composing {
            return;
        }

        let len = self.draft.chars().count();
        self.draft_cursor = (self.draft_cursor + 1).min(len);
        self.sync_composer_from_draft();
    }

    pub fn move_draft_cursor_home(&mut self) {
        if !self.composing {
            return;
        }

        self.draft_cursor = 0;
        self.sync_composer_from_draft();
    }

    pub fn move_draft_cursor_end(&mut self) {
        if !self.composing {
            return;
        }

        self.draft_cursor = self.draft.chars().count();
        self.sync_composer_from_draft();
    }

    pub fn clear_draft(&mut self) {
        if !self.composing {
            return;
        }

        self.draft.clear();
        self.draft_cursor = 0;
        self.sync_composer_from_draft();
        self.sync_preview_editor();
    }

    pub fn draft_cursor(&self) -> usize {
        self.draft_cursor
    }

    pub fn scroll_chat_up(&mut self, amount: u16) {
        self.chat_scroll_lines = self.chat_scroll_lines.saturating_add(amount.max(1));
    }

    pub fn scroll_chat_down(&mut self, amount: u16) {
        self.chat_scroll_lines = self.chat_scroll_lines.saturating_sub(amount.max(1));
    }

    pub fn reset_chat_scroll(&mut self) {
        self.chat_scroll_lines = 0;
    }

    pub fn scroll_runtime_up(&mut self, amount: u16) {
        self.runtime_scroll_lines = self.runtime_scroll_lines.saturating_sub(amount.max(1));
    }

    pub fn scroll_runtime_down(&mut self, amount: u16) {
        self.runtime_scroll_lines = self.runtime_scroll_lines.saturating_add(amount.max(1));
    }

    pub fn reset_runtime_scroll(&mut self) {
        self.runtime_scroll_lines = 0;
    }

    pub fn take_prompt(&self) -> Option<String> {
        if !self.composing {
            return None;
        }

        let trimmed = self.draft.trim();
        if trimmed.is_empty() {
            return None;
        }

        Some(trimmed.to_string())
    }

    pub fn clear_draft_after_send(&mut self) {
        self.draft.clear();
        self.draft_cursor = 0;
        self.sync_composer_from_draft();
        self.composing = false;
        self.focus = FocusPane::Chat;
        self.chat_scroll_lines = 0;
        self.sync_preview_editor();
    }

    pub fn set_refresh_in_flight(&mut self, value: bool) {
        self.refresh_in_flight = value;
    }

    pub fn set_send_in_flight(&mut self, value: bool) {
        self.send_in_flight = value;
    }

    pub fn set_create_in_flight(&mut self, value: bool) {
        self.create_in_flight = value;
    }

    pub fn set_realtime_supported(&mut self, value: bool) {
        self.realtime_supported = value;
    }

    pub fn set_realtime_connected(&mut self, value: bool) {
        self.realtime_connected = value;
    }

    pub fn record_realtime_event(&mut self, event_type: &str) {
        self.realtime_last_event = Some(event_type.to_string());
        self.realtime_event_count = self.realtime_event_count.saturating_add(1);
    }

    pub fn realtime_supported(&self) -> bool {
        self.realtime_supported
    }

    pub fn realtime_connected(&self) -> bool {
        self.realtime_connected
    }

    pub fn realtime_last_event(&self) -> Option<&str> {
        self.realtime_last_event.as_deref()
    }

    pub fn realtime_event_count(&self) -> u64 {
        self.realtime_event_count
    }

    pub fn activity_label(&self) -> String {
        let mut tags = Vec::new();

        if self.refresh_in_flight {
            tags.push("refresh");
        }
        if self.send_in_flight {
            tags.push("send");
        }
        if self.create_in_flight {
            tags.push("new-session");
        }

        if tags.is_empty() {
            "idle".to_string()
        } else {
            tags.join("+")
        }
    }

    fn sync_composer_from_draft(&mut self) {
        self.composer = build_composer_textarea(&self.draft, self.draft_cursor);
    }

    fn sync_preview_editor(&mut self) {
        let (source, content) = if self.composing && !self.draft.trim().is_empty() {
            ("draft".to_string(), clip_preview_content(&self.draft, 5000))
        } else if let Some(message) = self
            .messages
            .iter()
            .rev()
            .find(|entry| !entry.text.trim().is_empty())
        {
            (
                format!("message:{}", normalize_role(&message.role)),
                clip_preview_content(&message.text, 5000),
            )
        } else {
            (
                "none".to_string(),
                "No prompt draft or message content available yet.".to_string(),
            )
        };

        if content != self.code_preview_last_content {
            self.code_preview_editor.set_content(&content);
            self.code_preview_last_content = content;
        }

        self.code_preview_source = source;
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
        self.reset_runtime_scroll();
    }

    pub fn show_help(&self) -> bool {
        self.show_help
    }

    pub fn last_synced(&self) -> &str {
        &self.last_synced
    }
}

fn resolve_session_index(sessions: &[ChatSession], preferred_id: Option<&str>) -> usize {
    if sessions.is_empty() {
        return 0;
    }

    let Some(preferred_id) = preferred_id else {
        return 0;
    };

    sessions
        .iter()
        .position(|session| session.id == preferred_id)
        .unwrap_or(0)
}

fn resolve_option_index(options: &[String], preferred: Option<&str>) -> usize {
    if options.is_empty() {
        return 0;
    }

    let Some(preferred) = preferred else {
        return 0;
    };

    options
        .iter()
        .position(|value| value == preferred)
        .unwrap_or(0)
}

fn normalize_options(mut values: Vec<String>) -> Vec<String> {
    values.retain(|value| !value.trim().is_empty());
    values.sort();
    values.dedup();
    values
}

fn build_composer_textarea(draft: &str, cursor_index: usize) -> TextArea<'static> {
    let mut textarea = if draft.is_empty() {
        TextArea::default()
    } else {
        TextArea::from(draft.split('\n'))
    };

    let (row, col) = row_col_from_cursor_index(draft, cursor_index);
    textarea.move_cursor(CursorMove::Jump(row as u16, col as u16));
    textarea.set_cursor_line_style(ratatui::style::Style::default());
    textarea.set_placeholder_text("Type your prompt. Enter sends, Shift+Enter adds newline.");
    textarea
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

fn normalize_role(value: &str) -> &str {
    match value.trim().to_ascii_lowercase().as_str() {
        "user" => "user",
        "assistant" => "assistant",
        "tool" => "tool",
        "system" => "system",
        _ => "other",
    }
}

fn clip_preview_content(value: &str, max_chars: usize) -> String {
    let trimmed = value.trim();
    if trimmed.chars().count() <= max_chars {
        return trimmed.to_string();
    }

    let clipped = trimmed.chars().take(max_chars).collect::<String>();
    format!("{clipped}\n... [preview truncated]")
}

fn insert_char_at_cursor(buffer: &mut String, cursor: usize, value: char) {
    let mut chars = buffer.chars().collect::<Vec<_>>();
    let index = cursor.min(chars.len());
    chars.insert(index, value);
    *buffer = chars.into_iter().collect();
}

fn remove_char_at_cursor(buffer: &mut String, cursor: usize) {
    let mut chars = buffer.chars().collect::<Vec<_>>();
    if cursor >= chars.len() {
        return;
    }

    chars.remove(cursor);
    *buffer = chars.into_iter().collect();
}

fn next_index(current: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }

    (current + 1) % len
}

fn previous_index(current: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }

    (current + len - 1) % len
}

fn now_label() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    format!("unix:{seconds}")
}
