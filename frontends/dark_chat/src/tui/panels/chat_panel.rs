use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, Paragraph, Wrap};
use ratatui::Frame;

use dark_tui_components::{
    ChatConversationHeaderComponent, ChatConversationHeaderProps, ChatMessageListComponent,
    ChatMessageListProps, ChatPalette, ChatStatusTone, ComponentThemeLike, PaneBlockComponent,
    StatusPill,
};

use crate::tui::app::{App, FocusPane};
use crate::tui::components::to_component_messages;

pub struct ChatPanel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelSelectorHit {
    Outside,
    Popup,
    Query,
    ListItem(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentSelectorHit {
    Outside,
    Popup,
    Query,
    ListItem(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerAutocompleteHit {
    Outside,
    Popup,
    ListItem(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerMetaHit {
    None,
    Model,
    Agent,
}

const COMPOSER_PANEL_HEIGHT: u16 = 5;

impl ChatPanel {
    pub fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let block = PaneBlockComponent::build(
            "Conversation",
            app.is_focus(FocusPane::Chat) || app.is_focus(FocusPane::Composer),
            theme,
        );
        let inner = block.inner(area);
        frame.render_widget(block, area);

        if inner.width < 18 || inner.height < 8 {
            return;
        }

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(4),
                Constraint::Length(COMPOSER_PANEL_HEIGHT),
            ])
            .split(inner);

        let header = match app.active_session() {
            Some(session) => ChatConversationHeaderProps {
                title: session.title.clone(),
                subtitle: Some(format!("session:{}", session.id)),
                status_label: Some(session.status.clone()),
                status_tone: status_tone(&session.status),
            },
            None => ChatConversationHeaderProps {
                title: "No session".to_string(),
                subtitle: Some("Press n to create a session".to_string()),
                status_label: Some("idle".to_string()),
                status_tone: ChatStatusTone::Muted,
            },
        };
        ChatConversationHeaderComponent::render(frame, rows[0], theme, header);

        render_messages(frame, rows[1], app, theme);
        render_composer_panel(frame, rows[2], app, theme);
        render_model_selector_popup(frame, inner, rows[2], app, theme);
        render_agent_selector_popup(frame, inner, rows[2], app, theme);
        render_composer_autocomplete_popup(frame, inner, rows[2], app, theme);
    }

    pub fn model_selector_hit(
        conversation_area: Rect,
        composer_area: Rect,
        app: &App,
        col: u16,
        row: u16,
    ) -> ModelSelectorHit {
        if !app.is_model_selector_open() {
            return ModelSelectorHit::Outside;
        }

        let Some(area) = model_selector_popup_area(conversation_area, composer_area, app) else {
            return ModelSelectorHit::Outside;
        };

        if !rect_contains(area, col, row) {
            return ModelSelectorHit::Outside;
        }

        let inner = Rect {
            x: area.x.saturating_add(1),
            y: area.y.saturating_add(1),
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        };

        if inner.width < 8 || inner.height < 4 {
            return ModelSelectorHit::Popup;
        }

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(2),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(inner);

        if rect_contains(rows[1], col, row) {
            return ModelSelectorHit::Query;
        }

        if app.model_selector_raw_mode() || !rect_contains(rows[0], col, row) {
            return ModelSelectorHit::Popup;
        }

        let items = app.model_selector_items();
        if items.is_empty() {
            return ModelSelectorHit::Popup;
        }

        let selected = app
            .model_selector_selected()
            .min(items.len().saturating_sub(1));
        let visible = rows[0].height.max(1) as usize;
        let start = selected
            .saturating_sub(visible / 2)
            .min(items.len().saturating_sub(visible));
        let local = row.saturating_sub(rows[0].y) as usize;
        let index = start + local;

        if index < items.len() {
            ModelSelectorHit::ListItem(index)
        } else {
            ModelSelectorHit::Popup
        }
    }

    pub fn composer_autocomplete_hit(
        conversation_area: Rect,
        composer_area: Rect,
        app: &App,
        col: u16,
        row: u16,
    ) -> ComposerAutocompleteHit {
        if !app.composer_autocomplete_open() {
            return ComposerAutocompleteHit::Outside;
        }

        let Some(area) = composer_autocomplete_popup_area(conversation_area, composer_area, app)
        else {
            return ComposerAutocompleteHit::Outside;
        };

        if !rect_contains(area, col, row) {
            return ComposerAutocompleteHit::Outside;
        }

        let inner = Rect {
            x: area.x.saturating_add(1),
            y: area.y.saturating_add(1),
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        };

        if inner.width == 0 || inner.height == 0 {
            return ComposerAutocompleteHit::Popup;
        }

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(inner);

        if !rect_contains(rows[0], col, row) {
            return ComposerAutocompleteHit::Popup;
        }

        let local = row.saturating_sub(rows[0].y) as usize;
        let items = app.composer_autocomplete_items();
        let visible = rows[0].height.max(1) as usize;
        let selected = app
            .composer_autocomplete_selected()
            .min(items.len().saturating_sub(1));
        let start = selected
            .saturating_sub(visible / 2)
            .min(items.len().saturating_sub(visible));
        let index = start + local;

        if index < items.len() {
            ComposerAutocompleteHit::ListItem(index)
        } else {
            ComposerAutocompleteHit::Popup
        }
    }

    pub fn agent_selector_hit(
        conversation_area: Rect,
        composer_area: Rect,
        app: &App,
        col: u16,
        row: u16,
    ) -> AgentSelectorHit {
        if !app.is_agent_selector_open() {
            return AgentSelectorHit::Outside;
        }

        let Some(area) = agent_selector_popup_area(conversation_area, composer_area, app) else {
            return AgentSelectorHit::Outside;
        };

        if !rect_contains(area, col, row) {
            return AgentSelectorHit::Outside;
        }

        let inner = Rect {
            x: area.x.saturating_add(1),
            y: area.y.saturating_add(1),
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        };

        if inner.width < 8 || inner.height < 3 {
            return AgentSelectorHit::Popup;
        }

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(inner);

        if rect_contains(rows[1], col, row) {
            return AgentSelectorHit::Query;
        }

        if !rect_contains(rows[0], col, row) {
            return AgentSelectorHit::Popup;
        }

        let items = app.agent_selector_items();
        if items.is_empty() {
            return AgentSelectorHit::Popup;
        }

        let selected = app
            .agent_selector_selected()
            .min(items.len().saturating_sub(1));
        let visible = rows[0].height.max(1) as usize;
        let start = selected
            .saturating_sub(visible / 2)
            .min(items.len().saturating_sub(visible));
        let local = row.saturating_sub(rows[0].y) as usize;
        let index = start + local;

        if index < items.len() {
            AgentSelectorHit::ListItem(index)
        } else {
            AgentSelectorHit::Popup
        }
    }

    pub fn composer_meta_hit(
        conversation_area: Rect,
        composer_area: Rect,
        app: &App,
        col: u16,
        row: u16,
    ) -> ComposerMetaHit {
        let Some((model_area, agent_area)) =
            composer_meta_areas(conversation_area, composer_area, app)
        else {
            return ComposerMetaHit::None;
        };

        if rect_contains(model_area, col, row) {
            return ComposerMetaHit::Model;
        }

        if rect_contains(agent_area, col, row) {
            return ComposerMetaHit::Agent;
        }

        ComposerMetaHit::None
    }
}

fn render_agent_selector_popup(
    frame: &mut Frame,
    conversation_area: Rect,
    composer_area: Rect,
    app: &App,
    theme: &impl ComponentThemeLike,
) {
    if !app.is_agent_selector_open() {
        return;
    }

    let Some(area) = agent_selector_popup_area(conversation_area, composer_area, app) else {
        return;
    };

    frame.render_widget(Clear, area);
    let block = PaneBlockComponent::build("Agent Picker", true, theme);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 8 || inner.height < 3 {
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

    let items = app.agent_selector_items();
    if items.is_empty() {
        frame.render_widget(
            Paragraph::new(vec![Line::from(Span::styled(
                "No matching agents.",
                Style::default().fg(theme.text_muted()),
            ))]),
            rows[0],
        );
    } else {
        let selected = app
            .agent_selector_selected()
            .min(items.len().saturating_sub(1));
        let visible = rows[0].height.max(1) as usize;
        let start = selected
            .saturating_sub(visible / 2)
            .min(items.len().saturating_sub(visible));

        let mut lines = Vec::new();
        for (offset, agent) in items.iter().skip(start).take(visible).enumerate() {
            let index = start + offset;
            let active = app.active_agent() == Some(agent.as_str());
            let prefix = if index == selected { "▸ " } else { "  " };
            let mut spans = vec![
                Span::styled(
                    prefix,
                    if index == selected {
                        Style::default().fg(theme.pill_accent_fg())
                    } else {
                        Style::default().fg(theme.text_muted())
                    },
                ),
                Span::styled(
                    compact_text(agent, 44),
                    if index == selected {
                        Style::default().fg(theme.pill_accent_fg())
                    } else {
                        Style::default().fg(theme.text_secondary())
                    },
                ),
            ];

            if active {
                spans.push(Span::raw(" "));
                spans.push(StatusPill::info("active", theme).span_compact());
            }

            lines.push(Line::from(spans));
        }

        frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), rows[0]);
    }

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            StatusPill::accent("FILTER", theme).span_compact(),
            Span::raw(" "),
            Span::styled(
                with_cursor_tail(app.agent_selector_query()),
                Style::default().fg(theme.text_secondary()),
            ),
        ]))
        .wrap(Wrap { trim: true }),
        rows[1],
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("enter", Style::default().fg(theme.pill_accent_fg())),
            Span::styled(" select  ", Style::default().fg(theme.text_muted())),
            Span::styled("esc", Style::default().fg(theme.pill_accent_fg())),
            Span::styled(" close", Style::default().fg(theme.text_muted())),
        ])),
        rows[2],
    );
}

fn render_messages(frame: &mut Frame, area: Rect, app: &App, theme: &impl ComponentThemeLike) {
    if area.width < 6 || area.height < 2 {
        return;
    }

    let block = PaneBlockComponent::build("Messages", app.is_focus(FocusPane::Chat), theme);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 4 || inner.height < 1 {
        return;
    }

    let component_messages = to_component_messages(app.messages());
    let message_list = ChatMessageListProps {
        messages: &component_messages,
        empty_label: "No messages yet. Send a prompt to begin.",
        max_messages: 80,
        max_body_lines_per_message: 18,
        scroll_offset_lines: app.chat_scroll_lines(),
        palette: ChatPalette {
            text_primary: Color::White,
            role_user: theme.pill_info_fg(),
            role_assistant: theme.pill_accent_fg(),
            role_system: theme.pill_warn_fg(),
            role_tool: theme.pill_ok_fg(),
            role_other: theme.text_secondary(),
        },
    };
    ChatMessageListComponent::render(frame, inner, theme, message_list);
}

fn render_composer_panel(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    theme: &impl ComponentThemeLike,
) {
    let block = PaneBlockComponent::build("Composer", app.is_focus(FocusPane::Composer), theme);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 6 || inner.height < 1 {
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner);

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            StatusPill::accent(
                format!(
                    "model:{}",
                    compact_text(app.active_model().unwrap_or("-"), 28)
                ),
                theme,
            )
            .span_compact(),
            Span::raw("  "),
            StatusPill::muted(
                format!(
                    "agent:{}",
                    compact_text(app.active_agent().unwrap_or("-"), 20)
                ),
                theme,
            )
            .span_compact(),
        ]))
        .wrap(Wrap { trim: true }),
        rows[0],
    );

    if app.is_composing() && app.active_session().is_some() {
        let composer = app.composer().clone();
        frame.render_widget(&composer, rows[1]);
        return;
    }

    let ready = app.active_session().is_some();
    let mode = if ready {
        StatusPill::accent("ready", theme)
    } else {
        StatusPill::muted("session required", theme)
    };

    let hint = if ready {
        "Press c to edit and Enter to send"
    } else {
        "Create or select a session to enable input"
    };

    let lines = vec![Line::from(vec![
        mode.span_compact(),
        Span::raw("  "),
        Span::styled(hint, Style::default().fg(theme.text_secondary())),
    ])];
    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), rows[1]);
}

fn render_model_selector_popup(
    frame: &mut Frame,
    conversation_area: Rect,
    composer_area: Rect,
    app: &App,
    theme: &impl ComponentThemeLike,
) {
    if !app.is_model_selector_open() {
        return;
    }

    let Some(area) = model_selector_popup_area(conversation_area, composer_area, app) else {
        return;
    };

    frame.render_widget(Clear, area);
    let block = PaneBlockComponent::build("Model Picker", true, theme);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 8 || inner.height < 4 {
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(2),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(inner);

    if app.model_selector_raw_mode() {
        let input = with_cursor_tail(app.model_selector_raw_input());
        let lines = vec![
            Line::from(vec![
                StatusPill::warn("RAW", theme).span_compact(),
                Span::raw("  "),
                Span::styled(
                    "custom model key",
                    Style::default().fg(theme.text_secondary()),
                ),
            ]),
            Line::from(Span::styled(input, Style::default())),
        ];
        frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), rows[0]);
    } else {
        let items = app.model_selector_items();
        if items.is_empty() {
            frame.render_widget(
                Paragraph::new(vec![Line::from(Span::styled(
                    "No matching models. Press Tab for raw mode.",
                    Style::default().fg(theme.text_muted()),
                ))]),
                rows[0],
            );
        } else {
            let selected = app
                .model_selector_selected()
                .min(items.len().saturating_sub(1));
            let visible = rows[0].height.max(1) as usize;
            let start = selected
                .saturating_sub(visible / 2)
                .min(items.len().saturating_sub(visible));

            let mut lines = Vec::new();
            for (offset, model) in items.iter().skip(start).take(visible).enumerate() {
                let index = start + offset;
                let active = app.active_model() == Some(model.as_str());
                let prefix = if index == selected { "▸ " } else { "  " };
                let mut spans = vec![
                    Span::styled(
                        prefix,
                        if index == selected {
                            Style::default().fg(theme.pill_accent_fg())
                        } else {
                            Style::default().fg(theme.text_muted())
                        },
                    ),
                    Span::styled(
                        compact_text(model, 40),
                        if index == selected {
                            Style::default().fg(theme.pill_accent_fg())
                        } else {
                            Style::default().fg(theme.text_secondary())
                        },
                    ),
                    Span::raw(" "),
                    StatusPill::muted(model_provider_tag(model), theme).span_compact(),
                ];

                if active {
                    spans.push(Span::raw(" "));
                    spans.push(StatusPill::info("active", theme).span_compact());
                }

                lines.push(Line::from(spans));
            }

            frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), rows[0]);
        }
    }

    let mode_pill = if app.model_selector_raw_mode() {
        StatusPill::warn("RAW", theme)
    } else {
        StatusPill::accent("FILTER", theme)
    };
    let query = if app.model_selector_raw_mode() {
        with_cursor_tail(app.model_selector_raw_input())
    } else {
        with_cursor_tail(app.model_selector_query())
    };
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            mode_pill.span_compact(),
            Span::raw(" "),
            Span::styled(query, Style::default().fg(theme.text_secondary())),
        ]))
        .wrap(Wrap { trim: true }),
        rows[1],
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("enter", Style::default().fg(theme.pill_accent_fg())),
            Span::styled(" select  ", Style::default().fg(theme.text_muted())),
            Span::styled("tab", Style::default().fg(theme.pill_accent_fg())),
            Span::styled(" raw  ", Style::default().fg(theme.text_muted())),
            Span::styled("esc", Style::default().fg(theme.pill_accent_fg())),
            Span::styled(" close", Style::default().fg(theme.text_muted())),
        ])),
        rows[2],
    );
}

fn render_composer_autocomplete_popup(
    frame: &mut Frame,
    conversation_area: Rect,
    composer_area: Rect,
    app: &App,
    theme: &impl ComponentThemeLike,
) {
    if !app.composer_autocomplete_open()
        || app.is_model_selector_open()
        || app.is_agent_selector_open()
    {
        return;
    }

    let Some(area) = composer_autocomplete_popup_area(conversation_area, composer_area, app) else {
        return;
    };

    frame.render_widget(Clear, area);
    let title = match app.composer_autocomplete_mode() {
        Some(crate::tui::app::ComposerAutocompleteMode::Slash) => "Commands",
        Some(crate::tui::app::ComposerAutocompleteMode::File) => "Context",
        None => "Autocomplete",
    };
    let block = PaneBlockComponent::build(title, true, theme);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 8 || inner.height < 2 {
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(inner);

    let items = app.composer_autocomplete_items();
    if items.is_empty() {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "No suggestions",
                Style::default().fg(theme.text_muted()),
            ))),
            rows[0],
        );
    } else {
        let selected = app
            .composer_autocomplete_selected()
            .min(items.len().saturating_sub(1));
        let visible = rows[0].height.max(1) as usize;
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
                        Style::default().fg(theme.pill_accent_fg())
                    } else {
                        Style::default().fg(theme.text_muted())
                    },
                ),
                Span::styled(
                    compact_text(&item.label, 38),
                    if index == selected {
                        Style::default().fg(theme.pill_accent_fg())
                    } else {
                        Style::default().fg(theme.text_secondary())
                    },
                ),
                Span::raw(" "),
                StatusPill::muted(item.tag.clone(), theme).span_compact(),
            ]));
        }

        frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), rows[0]);
    }

    let trigger = match app.composer_autocomplete_mode() {
        Some(crate::tui::app::ComposerAutocompleteMode::Slash) => "/",
        Some(crate::tui::app::ComposerAutocompleteMode::File) => "@",
        None => "",
    };
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            StatusPill::accent(trigger, theme).span_compact(),
            Span::raw(" "),
            Span::styled(
                with_cursor_tail(app.composer_autocomplete_query()),
                Style::default().fg(theme.text_secondary()),
            ),
        ]))
        .wrap(Wrap { trim: true }),
        rows[1],
    );
}

fn with_cursor_tail(value: &str) -> String {
    if value.is_empty() {
        return "|".to_string();
    }

    format!("{value}|")
}

fn model_provider_tag(model: &str) -> String {
    model
        .split_once('/')
        .map(|(provider, _)| provider.to_string())
        .unwrap_or_else(|| "custom".to_string())
}

fn compact_text(value: &str, max_width: usize) -> String {
    if value.chars().count() <= max_width {
        return value.to_string();
    }

    let head = value
        .chars()
        .take(max_width.saturating_sub(3))
        .collect::<String>();
    format!("{head}...")
}

fn composer_meta_areas(
    conversation_area: Rect,
    composer_area: Rect,
    app: &App,
) -> Option<(Rect, Rect)> {
    if conversation_area.width < 18 || conversation_area.height < 8 {
        return None;
    }

    let composer_inner = Rect {
        x: composer_area.x.saturating_add(1),
        y: composer_area.y.saturating_add(1),
        width: composer_area.width.saturating_sub(2),
        height: composer_area.height.saturating_sub(2),
    };
    if composer_inner.width < 4 || composer_inner.height < 1 {
        return None;
    }

    let model_label = format!(
        "model:{}",
        compact_text(app.active_model().unwrap_or("-"), 28)
    );
    let agent_label = format!(
        "agent:{}",
        compact_text(app.active_agent().unwrap_or("-"), 20)
    );

    let model_width = model_label
        .chars()
        .count()
        .min(composer_inner.width as usize) as u16;
    let agent_x = composer_inner
        .x
        .saturating_add(model_width.saturating_add(2));
    let remaining = composer_inner
        .x
        .saturating_add(composer_inner.width)
        .saturating_sub(agent_x);
    let agent_width = agent_label.chars().count().min(remaining as usize) as u16;

    Some((
        Rect {
            x: composer_inner.x,
            y: composer_inner.y,
            width: model_width,
            height: 1,
        },
        Rect {
            x: agent_x,
            y: composer_inner.y,
            width: agent_width,
            height: 1,
        },
    ))
}

fn model_selector_popup_area(
    conversation_area: Rect,
    composer_area: Rect,
    app: &App,
) -> Option<Rect> {
    if conversation_area.width < 28 || conversation_area.height < 8 {
        return None;
    }

    let max_width = conversation_area.width.saturating_sub(2);
    if max_width < 26 {
        return None;
    }

    let popup_width = max_width.min(68);
    let popup_height = conversation_area.height.min(14);
    let anchor_col = app
        .model_selector_anchor_col()
        .or_else(|| {
            composer_meta_areas(conversation_area, composer_area, app)
                .map(|(model, _)| model.x.saturating_add(model.width / 2))
        })
        .unwrap_or(composer_area.x.saturating_add(1));
    let desired_x = anchor_col.saturating_sub(popup_width / 2);
    let min_x = conversation_area.x.saturating_add(1);
    let max_x = conversation_area.x.saturating_add(
        conversation_area
            .width
            .saturating_sub(popup_width)
            .saturating_sub(1),
    );
    let popup_x = desired_x.clamp(min_x, max_x);
    let popup_y = composer_area
        .y
        .saturating_sub(popup_height.saturating_sub(1))
        .max(conversation_area.y.saturating_add(1));

    Some(Rect {
        x: popup_x,
        y: popup_y,
        width: popup_width,
        height: popup_height,
    })
}

fn agent_selector_popup_area(
    conversation_area: Rect,
    composer_area: Rect,
    app: &App,
) -> Option<Rect> {
    if conversation_area.width < 28 || conversation_area.height < 8 {
        return None;
    }

    let max_width = conversation_area.width.saturating_sub(2);
    if max_width < 24 {
        return None;
    }

    let popup_width = max_width.min(56);
    let popup_height = conversation_area.height.min(12);
    let anchor_col = app
        .agent_selector_anchor_col()
        .or_else(|| {
            composer_meta_areas(conversation_area, composer_area, app)
                .map(|(_, agent)| agent.x.saturating_add(agent.width / 2))
        })
        .unwrap_or(composer_area.x.saturating_add(1));
    let desired_x = anchor_col.saturating_sub(popup_width / 2);
    let min_x = conversation_area.x.saturating_add(1);
    let max_x = conversation_area.x.saturating_add(
        conversation_area
            .width
            .saturating_sub(popup_width)
            .saturating_sub(1),
    );
    let popup_x = desired_x.clamp(min_x, max_x);
    let popup_y = composer_area
        .y
        .saturating_sub(popup_height.saturating_sub(1))
        .max(conversation_area.y.saturating_add(1));

    Some(Rect {
        x: popup_x,
        y: popup_y,
        width: popup_width,
        height: popup_height,
    })
}

fn composer_autocomplete_popup_area(
    conversation_area: Rect,
    composer_area: Rect,
    app: &App,
) -> Option<Rect> {
    if conversation_area.width < 22 || conversation_area.height < 6 {
        return None;
    }

    let max_width = conversation_area.width.saturating_sub(4);
    let width = max_width.min(52).max(24);
    let visible_items = app.composer_autocomplete_items().len().min(6) as u16;
    let height = (visible_items + 3).min(conversation_area.height.saturating_sub(1));

    let composer_inner = Rect {
        x: composer_area.x.saturating_add(1),
        y: composer_area.y.saturating_add(1),
        width: composer_area.width.saturating_sub(2),
        height: composer_area.height.saturating_sub(2),
    };
    let composer_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(composer_inner);
    let editor_area = composer_rows[1];

    let anchor_col = app
        .composer_autocomplete_anchor_position()
        .map(|(_, col)| col as u16)
        .unwrap_or(0);
    let desired_x = editor_area.x.saturating_add(anchor_col);
    let min_x = conversation_area.x.saturating_add(1);
    let max_x = conversation_area.x.saturating_add(
        conversation_area
            .width
            .saturating_sub(width)
            .saturating_sub(1),
    );
    let x = desired_x.clamp(min_x, max_x);

    let y = composer_area
        .y
        .saturating_sub(height.saturating_sub(1))
        .max(conversation_area.y.saturating_add(1));

    Some(Rect {
        x,
        y,
        width,
        height,
    })
}

fn rect_contains(area: Rect, col: u16, row: u16) -> bool {
    col >= area.x
        && col < area.x.saturating_add(area.width)
        && row >= area.y
        && row < area.y.saturating_add(area.height)
}

fn status_tone(status: &str) -> ChatStatusTone {
    match status.trim().to_ascii_lowercase().as_str() {
        "idle" | "ready" => ChatStatusTone::Ok,
        "busy" | "running" => ChatStatusTone::Info,
        "retry" | "retrying" => ChatStatusTone::Warn,
        "error" | "failed" => ChatStatusTone::Error,
        _ => ChatStatusTone::Muted,
    }
}
