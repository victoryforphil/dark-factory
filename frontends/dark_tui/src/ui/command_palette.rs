use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, FocusPane, ResultsViewMode, VizSelection};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CommandId {
    Quit,
    ToggleFocus,
    MoveDown,
    MoveUp,
    Refresh,
    ToggleFilter,
    ToggleView,
    ToggleInspector,
    PollVariant,
    PollActor,
    OpenMoveActorForm,
    OpenCloneForm,
    OpenDeleteVariantForm,
    ImportVariantActors,
    InitProduct,
    OpenSpawnForm,
    BuildAttach,
    ToggleChat,
    OpenChatCompose,
    ResetPan,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct CommandBinding {
    pub(crate) id: CommandId,
    pub(crate) key: &'static str,
    pub(crate) label: &'static str,
}

#[derive(Debug, Clone)]
pub(crate) struct ContextMenuState {
    pub(crate) anchor_col: u16,
    pub(crate) anchor_row: u16,
    pub(crate) target: VizSelection,
    pub(crate) entries: Vec<CommandBinding>,
    pub(crate) selected: usize,
}

#[allow(dead_code)]
const TOOLBAR_COMMON_COMMANDS: &[CommandBinding] = &[
    CommandBinding {
        id: CommandId::Quit,
        key: "q",
        label: "[Q]uit",
    },
    CommandBinding {
        id: CommandId::ToggleFocus,
        key: "Tab",
        label: "[Tab] Focus",
    },
    CommandBinding {
        id: CommandId::MoveDown,
        key: "j/k",
        label: "[J/K] Select",
    },
    CommandBinding {
        id: CommandId::Refresh,
        key: "r",
        label: "[R]efresh",
    },
    CommandBinding {
        id: CommandId::ToggleView,
        key: "v",
        label: "[V]iew",
    },
    CommandBinding {
        id: CommandId::ToggleFilter,
        key: "f",
        label: "[F]ilter",
    },
    CommandBinding {
        id: CommandId::ToggleInspector,
        key: "s",
        label: "[S]idebar",
    },
    CommandBinding {
        id: CommandId::ToggleChat,
        key: "t",
        label: "[T]oggle chat",
    },
];

const TOOLBAR_PRODUCT_COMMANDS: &[CommandBinding] = &[
    CommandBinding {
        id: CommandId::InitProduct,
        key: "i",
        label: "[I]nit product",
    },
    CommandBinding {
        id: CommandId::OpenCloneForm,
        key: "x",
        label: "[X] Clone variant",
    },
];

const TOOLBAR_VARIANT_COMMANDS: &[CommandBinding] = &[
    CommandBinding {
        id: CommandId::PollVariant,
        key: "p",
        label: "[P]oll variant",
    },
    CommandBinding {
        id: CommandId::ImportVariantActors,
        key: "m",
        label: "[M] Import actors",
    },
    CommandBinding {
        id: CommandId::OpenDeleteVariantForm,
        key: "d",
        label: "[D]elete variant",
    },
    CommandBinding {
        id: CommandId::OpenSpawnForm,
        key: "n",
        label: "[N] Spawn actor",
    },
];

const TOOLBAR_ACTOR_COMMANDS: &[CommandBinding] = &[
    CommandBinding {
        id: CommandId::PollActor,
        key: "o",
        label: "[O] Poll actor",
    },
    CommandBinding {
        id: CommandId::OpenMoveActorForm,
        key: "g",
        label: "[G] Move actor",
    },
    CommandBinding {
        id: CommandId::BuildAttach,
        key: "a",
        label: "[A]ttach",
    },
    CommandBinding {
        id: CommandId::OpenChatCompose,
        key: "c",
        label: "[C]ompose",
    },
    CommandBinding {
        id: CommandId::OpenSpawnForm,
        key: "n",
        label: "New actor",
    },
];

const TOOLBAR_VIZ_COMMANDS: &[CommandBinding] = &[CommandBinding {
    id: CommandId::ResetPan,
    key: "0",
    label: "[0] Reset pan",
}];

pub(crate) fn toolbar_bindings(app: &App) -> Vec<CommandBinding> {
    let mut commands: Vec<CommandBinding> = TOOLBAR_COMMON_COMMANDS.to_vec();

    if app.results_view_mode() == ResultsViewMode::Viz {
        commands.extend_from_slice(TOOLBAR_VIZ_COMMANDS);
    }

    match toolbar_selection_context(app) {
        ToolbarSelectionContext::Product => commands.extend_from_slice(TOOLBAR_PRODUCT_COMMANDS),
        ToolbarSelectionContext::Variant => commands.extend_from_slice(TOOLBAR_VARIANT_COMMANDS),
        ToolbarSelectionContext::Actor => commands.extend_from_slice(TOOLBAR_ACTOR_COMMANDS),
    }

    let mut deduped: Vec<CommandBinding> = Vec::new();
    for binding in commands {
        if deduped.iter().any(|existing| existing.id == binding.id) {
            continue;
        }

        if is_command_enabled(app, binding.id) {
            deduped.push(binding);
        }
    }

    deduped
}

pub(crate) fn resolve_key_command(app: &App, key: KeyEvent) -> Option<CommandId> {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Some(CommandId::Quit);
    }

    let command = match key.code {
        KeyCode::Char('q') | KeyCode::Esc => CommandId::Quit,
        KeyCode::Tab | KeyCode::BackTab => CommandId::ToggleFocus,
        KeyCode::Down | KeyCode::Char('j') => CommandId::MoveDown,
        KeyCode::Up | KeyCode::Char('k') => CommandId::MoveUp,
        KeyCode::Char('r') => CommandId::Refresh,
        KeyCode::Char('f') => CommandId::ToggleFilter,
        KeyCode::Char('s') | KeyCode::Char('b') => CommandId::ToggleInspector,
        KeyCode::Char('v') | KeyCode::Char(' ') => CommandId::ToggleView,
        KeyCode::Char('p') => CommandId::PollVariant,
        KeyCode::Char('o') => CommandId::PollActor,
        KeyCode::Char('g') => CommandId::OpenMoveActorForm,
        KeyCode::Char('x') => CommandId::OpenCloneForm,
        KeyCode::Char('d') => CommandId::OpenDeleteVariantForm,
        KeyCode::Char('m') => CommandId::ImportVariantActors,
        KeyCode::Char('i') => CommandId::InitProduct,
        KeyCode::Char('n') => CommandId::OpenSpawnForm,
        KeyCode::Char('a') => CommandId::BuildAttach,
        KeyCode::Char('t') => CommandId::ToggleChat,
        KeyCode::Char('c') => CommandId::OpenChatCompose,
        KeyCode::Char('0') => CommandId::ResetPan,
        _ => return None,
    };

    if is_command_enabled(app, command) {
        Some(command)
    } else {
        None
    }
}

pub(crate) fn is_command_enabled(app: &App, command: CommandId) -> bool {
    match command {
        CommandId::Quit
        | CommandId::ToggleFocus
        | CommandId::MoveDown
        | CommandId::MoveUp
        | CommandId::Refresh
        | CommandId::ToggleFilter
        | CommandId::ToggleInspector
        | CommandId::ToggleView
        | CommandId::InitProduct
        | CommandId::ToggleChat => true,
        CommandId::ResetPan => app.results_view_mode() == ResultsViewMode::Viz,
        CommandId::PollVariant
        | CommandId::OpenDeleteVariantForm
        | CommandId::ImportVariantActors
        | CommandId::OpenSpawnForm => app.selected_variant_id().is_some(),
        CommandId::PollActor | CommandId::OpenMoveActorForm => app.selected_actor_id().is_some(),
        CommandId::OpenCloneForm => app.selected_product().is_some(),
        CommandId::BuildAttach | CommandId::OpenChatCompose => app.selected_actor_id().is_some(),
    }
}

pub(crate) fn context_menu_commands(app: &App, target: &VizSelection) -> Vec<CommandBinding> {
    let entries: &[CommandBinding] = match target {
        VizSelection::Product { .. } => &[
            CommandBinding {
                id: CommandId::InitProduct,
                key: "i",
                label: "[I]nit product",
            },
            CommandBinding {
                id: CommandId::OpenCloneForm,
                key: "x",
                label: "[X] Clone variant",
            },
        ],
        VizSelection::Variant { .. } => &[
            CommandBinding {
                id: CommandId::OpenSpawnForm,
                key: "n",
                label: "[N] Spawn actor",
            },
            CommandBinding {
                id: CommandId::PollVariant,
                key: "p",
                label: "[P]oll variant",
            },
            CommandBinding {
                id: CommandId::ImportVariantActors,
                key: "m",
                label: "[M] Import actors",
            },
            CommandBinding {
                id: CommandId::OpenDeleteVariantForm,
                key: "d",
                label: "[D]elete variant",
            },
        ],
        VizSelection::Actor { .. } => &[
            CommandBinding {
                id: CommandId::BuildAttach,
                key: "a",
                label: "[A]ttach",
            },
            CommandBinding {
                id: CommandId::PollActor,
                key: "o",
                label: "[O] Poll actor",
            },
            CommandBinding {
                id: CommandId::OpenMoveActorForm,
                key: "g",
                label: "[G] Move actor",
            },
            CommandBinding {
                id: CommandId::OpenChatCompose,
                key: "c",
                label: "[C]ompose",
            },
            CommandBinding {
                id: CommandId::OpenSpawnForm,
                key: "n",
                label: "[N] Spawn actor",
            },
        ],
    };

    entries
        .iter()
        .copied()
        .filter(|entry| is_command_enabled(app, entry.id))
        .collect()
}

impl ContextMenuState {
    pub(crate) fn open(
        app: &App,
        target: VizSelection,
        anchor_col: u16,
        anchor_row: u16,
    ) -> Option<Self> {
        let entries = context_menu_commands(app, &target);
        if entries.is_empty() {
            return None;
        }

        Some(Self {
            anchor_col,
            anchor_row,
            target,
            entries,
            selected: 0,
        })
    }

    pub(crate) fn move_up(&mut self) {
        if self.entries.is_empty() {
            self.selected = 0;
            return;
        }

        self.selected = if self.selected == 0 {
            self.entries.len() - 1
        } else {
            self.selected - 1
        };
    }

    pub(crate) fn move_down(&mut self) {
        if self.entries.is_empty() {
            self.selected = 0;
            return;
        }

        self.selected = (self.selected + 1) % self.entries.len();
    }

    pub(crate) fn set_selected(&mut self, index: usize) {
        if self.entries.is_empty() {
            self.selected = 0;
            return;
        }

        self.selected = index.min(self.entries.len().saturating_sub(1));
    }

    pub(crate) fn selected_command(&self) -> Option<CommandId> {
        self.entries.get(self.selected).map(|entry| entry.id)
    }

    pub(crate) fn shortcut_command(&self, key: KeyEvent) -> Option<CommandId> {
        let KeyCode::Char(pressed) = key.code else {
            return None;
        };

        let pressed = pressed.to_ascii_lowercase();
        self.entries
            .iter()
            .find(|entry| entry.key.eq_ignore_ascii_case(&pressed.to_string()))
            .map(|entry| entry.id)
    }
}

#[allow(dead_code)]
enum ToolbarSelectionContext {
    Product,
    Variant,
    Actor,
}

fn toolbar_selection_context(app: &App) -> ToolbarSelectionContext {
    if app.results_view_mode() == ResultsViewMode::Viz {
        if let Some(selection) = app.viz_selection() {
            return match selection {
                VizSelection::Product { .. } => ToolbarSelectionContext::Product,
                VizSelection::Variant { .. } => ToolbarSelectionContext::Variant,
                VizSelection::Actor { .. } => ToolbarSelectionContext::Actor,
            };
        }
    }

    match app.focus() {
        FocusPane::Products => ToolbarSelectionContext::Product,
        FocusPane::Variants => ToolbarSelectionContext::Variant,
    }
}
