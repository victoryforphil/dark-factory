use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::Frame;

use dark_tui_components::{
    compact_text, rect_contains, ChatConversationHeaderComponent, ChatConversationHeaderProps,
    ChatMessageListComponent, ChatMessageListProps, ChatPalette, ChatStatusTone,
    ComponentThemeLike, PaneBlockComponent, PopupAnchor, PopupHit, PopupItem, PopupOverlay,
    PopupOverlayProps, StatusPill,
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

        let Some(props) = model_selector_popup_props(conversation_area, composer_area, app) else {
            return ModelSelectorHit::Outside;
        };

        match PopupOverlay::hit_test(conversation_area, &props, col, row) {
            PopupHit::Outside => ModelSelectorHit::Outside,
            PopupHit::Popup => ModelSelectorHit::Popup,
            PopupHit::Query => ModelSelectorHit::Query,
            PopupHit::ListItem(index) => {
                if app.model_selector_raw_mode() {
                    ModelSelectorHit::Popup
                } else {
                    ModelSelectorHit::ListItem(index)
                }
            }
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

        let Some(props) = composer_autocomplete_popup_props(conversation_area, composer_area, app)
        else {
            return ComposerAutocompleteHit::Outside;
        };

        match PopupOverlay::hit_test(conversation_area, &props, col, row) {
            PopupHit::Outside => ComposerAutocompleteHit::Outside,
            PopupHit::ListItem(index) => ComposerAutocompleteHit::ListItem(index),
            PopupHit::Popup | PopupHit::Query => ComposerAutocompleteHit::Popup,
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

        let Some(props) = agent_selector_popup_props(conversation_area, composer_area, app) else {
            return AgentSelectorHit::Outside;
        };

        match PopupOverlay::hit_test(conversation_area, &props, col, row) {
            PopupHit::Outside => AgentSelectorHit::Outside,
            PopupHit::Popup => AgentSelectorHit::Popup,
            PopupHit::Query => AgentSelectorHit::Query,
            PopupHit::ListItem(index) => AgentSelectorHit::ListItem(index),
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
    let Some(props) = agent_selector_popup_props(conversation_area, composer_area, app) else {
        return;
    };

    PopupOverlay::render(frame, conversation_area, &props, theme);
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
        max_body_lines_per_message: 30,
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
    let Some(props) = model_selector_popup_props(conversation_area, composer_area, app) else {
        return;
    };

    PopupOverlay::render(frame, conversation_area, &props, theme);
}

fn render_composer_autocomplete_popup(
    frame: &mut Frame,
    conversation_area: Rect,
    composer_area: Rect,
    app: &App,
    theme: &impl ComponentThemeLike,
) {
    let Some(props) = composer_autocomplete_popup_props(conversation_area, composer_area, app)
    else {
        return;
    };

    PopupOverlay::render(frame, conversation_area, &props, theme);
}

fn model_provider_tag(model: &str) -> String {
    model
        .split_once('/')
        .map(|(provider, _)| provider.to_string())
        .unwrap_or_else(|| "custom".to_string())
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

fn model_selector_popup_props(
    conversation_area: Rect,
    composer_area: Rect,
    app: &App,
) -> Option<PopupOverlayProps> {
    if conversation_area.width < 28 || conversation_area.height < 8 {
        return None;
    }

    let max_width = conversation_area.width.saturating_sub(2);
    if max_width < 26 {
        return None;
    }

    let anchor_col = app
        .model_selector_anchor_col()
        .or_else(|| {
            composer_meta_areas(conversation_area, composer_area, app)
                .map(|(model, _)| model.x.saturating_add(model.width / 2))
        })
        .unwrap_or(composer_area.x.saturating_add(1));

    let items = if app.model_selector_raw_mode() {
        vec![PopupItem {
            label: "custom model key".to_string(),
            tag: None,
            active: false,
        }]
    } else {
        app.model_selector_items()
            .into_iter()
            .map(|model| PopupItem {
                label: compact_text(&model, 40),
                tag: Some(model_provider_tag(&model)),
                active: app.active_model() == Some(model.as_str()),
            })
            .collect()
    };

    Some(PopupOverlayProps {
        title: "Model Picker".to_string(),
        items,
        selected: app.model_selector_selected(),
        query: Some(if app.model_selector_raw_mode() {
            app.model_selector_raw_input().to_string()
        } else {
            app.model_selector_query().to_string()
        }),
        query_label: Some(if app.model_selector_raw_mode() {
            "RAW".to_string()
        } else {
            "FILTER".to_string()
        }),
        hint: Some("enter select  tab raw  esc close".to_string()),
        anchor: PopupAnchor::At {
            x: anchor_col,
            y: composer_area.y,
        },
        max_visible: 8,
        min_width: 26,
        max_width: max_width.min(68),
    })
}

fn agent_selector_popup_props(
    conversation_area: Rect,
    composer_area: Rect,
    app: &App,
) -> Option<PopupOverlayProps> {
    if conversation_area.width < 28 || conversation_area.height < 8 {
        return None;
    }

    let max_width = conversation_area.width.saturating_sub(2);
    if max_width < 24 {
        return None;
    }

    let anchor_col = app
        .agent_selector_anchor_col()
        .or_else(|| {
            composer_meta_areas(conversation_area, composer_area, app)
                .map(|(_, agent)| agent.x.saturating_add(agent.width / 2))
        })
        .unwrap_or(composer_area.x.saturating_add(1));

    Some(PopupOverlayProps {
        title: "Agent Picker".to_string(),
        items: app
            .agent_selector_items()
            .into_iter()
            .map(|agent| PopupItem {
                label: compact_text(&agent, 44),
                tag: None,
                active: app.active_agent() == Some(agent.as_str()),
            })
            .collect(),
        selected: app.agent_selector_selected(),
        query: Some(app.agent_selector_query().to_string()),
        query_label: Some("FILTER".to_string()),
        hint: Some("enter select  esc close".to_string()),
        anchor: PopupAnchor::At {
            x: anchor_col,
            y: composer_area.y,
        },
        max_visible: 8,
        min_width: 24,
        max_width: max_width.min(56),
    })
}

fn composer_autocomplete_popup_props(
    conversation_area: Rect,
    composer_area: Rect,
    app: &App,
) -> Option<PopupOverlayProps> {
    if !app.composer_autocomplete_open()
        || app.is_model_selector_open()
        || app.is_agent_selector_open()
    {
        return None;
    }

    if conversation_area.width < 22 || conversation_area.height < 6 {
        return None;
    }

    let max_width = conversation_area.width.saturating_sub(4);
    let width = max_width.min(52).max(24).min(conversation_area.width);

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

    let title = match app.composer_autocomplete_mode() {
        Some(crate::tui::app::ComposerAutocompleteMode::Slash) => "Commands",
        Some(crate::tui::app::ComposerAutocompleteMode::File) => "Context",
        None => "Autocomplete",
    };
    let trigger = match app.composer_autocomplete_mode() {
        Some(crate::tui::app::ComposerAutocompleteMode::Slash) => "/",
        Some(crate::tui::app::ComposerAutocompleteMode::File) => "@",
        None => "",
    };

    Some(PopupOverlayProps {
        title: title.to_string(),
        items: app
            .composer_autocomplete_items()
            .iter()
            .map(|item| PopupItem {
                label: compact_text(&item.label, 38),
                tag: Some(item.tag.clone()),
                active: false,
            })
            .collect(),
        selected: app.composer_autocomplete_selected(),
        query: Some(app.composer_autocomplete_query().to_string()),
        query_label: Some(trigger.to_string()),
        hint: None,
        anchor: PopupAnchor::At {
            x: editor_area.x.saturating_add(anchor_col),
            y: composer_area.y,
        },
        max_visible: 6,
        min_width: 24,
        max_width: width,
    })
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
