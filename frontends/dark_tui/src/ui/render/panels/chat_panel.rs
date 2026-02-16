use dark_chat::framework::{
    ConversationComposer, ConversationHeader, ConversationMessage, ConversationPalette,
    ConversationPanelProps, ConversationStatusTone, render_conversation_panel,
    status_tone_for_status,
};
use dark_tui_components::{LoadingSpinner, PaneBlockComponent, StatusPill};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, Paragraph, Wrap};
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
        if let Some(popup) = picker_popup_area(area, app) {
            if contains(popup, col, row) {
                let inner = inner_rect(popup);
                if inner.height < 3 {
                    return ChatPanelHit::PickerPopup;
                }
                let rows = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                    ])
                    .split(inner);
                if contains(rows[0], col, row) {
                    let local = row.saturating_sub(rows[0].y) as usize;
                    let items = app.chat_picker_items();
                    let visible = rows[0].height.max(1) as usize;
                    let selected = app
                        .chat_picker_selected()
                        .min(items.len().saturating_sub(1));
                    let start = selected
                        .saturating_sub(visible / 2)
                        .min(items.len().saturating_sub(visible));
                    let index = start + local;
                    if index < items.len() {
                        return ChatPanelHit::PickerItem(index);
                    }
                }
                return ChatPanelHit::PickerPopup;
            }
        }

        if let Some(popup) = autocomplete_popup_area(area, app) {
            if contains(popup, col, row) {
                let inner = inner_rect(popup);
                if inner.height < 2 {
                    return ChatPanelHit::AutocompletePopup;
                }
                let rows = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(1), Constraint::Length(1)])
                    .split(inner);
                if contains(rows[0], col, row) {
                    let local = row.saturating_sub(rows[0].y) as usize;
                    let items = app.chat_autocomplete_items();
                    let visible = rows[0].height.max(1) as usize;
                    let selected = app
                        .chat_autocomplete_selected()
                        .min(items.len().saturating_sub(1));
                    let start = selected
                        .saturating_sub(visible / 2)
                        .min(items.len().saturating_sub(visible));
                    let index = start + local;
                    if index < items.len() {
                        return ChatPanelHit::AutocompleteItem(index);
                    }
                }

                return ChatPanelHit::AutocompletePopup;
            }
        }

        if let Some((model_rect, agent_rect)) = composer_label_areas(area, app) {
            if contains(model_rect, col, row) {
                return ChatPanelHit::ModelLabel;
            }
            if contains(agent_rect, col, row) {
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
    let Some(kind) = app.chat_picker_open() else {
        return;
    };

    let Some(popup) = picker_popup_area(area, app) else {
        return;
    };

    let theme = app.theme();
    frame.render_widget(Clear, popup);
    let title = match kind {
        ChatPickerKind::Model => "Select Model",
        ChatPickerKind::Agent => "Select Agent",
    };
    let block = PaneBlockComponent::build(title, true, theme);
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    if inner.width < 4 || inner.height < 3 {
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(inner);

    let items = app.chat_picker_items();
    if items.is_empty() {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "No matching options.",
                Style::default().fg(theme.text_muted),
            ))),
            rows[0],
        );
    } else {
        let visible = rows[0].height.max(1) as usize;
        let selected = app
            .chat_picker_selected()
            .min(items.len().saturating_sub(1));
        let start = selected
            .saturating_sub(visible / 2)
            .min(items.len().saturating_sub(visible));

        let mut lines = Vec::new();
        for (offset, item) in items.iter().skip(start).take(visible).enumerate() {
            let index = start + offset;
            lines.push(Line::from(vec![
                Span::styled(
                    if index == selected { "▸ " } else { "  " },
                    if index == selected {
                        Style::default().fg(theme.pane_focused_border)
                    } else {
                        Style::default().fg(theme.text_muted)
                    },
                ),
                Span::styled(item.clone(), Style::default().fg(theme.text_primary)),
            ]));
        }
        frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), rows[0]);
    }

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            StatusPill::accent("FILTER", theme).span_compact(),
            Span::raw(" "),
            Span::styled(
                with_cursor_tail(app.chat_picker_query()),
                Style::default().fg(theme.text_secondary),
            ),
        ])),
        rows[1],
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("enter", Style::default().fg(theme.pane_focused_border)),
            Span::styled(" select  ", Style::default().fg(theme.text_muted)),
            Span::styled("bksp", Style::default().fg(theme.pane_focused_border)),
            Span::styled(" delete  ", Style::default().fg(theme.text_muted)),
            Span::styled("esc", Style::default().fg(theme.pane_focused_border)),
            Span::styled(" close", Style::default().fg(theme.text_muted)),
        ])),
        rows[2],
    );
}

fn render_autocomplete_popup(frame: &mut Frame, area: Rect, app: &App) {
    if !app.chat_autocomplete_open() || app.chat_picker_open().is_some() {
        return;
    }

    let Some(popup) = autocomplete_popup_area(area, app) else {
        return;
    };

    let theme = app.theme();
    frame.render_widget(Clear, popup);
    let title = if app.chat_autocomplete_mode() == Some('@') {
        "Context"
    } else {
        "Commands"
    };
    let block = PaneBlockComponent::build(title, true, theme);
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    if inner.width < 4 || inner.height < 2 {
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(inner);

    let items = app.chat_autocomplete_items();
    let visible = rows[0].height.max(1) as usize;
    let selected = app
        .chat_autocomplete_selected()
        .min(items.len().saturating_sub(1));
    let start = selected
        .saturating_sub(visible / 2)
        .min(items.len().saturating_sub(visible));

    let mut lines = Vec::new();
    for (offset, item) in items.iter().skip(start).take(visible).enumerate() {
        let index = start + offset;
        lines.push(Line::from(vec![
            Span::styled(
                if index == selected { "▸ " } else { "  " },
                if index == selected {
                    Style::default().fg(theme.pane_focused_border)
                } else {
                    Style::default().fg(theme.text_muted)
                },
            ),
            Span::styled(item.clone(), Style::default().fg(theme.text_primary)),
        ]));
    }
    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), rows[0]);

    let trigger = app.chat_autocomplete_mode().unwrap_or('/');
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            StatusPill::accent(trigger.to_string(), theme).span_compact(),
            Span::raw(" "),
            Span::styled(
                with_cursor_tail(app.chat_autocomplete_query()),
                Style::default().fg(theme.text_secondary),
            ),
        ])),
        rows[1],
    );
}

fn picker_items(app: &App) -> &[String] {
    match app.chat_picker_open() {
        Some(ChatPickerKind::Model) => app.chat_model_options(),
        Some(ChatPickerKind::Agent) => app.chat_agent_options(),
        None => &[],
    }
}

fn picker_popup_area(area: Rect, app: &App) -> Option<Rect> {
    let items = picker_items(app);
    if items.is_empty() {
        return None;
    }

    let inner = inner_rect(area);
    if inner.width < 16 || inner.height < 8 {
        return None;
    }

    let width = inner.width.min(42);
    let height = (items.len().min(8) as u16 + 3).min(inner.height.saturating_sub(1));
    let (model_rect, agent_rect) = composer_label_areas(area, app)?;
    let anchor = match app.chat_picker_open() {
        Some(ChatPickerKind::Model) => model_rect,
        Some(ChatPickerKind::Agent) => agent_rect,
        None => model_rect,
    };

    let min_x = inner.x.saturating_add(1);
    let max_x = inner
        .x
        .saturating_add(inner.width.saturating_sub(width).saturating_sub(1));
    let x = anchor.x.clamp(min_x, max_x);
    let y = anchor
        .y
        .saturating_sub(height.saturating_add(1))
        .max(inner.y.saturating_add(1));

    Some(Rect {
        x,
        y,
        width,
        height,
    })
}

fn autocomplete_popup_area(area: Rect, app: &App) -> Option<Rect> {
    let items = app.chat_autocomplete_items();
    if items.is_empty() {
        return None;
    }

    let inner = inner_rect(area);
    if inner.width < 16 || inner.height < 8 {
        return None;
    }

    let width = inner.width.min(44);
    let height = (items.len().min(6) as u16 + 3).min(inner.height.saturating_sub(1));
    Some(Rect {
        x: inner
            .x
            .saturating_add(inner.width.saturating_sub(width).saturating_sub(1)),
        y: inner
            .y
            .saturating_add(inner.height.saturating_sub(height).saturating_sub(6)),
        width,
        height,
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

fn contains(area: Rect, col: u16, row: u16) -> bool {
    col >= area.x
        && col < area.x.saturating_add(area.width)
        && row >= area.y
        && row < area.y.saturating_add(area.height)
}

fn inner_rect(area: Rect) -> Rect {
    Rect {
        x: area.x.saturating_add(1),
        y: area.y.saturating_add(1),
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    }
}

fn with_cursor_tail(value: &str) -> String {
    if value.is_empty() {
        return "|".to_string();
    }

    format!("{value}|")
}

fn compact_text(value: &str, max_len: usize) -> String {
    if value.chars().count() <= max_len {
        return value.to_string();
    }

    let head = value
        .chars()
        .take(max_len.saturating_sub(3))
        .collect::<String>();
    format!("{head}...")
}

fn compact_session_id(value: &str) -> &str {
    if value.len() <= 14 {
        return value;
    }

    &value[..14]
}
