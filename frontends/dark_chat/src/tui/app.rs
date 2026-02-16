use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use dark_tui_components::ComponentTheme;
use serde::{Deserialize, Serialize};
use tui_textarea::{CursorMove, TextArea};

use crate::core::{ChatMessage, ChatSession, ChatSnapshot, ProviderHealth, ProviderRuntimeStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusPane {
    Sessions,
    Chat,
    Runtime,
    Composer,
}

#[derive(Debug, Clone)]
pub struct ComposerAutocompleteItem {
    pub label: String,
    pub insert: String,
    pub tag: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerAutocompleteMode {
    Slash,
    File,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct PersistedChatSelection {
    #[serde(default)]
    model: Option<String>,
    #[serde(default)]
    agent: Option<String>,
}

pub struct App {
    base_url: String,
    directory: String,
    provider_name: String,
    preferences_path: PathBuf,
    refresh_seconds: u64,
    theme: ComponentTheme,
    health: ProviderHealth,
    sessions: Vec<ChatSession>,
    selected_session: usize,
    agents: Vec<String>,
    selected_agent: usize,
    models: Vec<String>,
    selected_model: usize,
    active_model_override: Option<String>,
    messages: Vec<ChatMessage>,
    runtime_status: ProviderRuntimeStatus,
    focus: FocusPane,
    status_message: String,
    draft: String,
    draft_cursor: usize,
    composer: TextArea<'static>,
    composing: bool,
    composer_autocomplete_open: bool,
    composer_autocomplete_mode: Option<ComposerAutocompleteMode>,
    composer_autocomplete_query: String,
    composer_autocomplete_selected: usize,
    composer_autocomplete_token_start: usize,
    composer_autocomplete_items: Vec<ComposerAutocompleteItem>,
    workspace_file_cache: Vec<String>,
    workspace_file_cache_loaded: bool,
    model_selector_open: bool,
    model_selector_raw_mode: bool,
    model_selector_query: String,
    model_selector_raw_input: String,
    model_selector_selected: usize,
    model_selector_anchor_col: Option<u16>,
    agent_selector_open: bool,
    agent_selector_query: String,
    agent_selector_selected: usize,
    agent_selector_anchor_col: Option<u16>,
    sessions_scroll_index: usize,
    chat_scroll_lines: u16,
    runtime_scroll_lines: u16,
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
        let preferences_path = Path::new(&directory)
            .join(".darkfactory")
            .join("darkchat.toml");

        Self {
            base_url,
            directory,
            provider_name,
            preferences_path,
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
            active_model_override: None,
            messages: Vec::new(),
            runtime_status: ProviderRuntimeStatus::default(),
            focus: FocusPane::Chat,
            status_message: "Booting dark_chat".to_string(),
            draft,
            draft_cursor,
            composer,
            composing: false,
            composer_autocomplete_open: false,
            composer_autocomplete_mode: None,
            composer_autocomplete_query: String::new(),
            composer_autocomplete_selected: 0,
            composer_autocomplete_token_start: 0,
            composer_autocomplete_items: Vec::new(),
            workspace_file_cache: Vec::new(),
            workspace_file_cache_loaded: false,
            model_selector_open: false,
            model_selector_raw_mode: false,
            model_selector_query: String::new(),
            model_selector_raw_input: String::new(),
            model_selector_selected: 0,
            model_selector_anchor_col: None,
            agent_selector_open: false,
            agent_selector_query: String::new(),
            agent_selector_selected: 0,
            agent_selector_anchor_col: None,
            sessions_scroll_index: 0,
            chat_scroll_lines: 0,
            runtime_scroll_lines: 0,
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

    pub fn sessions_scroll_index(&self) -> usize {
        self.sessions_scroll_index
    }

    pub fn runtime_scroll_lines(&self) -> u16 {
        self.runtime_scroll_lines
    }

    pub fn composer(&self) -> &TextArea<'static> {
        &self.composer
    }

    pub fn composer_autocomplete_open(&self) -> bool {
        self.composer_autocomplete_open
    }

    pub fn composer_autocomplete_mode(&self) -> Option<ComposerAutocompleteMode> {
        self.composer_autocomplete_mode
    }

    pub fn composer_autocomplete_query(&self) -> &str {
        &self.composer_autocomplete_query
    }

    pub fn composer_autocomplete_selected(&self) -> usize {
        self.composer_autocomplete_selected
    }

    pub fn composer_autocomplete_items(&self) -> &[ComposerAutocompleteItem] {
        &self.composer_autocomplete_items
    }

    pub fn composer_autocomplete_anchor_position(&self) -> Option<(usize, usize)> {
        if !self.composer_autocomplete_open {
            return None;
        }

        Some(row_col_from_cursor_index(
            &self.draft,
            self.composer_autocomplete_token_start,
        ))
    }

    pub fn active_agent(&self) -> Option<&str> {
        self.agents.get(self.selected_agent).map(String::as_str)
    }

    pub fn active_model(&self) -> Option<&str> {
        self.active_model_override
            .as_deref()
            .or_else(|| self.models.get(self.selected_model).map(String::as_str))
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
        self.sessions.sort_by(|left, right| {
            right
                .updated_unix
                .unwrap_or_default()
                .cmp(&left.updated_unix.unwrap_or_default())
                .then_with(|| left.title.cmp(&right.title))
        });
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
        self.active_model_override =
            previous_model.filter(|value| self.models.iter().all(|model| model != value));
        self.sessions_scroll_index = self
            .sessions_scroll_index
            .min(self.sessions.len().saturating_sub(1));
        self.model_selector_selected = 0;
        self.agent_selector_selected = 0;
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

    pub fn set_selected_session_index(&mut self, index: usize) -> bool {
        if index >= self.sessions.len() {
            return false;
        }

        let changed = self.selected_session != index;
        self.selected_session = index;
        self.chat_scroll_lines = 0;
        changed
    }

    pub fn select_next_agent(&mut self) {
        self.selected_agent = next_index(self.selected_agent, self.agents.len());
        let _ = self.persist_selection();
    }

    pub fn set_active_agent_by_name(&mut self, value: &str) -> bool {
        let target = value.trim();
        if target.is_empty() {
            return false;
        }

        if let Some(index) = self.agents.iter().position(|agent| agent == target) {
            self.selected_agent = index;
            let _ = self.persist_selection();
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
            self.active_model_override = None;
            let _ = self.persist_selection();
            return true;
        }

        self.active_model_override = Some(target.to_string());
        let _ = self.persist_selection();
        true
    }

    pub fn restore_selection_from_disk(&mut self) -> io::Result<bool> {
        let saved = self.load_selection_from_disk()?;
        let Some(saved) = saved else {
            return Ok(false);
        };

        let mut restored = false;

        if let Some(model) = saved.model {
            restored |= self.set_active_model_by_name(&model);
        }

        if let Some(agent) = saved.agent {
            restored |= self.set_active_agent_by_name(&agent);
        }

        Ok(restored)
    }

    pub fn is_model_selector_open(&self) -> bool {
        self.model_selector_open
    }

    pub fn model_selector_raw_mode(&self) -> bool {
        self.model_selector_raw_mode
    }

    pub fn model_selector_query(&self) -> &str {
        &self.model_selector_query
    }

    pub fn model_selector_raw_input(&self) -> &str {
        &self.model_selector_raw_input
    }

    pub fn model_selector_selected(&self) -> usize {
        self.model_selector_selected
    }

    pub fn model_selector_anchor_col(&self) -> Option<u16> {
        self.model_selector_anchor_col
    }

    pub fn open_model_selector(&mut self) {
        self.model_selector_open = true;
        self.model_selector_raw_mode = false;
        self.model_selector_query.clear();
        self.model_selector_raw_input = self.active_model().unwrap_or_default().to_string();
        self.model_selector_selected = 0;
        self.model_selector_anchor_col = None;
        self.agent_selector_open = false;
    }

    pub fn open_model_selector_at(&mut self, anchor_col: u16) {
        self.open_model_selector();
        self.model_selector_anchor_col = Some(anchor_col);
    }

    pub fn close_model_selector(&mut self) {
        self.model_selector_open = false;
        self.model_selector_raw_mode = false;
        self.model_selector_query.clear();
        self.model_selector_raw_input.clear();
        self.model_selector_selected = 0;
        self.model_selector_anchor_col = None;
    }

    pub fn is_agent_selector_open(&self) -> bool {
        self.agent_selector_open
    }

    pub fn agent_selector_query(&self) -> &str {
        &self.agent_selector_query
    }

    pub fn agent_selector_selected(&self) -> usize {
        self.agent_selector_selected
    }

    pub fn agent_selector_anchor_col(&self) -> Option<u16> {
        self.agent_selector_anchor_col
    }

    pub fn open_agent_selector(&mut self) {
        self.agent_selector_open = true;
        self.agent_selector_query.clear();
        self.agent_selector_selected = 0;
        self.agent_selector_anchor_col = None;
        self.close_model_selector();
    }

    pub fn open_agent_selector_at(&mut self, anchor_col: u16) {
        self.open_agent_selector();
        self.agent_selector_anchor_col = Some(anchor_col);
    }

    pub fn close_agent_selector(&mut self) {
        self.agent_selector_open = false;
        self.agent_selector_query.clear();
        self.agent_selector_selected = 0;
        self.agent_selector_anchor_col = None;
    }

    pub fn agent_selector_insert_char(&mut self, value: char) {
        self.agent_selector_query.push(value);
        self.agent_selector_selected = 0;
    }

    pub fn agent_selector_backspace(&mut self) {
        self.agent_selector_query.pop();
        self.agent_selector_selected = 0;
    }

    pub fn agent_selector_clear(&mut self) {
        self.agent_selector_query.clear();
        self.agent_selector_selected = 0;
    }

    pub fn agent_selector_items(&self) -> Vec<String> {
        let filter = self.agent_selector_query.trim().to_ascii_lowercase();
        let mut items = self.agents.clone();
        if filter.is_empty() {
            return items;
        }

        items.retain(|agent| agent.to_ascii_lowercase().contains(&filter));
        items
    }

    pub fn agent_selector_move_up(&mut self) {
        let len = self.agent_selector_items().len();
        if len == 0 {
            self.agent_selector_selected = 0;
            return;
        }

        self.agent_selector_selected = previous_index(self.agent_selector_selected, len);
    }

    pub fn agent_selector_move_down(&mut self) {
        let len = self.agent_selector_items().len();
        if len == 0 {
            self.agent_selector_selected = 0;
            return;
        }

        self.agent_selector_selected = next_index(self.agent_selector_selected, len);
    }

    pub fn agent_selector_set_selected(&mut self, index: usize) {
        let len = self.agent_selector_items().len();
        if len == 0 {
            self.agent_selector_selected = 0;
            return;
        }

        self.agent_selector_selected = index.min(len.saturating_sub(1));
    }

    pub fn confirm_agent_selector(&mut self) -> Option<String> {
        let items = self.agent_selector_items();
        let selected = items.get(self.agent_selector_selected)?.clone();
        self.set_active_agent_by_name(&selected);
        self.close_agent_selector();
        Some(selected)
    }

    pub fn model_selector_toggle_mode(&mut self) {
        self.model_selector_raw_mode = !self.model_selector_raw_mode;
        if self.model_selector_raw_mode && self.model_selector_raw_input.is_empty() {
            self.model_selector_raw_input = self.model_selector_query.clone();
        }
    }

    pub fn model_selector_insert_char(&mut self, value: char) {
        if self.model_selector_raw_mode {
            self.model_selector_raw_input.push(value);
            return;
        }

        self.model_selector_query.push(value);
        self.model_selector_selected = 0;
    }

    pub fn model_selector_backspace(&mut self) {
        if self.model_selector_raw_mode {
            self.model_selector_raw_input.pop();
            return;
        }

        self.model_selector_query.pop();
        self.model_selector_selected = 0;
    }

    pub fn model_selector_clear(&mut self) {
        if self.model_selector_raw_mode {
            self.model_selector_raw_input.clear();
            return;
        }

        self.model_selector_query.clear();
        self.model_selector_selected = 0;
    }

    pub fn model_selector_move_up(&mut self) {
        let len = self.model_selector_items().len();
        if len == 0 {
            self.model_selector_selected = 0;
            return;
        }

        self.model_selector_selected = previous_index(self.model_selector_selected, len);
    }

    pub fn model_selector_move_down(&mut self) {
        let len = self.model_selector_items().len();
        if len == 0 {
            self.model_selector_selected = 0;
            return;
        }

        self.model_selector_selected = next_index(self.model_selector_selected, len);
    }

    pub fn model_selector_set_selected(&mut self, index: usize) {
        let len = self.model_selector_items().len();
        if len == 0 {
            self.model_selector_selected = 0;
            return;
        }

        self.model_selector_selected = index.min(len.saturating_sub(1));
    }

    pub fn model_selector_items(&self) -> Vec<String> {
        let filter = self.model_selector_query.trim().to_ascii_lowercase();
        let mut items = self.models.clone();
        if filter.is_empty() {
            return items;
        }

        items.retain(|model| model.to_ascii_lowercase().contains(&filter));
        items
    }

    pub fn confirm_model_selector(&mut self) -> Option<String> {
        if self.model_selector_raw_mode {
            let value = self.model_selector_raw_input.trim();
            if value.is_empty() {
                return None;
            }

            let selected = value.to_string();
            self.set_active_model_by_name(&selected);
            self.close_model_selector();
            return Some(selected);
        }

        let items = self.model_selector_items();
        let selected = items.get(self.model_selector_selected)?.clone();
        self.set_active_model_by_name(&selected);
        self.close_model_selector();
        Some(selected)
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
        self.chat_scroll_lines = 0;
    }

    pub fn close_composer_autocomplete(&mut self) {
        self.composer_autocomplete_open = false;
        self.composer_autocomplete_mode = None;
        self.composer_autocomplete_query.clear();
        self.composer_autocomplete_selected = 0;
        self.composer_autocomplete_token_start = 0;
        self.composer_autocomplete_items.clear();
    }

    pub fn composer_autocomplete_move_up(&mut self) {
        let len = self.composer_autocomplete_items.len();
        if len == 0 {
            self.composer_autocomplete_selected = 0;
            return;
        }

        self.composer_autocomplete_selected =
            previous_index(self.composer_autocomplete_selected, len);
    }

    pub fn composer_autocomplete_move_down(&mut self) {
        let len = self.composer_autocomplete_items.len();
        if len == 0 {
            self.composer_autocomplete_selected = 0;
            return;
        }

        self.composer_autocomplete_selected = next_index(self.composer_autocomplete_selected, len);
    }

    pub fn composer_autocomplete_set_selected(&mut self, index: usize) {
        let len = self.composer_autocomplete_items.len();
        if len == 0 {
            self.composer_autocomplete_selected = 0;
            return;
        }

        self.composer_autocomplete_selected = index.min(len.saturating_sub(1));
    }

    pub fn apply_composer_autocomplete_selection(&mut self) -> Option<String> {
        let index = self
            .composer_autocomplete_selected
            .min(self.composer_autocomplete_items.len().saturating_sub(1));
        let selected = self.composer_autocomplete_items.get(index)?.clone();

        let mut chars = self.draft.chars().collect::<Vec<_>>();
        let start = self.composer_autocomplete_token_start.min(chars.len());
        let end = self.draft_cursor.min(chars.len());
        chars.splice(start..end, selected.insert.chars());
        self.draft = chars.into_iter().collect();
        self.draft_cursor = start + selected.insert.chars().count();
        self.sync_composer_from_draft();
        self.close_composer_autocomplete();
        Some(selected.label)
    }

    pub fn is_composing(&self) -> bool {
        self.composing
    }

    pub fn open_composer(&mut self) {
        if self.active_session().is_none() {
            return;
        }

        self.ensure_workspace_file_cache();
        self.focus = FocusPane::Composer;
        self.composing = true;
        self.draft_cursor = self.draft.chars().count();
        self.sync_composer_from_draft();
    }

    pub fn cancel_composer(&mut self) {
        self.composing = false;
        self.focus = FocusPane::Chat;
        self.close_composer_autocomplete();
    }

    pub fn insert_draft_char(&mut self, value: char) {
        if !self.composing {
            return;
        }

        insert_char_at_cursor(&mut self.draft, self.draft_cursor, value);
        self.draft_cursor = self.draft_cursor.saturating_add(1);
        self.sync_composer_from_draft();
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
    }

    pub fn delete_draft_char(&mut self) {
        if !self.composing {
            return;
        }

        remove_char_at_cursor(&mut self.draft, self.draft_cursor);
        self.sync_composer_from_draft();
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

    pub fn scroll_sessions_up(&mut self, amount: usize) {
        self.sessions_scroll_index = self.sessions_scroll_index.saturating_sub(amount.max(1));
    }

    pub fn scroll_sessions_down(&mut self, amount: usize) {
        if self.sessions.is_empty() {
            self.sessions_scroll_index = 0;
            return;
        }

        let max_index = self.sessions.len().saturating_sub(1);
        self.sessions_scroll_index = (self.sessions_scroll_index + amount.max(1)).min(max_index);
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
        self.close_composer_autocomplete();
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
        self.refresh_composer_autocomplete();
        self.composer = build_composer_textarea(&self.draft, self.draft_cursor);
    }

    fn refresh_composer_autocomplete(&mut self) {
        if !self.composing {
            self.close_composer_autocomplete();
            return;
        }

        let Some((trigger, query, token_start)) =
            current_composer_trigger(&self.draft, self.draft_cursor)
        else {
            self.close_composer_autocomplete();
            return;
        };

        let items = match trigger {
            '/' => slash_autocomplete_items(&query),
            '@' => {
                self.ensure_workspace_file_cache();
                file_autocomplete_items(&self.workspace_file_cache, &query)
            }
            _ => Vec::new(),
        };

        if items.is_empty() {
            self.close_composer_autocomplete();
            return;
        }

        self.composer_autocomplete_open = true;
        self.composer_autocomplete_mode = match trigger {
            '/' => Some(ComposerAutocompleteMode::Slash),
            '@' => Some(ComposerAutocompleteMode::File),
            _ => None,
        };
        self.composer_autocomplete_query = query;
        self.composer_autocomplete_token_start = token_start;
        self.composer_autocomplete_items = items;
        self.composer_autocomplete_selected = self
            .composer_autocomplete_selected
            .min(self.composer_autocomplete_items.len().saturating_sub(1));
    }

    fn ensure_workspace_file_cache(&mut self) {
        if self.workspace_file_cache_loaded {
            return;
        }

        self.workspace_file_cache = collect_workspace_files(&self.directory, 2000, 6);
        self.workspace_file_cache_loaded = true;
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

    fn persist_selection(&self) -> io::Result<()> {
        let parent = self
            .preferences_path
            .parent()
            .ok_or_else(|| io::Error::other("missing darkchat.toml parent"))?;
        fs::create_dir_all(parent)?;

        let payload = PersistedChatSelection {
            model: self.active_model().map(ToString::to_string),
            agent: self.active_agent().map(ToString::to_string),
        };
        let encoded = toml::to_string_pretty(&payload)
            .map_err(|error| io::Error::other(error.to_string()))?;

        fs::write(&self.preferences_path, encoded)
    }

    fn load_selection_from_disk(&self) -> io::Result<Option<PersistedChatSelection>> {
        if !self.preferences_path.exists() {
            return Ok(None);
        }

        let raw = fs::read_to_string(&self.preferences_path)?;
        let decoded = toml::from_str::<PersistedChatSelection>(&raw)
            .map_err(|error| io::Error::other(error.to_string()))?;
        Ok(Some(decoded))
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

fn current_composer_trigger(value: &str, cursor_index: usize) -> Option<(char, String, usize)> {
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

fn slash_autocomplete_items(query: &str) -> Vec<ComposerAutocompleteItem> {
    const COMMANDS: [(&str, &str); 8] = [
        ("help", "toggle help"),
        ("refresh", "refresh snapshot"),
        ("new", "create session"),
        ("clear", "clear messages"),
        ("sessions", "session summary"),
        ("agent", "set agent"),
        ("model", "set model"),
        ("grep", "search workspace"),
    ];

    let needle = query.trim().to_ascii_lowercase();
    let mut items = COMMANDS
        .into_iter()
        .filter(|(name, _)| needle.is_empty() || name.contains(&needle))
        .map(|(name, desc)| ComposerAutocompleteItem {
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

fn file_autocomplete_items(paths: &[String], query: &str) -> Vec<ComposerAutocompleteItem> {
    let needle = query.trim().to_ascii_lowercase();
    paths
        .iter()
        .filter(|path| needle.is_empty() || path.to_ascii_lowercase().contains(&needle))
        .take(80)
        .map(|path| ComposerAutocompleteItem {
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
