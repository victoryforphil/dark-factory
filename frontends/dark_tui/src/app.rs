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
    pub provider: String,
    pub initial_prompt: Option<String>,
}

#[derive(Debug, Clone)]
struct SpawnFormState {
    providers: Vec<String>,
    selected_provider: usize,
    initial_prompt: String,
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
            Self::Viz => "viz",
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

#[derive(Debug)]
pub struct App {
    directory: String,
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
    command_message: String,
    runtime_status: String,
    last_updated: String,
    /// Viz-mode camera pan offset (pixels = terminal cells).
    viz_offset_x: i32,
    viz_offset_y: i32,
    /// Active drag anchor (set on mouse-down, cleared on mouse-up).
    drag_anchor: Option<DragAnchor>,
    /// Color theme — loaded once at startup.
    theme: Theme,
    spawn_form: Option<SpawnFormState>,
    chat_visible: bool,
    chat_actor_id: Option<String>,
    chat_messages: Vec<ActorChatMessageRow>,
    chat_draft: String,
    chat_composing: bool,
    chat_needs_refresh: bool,
    snapshot_refresh_in_flight: bool,
    chat_refresh_in_flight: bool,
    chat_send_in_flight: bool,
    action_requests_in_flight: usize,
}

impl App {
    pub fn new(directory: String, refresh_seconds: u64, theme: Theme) -> Self {
        Self {
            directory,
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
            command_message: String::new(),
            runtime_status: "unknown".to_string(),
            last_updated: "-".to_string(),
            viz_offset_x: 0,
            viz_offset_y: 0,
            drag_anchor: None,
            theme,
            spawn_form: None,
            chat_visible: false,
            chat_actor_id: None,
            chat_messages: Vec::new(),
            chat_draft: String::new(),
            chat_composing: false,
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

    #[allow(dead_code)]
    pub fn last_updated(&self) -> &str {
        &self.last_updated
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
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

    pub fn open_spawn_form(&mut self, mut providers: Vec<String>, default_provider: Option<&str>) {
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
            providers,
            selected_provider,
            initial_prompt: String::new(),
        });
    }

    pub fn close_spawn_form(&mut self) {
        self.spawn_form = None;
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
            provider,
            initial_prompt,
        })
    }

    pub fn is_chat_visible(&self) -> bool {
        self.chat_visible
    }

    pub fn toggle_chat_visibility(&mut self) {
        self.chat_visible = !self.chat_visible;

        if !self.chat_visible {
            self.chat_composing = false;
            return;
        }

        if self.chat_actor_id.is_some() {
            self.chat_needs_refresh = true;
        }
    }

    pub fn chat_actor_id(&self) -> Option<&str> {
        self.chat_actor_id.as_deref()
    }

    pub fn chat_actor(&self) -> Option<&ActorRow> {
        let actor_id = self.chat_actor_id.as_deref()?;
        self.actors.iter().find(|actor| actor.id == actor_id)
    }

    pub fn chat_messages(&self) -> &[ActorChatMessageRow] {
        &self.chat_messages
    }

    pub fn is_chat_composing(&self) -> bool {
        self.chat_composing
    }

    pub fn chat_draft(&self) -> &str {
        &self.chat_draft
    }

    pub fn open_chat_composer(&mut self) -> bool {
        if self.chat_actor_id.is_none() {
            return false;
        }

        self.chat_visible = true;
        self.chat_needs_refresh = true;
        self.chat_composing = true;
        true
    }

    pub fn cancel_chat_composer(&mut self) {
        self.chat_composing = false;
    }

    pub fn commit_sent_chat_prompt(&mut self) {
        self.chat_draft.clear();
        self.chat_composing = false;
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
    }

    pub fn chat_backspace(&mut self) {
        if !self.chat_composing {
            return;
        }

        self.chat_draft.pop();
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

        messages.sort_by(|left, right| left.created_at.cmp(&right.created_at));
        self.chat_messages = messages;
    }

    pub fn apply_snapshot(&mut self, snapshot: DashboardSnapshot) {
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
            previous_product_id.as_deref(),
            previous_variant_id.as_deref(),
            previous_actor_id.as_deref(),
        );

        self.prune_chat_actor();
        if self.chat_visible && self.chat_actor_id.is_some() {
            self.chat_needs_refresh = true;
        }
    }

    pub fn set_status(&mut self, status: impl Into<String>) {
        self.status_message = status.into();
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
        // Initialize viz selection when entering viz mode.
        if self.results_view_mode == ResultsViewMode::Viz && self.viz_selection.is_none() {
            self.sync_viz_selection_from_table();
        }
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
                self.chat_composing = false;
            }
            VizSelection::Variant {
                product_index,
                variant_id,
            } => {
                self.selected_product = *product_index;
                self.ensure_variant_selection(Some(variant_id));
                self.focus = FocusPane::Variants;
                self.chat_composing = false;
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
            "  q / Ctrl+C    Quit".to_string(),
            "  Tab / Shift+Tab  Switch focus".to_string(),
            "  j/k or arrows    Move selection".to_string(),
            "  r             Refresh now".to_string(),
            "  f             Toggle variant filter".to_string(),
            "  space or v    Toggle table/viz mode".to_string(),
            "  p             Poll selected variant".to_string(),
            "  m             Import active actors".to_string(),
            "  i             Init product from directory".to_string(),
            "  n             Spawn actor".to_string(),
            "  a             Build attach command".to_string(),
            "  t             Toggle chat panel".to_string(),
            "  c             Compose chat prompt".to_string(),
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
            format!("Branch: {}", product.branch),
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
        self.chat_visible = true;
        self.chat_needs_refresh = true;

        if changed {
            self.chat_messages.clear();
            self.chat_draft.clear();
            self.chat_composing = false;
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
        self.chat_draft.clear();
        self.chat_composing = false;
        self.chat_needs_refresh = false;
    }

    fn sync_catalog_selection(
        &mut self,
        previous_product_id: Option<&str>,
        previous_variant_id: Option<&str>,
        previous_actor_id: Option<&str>,
    ) {
        if self.products.is_empty() {
            self.viz_selection = None;
            return;
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
