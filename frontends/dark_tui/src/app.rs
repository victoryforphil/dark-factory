use crate::models::{
    compact_id, compact_locator, compact_timestamp, ActorRow, DashboardSnapshot, ProductRow,
    VariantRow,
};
use crate::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusPane {
    Products,
    Variants,
    Sessions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultsViewMode {
    Table,
    Viz,
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
            Self::Variants => Self::Sessions,
            Self::Sessions => Self::Products,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            Self::Products => Self::Sessions,
            Self::Variants => Self::Products,
            Self::Sessions => Self::Variants,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Products => "products",
            Self::Variants => "variants",
            Self::Sessions => "actors",
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
            status_message: "Booting dashboard".to_string(),
            command_message: String::new(),
            runtime_status: "unknown".to_string(),
            last_updated: "-".to_string(),
            viz_offset_x: 0,
            viz_offset_y: 0,
            drag_anchor: None,
            theme,
        }
    }

    pub fn refresh_seconds(&self) -> u64 {
        self.refresh_seconds
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

    pub fn selected_product_index(&self) -> usize {
        self.selected_product
    }

    pub fn selected_variant_index(&self) -> usize {
        self.selected_variant
    }

    pub fn selected_actor_index(&self) -> usize {
        self.selected_actor
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
        match self.focus {
            FocusPane::Products => {
                self.selected_product = next_index(self.selected_product, self.products.len());
                self.ensure_variant_selection(None);
            }
            FocusPane::Variants => {
                let len = self.visible_variant_indices().len();
                self.selected_variant = next_index(self.selected_variant, len);
            }
            FocusPane::Sessions => {
                self.selected_actor = next_index(self.selected_actor, self.actors.len());
            }
        }
    }

    pub fn move_selection_up(&mut self) {
        match self.focus {
            FocusPane::Products => {
                self.selected_product = previous_index(self.selected_product, self.products.len());
                self.ensure_variant_selection(None);
            }
            FocusPane::Variants => {
                let len = self.visible_variant_indices().len();
                self.selected_variant = previous_index(self.selected_variant, len);
            }
            FocusPane::Sessions => {
                self.selected_actor = previous_index(self.selected_actor, self.actors.len());
            }
        }
    }

    pub fn toggle_variant_filter(&mut self) {
        self.filter_variants_to_product = !self.filter_variants_to_product;
        self.ensure_variant_selection(None);
    }

    pub fn toggle_results_view_mode(&mut self) {
        self.results_view_mode = self.results_view_mode.toggle();
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

    pub fn visible_variants(&self) -> Vec<&VariantRow> {
        self.visible_variant_indices()
            .into_iter()
            .filter_map(|index| self.variants.get(index))
            .collect()
    }

    pub fn detail_lines(&self) -> Vec<String> {
        match self.focus {
            FocusPane::Products => self.product_detail_lines(),
            FocusPane::Variants => self.variant_detail_lines(),
            FocusPane::Sessions => self.session_detail_lines(),
        }
    }

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
            "  i             Init product from directory".to_string(),
            "  n             Spawn mock actor".to_string(),
            "  a             Build attach command".to_string(),
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

    fn session_detail_lines(&self) -> Vec<String> {
        let Some(actor) = self.selected_actor() else {
            return vec![
                "No actor selected.".to_string(),
                format!("Directory: {}", self.directory),
                format!("Runtime: {}", self.runtime_status),
            ];
        };

        vec![
            format!("Actor: {}", compact_id(&actor.id)),
            format!("Title: {}", actor.title),
            format!("Provider: {}", actor.provider),
            format!("Status: {}", actor.status),
            format!("Directory: {}", compact_locator(&actor.directory, 58)),
            format!("Created: {}", actor.created_at),
            format!("Updated: {}", actor.updated_at),
            format!("Runtime: {}", self.runtime_status),
        ]
    }

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
