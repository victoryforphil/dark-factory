use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use dark_tui_components::{next_index, previous_index, HorizontalSplit};

use crate::models::{
    compact_id, compact_locator, compact_timestamp, ActorChatMessageRow, ActorRow,
    DashboardSnapshot, ProductRow, VariantRow,
};
use crate::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusPane {
    Products,
    Variants,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultsViewMode {
    Table,
    Viz,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VizDensity {
    Compact,
    Normal,
    Wide,
    XWide,
}

/// Identifies which node is selected in the viz catalog view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VizSelection {
    Product {
        product_index: usize,
    },
    Variant {
        product_index: usize,
        variant_id: String,
    },
    Actor {
        product_index: usize,
        variant_id: String,
        actor_id: String,
    },
}

#[derive(Debug, Clone)]
pub struct SpawnRequest {
    pub variant_id: String,
    pub provider: String,
    pub initial_prompt: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CloneVariantRequest {
    pub name: Option<String>,
    pub target_path: Option<String>,
    pub branch_name: Option<String>,
    pub clone_type: Option<String>,
    pub source_variant_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BranchVariantRequest {
    pub variant_id: String,
    pub branch_name: String,
}

#[derive(Debug, Clone)]
pub struct DeleteVariantRequest {
    pub variant_id: String,
    pub dry: bool,
}

#[derive(Debug, Clone)]
pub struct MoveActorRequest {
    pub actor_id: String,
    pub source_variant_id: String,
    pub target_variant_id: String,
    pub target_variant_name: String,
}

#[derive(Debug, Clone)]
pub struct InitProductRequest {
    pub directory: String,
}

#[derive(Debug, Clone)]
struct SpawnFormState {
    variant_id: String,
    providers: Vec<String>,
    selected_provider: usize,
    initial_prompt: String,
}

#[derive(Debug, Clone)]
struct CloneFormState {
    selected_field: usize,
    name: String,
    target_path: String,
    branch_name: String,
    clone_type: String,
    source_variant_id: String,
}

#[derive(Debug, Clone)]
struct BranchFormState {
    variant_id: String,
    branch_name: String,
    suggestions: Vec<String>,
    selected_suggestion: usize,
}

#[derive(Debug, Clone)]
struct DeleteVariantFormState {
    variant_id: String,
    remove_clone_directory: bool,
}

#[derive(Debug, Clone)]
struct MoveActorOption {
    variant_id: String,
    variant_name: String,
    product_name: String,
}

#[derive(Debug, Clone)]
struct MoveActorFormState {
    actor_id: String,
    actor_title: String,
    source_variant_id: String,
    source_variant_name: String,
    options: Vec<MoveActorOption>,
    selected_option: usize,
}

#[derive(Debug, Clone)]
struct InitProductFormState {
    directory: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatPickerKind {
    Model,
    Agent,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct PersistedChatSelection {
    #[serde(default)]
    model: Option<String>,
    #[serde(default)]
    agent: Option<String>,
}

impl ResultsViewMode {
    pub fn toggle(self) -> Self {
        match self {
            Self::Table => Self::Viz,
            Self::Viz => Self::Table,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Table => "table",
            Self::Viz => "graphical-tree",
        }
    }

    pub fn display_label(self) -> &'static str {
        match self {
            Self::Table => "table",
            Self::Viz => "graphical tree",
        }
    }

    pub fn is_spatial(self) -> bool {
        matches!(self, Self::Viz)
    }
}

impl VizDensity {
    pub fn cycle(self) -> Self {
        match self {
            Self::Compact => Self::Normal,
            Self::Normal => Self::Wide,
            Self::Wide => Self::XWide,
            Self::XWide => Self::Compact,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Compact => "compact",
            Self::Normal => "normal",
            Self::Wide => "wide",
            Self::XWide => "xwide",
        }
    }
}

impl FocusPane {
    pub fn next(self) -> Self {
        match self {
            Self::Products => Self::Variants,
            Self::Variants => Self::Products,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            Self::Products => Self::Variants,
            Self::Variants => Self::Products,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Products => "products",
            Self::Variants => "variants",
        }
    }
}

/// Tracks an active mouse drag gesture for 2D panning.
#[derive(Debug, Clone, Copy)]
pub struct DragAnchor {
    pub col: u16,
    pub row: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeTarget {
    BodyWithChat(usize),
    BodyWithoutChat(usize),
}

#[derive(Debug)]
pub struct App {
    directory: String,
    chat_preferences_path: PathBuf,
    refresh_seconds: u64,
    focus: FocusPane,
    results_view_mode: ResultsViewMode,
    filter_variants_to_product: bool,
    products: Vec<ProductRow>,
    variants: Vec<VariantRow>,
    actors: Vec<ActorRow>,
    selected_product: usize,
    selected_variant: usize,
    selected_actor: usize,
    /// Current viz-mode node selection.
    viz_selection: Option<VizSelection>,
    status_message: String,
    core_runtime_hint: String,
    core_logs_visible: bool,
    core_logs_session: String,
    core_logs_status: String,
    core_logs_lines: Vec<String>,
    actor_last_message_previews: HashMap<String, String>,
    command_message: String,
    runtime_status: String,
    last_updated: String,
    /// Viz-mode camera pan offset (pixels = terminal cells).
    viz_offset_x: i32,
    viz_offset_y: i32,
    viz_density: VizDensity,
    body_split_with_chat: HorizontalSplit,
    body_split_without_chat: HorizontalSplit,
    resizing_target: Option<ResizeTarget>,
    /// Active drag anchor (set on mouse-down, cleared on mouse-up).
    drag_anchor: Option<DragAnchor>,
    /// Color theme — loaded once at startup.
    theme: Theme,
    spawn_form: Option<SpawnFormState>,
    init_product_form: Option<InitProductFormState>,
    clone_form: Option<CloneFormState>,
    branch_form: Option<BranchFormState>,
    delete_variant_form: Option<DeleteVariantFormState>,
    move_actor_form: Option<MoveActorFormState>,
    inspector_visible: bool,
    chat_visible: bool,
    chat_actor_id: Option<String>,
    chat_messages: Vec<ActorChatMessageRow>,
    chat_history_limit: usize,
    chat_render_limit: usize,
    chat_max_body_lines: usize,
    chat_message_max_chars: usize,
    chat_scroll_lines: u16,
    chat_draft: String,
    chat_composing: bool,
    chat_model_options: Vec<String>,
    chat_agent_options: Vec<String>,
    chat_selected_model: Option<String>,
    chat_selected_agent: Option<String>,
    chat_preferred_model: Option<String>,
    chat_preferred_agent: Option<String>,
    chat_picker_open: Option<ChatPickerKind>,
    chat_picker_query: String,
    chat_picker_selected: usize,
    chat_autocomplete_open: bool,
    chat_autocomplete_mode: Option<char>,
    chat_autocomplete_query: String,
    chat_autocomplete_selected: usize,
    chat_autocomplete_items: Vec<String>,
    chat_detail_popup_open: bool,
    chat_detail_popup_scroll_lines: u16,
    chat_detail_popup_message_index: Option<usize>,
    chat_workspace_file_cache: Vec<String>,
    chat_workspace_file_cache_loaded: bool,
    chat_needs_refresh: bool,
    snapshot_refresh_in_flight: bool,
    chat_refresh_in_flight: bool,
    chat_send_in_flight: bool,
    action_requests_in_flight: usize,
}

impl App {
    pub fn new(directory: String, refresh_seconds: u64, theme: Theme) -> Self {
        let chat_preferences_path = Path::new(&directory)
            .join(".darkfactory")
            .join("darktui.toml");

        Self {
            directory,
            chat_preferences_path,
            refresh_seconds,
            focus: FocusPane::Products,
            results_view_mode: ResultsViewMode::Table,
            filter_variants_to_product: true,
            products: Vec::new(),
            variants: Vec::new(),
            actors: Vec::new(),
            selected_product: 0,
            selected_variant: 0,
            selected_actor: 0,
            viz_selection: None,
            status_message: "Booting dashboard".to_string(),
            core_runtime_hint: "core:unknown".to_string(),
            core_logs_visible: false,
            core_logs_session: "dark-core".to_string(),
            core_logs_status: "idle".to_string(),
            core_logs_lines: Vec::new(),
            actor_last_message_previews: HashMap::new(),
            command_message: String::new(),
            runtime_status: "unknown".to_string(),
            last_updated: "-".to_string(),
            viz_offset_x: 0,
            viz_offset_y: 0,
            viz_density: VizDensity::Normal,
            body_split_with_chat: HorizontalSplit::three(44, 32, 24, 20, 18, 16),
            body_split_without_chat: HorizontalSplit::two(76, 24, 20, 16),
            resizing_target: None,
            drag_anchor: None,
            theme,
            spawn_form: None,
            init_product_form: None,
            clone_form: None,
            branch_form: None,
            delete_variant_form: None,
            move_actor_form: None,
            inspector_visible: true,
            chat_visible: false,
            chat_actor_id: None,
            chat_messages: Vec::new(),
            chat_history_limit: 80,
            chat_render_limit: 40,
            chat_max_body_lines: 24,
            chat_message_max_chars: 12_000,
            chat_scroll_lines: 0,
            chat_draft: String::new(),
            chat_composing: false,
            chat_model_options: Vec::new(),
            chat_agent_options: Vec::new(),
            chat_selected_model: None,
            chat_selected_agent: None,
            chat_preferred_model: None,
            chat_preferred_agent: None,
            chat_picker_open: None,
            chat_picker_query: String::new(),
            chat_picker_selected: 0,
            chat_autocomplete_open: false,
            chat_autocomplete_mode: None,
            chat_autocomplete_query: String::new(),
            chat_autocomplete_selected: 0,
            chat_autocomplete_items: Vec::new(),
            chat_detail_popup_open: false,
            chat_detail_popup_scroll_lines: 0,
            chat_detail_popup_message_index: None,
            chat_workspace_file_cache: Vec::new(),
            chat_workspace_file_cache_loaded: false,
            chat_needs_refresh: false,
            snapshot_refresh_in_flight: false,
            chat_refresh_in_flight: false,
            chat_send_in_flight: false,
            action_requests_in_flight: 0,
        }
    }

    pub fn refresh_seconds(&self) -> u64 {
        self.refresh_seconds
    }

    /// Compact directory display: last 2 path components or full path if short.
    pub fn directory_display(&self) -> &str {
        let d = self.directory.as_str();
        // Show last component or last 2 segments for context.
        if d.len() <= 40 {
            return d;
        }
        d.rsplit_once('/').map_or(d, |(_, tail)| tail)
    }

    pub fn focus(&self) -> FocusPane {
        self.focus
    }

    pub fn results_view_mode(&self) -> ResultsViewMode {
        self.results_view_mode
    }

    pub fn viz_density(&self) -> VizDensity {
        self.viz_density
    }

    pub fn products(&self) -> &[ProductRow] {
        &self.products
    }

    pub fn variants(&self) -> &[VariantRow] {
        &self.variants
    }

    pub fn actors(&self) -> &[ActorRow] {
        &self.actors
    }

    pub fn runtime_status(&self) -> &str {
        &self.runtime_status
    }

    pub fn core_runtime_hint(&self) -> &str {
        &self.core_runtime_hint
    }

    pub fn is_core_logs_visible(&self) -> bool {
        self.core_logs_visible
    }

    pub fn core_logs_session(&self) -> &str {
        &self.core_logs_session
    }

    pub fn core_logs_status(&self) -> &str {
        &self.core_logs_status
    }

    pub fn core_logs_lines(&self) -> &[String] {
        &self.core_logs_lines
    }

    pub fn actor_last_message_preview(&self, actor_id: &str) -> Option<&str> {
        self.actor_last_message_previews
            .get(actor_id)
            .map(String::as_str)
    }

    pub fn apply_actor_last_message_previews(&mut self, previews: Vec<(String, String)>) {
        for (actor_id, preview) in previews {
            let trimmed = preview.trim();
            if trimmed.is_empty() {
                continue;
            }
            self.actor_last_message_previews
                .insert(actor_id, trimmed.to_string());
        }

        self.prune_actor_last_message_previews();
    }

    fn prune_actor_last_message_previews(&mut self) {
        if self.actor_last_message_previews.is_empty() {
            return;
        }

        self.actor_last_message_previews
            .retain(|actor_id, _| self.actors.iter().any(|actor| actor.id == *actor_id));
    }

    #[allow(dead_code)]
    pub fn last_updated(&self) -> &str {
        &self.last_updated
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn body_split_with_chat(&self) -> &HorizontalSplit {
        &self.body_split_with_chat
    }

    pub fn body_split_with_chat_mut(&mut self) -> &mut HorizontalSplit {
        &mut self.body_split_with_chat
    }

    pub fn body_split_without_chat(&self) -> &HorizontalSplit {
        &self.body_split_without_chat
    }

    pub fn body_split_without_chat_mut(&mut self) -> &mut HorizontalSplit {
        &mut self.body_split_without_chat
    }

    pub fn resizing_target(&self) -> Option<ResizeTarget> {
        self.resizing_target
    }

    pub fn start_resize(&mut self, target: ResizeTarget) {
        self.resizing_target = Some(target);
    }

    pub fn stop_resize(&mut self) {
        self.resizing_target = None;
    }

    pub fn status_message(&self) -> &str {
        &self.status_message
    }

    pub fn filter_variants_to_product(&self) -> bool {
        self.filter_variants_to_product
    }

    #[allow(dead_code)]
    pub fn selected_product_index(&self) -> usize {
        self.selected_product
    }

    #[allow(dead_code)]
    pub fn selected_variant_index(&self) -> usize {
        self.selected_variant
    }

    #[allow(dead_code)]
    pub fn selected_actor_index(&self) -> usize {
        self.selected_actor
    }

    pub fn is_spawn_form_open(&self) -> bool {
        self.spawn_form.is_some()
    }

    pub fn is_clone_form_open(&self) -> bool {
        self.clone_form.is_some()
    }

    pub fn is_branch_form_open(&self) -> bool {
        self.branch_form.is_some()
    }

    pub fn is_delete_variant_form_open(&self) -> bool {
        self.delete_variant_form.is_some()
    }

    pub fn is_move_actor_form_open(&self) -> bool {
        self.move_actor_form.is_some()
    }

    pub fn is_init_product_form_open(&self) -> bool {
        self.init_product_form.is_some()
    }

    pub fn open_init_product_form(&mut self) {
        self.init_product_form = Some(InitProductFormState {
            directory: self.directory.clone(),
        });
    }

    pub fn close_init_product_form(&mut self) {
        self.init_product_form = None;
    }

    pub fn init_product_form_directory(&self) -> Option<&str> {
        self.init_product_form
            .as_ref()
            .map(|form| form.directory.as_str())
    }

    pub fn init_product_form_insert_char(&mut self, value: char) {
        let Some(form) = self.init_product_form.as_mut() else {
            return;
        };
        form.directory.push(value);
    }

    pub fn init_product_form_backspace(&mut self) {
        let Some(form) = self.init_product_form.as_mut() else {
            return;
        };
        form.directory.pop();
    }

    pub fn take_init_product_request(&mut self) -> Option<InitProductRequest> {
        let form = self.init_product_form.take()?;
        let directory = form.directory.trim().to_string();
        if directory.is_empty() {
            return None;
        }

        Some(InitProductRequest { directory })
    }

    pub fn spawn_form_providers(&self) -> Option<&[String]> {
        self.spawn_form
            .as_ref()
            .map(|form| form.providers.as_slice())
    }

    pub fn spawn_form_selected_provider_index(&self) -> Option<usize> {
        self.spawn_form.as_ref().map(|form| form.selected_provider)
    }

    pub fn spawn_form_prompt(&self) -> Option<&str> {
        self.spawn_form
            .as_ref()
            .map(|form| form.initial_prompt.as_str())
    }

    pub fn open_spawn_form(
        &mut self,
        variant_id: &str,
        mut providers: Vec<String>,
        default_provider: Option<&str>,
    ) {
        providers.retain(|provider| !provider.trim().is_empty());
        providers.sort();
        providers.dedup();

        if providers.is_empty() {
            providers.push("mock".to_string());
        }

        let selected_provider = default_provider
            .and_then(|default| providers.iter().position(|provider| provider == default))
            .unwrap_or(0);

        self.spawn_form = Some(SpawnFormState {
            variant_id: variant_id.to_string(),
            providers,
            selected_provider,
            initial_prompt: String::new(),
        });
    }

    pub fn close_spawn_form(&mut self) {
        self.spawn_form = None;
    }

    pub fn open_clone_form(&mut self) {
        self.clone_form = Some(CloneFormState {
            selected_field: 0,
            name: String::new(),
            target_path: String::new(),
            branch_name: String::new(),
            clone_type: String::new(),
            source_variant_id: String::new(),
        });
    }

    pub fn open_branch_form(&mut self) -> bool {
        let Some(variant) = self.selected_variant().cloned() else {
            return false;
        };

        let mut suggestions = self
            .variants
            .iter()
            .filter(|row| row.product_id == variant.product_id)
            .flat_map(|row| [row.branch.as_str(), row.worktree.as_str()])
            .map(str::trim)
            .filter(|value| !value.is_empty() && *value != "-")
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        suggestions.sort();
        suggestions.dedup();

        self.branch_form = Some(BranchFormState {
            variant_id: variant.id,
            branch_name: variant.branch,
            suggestions,
            selected_suggestion: 0,
        });

        true
    }

    pub fn close_branch_form(&mut self) {
        self.branch_form = None;
    }

    pub fn close_clone_form(&mut self) {
        self.clone_form = None;
    }

    pub fn branch_form_branch_name(&self) -> Option<&str> {
        self.branch_form
            .as_ref()
            .map(|form| form.branch_name.as_str())
    }

    pub fn branch_form_suggestions(&self) -> Option<Vec<&str>> {
        self.branch_form.as_ref().map(branch_suggestions_for)
    }

    pub fn branch_form_selected_suggestion_index(&self) -> Option<usize> {
        self.branch_form
            .as_ref()
            .map(|form| form.selected_suggestion)
    }

    pub fn branch_form_move_up(&mut self) {
        let Some(form) = self.branch_form.as_mut() else {
            return;
        };

        let len = branch_suggestions_for(form).len();
        form.selected_suggestion = previous_index(form.selected_suggestion, len);
    }

    pub fn branch_form_move_down(&mut self) {
        let Some(form) = self.branch_form.as_mut() else {
            return;
        };

        let len = branch_suggestions_for(form).len();
        form.selected_suggestion = next_index(form.selected_suggestion, len);
    }

    pub fn branch_form_set_selected(&mut self, index: usize) {
        let Some(form) = self.branch_form.as_mut() else {
            return;
        };

        let len = branch_suggestions_for(form).len();
        if len == 0 {
            form.selected_suggestion = 0;
            return;
        }

        form.selected_suggestion = index.min(len.saturating_sub(1));
    }

    pub fn branch_form_insert_char(&mut self, value: char) {
        let Some(form) = self.branch_form.as_mut() else {
            return;
        };

        form.branch_name.push(value);
        form.selected_suggestion = 0;
    }

    pub fn branch_form_backspace(&mut self) {
        let Some(form) = self.branch_form.as_mut() else {
            return;
        };

        form.branch_name.pop();
        form.selected_suggestion = 0;
    }

    pub fn branch_form_apply_suggestion(&mut self) {
        let Some(form) = self.branch_form.as_mut() else {
            return;
        };

        let suggestions = branch_suggestions_for(form);
        if suggestions.is_empty() {
            return;
        }

        let index = form
            .selected_suggestion
            .min(suggestions.len().saturating_sub(1));
        form.branch_name = suggestions[index].to_string();
    }

    pub fn take_branch_request(&mut self) -> Option<BranchVariantRequest> {
        let form = self.branch_form.take()?;
        let branch_name = form.branch_name.trim().to_string();
        if branch_name.is_empty() {
            return None;
        }

        Some(BranchVariantRequest {
            variant_id: form.variant_id,
            branch_name,
        })
    }

    pub fn open_delete_variant_form(&mut self, variant_id: &str) {
        self.delete_variant_form = Some(DeleteVariantFormState {
            variant_id: variant_id.to_string(),
            remove_clone_directory: false,
        });
    }

    pub fn close_delete_variant_form(&mut self) {
        self.delete_variant_form = None;
    }

    pub fn delete_variant_form_variant_id(&self) -> Option<&str> {
        self.delete_variant_form
            .as_ref()
            .map(|form| form.variant_id.as_str())
    }

    pub fn delete_variant_form_remove_clone_directory(&self) -> bool {
        self.delete_variant_form
            .as_ref()
            .map(|form| form.remove_clone_directory)
            .unwrap_or(false)
    }

    pub fn toggle_delete_variant_remove_clone_directory(&mut self) {
        let Some(form) = self.delete_variant_form.as_mut() else {
            return;
        };

        form.remove_clone_directory = !form.remove_clone_directory;
    }

    pub fn take_delete_variant_request(&mut self) -> Option<DeleteVariantRequest> {
        let form = self.delete_variant_form.take()?;
        Some(DeleteVariantRequest {
            variant_id: form.variant_id,
            dry: !form.remove_clone_directory,
        })
    }

    pub fn move_actor_form_actor_title(&self) -> Option<&str> {
        self.move_actor_form
            .as_ref()
            .map(|form| form.actor_title.as_str())
    }

    pub fn move_actor_form_source_variant_id(&self) -> Option<&str> {
        self.move_actor_form
            .as_ref()
            .map(|form| form.source_variant_id.as_str())
    }

    pub fn move_actor_form_source_variant_name(&self) -> Option<&str> {
        self.move_actor_form
            .as_ref()
            .map(|form| form.source_variant_name.as_str())
    }

    pub fn move_actor_form_options(&self) -> Option<Vec<(&str, &str, &str)>> {
        self.move_actor_form.as_ref().map(|form| {
            form.options
                .iter()
                .map(|option| {
                    (
                        option.variant_id.as_str(),
                        option.variant_name.as_str(),
                        option.product_name.as_str(),
                    )
                })
                .collect()
        })
    }

    pub fn move_actor_form_selected_option_index(&self) -> Option<usize> {
        self.move_actor_form
            .as_ref()
            .map(|form| form.selected_option)
    }

    pub fn open_move_actor_form(&mut self) -> bool {
        let Some(actor) = self.selected_actor().cloned() else {
            return false;
        };

        let source_variant = self
            .variants
            .iter()
            .find(|variant| variant.id == actor.variant_id)
            .cloned();

        let source_product_id = source_variant
            .as_ref()
            .map(|variant| variant.product_id.clone());
        let source_variant_name = source_variant
            .as_ref()
            .map(|variant| variant.name.clone())
            .unwrap_or_else(|| "unknown".to_string());

        let mut options: Vec<MoveActorOption> = self
            .variants
            .iter()
            .filter(|variant| variant.id != actor.variant_id)
            .filter(|variant| {
                source_product_id
                    .as_ref()
                    .map_or(true, |product_id| variant.product_id == *product_id)
            })
            .map(|variant| MoveActorOption {
                variant_id: variant.id.clone(),
                variant_name: variant.name.clone(),
                product_name: self
                    .products
                    .iter()
                    .find(|product| product.id == variant.product_id)
                    .map(|product| product.display_name.clone())
                    .unwrap_or_else(|| variant.product_id.clone()),
            })
            .collect();

        if options.is_empty() {
            options = self
                .variants
                .iter()
                .filter(|variant| variant.id != actor.variant_id)
                .map(|variant| MoveActorOption {
                    variant_id: variant.id.clone(),
                    variant_name: variant.name.clone(),
                    product_name: self
                        .products
                        .iter()
                        .find(|product| product.id == variant.product_id)
                        .map(|product| product.display_name.clone())
                        .unwrap_or_else(|| variant.product_id.clone()),
                })
                .collect();
        }

        if options.is_empty() {
            return false;
        }

        options.sort_by(|left, right| {
            left.product_name
                .cmp(&right.product_name)
                .then_with(|| left.variant_name.cmp(&right.variant_name))
                .then_with(|| left.variant_id.cmp(&right.variant_id))
        });

        self.move_actor_form = Some(MoveActorFormState {
            actor_id: actor.id,
            actor_title: actor.title,
            source_variant_id: actor.variant_id,
            source_variant_name,
            options,
            selected_option: 0,
        });

        true
    }

    pub fn close_move_actor_form(&mut self) {
        self.move_actor_form = None;
    }

    pub fn move_actor_form_move_up(&mut self) {
        let Some(form) = self.move_actor_form.as_mut() else {
            return;
        };

        form.selected_option = previous_index(form.selected_option, form.options.len());
    }

    pub fn move_actor_form_move_down(&mut self) {
        let Some(form) = self.move_actor_form.as_mut() else {
            return;
        };

        form.selected_option = next_index(form.selected_option, form.options.len());
    }

    pub fn take_move_actor_request(&mut self) -> Option<MoveActorRequest> {
        let form = self.move_actor_form.take()?;
        let destination = form.options.get(form.selected_option)?;

        Some(MoveActorRequest {
            actor_id: form.actor_id,
            source_variant_id: form.source_variant_id,
            target_variant_id: destination.variant_id.clone(),
            target_variant_name: destination.variant_name.clone(),
        })
    }

    pub fn clone_form_selected_field(&self) -> Option<usize> {
        self.clone_form.as_ref().map(|form| form.selected_field)
    }

    pub fn clone_form_name(&self) -> Option<&str> {
        self.clone_form.as_ref().map(|form| form.name.as_str())
    }

    pub fn clone_form_target_path(&self) -> Option<&str> {
        self.clone_form
            .as_ref()
            .map(|form| form.target_path.as_str())
    }

    pub fn clone_form_branch_name(&self) -> Option<&str> {
        self.clone_form
            .as_ref()
            .map(|form| form.branch_name.as_str())
    }

    pub fn clone_form_clone_type(&self) -> Option<&str> {
        self.clone_form
            .as_ref()
            .map(|form| form.clone_type.as_str())
    }

    pub fn clone_form_source_variant_id(&self) -> Option<&str> {
        self.clone_form
            .as_ref()
            .map(|form| form.source_variant_id.as_str())
    }

    pub fn clone_form_move_up(&mut self) {
        let Some(form) = self.clone_form.as_mut() else {
            return;
        };

        form.selected_field = previous_index(form.selected_field, 5);
    }

    pub fn clone_form_move_down(&mut self) {
        let Some(form) = self.clone_form.as_mut() else {
            return;
        };

        form.selected_field = next_index(form.selected_field, 5);
    }

    pub fn clone_form_insert_char(&mut self, value: char) {
        let Some(form) = self.clone_form.as_mut() else {
            return;
        };

        match form.selected_field {
            0 => form.name.push(value),
            1 => form.target_path.push(value),
            2 => form.branch_name.push(value),
            3 => form.clone_type.push(value),
            _ => form.source_variant_id.push(value),
        }
    }

    pub fn clone_form_backspace(&mut self) {
        let Some(form) = self.clone_form.as_mut() else {
            return;
        };

        match form.selected_field {
            0 => {
                form.name.pop();
            }
            1 => {
                form.target_path.pop();
            }
            2 => {
                form.branch_name.pop();
            }
            3 => {
                form.clone_type.pop();
            }
            _ => {
                form.source_variant_id.pop();
            }
        }
    }

    pub fn take_clone_request(&mut self) -> Option<CloneVariantRequest> {
        let form = self.clone_form.take()?;

        Some(CloneVariantRequest {
            name: normalize_optional_input(&form.name),
            target_path: normalize_optional_input(&form.target_path),
            branch_name: normalize_optional_input(&form.branch_name),
            clone_type: normalize_optional_input(&form.clone_type),
            source_variant_id: normalize_optional_input(&form.source_variant_id),
        })
    }

    pub fn spawn_form_move_provider_up(&mut self) {
        let Some(form) = self.spawn_form.as_mut() else {
            return;
        };

        form.selected_provider = previous_index(form.selected_provider, form.providers.len());
    }

    pub fn spawn_form_move_provider_down(&mut self) {
        let Some(form) = self.spawn_form.as_mut() else {
            return;
        };

        form.selected_provider = next_index(form.selected_provider, form.providers.len());
    }

    pub fn spawn_form_insert_char(&mut self, value: char) {
        let Some(form) = self.spawn_form.as_mut() else {
            return;
        };

        form.initial_prompt.push(value);
    }

    pub fn spawn_form_backspace(&mut self) {
        let Some(form) = self.spawn_form.as_mut() else {
            return;
        };

        form.initial_prompt.pop();
    }

    pub fn take_spawn_request(&mut self) -> Option<SpawnRequest> {
        let form = self.spawn_form.take()?;
        let provider = form.providers.get(form.selected_provider)?.to_string();
        let trimmed_prompt = form.initial_prompt.trim();

        let initial_prompt = if trimmed_prompt.is_empty() {
            None
        } else {
            Some(trimmed_prompt.to_string())
        };

        Some(SpawnRequest {
            variant_id: form.variant_id,
            provider,
            initial_prompt,
        })
    }

    pub fn is_chat_visible(&self) -> bool {
        self.chat_visible
    }

    pub fn is_inspector_visible(&self) -> bool {
        self.inspector_visible
    }

    pub fn toggle_inspector_visibility(&mut self) {
        self.inspector_visible = !self.inspector_visible;
        if !self.inspector_visible {
            self.resizing_target = None;
        }
    }

    pub fn toggle_chat_visibility(&mut self) {
        self.chat_visible = !self.chat_visible;

        if !self.chat_visible {
            self.chat_composing = false;
            self.chat_scroll_lines = 0;
            self.close_chat_detail_popup();
            return;
        }

        if self.chat_actor_id.is_some() {
            self.chat_needs_refresh = true;
        }
    }

    pub fn chat_actor(&self) -> Option<&ActorRow> {
        let actor_id = self.chat_actor_id.as_deref()?;
        self.actors.iter().find(|actor| actor.id == actor_id)
    }

    pub fn chat_messages(&self) -> &[ActorChatMessageRow] {
        &self.chat_messages
    }

    pub fn configure_chat_performance(
        &mut self,
        history_limit: u32,
        render_limit: usize,
        max_body_lines: usize,
        message_max_chars: usize,
    ) {
        self.chat_history_limit = history_limit as usize;
        self.chat_render_limit = render_limit.max(1);
        self.chat_max_body_lines = max_body_lines.max(1);
        self.chat_message_max_chars = message_max_chars;

        if self.chat_history_limit > 0 && self.chat_messages.len() > self.chat_history_limit {
            let keep_from = self
                .chat_messages
                .len()
                .saturating_sub(self.chat_history_limit);
            self.chat_messages = self.chat_messages.split_off(keep_from);
        }
    }

    pub fn chat_history_limit_query(&self) -> Option<u32> {
        if self.chat_history_limit == 0 {
            None
        } else {
            Some(self.chat_history_limit.min(u32::MAX as usize) as u32)
        }
    }

    pub fn chat_render_limit(&self) -> usize {
        self.chat_render_limit.max(1)
    }

    pub fn chat_max_body_lines(&self) -> usize {
        self.chat_max_body_lines.max(1)
    }

    pub fn chat_scroll_lines(&self) -> u16 {
        self.chat_scroll_lines
    }

    pub fn scroll_chat_up(&mut self, amount: u16) {
        self.chat_scroll_lines = self.chat_scroll_lines.saturating_sub(amount);
    }

    pub fn scroll_chat_down(&mut self, amount: u16) {
        self.chat_scroll_lines = self.chat_scroll_lines.saturating_add(amount);
    }

    pub fn is_chat_composing(&self) -> bool {
        self.chat_composing
    }

    pub fn is_chat_refresh_in_flight(&self) -> bool {
        self.chat_refresh_in_flight
    }

    pub fn is_chat_send_in_flight(&self) -> bool {
        self.chat_send_in_flight
    }

    pub fn chat_draft(&self) -> &str {
        &self.chat_draft
    }

    pub fn chat_active_model(&self) -> Option<&str> {
        self.chat_selected_model.as_deref()
    }

    pub fn chat_active_agent(&self) -> Option<&str> {
        self.chat_selected_agent.as_deref()
    }

    pub fn chat_model_options(&self) -> &[String] {
        &self.chat_model_options
    }

    pub fn chat_agent_options(&self) -> &[String] {
        &self.chat_agent_options
    }

    pub fn chat_picker_open(&self) -> Option<ChatPickerKind> {
        self.chat_picker_open
    }

    pub fn chat_picker_query(&self) -> &str {
        &self.chat_picker_query
    }

    pub fn chat_picker_items(&self) -> Vec<String> {
        let items = self.current_chat_picker_items();
        if self.chat_picker_query.is_empty() {
            return items.to_vec();
        }

        let needle = self.chat_picker_query.to_ascii_lowercase();
        items
            .iter()
            .filter(|item| item.to_ascii_lowercase().contains(&needle))
            .cloned()
            .collect()
    }

    pub fn chat_picker_selected(&self) -> usize {
        self.chat_picker_selected
    }

    pub fn chat_autocomplete_open(&self) -> bool {
        self.chat_autocomplete_open
    }

    pub fn chat_autocomplete_mode(&self) -> Option<char> {
        self.chat_autocomplete_mode
    }

    pub fn chat_autocomplete_query(&self) -> &str {
        &self.chat_autocomplete_query
    }

    pub fn chat_autocomplete_items(&self) -> &[String] {
        &self.chat_autocomplete_items
    }

    pub fn chat_autocomplete_selected(&self) -> usize {
        self.chat_autocomplete_selected
    }

    pub fn is_chat_detail_popup_open(&self) -> bool {
        self.chat_detail_popup_open
    }

    pub fn open_chat_detail_popup(&mut self) -> bool {
        if self.chat_messages.is_empty() {
            return false;
        }

        self.chat_detail_popup_message_index = self.latest_rich_chat_message_index();
        if self.chat_detail_popup_message_index.is_none() {
            self.chat_detail_popup_message_index = Some(self.chat_messages.len().saturating_sub(1));
        }
        self.chat_detail_popup_open = true;
        self.chat_detail_popup_scroll_lines = 0;
        true
    }

    pub fn open_chat_detail_popup_for_message(&mut self, message_index: usize) -> bool {
        if message_index >= self.chat_messages.len() {
            return false;
        }

        self.chat_detail_popup_message_index = Some(message_index);
        self.chat_detail_popup_open = true;
        self.chat_detail_popup_scroll_lines = 0;
        true
    }

    pub fn close_chat_detail_popup(&mut self) {
        self.chat_detail_popup_open = false;
        self.chat_detail_popup_scroll_lines = 0;
        self.chat_detail_popup_message_index = None;
    }

    pub fn toggle_chat_detail_popup(&mut self) -> bool {
        if self.chat_detail_popup_open {
            self.close_chat_detail_popup();
            false
        } else {
            self.open_chat_detail_popup()
        }
    }

    pub fn chat_detail_popup_scroll_lines(&self) -> u16 {
        self.chat_detail_popup_scroll_lines
    }

    pub fn scroll_chat_detail_popup_up(&mut self, amount: u16) {
        self.chat_detail_popup_scroll_lines =
            self.chat_detail_popup_scroll_lines.saturating_sub(amount);
    }

    pub fn scroll_chat_detail_popup_down(&mut self, amount: u16) {
        self.chat_detail_popup_scroll_lines =
            self.chat_detail_popup_scroll_lines.saturating_add(amount);
    }

    /// Returns the full message row for the detail popup (text + role + timestamp).
    pub fn chat_detail_popup_message(&self) -> Option<&ActorChatMessageRow> {
        if let Some(index) = self.chat_detail_popup_message_index {
            return self.chat_messages.get(index);
        }

        self.latest_rich_chat_message_index()
            .and_then(|index| self.chat_messages.get(index))
            .or_else(|| self.chat_messages.last())
    }

    fn latest_rich_chat_message_index(&self) -> Option<usize> {
        self.chat_messages.iter().rposition(|message| {
            message.text.contains("### Tool //")
                || message.text.contains("Tool //")
                || message.text.contains("### Shell")
                || message.text.contains("Shell ")
                || message.text.contains("### Thinking")
                || message.text.contains("Thinking")
        })
    }

    pub fn open_chat_composer(&mut self) -> bool {
        if self.chat_actor_id.is_none() {
            return false;
        }

        self.chat_visible = true;
        self.chat_needs_refresh = true;
        self.chat_composing = true;
        self.ensure_chat_workspace_file_cache();
        true
    }

    pub fn cancel_chat_composer(&mut self) {
        self.chat_composing = false;
        self.close_chat_picker();
        self.close_chat_autocomplete();
    }

    pub fn commit_sent_chat_prompt(&mut self) {
        self.chat_draft.clear();
        self.chat_composing = false;
        self.close_chat_autocomplete();
    }

    pub fn current_chat_prompt(&self) -> Option<String> {
        if !self.chat_composing {
            return None;
        }

        let trimmed = self.chat_draft.trim();
        if trimmed.is_empty() {
            return None;
        }

        Some(trimmed.to_string())
    }

    pub fn chat_insert_char(&mut self, value: char) {
        if !self.chat_composing {
            return;
        }

        self.chat_draft.push(value);
        self.refresh_chat_autocomplete();
    }

    pub fn chat_backspace(&mut self) {
        if !self.chat_composing {
            return;
        }

        self.chat_draft.pop();
        self.refresh_chat_autocomplete();
    }

    pub fn set_chat_options(&mut self, models: Vec<String>, agents: Vec<String>) {
        self.chat_model_options = normalize_string_options(models);
        self.chat_agent_options = normalize_string_options(agents);

        self.chat_selected_model = resolve_selected_option(
            &self.chat_model_options,
            self.chat_selected_model.as_deref(),
            self.chat_preferred_model.as_deref(),
        );

        self.chat_selected_agent = resolve_selected_option(
            &self.chat_agent_options,
            self.chat_selected_agent.as_deref(),
            self.chat_preferred_agent.as_deref(),
        );
    }

    pub fn open_chat_model_picker(&mut self) {
        if self.chat_model_options.is_empty() {
            return;
        }

        self.chat_picker_open = Some(ChatPickerKind::Model);
        self.chat_picker_query.clear();
        let selected = self
            .chat_selected_model
            .as_deref()
            .and_then(|value| {
                self.chat_model_options
                    .iter()
                    .position(|item| item == value)
            })
            .unwrap_or(0);
        self.chat_picker_selected = selected;
        self.clamp_chat_picker_selection();
    }

    pub fn open_chat_agent_picker(&mut self) {
        if self.chat_agent_options.is_empty() {
            return;
        }

        self.chat_picker_open = Some(ChatPickerKind::Agent);
        self.chat_picker_query.clear();
        let selected = self
            .chat_selected_agent
            .as_deref()
            .and_then(|value| {
                self.chat_agent_options
                    .iter()
                    .position(|item| item == value)
            })
            .unwrap_or(0);
        self.chat_picker_selected = selected;
        self.clamp_chat_picker_selection();
    }

    pub fn close_chat_picker(&mut self) {
        self.chat_picker_open = None;
        self.chat_picker_query.clear();
        self.chat_picker_selected = 0;
    }

    pub fn chat_picker_insert_char(&mut self, value: char) {
        if self.chat_picker_open.is_none() {
            return;
        }

        self.chat_picker_query.push(value);
        self.clamp_chat_picker_selection();
    }

    pub fn chat_picker_backspace(&mut self) {
        if self.chat_picker_open.is_none() {
            return;
        }

        self.chat_picker_query.pop();
        self.clamp_chat_picker_selection();
    }

    pub fn clear_chat_picker_query(&mut self) {
        if self.chat_picker_open.is_none() {
            return;
        }

        self.chat_picker_query.clear();
        self.clamp_chat_picker_selection();
    }

    pub fn chat_picker_move_up(&mut self) {
        let len = self.chat_picker_items().len();
        if len == 0 {
            self.chat_picker_selected = 0;
            return;
        }

        self.chat_picker_selected = previous_index(self.chat_picker_selected, len);
    }

    pub fn chat_picker_move_down(&mut self) {
        let len = self.chat_picker_items().len();
        if len == 0 {
            self.chat_picker_selected = 0;
            return;
        }

        self.chat_picker_selected = next_index(self.chat_picker_selected, len);
    }

    pub fn chat_picker_set_selected(&mut self, index: usize) {
        let len = self.chat_picker_items().len();
        if len == 0 {
            self.chat_picker_selected = 0;
            return;
        }

        self.chat_picker_selected = index.min(len.saturating_sub(1));
    }

    pub fn apply_chat_picker_selection(&mut self) -> Option<String> {
        let index = self.chat_picker_selected;
        let kind = self.chat_picker_open?;
        let selected = self.chat_picker_items().get(index)?.clone();

        match kind {
            ChatPickerKind::Model => {
                self.chat_selected_model = Some(selected.clone());
                self.chat_preferred_model = Some(selected.clone());
            }
            ChatPickerKind::Agent => {
                self.chat_selected_agent = Some(selected.clone());
                self.chat_preferred_agent = Some(selected.clone());
            }
        }

        let _ = self.persist_chat_selection();

        self.close_chat_picker();
        Some(selected)
    }

    pub fn restore_chat_selection_from_disk(&mut self) -> io::Result<bool> {
        let Some(saved) = self.load_chat_selection_from_disk()? else {
            return Ok(false);
        };

        self.chat_preferred_model = saved.model;
        self.chat_preferred_agent = saved.agent;

        self.chat_selected_model = resolve_selected_option(
            &self.chat_model_options,
            self.chat_selected_model.as_deref(),
            self.chat_preferred_model.as_deref(),
        );
        self.chat_selected_agent = resolve_selected_option(
            &self.chat_agent_options,
            self.chat_selected_agent.as_deref(),
            self.chat_preferred_agent.as_deref(),
        );

        Ok(true)
    }

    fn clamp_chat_picker_selection(&mut self) {
        let len = self.chat_picker_items().len();
        if len == 0 {
            self.chat_picker_selected = 0;
            return;
        }

        self.chat_picker_selected = self.chat_picker_selected.min(len.saturating_sub(1));
    }

    pub fn close_chat_autocomplete(&mut self) {
        self.chat_autocomplete_open = false;
        self.chat_autocomplete_mode = None;
        self.chat_autocomplete_query.clear();
        self.chat_autocomplete_selected = 0;
        self.chat_autocomplete_items.clear();
    }

    pub fn chat_autocomplete_move_up(&mut self) {
        let len = self.chat_autocomplete_items.len();
        if len == 0 {
            self.chat_autocomplete_selected = 0;
            return;
        }

        self.chat_autocomplete_selected = previous_index(self.chat_autocomplete_selected, len);
    }

    pub fn chat_autocomplete_move_down(&mut self) {
        let len = self.chat_autocomplete_items.len();
        if len == 0 {
            self.chat_autocomplete_selected = 0;
            return;
        }

        self.chat_autocomplete_selected = next_index(self.chat_autocomplete_selected, len);
    }

    pub fn chat_autocomplete_set_selected(&mut self, index: usize) {
        let len = self.chat_autocomplete_items.len();
        if len == 0 {
            self.chat_autocomplete_selected = 0;
            return;
        }

        self.chat_autocomplete_selected = index.min(len.saturating_sub(1));
    }

    pub fn apply_chat_autocomplete_selection(&mut self) -> Option<String> {
        let selected = self
            .chat_autocomplete_items
            .get(self.chat_autocomplete_selected)
            .cloned()?;

        let token_start = chat_token_start(&self.chat_draft);
        self.chat_draft.truncate(token_start);
        self.chat_draft.push_str(&selected);
        self.close_chat_autocomplete();
        Some(selected)
    }

    fn current_chat_picker_items(&self) -> &[String] {
        match self.chat_picker_open {
            Some(ChatPickerKind::Model) => &self.chat_model_options,
            Some(ChatPickerKind::Agent) => &self.chat_agent_options,
            None => &[],
        }
    }

    fn refresh_chat_autocomplete(&mut self) {
        if !self.chat_composing {
            self.close_chat_autocomplete();
            return;
        }

        let Some((mode, query)) = current_chat_trigger(&self.chat_draft) else {
            self.close_chat_autocomplete();
            return;
        };

        let items = match mode {
            '/' => slash_suggestions(&query),
            '@' => {
                self.ensure_chat_workspace_file_cache();
                file_suggestions(&self.chat_workspace_file_cache, &query)
            }
            _ => Vec::new(),
        };

        if items.is_empty() {
            self.close_chat_autocomplete();
            return;
        }

        self.chat_autocomplete_open = true;
        self.chat_autocomplete_mode = Some(mode);
        self.chat_autocomplete_query = query;
        self.chat_autocomplete_items = items;
        self.chat_autocomplete_selected = self
            .chat_autocomplete_selected
            .min(self.chat_autocomplete_items.len().saturating_sub(1));
    }

    fn ensure_chat_workspace_file_cache(&mut self) {
        if self.chat_workspace_file_cache_loaded {
            return;
        }

        self.chat_workspace_file_cache = collect_workspace_files(&self.directory, 1200, 6);
        self.chat_workspace_file_cache_loaded = true;
    }

    pub fn request_chat_refresh(&mut self) {
        if self.chat_actor_id.is_some() {
            self.chat_needs_refresh = true;
        }
    }

    pub fn set_snapshot_refresh_in_flight(&mut self, in_flight: bool) {
        self.snapshot_refresh_in_flight = in_flight;
    }

    pub fn set_chat_refresh_in_flight(&mut self, in_flight: bool) {
        self.chat_refresh_in_flight = in_flight;
    }

    pub fn set_chat_send_in_flight(&mut self, in_flight: bool) {
        self.chat_send_in_flight = in_flight;
    }

    pub fn set_action_requests_in_flight(&mut self, count: usize) {
        self.action_requests_in_flight = count;
    }

    pub fn has_background_activity(&self) -> bool {
        self.snapshot_refresh_in_flight
            || self.chat_refresh_in_flight
            || self.chat_send_in_flight
            || self.action_requests_in_flight > 0
    }

    pub fn background_activity_label(&self) -> String {
        let mut tags: Vec<String> = Vec::new();

        if self.snapshot_refresh_in_flight {
            tags.push("refresh".to_string());
        }
        if self.chat_refresh_in_flight {
            tags.push("chat:sync".to_string());
        }
        if self.chat_send_in_flight {
            tags.push("chat:send".to_string());
        }
        if self.action_requests_in_flight > 0 {
            tags.push(format!("actions:{}", self.action_requests_in_flight));
        }

        if tags.is_empty() {
            "idle".to_string()
        } else {
            tags.join("+")
        }
    }

    pub fn take_chat_refresh_request(&mut self) -> Option<String> {
        if !self.chat_visible || !self.chat_needs_refresh {
            return None;
        }

        let actor_id = self.chat_actor_id.clone()?;
        self.chat_needs_refresh = false;
        Some(actor_id)
    }

    pub fn apply_chat_messages(&mut self, actor_id: &str, mut messages: Vec<ActorChatMessageRow>) {
        if self.chat_actor_id.as_deref() != Some(actor_id) {
            return;
        }

        let was_scrolled_up = self.chat_scroll_lines > 0;
        let previous_tail = self.chat_messages.last().cloned();

        messages.sort_by(|left, right| left.created_at.cmp(&right.created_at));
        if self.chat_history_limit > 0 && messages.len() > self.chat_history_limit {
            let keep_from = messages.len().saturating_sub(self.chat_history_limit);
            messages = messages.split_off(keep_from);
        }
        if self.chat_message_max_chars > 0 {
            for message in &mut messages {
                if message.text.chars().count() > self.chat_message_max_chars {
                    let clipped = message
                        .text
                        .chars()
                        .take(self.chat_message_max_chars)
                        .collect::<String>();
                    message.text = format!(
                        "{clipped}\n\n... (message truncated at {} chars)",
                        self.chat_message_max_chars
                    );
                }
            }
        }

        if was_scrolled_up {
            let appended_count = previous_tail
                .as_ref()
                .and_then(|tail| {
                    messages
                        .iter()
                        .position(|message| {
                            message.created_at == tail.created_at
                                && message.role == tail.role
                                && message.text == tail.text
                        })
                        .map(|index| messages.len().saturating_sub(index + 1))
                })
                .unwrap_or(0);

            if appended_count > 0 {
                self.chat_scroll_lines = self
                    .chat_scroll_lines
                    .saturating_add(appended_count.min(u16::MAX as usize) as u16);
            }
        }

        self.chat_messages = messages;
        if self.chat_messages.is_empty() {
            self.close_chat_detail_popup();
        } else if let Some(index) = self.chat_detail_popup_message_index {
            let last = self.chat_messages.len().saturating_sub(1);
            self.chat_detail_popup_message_index = Some(index.min(last));
        }

        // Keep scroll offsets bounded so repeated background refreshes cannot
        // accumulate unbounded offsets when messages are pruned.
        let max_scroll = self
            .chat_messages
            .len()
            .saturating_mul(8)
            .min(u16::MAX as usize) as u16;
        self.chat_scroll_lines = self.chat_scroll_lines.min(max_scroll);
    }

    pub fn apply_snapshot(&mut self, snapshot: DashboardSnapshot) {
        let previous_viz_selection = self.viz_selection.clone();
        let previous_product_id = self
            .products
            .get(self.selected_product)
            .map(|row| row.id.clone());
        let previous_variant_id = self.selected_variant().map(|row| row.id.clone());
        let previous_actor_id = self
            .actors
            .get(self.selected_actor)
            .map(|row| row.id.clone());

        self.products = snapshot.products;
        self.variants = snapshot.variants;
        self.actors = snapshot.actors;
        self.runtime_status = snapshot.runtime_status;
        self.last_updated = snapshot.last_updated;

        self.selected_product =
            resolve_index_by_id(&self.products, previous_product_id.as_deref(), |row| {
                row.id.as_str()
            });

        self.selected_actor =
            resolve_index_by_id(&self.actors, previous_actor_id.as_deref(), |row| {
                row.id.as_str()
            });

        self.ensure_variant_selection(previous_variant_id.as_deref());

        self.sync_catalog_selection(
            previous_viz_selection.as_ref(),
            previous_product_id.as_deref(),
            previous_variant_id.as_deref(),
            previous_actor_id.as_deref(),
        );

        self.prune_chat_actor();
        self.prune_actor_last_message_previews();
        if self.chat_visible && self.chat_actor_id.is_some() {
            self.chat_needs_refresh = true;
        }
    }

    pub fn set_status(&mut self, status: impl Into<String>) {
        self.status_message = status.into();
    }

    pub fn set_core_runtime_hint(&mut self, value: impl Into<String>) {
        self.core_runtime_hint = value.into();
    }

    pub fn toggle_core_logs_visibility(&mut self) {
        self.core_logs_visible = !self.core_logs_visible;
    }

    pub fn set_core_logs_snapshot(
        &mut self,
        session: impl Into<String>,
        lines: Vec<String>,
        status: impl Into<String>,
    ) {
        self.core_logs_session = session.into();
        self.core_logs_lines = lines;
        self.core_logs_status = status.into();
    }

    pub fn set_command_message(&mut self, command: impl Into<String>) {
        self.command_message = command.into();
    }

    pub fn focus_next(&mut self) {
        self.focus = self.focus.next();
    }

    pub fn focus_previous(&mut self) {
        self.focus = self.focus.previous();
    }

    pub fn move_selection_down(&mut self) {
        self.viz_select_next();
    }

    pub fn move_selection_up(&mut self) {
        self.viz_select_prev();
    }

    pub fn select_product_by_index(&mut self, product_index: usize) {
        if product_index >= self.products.len() {
            return;
        }

        self.selected_product = product_index;
        self.ensure_variant_selection(None);
        self.focus = FocusPane::Products;
        self.viz_selection = Some(VizSelection::Product { product_index });
    }

    pub fn select_variant_in_product(&mut self, product_index: usize, variant_id: &str) {
        if product_index >= self.products.len() {
            return;
        }

        self.selected_product = product_index;
        self.ensure_variant_selection(Some(variant_id));
        self.focus = FocusPane::Variants;
        self.viz_selection = Some(VizSelection::Variant {
            product_index,
            variant_id: variant_id.to_string(),
        });
    }

    pub fn select_actor_in_viz(&mut self, product_index: usize, variant_id: &str, actor_id: &str) {
        if product_index >= self.products.len() {
            return;
        }

        self.selected_product = product_index;
        self.ensure_variant_selection(Some(variant_id));
        if let Some(idx) = self.actors.iter().position(|a| a.id == actor_id) {
            self.selected_actor = idx;
        }
        self.set_chat_actor(actor_id);
        self.focus = FocusPane::Variants;
        self.viz_selection = Some(VizSelection::Actor {
            product_index,
            variant_id: variant_id.to_string(),
            actor_id: actor_id.to_string(),
        });
    }

    pub fn toggle_variant_filter(&mut self) {
        self.filter_variants_to_product = !self.filter_variants_to_product;
        self.ensure_variant_selection(None);
    }

    pub fn toggle_results_view_mode(&mut self) {
        self.results_view_mode = self.results_view_mode.toggle();
        if self.results_view_mode.is_spatial() {
            // Keep spatial views anchored when switching modes so the
            // baseline layout stays comparable across spatial renders.
            self.reset_viz_offset();
        }
        // Initialize spatial selection when entering spatial modes.
        if self.results_view_mode.is_spatial() && self.viz_selection.is_none() {
            self.sync_viz_selection_from_table();
        }
    }

    pub fn cycle_viz_density(&mut self) {
        self.viz_density = self.viz_density.cycle();
    }

    // --- Viz-mode node selection ---

    pub fn viz_selection(&self) -> Option<&VizSelection> {
        self.viz_selection.as_ref()
    }

    pub fn catalog_nodes(&self) -> Vec<VizSelection> {
        self.viz_node_list()
    }

    /// Set viz selection directly (used by click-select).
    pub fn set_viz_selection(&mut self, selection: VizSelection) {
        // Also sync the table selection state for details panel / actions.
        match &selection {
            VizSelection::Product { product_index } => {
                self.selected_product = *product_index;
                self.ensure_variant_selection(None);
                self.focus = FocusPane::Products;
            }
            VizSelection::Variant {
                product_index,
                variant_id,
            } => {
                self.selected_product = *product_index;
                self.ensure_variant_selection(Some(variant_id));
                self.focus = FocusPane::Variants;
            }
            VizSelection::Actor {
                actor_id,
                product_index,
                variant_id,
            } => {
                self.selected_product = *product_index;
                self.ensure_variant_selection(Some(variant_id));
                // Sync selected_actor to the matching actor.
                if let Some(idx) = self.actors.iter().position(|a| a.id == *actor_id) {
                    self.selected_actor = idx;
                }
                self.set_chat_actor(actor_id);
                self.focus = FocusPane::Variants;
            }
        }
        self.viz_selection = Some(selection);
    }

    /// Build the flattened node list for viz navigation.
    /// Order: for each product, its variants, then each variant's actors.
    fn viz_node_list(&self) -> Vec<VizSelection> {
        let mut nodes = Vec::new();
        for (pi, product) in self.products.iter().enumerate() {
            nodes.push(VizSelection::Product { product_index: pi });
            let variants: Vec<&VariantRow> = self
                .variants
                .iter()
                .filter(|v| v.product_id == product.id)
                .collect();
            for variant in &variants {
                nodes.push(VizSelection::Variant {
                    product_index: pi,
                    variant_id: variant.id.clone(),
                });
                let actors: Vec<&ActorRow> = self
                    .actors
                    .iter()
                    .filter(|a| a.variant_id == variant.id)
                    .collect();
                for actor in &actors {
                    nodes.push(VizSelection::Actor {
                        product_index: pi,
                        variant_id: variant.id.clone(),
                        actor_id: actor.id.clone(),
                    });
                }
            }
        }
        nodes
    }

    /// Move viz selection to next node in the flattened list.
    pub fn viz_select_next(&mut self) {
        let nodes = self.viz_node_list();
        if nodes.is_empty() {
            return;
        }
        let current_pos = self
            .viz_selection
            .as_ref()
            .and_then(|sel| nodes.iter().position(|n| n == sel))
            .unwrap_or(0);
        let next = (current_pos + 1) % nodes.len();
        let sel = nodes[next].clone();
        self.set_viz_selection(sel);
    }

    /// Move viz selection to previous node in the flattened list.
    pub fn viz_select_prev(&mut self) {
        let nodes = self.viz_node_list();
        if nodes.is_empty() {
            return;
        }
        let current_pos = self
            .viz_selection
            .as_ref()
            .and_then(|sel| nodes.iter().position(|n| n == sel))
            .unwrap_or(0);
        let prev = (current_pos + nodes.len() - 1) % nodes.len();
        let sel = nodes[prev].clone();
        self.set_viz_selection(sel);
    }

    /// Sync viz selection from the current table selection state.
    fn sync_viz_selection_from_table(&mut self) {
        if self.products.is_empty() {
            self.viz_selection = None;
            return;
        }
        match self.focus {
            FocusPane::Products => {
                self.viz_selection = Some(VizSelection::Product {
                    product_index: self.selected_product,
                });
            }
            FocusPane::Variants => {
                if let Some(variant) = self.selected_variant() {
                    self.viz_selection = Some(VizSelection::Variant {
                        product_index: self.selected_product,
                        variant_id: variant.id.clone(),
                    });
                } else {
                    self.viz_selection = Some(VizSelection::Product {
                        product_index: self.selected_product,
                    });
                }
            }
        }
    }

    /// Return actors belonging to a specific variant.
    pub fn actors_for_variant(&self, variant_id: &str) -> Vec<&ActorRow> {
        self.actors
            .iter()
            .filter(|a| a.variant_id == variant_id)
            .collect()
    }

    // --- Viz-mode 2D pan / drag ---

    pub fn viz_offset(&self) -> (i32, i32) {
        (self.viz_offset_x, self.viz_offset_y)
    }

    pub fn reset_viz_offset(&mut self) {
        self.viz_offset_x = 0;
        self.viz_offset_y = 0;
    }

    pub fn start_drag(&mut self, col: u16, row: u16) {
        self.drag_anchor = Some(DragAnchor { col, row });
    }

    pub fn end_drag(&mut self) {
        self.drag_anchor = None;
    }

    #[allow(dead_code)]
    pub fn is_dragging(&self) -> bool {
        self.drag_anchor.is_some()
    }

    /// Apply mouse-drag delta: grab-and-pull semantics (content follows cursor).
    pub fn apply_drag(&mut self, col: u16, row: u16) {
        if let Some(anchor) = self.drag_anchor {
            let dx = col as i32 - anchor.col as i32;
            let dy = row as i32 - anchor.row as i32;
            self.viz_offset_x += dx;
            self.viz_offset_y += dy;
            self.drag_anchor = Some(DragAnchor { col, row });
        }
    }

    /// Scroll the viz camera vertically (positive = scroll down = content moves up).
    pub fn viz_scroll(&mut self, delta_y: i32) {
        self.viz_offset_y -= delta_y;
    }

    pub fn selected_product(&self) -> Option<&ProductRow> {
        self.products.get(self.selected_product)
    }

    pub fn selected_variant(&self) -> Option<&VariantRow> {
        let visible = self.visible_variant_indices();
        let global_index = *visible.get(self.selected_variant)?;
        self.variants.get(global_index)
    }

    pub fn selected_actor(&self) -> Option<&ActorRow> {
        self.actors.get(self.selected_actor)
    }

    pub fn selected_variant_id(&self) -> Option<&str> {
        self.selected_variant().map(|row| row.id.as_str())
    }

    pub fn selected_actor_id(&self) -> Option<&str> {
        self.selected_actor().map(|row| row.id.as_str())
    }

    #[allow(dead_code)]
    pub fn visible_variants(&self) -> Vec<&VariantRow> {
        self.visible_variant_indices()
            .into_iter()
            .filter_map(|index| self.variants.get(index))
            .collect()
    }

    #[allow(dead_code)]
    pub fn detail_lines(&self) -> Vec<String> {
        if let Some(sel) = &self.viz_selection {
            return match sel {
                VizSelection::Product { .. } => self.product_detail_lines(),
                VizSelection::Variant { .. } => self.variant_detail_lines(),
                VizSelection::Actor { actor_id, .. } => {
                    if let Some(actor) = self.actors.iter().find(|a| a.id == *actor_id) {
                        self.actor_detail_lines(actor)
                    } else {
                        vec!["No actor selected.".to_string()]
                    }
                }
            };
        }

        match self.focus {
            FocusPane::Products => self.product_detail_lines(),
            FocusPane::Variants => self.variant_detail_lines(),
        }
    }

    #[allow(dead_code)]
    pub fn action_lines(&self) -> Vec<String> {
        let mut lines = vec![
            "Keys:".to_string(),
            "  q / Ctrl+C      [Q]uit".to_string(),
            "  Tab / Shift+Tab [Tab] Focus".to_string(),
            "  arrows          Select".to_string(),
            "  r               [R]efresh".to_string(),
            "  f               [F]ilter variants".to_string(),
            "  space or v      [V]iew toggle".to_string(),
            "  p               [P]oll variant".to_string(),
            "  m               [M] Import actors".to_string(),
            "  g               [G] Move actor".to_string(),
            "  i               [I]nit product".to_string(),
            "  n               [N] Spawn actor".to_string(),
            "  a               [A]ttach command".to_string(),
            "  l               Core [L]ogs toggle".to_string(),
            "  t               [T]oggle chat".to_string(),
            "  s               [S]idebar toggle".to_string(),
            "  c               [C]ompose chat".to_string(),
            "".to_string(),
            "CLI Parity:".to_string(),
        ];

        lines.extend(self.command_examples());

        if !self.command_message.is_empty() {
            lines.push("".to_string());
            lines.push(format!("Last attach cmd: {}", self.command_message));
        }

        lines
    }

    #[allow(dead_code)]
    fn product_detail_lines(&self) -> Vec<String> {
        let Some(product) = self.selected_product() else {
            return vec!["No product selected.".to_string()];
        };

        vec![
            format!("Product: {}", compact_id(&product.id)),
            format!("Name: {}", product.display_name),
            format!("Status: {}", product.status),
            format!("Locator: {}", compact_locator(&product.locator, 58)),
            format!(
                "Variants: total={} dirty={} drift={}",
                product.variant_total, product.variant_dirty, product.variant_drift
            ),
            format!("Repo: {}", product.repo_name),
            format!("Branches: {}", product.branches),
            format!("Updated: {}", product.updated_at),
        ]
    }

    #[allow(dead_code)]
    fn variant_detail_lines(&self) -> Vec<String> {
        let Some(variant) = self.selected_variant() else {
            return vec!["No variant selected.".to_string()];
        };

        vec![
            format!("Variant: {}", compact_id(&variant.id)),
            format!("Product: {}", compact_id(&variant.product_id)),
            format!("Name: {}", variant.name),
            format!("Git state: {}", variant.git_state),
            format!("Ahead/Behind: {}/{}", variant.ahead, variant.behind),
            format!("Branch: {}", variant.branch),
            format!("Worktree: {}", variant.worktree),
            format!("Locator: {}", compact_locator(&variant.locator, 58)),
            format!(
                "Last polled: {}",
                compact_timestamp(&variant.last_polled_at)
            ),
            format!("Updated: {}", variant.updated_at),
        ]
    }

    #[allow(dead_code)]
    fn actor_detail_lines(&self, actor: &ActorRow) -> Vec<String> {
        vec![
            format!("Actor: {}", compact_id(&actor.id)),
            format!("Title: {}", actor.title),
            format!("Description: {}", compact_locator(&actor.description, 58)),
            format!("Provider: {}", actor.provider),
            format!("Status: {}", actor.status),
            format!("Variant: {}", compact_id(&actor.variant_id)),
            format!("Dir: {}", compact_locator(&actor.directory, 58)),
            format!("Created: {}", actor.created_at),
            format!("Updated: {}", actor.updated_at),
        ]
    }

    #[allow(dead_code)]
    pub fn command_examples(&self) -> Vec<String> {
        let mut commands = vec![
            "  dark_cli products list".to_string(),
            "  dark_cli variants list --poll=true".to_string(),
            "  dark_cli actors list --provider mock".to_string(),
        ];

        if let Some(product) = self.selected_product() {
            commands.push(format!("  dark_cli products get --id {}", product.id));
        }

        if let Some(variant) = self.selected_variant() {
            commands.push(format!("  dark_cli variants poll --id {}", variant.id));
            commands.push(format!(
                "  dark_cli variants import-actors --id {} --provider opencode/server",
                variant.id
            ));
        }

        if let Some(actor) = self.selected_actor() {
            commands.push(format!("  dark_cli actors attach --id {}", actor.id));
        }

        commands.into_iter().take(6).collect()
    }

    fn visible_variant_indices(&self) -> Vec<usize> {
        if !self.filter_variants_to_product {
            return (0..self.variants.len()).collect();
        }

        let Some(product_id) = self.selected_product().map(|product| product.id.as_str()) else {
            return (0..self.variants.len()).collect();
        };

        self.variants
            .iter()
            .enumerate()
            .filter_map(|(index, variant)| {
                if variant.product_id == product_id {
                    Some(index)
                } else {
                    None
                }
            })
            .collect()
    }

    fn ensure_variant_selection(&mut self, preferred_variant_id: Option<&str>) {
        let visible = self.visible_variant_indices();

        if visible.is_empty() {
            self.selected_variant = 0;
            return;
        }

        if let Some(variant_id) = preferred_variant_id {
            if let Some(position) = visible
                .iter()
                .position(|index| self.variants[*index].id == variant_id)
            {
                self.selected_variant = position;
                return;
            }
        }

        let max_index = visible.len().saturating_sub(1);
        self.selected_variant = self.selected_variant.min(max_index);
    }

    fn set_chat_actor(&mut self, actor_id: &str) {
        let changed = self.chat_actor_id.as_deref() != Some(actor_id);
        self.chat_actor_id = Some(actor_id.to_string());
        if self.chat_visible {
            self.chat_needs_refresh = true;
        }

        if changed {
            self.chat_messages.clear();
            self.chat_scroll_lines = 0;
            self.chat_draft.clear();
            self.chat_composing = false;
            self.close_chat_detail_popup();
        }
    }

    fn prune_chat_actor(&mut self) {
        let Some(chat_actor_id) = self.chat_actor_id.as_deref() else {
            return;
        };

        if self.actors.iter().any(|actor| actor.id == chat_actor_id) {
            return;
        }

        self.chat_actor_id = None;
        self.chat_messages.clear();
        self.chat_scroll_lines = 0;
        self.chat_draft.clear();
        self.chat_composing = false;
        self.close_chat_detail_popup();
        self.chat_needs_refresh = false;
    }

    fn sync_catalog_selection(
        &mut self,
        previous_viz_selection: Option<&VizSelection>,
        previous_product_id: Option<&str>,
        previous_variant_id: Option<&str>,
        previous_actor_id: Option<&str>,
    ) {
        if self.products.is_empty() {
            self.viz_selection = None;
            return;
        }

        if let Some(selection) = previous_viz_selection {
            match selection {
                VizSelection::Product { .. } => {
                    if let Some(product_id) = previous_product_id {
                        if let Some(product_index) =
                            self.products.iter().position(|row| row.id == product_id)
                        {
                            self.set_viz_selection(VizSelection::Product { product_index });
                            return;
                        }
                    }
                }
                VizSelection::Variant { variant_id, .. } => {
                    if let Some((product_index, variant_id)) = self
                        .variants
                        .iter()
                        .find(|variant| variant.id == *variant_id)
                        .and_then(|variant| {
                            self.products
                                .iter()
                                .position(|product| product.id == variant.product_id)
                                .map(|product_index| (product_index, variant.id.clone()))
                        })
                    {
                        self.set_viz_selection(VizSelection::Variant {
                            product_index,
                            variant_id,
                        });
                        return;
                    }
                }
                VizSelection::Actor { actor_id, .. } => {
                    if let Some(actor) = self.actors.iter().find(|actor| actor.id == *actor_id) {
                        if let Some((product_index, variant_id)) = self
                            .variants
                            .iter()
                            .find(|variant| variant.id == actor.variant_id)
                            .and_then(|variant| {
                                self.products
                                    .iter()
                                    .position(|product| product.id == variant.product_id)
                                    .map(|product_index| (product_index, variant.id.clone()))
                            })
                        {
                            self.set_viz_selection(VizSelection::Actor {
                                product_index,
                                variant_id,
                                actor_id: actor_id.clone(),
                            });
                            return;
                        }
                    }
                }
            }
        }

        if let Some(actor_id) = previous_actor_id {
            if let Some(actor) = self.actors.iter().find(|row| row.id == actor_id) {
                if let Some((product_index, variant_id)) = self
                    .variants
                    .iter()
                    .find(|variant| variant.id == actor.variant_id)
                    .and_then(|variant| {
                        self.products
                            .iter()
                            .position(|product| product.id == variant.product_id)
                            .map(|product_index| (product_index, variant.id.clone()))
                    })
                {
                    self.set_viz_selection(VizSelection::Actor {
                        product_index,
                        variant_id,
                        actor_id: actor.id.clone(),
                    });
                    return;
                }
            }
        }

        if let Some(variant_id) = previous_variant_id {
            if let Some((product_index, variant_id)) = self
                .variants
                .iter()
                .find(|variant| variant.id == variant_id)
                .and_then(|variant| {
                    self.products
                        .iter()
                        .position(|product| product.id == variant.product_id)
                        .map(|product_index| (product_index, variant.id.clone()))
                })
            {
                self.set_viz_selection(VizSelection::Variant {
                    product_index,
                    variant_id,
                });
                return;
            }
        }

        if let Some(product_id) = previous_product_id {
            if let Some(product_index) = self.products.iter().position(|row| row.id == product_id) {
                self.set_viz_selection(VizSelection::Product { product_index });
                return;
            }
        }

        self.set_viz_selection(VizSelection::Product {
            product_index: self.selected_product,
        });
    }

    fn persist_chat_selection(&self) -> io::Result<()> {
        let parent = self
            .chat_preferences_path
            .parent()
            .ok_or_else(|| io::Error::other("missing darktui.toml parent"))?;
        fs::create_dir_all(parent)?;

        let payload = PersistedChatSelection {
            model: self.chat_selected_model.clone(),
            agent: self.chat_selected_agent.clone(),
        };
        let encoded = toml::to_string_pretty(&payload)
            .map_err(|error| io::Error::other(error.to_string()))?;
        fs::write(&self.chat_preferences_path, encoded)
    }

    fn load_chat_selection_from_disk(&self) -> io::Result<Option<PersistedChatSelection>> {
        if !self.chat_preferences_path.exists() {
            return Ok(None);
        }

        let raw = fs::read_to_string(&self.chat_preferences_path)?;
        let decoded = toml::from_str::<PersistedChatSelection>(&raw)
            .map_err(|error| io::Error::other(error.to_string()))?;
        Ok(Some(decoded))
    }
}

fn resolve_index_by_id<T>(rows: &[T], id: Option<&str>, id_accessor: impl Fn(&T) -> &str) -> usize {
    if rows.is_empty() {
        return 0;
    }

    let Some(id) = id else {
        return 0;
    };

    rows.iter()
        .position(|row| id_accessor(row) == id)
        .unwrap_or_default()
}

fn normalize_string_options(mut values: Vec<String>) -> Vec<String> {
    values.retain(|value| !value.trim().is_empty());
    values.sort();
    values.dedup();
    values
}

fn normalize_optional_input(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn branch_suggestions_for(form: &BranchFormState) -> Vec<&str> {
    let query = form.branch_name.trim().to_ascii_lowercase();

    let mut scored = form
        .suggestions
        .iter()
        .map(String::as_str)
        .filter_map(|value| fuzzy_branch_score(&query, value).map(|score| (value, score)))
        .collect::<Vec<_>>();

    scored.sort_by(|left, right| {
        left.1.cmp(&right.1).then_with(|| {
            left.0
                .to_ascii_lowercase()
                .cmp(&right.0.to_ascii_lowercase())
        })
    });

    scored.into_iter().take(8).map(|entry| entry.0).collect()
}

fn fuzzy_branch_score(query: &str, candidate: &str) -> Option<(u8, usize)> {
    if query.is_empty() {
        return Some((0, 0));
    }

    let value = candidate.to_ascii_lowercase();
    if value.starts_with(query) {
        return Some((0, value.len().saturating_sub(query.len())));
    }

    if let Some(position) = value.find(query) {
        return Some((1, position));
    }

    if is_subsequence(query, &value) {
        return Some((2, value.len()));
    }

    None
}

fn is_subsequence(query: &str, candidate: &str) -> bool {
    let mut query_chars = query.chars();
    let mut current = query_chars.next();

    for ch in candidate.chars() {
        if Some(ch) == current {
            current = query_chars.next();
            if current.is_none() {
                return true;
            }
        }
    }

    false
}

fn resolve_selected_option(
    options: &[String],
    current: Option<&str>,
    preferred: Option<&str>,
) -> Option<String> {
    if options.is_empty() {
        return None;
    }

    if let Some(preferred) = preferred {
        if options.iter().any(|item| item == preferred) {
            return Some(preferred.to_string());
        }
    }

    if let Some(current) = current {
        if options.iter().any(|item| item == current) {
            return Some(current.to_string());
        }
    }

    options.first().cloned()
}

fn current_chat_trigger(value: &str) -> Option<(char, String)> {
    let token_start = chat_token_start(value);
    let token = &value[token_start..];

    if let Some(query) = token.strip_prefix('/') {
        return Some(('/', query.to_string()));
    }

    if let Some(query) = token.strip_prefix('@') {
        return Some(('@', query.to_string()));
    }

    None
}

fn chat_token_start(value: &str) -> usize {
    value
        .rfind(|ch: char| ch.is_whitespace())
        .map_or(0, |index| index + 1)
}

fn slash_suggestions(query: &str) -> Vec<String> {
    const COMMANDS: [&str; 8] = [
        "/help",
        "/refresh",
        "/new",
        "/clear",
        "/sessions",
        "/agent ",
        "/model ",
        "/grep ",
    ];

    let needle = query.to_ascii_lowercase();
    COMMANDS
        .into_iter()
        .filter(|command| {
            needle.is_empty()
                || command
                    .trim_start_matches('/')
                    .to_ascii_lowercase()
                    .contains(&needle)
        })
        .map(ToString::to_string)
        .collect()
}

fn file_suggestions(paths: &[String], query: &str) -> Vec<String> {
    let needle = query.to_ascii_lowercase();
    paths
        .iter()
        .filter(|path| needle.is_empty() || path.to_ascii_lowercase().contains(&needle))
        .take(60)
        .map(|path| format!("@{path}"))
        .collect()
}

fn collect_workspace_files(root: &str, limit: usize, max_depth: usize) -> Vec<String> {
    fn walk(
        root: &std::path::Path,
        current: &std::path::Path,
        depth: usize,
        max_depth: usize,
        limit: usize,
        output: &mut Vec<String>,
    ) {
        if output.len() >= limit || depth > max_depth {
            return;
        }

        let Ok(entries) = std::fs::read_dir(current) else {
            return;
        };

        for entry in entries.flatten() {
            if output.len() >= limit {
                break;
            }

            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.')
                || matches!(name.as_str(), "target" | "node_modules" | "generated")
            {
                continue;
            }

            if path.is_dir() {
                walk(root, &path, depth + 1, max_depth, limit, output);
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

    let mut output = Vec::new();
    let root_path = std::path::Path::new(root);
    walk(root_path, root_path, 0, max_depth, limit, &mut output);
    output.sort();
    output
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn refresh_keeps_product_selection_after_actor_was_selected() {
        let mut app = App::new(".".to_string(), 5, Theme::default());
        app.apply_snapshot(snapshot());

        app.select_actor_in_viz(0, "var_1", "act_1");
        app.select_product_by_index(1);
        app.apply_snapshot(snapshot());

        assert!(matches!(
            app.viz_selection(),
            Some(VizSelection::Product { product_index: 1 })
        ));
    }

    #[test]
    fn refresh_keeps_variant_selection_after_actor_was_selected() {
        let mut app = App::new(".".to_string(), 5, Theme::default());
        app.apply_snapshot(snapshot());

        app.select_actor_in_viz(0, "var_1", "act_1");
        app.select_variant_in_product(1, "var_2");
        app.apply_snapshot(snapshot());

        assert!(matches!(
            app.viz_selection(),
            Some(VizSelection::Variant {
                product_index: 1,
                variant_id
            }) if variant_id == "var_2"
        ));
    }

    #[test]
    fn spawn_request_uses_opened_variant_target() {
        let mut app = App::new(".".to_string(), 5, Theme::default());
        app.open_spawn_form("var_2", vec!["mock".to_string()], Some("mock"));
        app.spawn_form_insert_char(' ');
        app.spawn_form_insert_char('h');
        app.spawn_form_insert_char('i');
        app.spawn_form_insert_char(' ');

        let request = app
            .take_spawn_request()
            .expect("spawn request should exist");
        assert_eq!(request.variant_id, "var_2");
        assert_eq!(request.provider, "mock");
        assert_eq!(request.initial_prompt.as_deref(), Some("hi"));
    }

    fn snapshot() -> DashboardSnapshot {
        DashboardSnapshot {
            products: vec![product("prd_1"), product("prd_2")],
            variants: vec![variant("var_1", "prd_1"), variant("var_2", "prd_2")],
            actors: vec![actor("act_1", "var_1")],
            runtime_status: "ok".to_string(),
            last_updated: "unix:1".to_string(),
        }
    }

    fn product(id: &str) -> ProductRow {
        ProductRow {
            id: id.to_string(),
            display_name: id.to_string(),
            locator: format!("@local:///{id}"),
            workspace_locator: format!("@local:///{id}"),
            product_type: "local".to_string(),
            is_git_repo: false,
            branch: "main".to_string(),
            branches: "main".to_string(),
            repo_name: id.to_string(),
            updated_at: "unix:1".to_string(),
            status: "ok".to_string(),
            variant_total: 1,
            variant_dirty: 0,
            variant_drift: 0,
        }
    }

    fn variant(id: &str, product_id: &str) -> VariantRow {
        VariantRow {
            id: id.to_string(),
            product_id: product_id.to_string(),
            locator: format!("@local:///tmp/{id}"),
            name: "default".to_string(),
            branch: "main".to_string(),
            git_state: "clean".to_string(),
            clone_status: "-".to_string(),
            clone_last_line: "-".to_string(),
            has_git: true,
            is_dirty: false,
            ahead: 0,
            behind: 0,
            worktree: "main".to_string(),
            last_polled_at: "unix:1".to_string(),
            updated_at: "unix:1".to_string(),
        }
    }

    fn actor(id: &str, variant_id: &str) -> ActorRow {
        ActorRow {
            id: id.to_string(),
            variant_id: variant_id.to_string(),
            title: id.to_string(),
            description: id.to_string(),
            provider: "mock".to_string(),
            provider_session_id: None,
            status: "running".to_string(),
            directory: format!("/tmp/{id}"),
            connection_info: json!({}),
            created_at: "unix:1".to_string(),
            updated_at: "unix:1".to_string(),
            sub_agents: vec![],
        }
    }
}
