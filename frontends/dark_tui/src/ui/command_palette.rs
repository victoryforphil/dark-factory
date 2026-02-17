use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, ResultsViewMode, VizSelection};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CommandId {
    Quit,
    ToggleFocus,
    MoveDown,
    MoveUp,
    Refresh,
    ToggleFilter,
    ToggleView,
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
const TOOLBAR_COMMANDS: &[CommandBinding] = &[
    CommandBinding {
        id: CommandId::Quit,
        key: "q",
        label: "Quit",
    },
    CommandBinding {
        id: CommandId::ToggleFocus,
        key: "Tab",
        label: "Focus",
    },
    CommandBinding {
        id: CommandId::MoveDown,
        key: "j/k",
        label: "Select",
    },
    CommandBinding {
        id: CommandId::Refresh,
        key: "r",
        label: "Refresh",
    },
    CommandBinding {
        id: CommandId::ToggleView,
        key: "v",
        label: "View",
    },
    CommandBinding {
        id: CommandId::ToggleFilter,
        key: "f",
        label: "Filter",
    },
    CommandBinding {
        id: CommandId::PollVariant,
        key: "p",
        label: "Poll",
    },
    CommandBinding {
        id: CommandId::OpenMoveActorForm,
        key: "g",
        label: "Move",
    },
    CommandBinding {
        id: CommandId::OpenCloneForm,
        key: "x",
        label: "Clone",
    },
    CommandBinding {
        id: CommandId::OpenDeleteVariantForm,
        key: "d",
        label: "Delete",
    },
    CommandBinding {
        id: CommandId::ImportVariantActors,
        key: "m",
        label: "Import",
    },
    CommandBinding {
        id: CommandId::InitProduct,
        key: "i",
        label: "Init",
    },
    CommandBinding {
        id: CommandId::OpenSpawnForm,
        key: "n",
        label: "New actor",
    },
    CommandBinding {
        id: CommandId::BuildAttach,
        key: "a",
        label: "Attach",
    },
    CommandBinding {
        id: CommandId::ToggleChat,
        key: "t",
        label: "Chat",
    },
    CommandBinding {
        id: CommandId::OpenChatCompose,
        key: "c",
        label: "Compose",
    },
    CommandBinding {
        id: CommandId::ResetPan,
        key: "0",
        label: "Reset pan",
    },
];

#[allow(dead_code)]
pub(crate) fn toolbar_bindings(app: &App) -> Vec<CommandBinding> {
    TOOLBAR_COMMANDS
        .iter()
        .copied()
        .filter(|binding| should_show_in_toolbar(app, binding.id))
        .collect()
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
        | CommandId::ToggleView
        | CommandId::InitProduct
        | CommandId::OpenSpawnForm
        | CommandId::ToggleChat => true,
        CommandId::ResetPan => app.results_view_mode() == ResultsViewMode::Viz,
        CommandId::PollVariant
        | CommandId::OpenDeleteVariantForm
        | CommandId::ImportVariantActors => app.selected_variant_id().is_some(),
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
                label: "Init product",
            },
            CommandBinding {
                id: CommandId::OpenCloneForm,
                key: "x",
                label: "Clone variant",
            },
        ],
        VizSelection::Variant { .. } => &[
            CommandBinding {
                id: CommandId::OpenSpawnForm,
                key: "n",
                label: "New actor",
            },
            CommandBinding {
                id: CommandId::PollVariant,
                key: "p",
                label: "Poll variant",
            },
            CommandBinding {
                id: CommandId::ImportVariantActors,
                key: "m",
                label: "Import actors",
            },
            CommandBinding {
                id: CommandId::OpenDeleteVariantForm,
                key: "d",
                label: "Delete variant",
            },
        ],
        VizSelection::Actor { .. } => &[
            CommandBinding {
                id: CommandId::BuildAttach,
                key: "a",
                label: "Attach",
            },
            CommandBinding {
                id: CommandId::PollActor,
                key: "o",
                label: "Poll actor",
            },
            CommandBinding {
                id: CommandId::OpenMoveActorForm,
                key: "g",
                label: "Move actor",
            },
            CommandBinding {
                id: CommandId::OpenChatCompose,
                key: "c",
                label: "Compose",
            },
            CommandBinding {
                id: CommandId::OpenSpawnForm,
                key: "n",
                label: "New actor",
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
fn should_show_in_toolbar(app: &App, command: CommandId) -> bool {
    if command == CommandId::ResetPan {
        return app.results_view_mode() == ResultsViewMode::Viz;
    }

    true
}
