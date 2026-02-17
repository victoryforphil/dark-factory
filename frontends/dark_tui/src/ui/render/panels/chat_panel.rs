use dark_chat::framework::{
    render_conversation_panel, status_tone_for_status, ConversationComposer, ConversationHeader,
    ConversationMessage, ConversationPalette, ConversationPanelProps, ConversationStatusTone,
};
use dark_tui_components::{
    compact_session_id, compact_text, inner_rect, rect_contains, LoadingSpinner, PopupAnchor,
    PopupHit, PopupItem, PopupOverlay, PopupOverlayProps,
};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Color;
use ratatui::Frame;
use std::borrow::Cow;

use crate::app::{App, ChatPickerKind};

pub(crate) struct ChatPanel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ChatPanelHit {
    Outside,
    ModelLabel,
    AgentLabel,
    PickerItem(usize),
    PickerPopup,
    AutocompleteItem(usize),
    AutocompletePopup,
}

impl ChatPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let header = Self::header(app);
        let messages = app
            .chat_messages()
            .iter()
            .map(|message| ConversationMessage {
                role: &message.role,
                text: &message.text,
                created_at: Some(message.created_at.as_str()),
            })
            .collect::<Vec<_>>();

        let empty_label = if app.chat_actor().is_some() {
            "No chat messages yet."
        } else {
            "Select an actor node to open chat."
        };

        let active_model = app.chat_active_model().unwrap_or("-");
        let active_agent = app.chat_active_agent().unwrap_or("-");

        render_conversation_panel(
            frame,
            area,
            theme,
            ConversationPanelProps {
                title: "Actor Chat",
                focused: true,
                active_model_label: active_model,
                active_agent_label: active_agent,
                header,
                messages: &messages,
                empty_label,
                max_messages: 40,
                max_body_lines_per_message: 16,
                scroll_offset_lines: 0,
                composer: Self::composer(app),
                palette: Self::palette(app),
            },
        );

        render_picker_popup(frame, area, app);
        render_autocomplete_popup(frame, area, app);
    }

    pub(crate) fn hit_test(area: Rect, app: &App, col: u16, row: u16) -> ChatPanelHit {
        if let Some(props) = picker_popup_props(area, app) {
            match PopupOverlay::hit_test(area, &props, col, row) {
                PopupHit::Outside => {}
                PopupHit::ListItem(index) => return ChatPanelHit::PickerItem(index),
                PopupHit::Popup | PopupHit::Query => return ChatPanelHit::PickerPopup,
            }
        }

        if let Some(props) = autocomplete_popup_props(area, app) {
            match PopupOverlay::hit_test(area, &props, col, row) {
                PopupHit::Outside => {}
                PopupHit::ListItem(index) => return ChatPanelHit::AutocompleteItem(index),
                PopupHit::Popup | PopupHit::Query => return ChatPanelHit::AutocompletePopup,
            }
        }

        if let Some((model_rect, agent_rect)) = composer_label_areas(area, app) {
            if rect_contains(model_rect, col, row) {
                return ChatPanelHit::ModelLabel;
            }
            if rect_contains(agent_rect, col, row) {
                return ChatPanelHit::AgentLabel;
            }
        }

        ChatPanelHit::Outside
    }

    fn header(app: &App) -> ConversationHeader<'_> {
        let Some(actor) = app.chat_actor() else {
            return ConversationHeader {
                title: Cow::Borrowed("No actor selected"),
                subtitle: Some(Cow::Borrowed("Select an actor node in the catalog")),
                status_label: Some(Cow::Borrowed("idle")),
                status_tone: ConversationStatusTone::Muted,
            };
        };

        let subtitle = if actor.description.trim().is_empty() || actor.description.trim() == "-" {
            let session = actor
                .provider_session_id
                .as_deref()
                .map(compact_session_id)
                .unwrap_or("-");
            Some(Cow::Owned(format!(
                "provider:{} session:{}",
                actor.provider, session
            )))
        } else {
            Some(Cow::Borrowed(actor.description.as_str()))
        };

        let (status_label, status_tone) = if app.is_chat_send_in_flight() {
            (
                Cow::Owned(format!("sending {}", LoadingSpinner::glyph())),
                ConversationStatusTone::Info,
            )
        } else if app.is_chat_refresh_in_flight() {
            (
                Cow::Owned(format!("syncing {}", LoadingSpinner::glyph())),
                ConversationStatusTone::Info,
            )
        } else {
            (
                Cow::Borrowed(actor.status.as_str()),
                status_tone_for_status(&actor.status),
            )
        };

        ConversationHeader {
            title: Cow::Borrowed(actor.title.as_str()),
            subtitle,
            status_label: Some(status_label),
            status_tone,
        }
    }

    fn palette(app: &App) -> ConversationPalette {
        let theme = app.theme();
        ConversationPalette {
            text_primary: theme.text_primary,
            role_user: theme.entity_variant,
            role_assistant: theme.entity_actor,
            role_system: Color::Yellow,
            role_tool: Color::Cyan,
            role_other: theme.text_secondary,
        }
    }

    fn composer(app: &App) -> ConversationComposer<'_> {
        ConversationComposer {
            enabled: app.chat_actor().is_some(),
            composing: app.is_chat_composing(),
            draft: app.chat_draft(),
            cursor_index: app.chat_draft().chars().count(),
            idle_hint: "Press c to compose, Enter to send, t to hide chat.",
            disabled_hint: "Input disabled until an actor is selected.",
        }
    }
}

fn render_picker_popup(frame: &mut Frame, area: Rect, app: &App) {
    let Some(_kind) = app.chat_picker_open() else {
        return;
    };

    let Some(props) = picker_popup_props(area, app) else {
        return;
    };

    let theme = app.theme();
    PopupOverlay::render(frame, area, &props, theme);
}

fn render_autocomplete_popup(frame: &mut Frame, area: Rect, app: &App) {
    let Some(props) = autocomplete_popup_props(area, app) else {
        return;
    };

    let theme = app.theme();
    PopupOverlay::render(frame, area, &props, theme);
}

fn picker_items(app: &App) -> &[String] {
    match app.chat_picker_open() {
        Some(ChatPickerKind::Model) => app.chat_model_options(),
        Some(ChatPickerKind::Agent) => app.chat_agent_options(),
        None => &[],
    }
}

fn picker_popup_props(area: Rect, app: &App) -> Option<PopupOverlayProps> {
    let items = picker_items(app);
    if items.is_empty() {
        return None;
    }

    let inner = inner_rect(area);
    if inner.width < 16 || inner.height < 8 {
        return None;
    }

    let (model_rect, agent_rect) = composer_label_areas(area, app)?;
    let anchor = match app.chat_picker_open() {
        Some(ChatPickerKind::Model) => model_rect,
        Some(ChatPickerKind::Agent) => agent_rect,
        None => model_rect,
    };

    let title = match app.chat_picker_open() {
        Some(ChatPickerKind::Model) => "Select Model",
        Some(ChatPickerKind::Agent) => "Select Agent",
        None => "Select",
    };

    Some(PopupOverlayProps {
        title: title.to_string(),
        items: items
            .iter()
            .map(|item| PopupItem {
                label: item.clone(),
                tag: None,
                active: false,
            })
            .collect(),
        selected: app.chat_picker_selected(),
        query: Some(app.chat_picker_query().to_string()),
        query_label: Some("FILTER".to_string()),
        hint: Some("enter select  bksp delete  esc close".to_string()),
        anchor: PopupAnchor::At {
            x: anchor.x,
            y: anchor.y,
        },
        max_visible: 8,
        min_width: 24,
        max_width: inner.width.min(42),
    })
}

fn autocomplete_popup_props(area: Rect, app: &App) -> Option<PopupOverlayProps> {
    if !app.chat_autocomplete_open() || app.chat_picker_open().is_some() {
        return None;
    }

    let items = app.chat_autocomplete_items();
    if items.is_empty() {
        return None;
    }

    let inner = inner_rect(area);
    if inner.width < 16 || inner.height < 8 {
        return None;
    }

    let title = if app.chat_autocomplete_mode() == Some('@') {
        "Context"
    } else {
        "Commands"
    };
    let trigger = app.chat_autocomplete_mode().unwrap_or('/');

    Some(PopupOverlayProps {
        title: title.to_string(),
        items: items
            .iter()
            .map(|item| PopupItem {
                label: item.clone(),
                tag: None,
                active: false,
            })
            .collect(),
        selected: app.chat_autocomplete_selected(),
        query: Some(app.chat_autocomplete_query().to_string()),
        query_label: Some(trigger.to_string()),
        hint: None,
        anchor: PopupAnchor::At {
            x: inner.x.saturating_add(
                inner
                    .width
                    .saturating_sub(inner.width.min(44))
                    .saturating_sub(1),
            ),
            y: inner.y.saturating_add(inner.height.saturating_sub(6)),
        },
        max_visible: 6,
        min_width: 24,
        max_width: inner.width.min(44),
    })
}

fn composer_label_areas(area: Rect, app: &App) -> Option<(Rect, Rect)> {
    let inner = inner_rect(area);
    if inner.width < 10 || inner.height < 6 {
        return None;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(4),
            Constraint::Length(5),
        ])
        .split(inner);
    let composer = inner_rect(chunks[2]);
    if composer.width < 4 || composer.height < 1 {
        return None;
    }

    let model = app.chat_active_model().unwrap_or("-");
    let agent = app.chat_active_agent().unwrap_or("-");
    let model_label = format!(" model:{} ", compact_text(model, 32));
    let agent_label = format!(" agent:{} ", compact_text(agent, 26));

    let model_w = model_label.chars().count().min(composer.width as usize) as u16;
    let agent_w = agent_label.chars().count().min(composer.width as usize) as u16;

    let model_rect = Rect {
        x: composer.x,
        y: composer.y,
        width: model_w,
        height: 1,
    };
    let agent_rect = Rect {
        x: composer.x.saturating_add(model_w.saturating_add(2)),
        y: composer.y,
        width: agent_w,
        height: 1,
    };

    Some((model_rect, agent_rect))
}
