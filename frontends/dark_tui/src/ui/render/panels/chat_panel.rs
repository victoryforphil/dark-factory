use dark_chat::framework::{
    render_conversation_panel, status_tone_for_status, ConversationComposer, ConversationHeader,
    ConversationMessage, ConversationPalette, ConversationPanelProps, ConversationStatusTone,
};
use dark_tui_components::{
    compact_session_id, compact_text, inner_rect, rect_contains, ChatMessageEntry,
    ChatMessageListComponent, ChatMessageListProps, ChatMessageRole, ChatPalette, LoadingSpinner,
    PopupAnchor, PopupHit, PopupItem, PopupOverlay, PopupOverlayProps, StatusPill,
};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
};
use ratatui::Frame;
use serde_json::Value;
use std::borrow::Cow;

use crate::app::{App, ChatPickerKind};

pub(crate) struct ChatPanel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ChatPanelHit {
    Outside,
    MessageBody,
    ModelLabel,
    AgentLabel,
    DetailButton,
    ComposerBody,
    PickerItem(usize),
    PickerPopup,
    AutocompleteItem(usize),
    AutocompletePopup,
    DetailPopup,
    DetailPopupBackground,
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

        let show_context_labels = should_show_context_labels(app);
        let active_model = if show_context_labels {
            app.chat_active_model().unwrap_or("-")
        } else {
            ""
        };
        let active_agent = if show_context_labels {
            app.chat_active_agent().unwrap_or("-")
        } else {
            ""
        };

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
                max_messages: app.chat_render_limit(),
                max_body_lines_per_message: app.chat_max_body_lines(),
                scroll_offset_lines: app.chat_scroll_lines(),
                composer: Self::composer(app),
                palette: Self::palette(app),
            },
        );

        render_detail_button(frame, area, app);
        render_picker_popup(frame, area, app);
        render_autocomplete_popup(frame, area, app);
        render_detail_popup(frame, area, app);
    }

    pub(crate) fn hit_test(area: Rect, app: &App, col: u16, row: u16) -> ChatPanelHit {
        // Detail popup overlays everything — check first.
        if app.is_chat_detail_popup_open() {
            if let Some(msg) = app.chat_detail_popup_message() {
                if let Some(popup_rect) = detail_popup_area(area, &msg.text) {
                    if rect_contains(popup_rect, col, row) {
                        return ChatPanelHit::DetailPopup;
                    }
                    // Click outside the popup but inside chat area = close popup
                    if rect_contains(area, col, row) {
                        return ChatPanelHit::DetailPopupBackground;
                    }
                }
            }
        }

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

        if let Some(messages_rect) = message_panel_area(area) {
            if rect_contains(messages_rect, col, row) {
                return ChatPanelHit::MessageBody;
            }
        }

        if let Some(detail_rect) = detail_button_area(area, app) {
            if rect_contains(detail_rect, col, row) {
                return ChatPanelHit::DetailButton;
            }
        }

        if let Some(composer_rect) = composer_body_area(area) {
            if rect_contains(composer_rect, col, row) {
                return ChatPanelHit::ComposerBody;
            }
        }

        ChatPanelHit::Outside
    }

    pub(crate) fn message_index_at_point(
        area: Rect,
        app: &App,
        col: u16,
        row: u16,
    ) -> Option<usize> {
        let messages_rect = message_body_area(area)?;
        if !rect_contains(messages_rect, col, row) {
            return None;
        }
        if app.chat_messages().is_empty() {
            return None;
        }

        let row_in_viewport = row.saturating_sub(messages_rect.y) as usize;
        let theme = app.theme();
        let entries = app
            .chat_messages()
            .iter()
            .map(|message| {
                ChatMessageEntry::new(
                    ChatMessageRole::from_role(&message.role),
                    &message.text,
                    Some(message.created_at.clone()),
                )
            })
            .collect::<Vec<_>>();
        let empty_label = if app.chat_actor().is_some() {
            "No chat messages yet."
        } else {
            "Select an actor node to open chat."
        };

        let props = ChatMessageListProps {
            messages: &entries,
            empty_label,
            max_messages: app.chat_render_limit(),
            max_body_lines_per_message: app.chat_max_body_lines(),
            scroll_offset_lines: app.chat_scroll_lines(),
            palette: ChatPalette {
                text_primary: theme.text_primary,
                role_user: theme.entity_variant,
                role_assistant: theme.entity_actor,
                role_system: Color::Yellow,
                role_tool: Color::Cyan,
                role_other: theme.text_secondary,
            },
        };

        ChatMessageListComponent::message_index_at_row(
            theme,
            &props,
            messages_rect.width as usize,
            messages_rect.height as usize,
            row_in_viewport,
        )
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
    if !should_show_context_labels(app) {
        return None;
    }

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

fn should_show_context_labels(app: &App) -> bool {
    app.is_chat_composing() || app.chat_picker_open().is_some()
}

fn composer_body_area(area: Rect) -> Option<Rect> {
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
    if composer.width < 3 || composer.height < 3 {
        return None;
    }

    Some(Rect {
        x: composer.x,
        y: composer.y.saturating_add(1),
        width: composer.width,
        height: composer.height.saturating_sub(1),
    })
}

fn message_body_area(area: Rect) -> Option<Rect> {
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

    let messages = inner_rect(chunks[1]);
    if messages.width < 3 || messages.height < 2 {
        return None;
    }

    Some(messages)
}

fn message_panel_area(area: Rect) -> Option<Rect> {
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

    let panel = chunks[1];
    if panel.width < 3 || panel.height < 2 {
        return None;
    }

    Some(panel)
}

/// Returns the rect for the clickable "Detail" pill in the header row.
///
/// Positioned at the right edge of the header area, visible whenever
/// the chat has messages (so there is content to expand).
fn detail_button_area(area: Rect, app: &App) -> Option<Rect> {
    if app.chat_messages().is_empty() {
        return None;
    }

    let inner = inner_rect(area);
    if inner.width < 20 || inner.height < 6 {
        return None;
    }

    // Match the vertical layout used by render_conversation_panel:
    // [header=3, messages=min(4), composer=5]
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(4),
            Constraint::Length(5),
        ])
        .split(inner);

    let header = inner_rect(chunks[0]);
    if header.width < 12 || header.height < 1 {
        return None;
    }

    // Pill label: " Detail " → width 8
    let pill_width: u16 = 8;
    let x = header.x + header.width.saturating_sub(pill_width);
    // Place on the last row of the header block.
    let y = header.y + header.height.saturating_sub(1);

    Some(Rect {
        x,
        y,
        width: pill_width,
        height: 1,
    })
}

/// Renders a clickable "Detail" pill in the chat header area.
fn render_detail_button(frame: &mut Frame, area: Rect, app: &App) {
    let Some(btn_rect) = detail_button_area(area, app) else {
        return;
    };

    let theme = app.theme();
    let pill = if app.is_chat_detail_popup_open() {
        StatusPill::accent("Detail ▾", theme)
    } else {
        StatusPill::info("Detail ▸", theme)
    };

    frame.render_widget(Paragraph::new(Line::from(pill.span())), btn_rect);
}

/// Returns the rect for the detail popup, centered and fitted to content.
///
/// Uses stable geometry: prefers a generous fixed sizing approach over
/// content-measured widths to avoid layout instability from varying text.
fn detail_popup_area(area: Rect, _text: &str) -> Option<Rect> {
    let inner = inner_rect(area);
    if inner.width < 24 || inner.height < 10 {
        return None;
    }

    // Use a generous, stable popup size relative to the parent area.
    // This avoids the instability of content-measured widths that shift
    // per-message and cause border/wrapping artifacts.
    let popup_width = inner.width.saturating_sub(6).min(88).max(32);
    let popup_height = inner.height.saturating_sub(4).min(40).max(8);

    if popup_width < 20 || popup_height < 6 {
        return None;
    }

    let x = inner.x + (inner.width.saturating_sub(popup_width)) / 2;
    let y = inner.y + (inner.height.saturating_sub(popup_height)) / 2;

    Some(Rect {
        x,
        y,
        width: popup_width,
        height: popup_height,
    })
}

/// Structured metadata extracted from a chat message for the detail popup.
struct DetailMeta {
    /// e.g. "Tool Call", "Shell Command", "Shell Output", "Thinking", "Message"
    kind_label: String,
    /// Optional tool name for tool calls.
    tool_name: Option<String>,
    /// Optional summary line (e.g. "git status --short", "2 todo item(s) updated").
    summary: Option<String>,
    /// Optional command text for shell sections.
    command: Option<String>,
    /// Body content lines (code blocks, output, thinking text).
    body_lines: Vec<String>,
}

/// Parse structured detail metadata from a chat message's raw text.
fn extract_detail_meta(text: &str) -> Vec<DetailMeta> {
    let mut metas = Vec::new();
    let mut current_kind: Option<String> = None;
    let mut current_tool: Option<String> = None;
    let mut current_summary: Option<String> = None;
    let mut current_command: Option<String> = None;
    let mut current_body: Vec<String> = Vec::new();
    let mut in_code_block = false;

    let flush = |metas: &mut Vec<DetailMeta>,
                 kind: Option<String>,
                 tool: Option<String>,
                 summary: Option<String>,
                 command: Option<String>,
                 body: Vec<String>| {
        if let Some(kind_label) = kind {
            metas.push(DetailMeta {
                kind_label,
                tool_name: tool,
                summary,
                command,
                body_lines: body,
            });
        }
    };

    for line in text.lines() {
        let trimmed = line.trim();

        // Detect section headers: ### Tool // name, ### Shell Command, ### Shell Output, ### Thinking
        if trimmed.starts_with("### Tool // ") {
            flush(
                &mut metas,
                current_kind.take(),
                current_tool.take(),
                current_summary.take(),
                current_command.take(),
                std::mem::take(&mut current_body),
            );
            let name = trimmed.strip_prefix("### Tool // ").unwrap_or("tool");
            current_kind = Some("Tool Call".to_string());
            current_tool = Some(name.to_string());
            in_code_block = false;
            continue;
        }

        if trimmed == "### Shell Command" {
            flush(
                &mut metas,
                current_kind.take(),
                current_tool.take(),
                current_summary.take(),
                current_command.take(),
                std::mem::take(&mut current_body),
            );
            current_kind = Some("Shell Command".to_string());
            in_code_block = false;
            continue;
        }

        if trimmed == "### Shell Output" {
            flush(
                &mut metas,
                current_kind.take(),
                current_tool.take(),
                current_summary.take(),
                current_command.take(),
                std::mem::take(&mut current_body),
            );
            current_kind = Some("Shell Output".to_string());
            in_code_block = false;
            continue;
        }

        if trimmed == "### Thinking" {
            flush(
                &mut metas,
                current_kind.take(),
                current_tool.take(),
                current_summary.take(),
                current_command.take(),
                std::mem::take(&mut current_body),
            );
            current_kind = Some("Thinking".to_string());
            in_code_block = false;
            continue;
        }

        // Sub-section headers (#### IN, #### OUT) — fold into body
        if trimmed.starts_with("####") {
            let label = trimmed.trim_start_matches('#').trim();
            current_body.push(format!("[{label}]"));
            continue;
        }

        // Summary line for tool calls
        if current_kind.as_deref() == Some("Tool Call")
            && current_summary.is_none()
            && trimmed.starts_with("summary: ")
        {
            current_summary = trimmed.strip_prefix("summary: ").map(|s| s.to_string());
            continue;
        }

        // Code fence markers — track but don't render
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }

        // If no section yet, start a generic text section
        if current_kind.is_none() && !trimmed.is_empty() {
            current_kind = Some("Message".to_string());
        }

        // Shell Command: first body line is the command itself
        if current_kind.as_deref() == Some("Shell Command")
            && current_command.is_none()
            && in_code_block
            && !trimmed.is_empty()
        {
            current_command = Some(trimmed.to_string());
            // Also add to body for display
        }

        // Thinking: strip blockquote markers
        if current_kind.as_deref() == Some("Thinking") {
            let clean = trimmed.strip_prefix("> ").unwrap_or(trimmed);
            current_body.push(clean.to_string());
            continue;
        }

        current_body.push(trimmed.to_string());
    }

    // Flush final section
    flush(
        &mut metas,
        current_kind.take(),
        current_tool.take(),
        current_summary.take(),
        current_command.take(),
        std::mem::take(&mut current_body),
    );

    metas
}

fn render_detail_popup(frame: &mut Frame, area: Rect, app: &App) {
    if !app.is_chat_detail_popup_open() {
        return;
    }

    let Some(msg) = app.chat_detail_popup_message() else {
        return;
    };

    let Some(popup_rect) = detail_popup_area(area, &msg.text) else {
        return;
    };

    let theme = app.theme();

    // Clear background — same as PopupOverlay / ContextMenu.
    frame.render_widget(Clear, popup_rect);

    // Chrome: match ContextMenu/PopupOverlay — pane_focused_border, subtle bg.
    let block = Block::default()
        .title(Line::from(vec![
            Span::raw(" "),
            Span::styled(
                "Detail",
                Style::default()
                    .fg(theme.pane_focused_border)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
        ]))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.pane_focused_border));
    let block_inner = block.inner(popup_rect);
    frame.render_widget(block, popup_rect);

    if block_inner.width < 4 || block_inner.height < 2 {
        return;
    }

    // Reserve 1 row for meta header, 1 row for hint at bottom.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // meta row: role + timestamp
            Constraint::Length(1), // separator
            Constraint::Min(1),    // scrollable content
            Constraint::Length(1), // hint row
        ])
        .split(block_inner);
    let meta_area = chunks[0];
    let sep_area = chunks[1];
    let content_area = chunks[2];
    let hint_area = chunks[3];

    // ── Meta row: role + timestamp ───────────────────────────────
    let role_pill = StatusPill::muted(msg.role.to_uppercase(), theme);
    let meta_line = Line::from(vec![
        Span::raw(" "),
        role_pill.span(),
        Span::raw("  "),
        Span::styled(
            msg.created_at.clone(),
            Style::default().fg(theme.text_muted),
        ),
    ]);
    frame.render_widget(Paragraph::new(meta_line), meta_area);

    // ── Separator ────────────────────────────────────────────────
    let sep_width = sep_area.width.saturating_sub(2) as usize;
    let sep_line = Line::from(Span::styled(
        format!(" {}", "─".repeat(sep_width)),
        Style::default().fg(theme.pane_unfocused_border),
    ));
    frame.render_widget(Paragraph::new(sep_line), sep_area);

    // ── Hint row ─────────────────────────────────────────────────
    let hint_line = Line::from(Span::styled(
        " esc close  j/k scroll",
        Style::default().fg(theme.text_muted),
    ));
    frame.render_widget(Paragraph::new(hint_line), hint_area);

    // ── Content area: structured sections ────────────────────────
    if content_area.width < 3 || content_area.height < 1 {
        return;
    }

    let sections = extract_detail_meta(&msg.text);
    let usable_width = content_area.width.saturating_sub(3) as usize; // 1 left pad + 2 right margin

    // Build rendered lines with clipped widths (no wrapping).
    let mut lines: Vec<Line<'static>> = Vec::new();

    for (si, section) in sections.iter().enumerate() {
        if si > 0 {
            // Blank line between sections.
            lines.push(Line::raw(""));
        }

        // ── Section header ──────────────────────────────────
        let pill = match section.kind_label.as_str() {
            "Tool Call" => StatusPill::accent(&section.kind_label, theme),
            "Shell Command" | "Shell Output" => StatusPill::info(&section.kind_label, theme),
            "Thinking" => StatusPill::warn("Thinking", theme),
            _ => StatusPill::muted(&section.kind_label, theme),
        };

        let mut header_spans = vec![Span::raw(" "), pill.span()];
        if let Some(tool_name) = &section.tool_name {
            header_spans.push(Span::raw("  "));
            header_spans.push(Span::styled(
                tool_name.clone(),
                Style::default()
                    .fg(theme.pill_accent_fg)
                    .add_modifier(Modifier::BOLD),
            ));
        }
        lines.push(Line::from(header_spans));

        // ── Summary ─────────────────────────────────────────
        if let Some(summary) = &section.summary {
            lines.push(Line::from(vec![
                Span::styled("   ▸ ", Style::default().fg(theme.pill_accent_fg)),
                Span::styled(
                    compact_text(summary, usable_width.saturating_sub(5)),
                    Style::default().fg(theme.text_secondary),
                ),
            ]));
        }

        // ── Command ─────────────────────────────────────────
        if let Some(command) = &section.command {
            lines.push(Line::from(vec![
                Span::styled("   $ ", Style::default().fg(theme.pill_info_fg)),
                Span::styled(
                    compact_text(command, usable_width.saturating_sub(5)),
                    Style::default().fg(theme.text_primary),
                ),
            ]));
        }

        if section.tool_name.as_deref() == Some("apply_patch") {
            render_apply_patch_detail_body(section, &mut lines, usable_width, theme);
            continue;
        }

        // ── Body lines (clipped, not wrapped) ───────────────
        let max_body = 30_usize;
        let body_count = section.body_lines.len();
        let show_count = body_count.min(max_body);
        let mut active_io: Option<&'static str> = None;
        let mut previous_blank = false;

        for (li, line_text) in section.body_lines.iter().take(show_count).enumerate() {
            let trimmed = line_text.trim();

            if trimmed.is_empty() {
                if li > 0 && !previous_blank {
                    lines.push(Line::raw(""));
                }
                previous_blank = true;
                continue;
            }
            previous_blank = false;

            // Sub-section label: [IN], [OUT]
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                let label_text = &trimmed[1..trimmed.len() - 1];
                active_io = parse_io_label(label_text);
                let io_pill = match active_io {
                    Some("IN") => StatusPill::info("IN", theme).span_compact(),
                    Some("OUT") => StatusPill::accent("OUT", theme).span_compact(),
                    _ => StatusPill::muted(label_text, theme).span_compact(),
                };
                lines.push(Line::from(vec![Span::raw("   "), io_pill]));
                continue;
            }

            // Todo checklist line: "- [x] task" / "- [ ] task"
            if let Some((checked, label)) = parse_todo_checkbox_line(trimmed) {
                let checkbox = if checked { "[x]" } else { "[ ]" };
                let checkbox_color = if checked {
                    theme.pill_ok_fg
                } else {
                    theme.pill_muted_fg
                };
                let text_style = if checked {
                    Style::default()
                        .fg(theme.text_muted)
                        .add_modifier(Modifier::CROSSED_OUT)
                } else {
                    Style::default().fg(theme.text_secondary)
                };
                lines.push(Line::from(vec![
                    Span::raw("   "),
                    Span::styled(checkbox, Style::default().fg(checkbox_color)),
                    Span::raw(" "),
                    Span::styled(
                        compact_text(label, usable_width.saturating_sub(8)),
                        text_style,
                    ),
                ]));
                continue;
            }

            if trimmed == "(empty)" && active_io == Some("OUT") {
                lines.push(Line::from(vec![
                    Span::styled("   · ", Style::default().fg(theme.text_muted)),
                    Span::styled("no output", Style::default().fg(theme.text_muted)),
                ]));
                continue;
            }

            // Regular body line — clipped to usable width.
            let display = compact_text(trimmed, usable_width.saturating_sub(3));
            if let Some(io_kind) = active_io {
                let marker_color = if io_kind == "IN" {
                    theme.pill_info_fg
                } else {
                    theme.pill_accent_fg
                };
                let text_color = if io_kind == "IN" {
                    theme.text_primary
                } else {
                    theme.text_secondary
                };
                lines.push(Line::from(vec![
                    Span::styled("   │ ", Style::default().fg(marker_color)),
                    Span::styled(display, Style::default().fg(text_color)),
                ]));
            } else {
                lines.push(Line::from(Span::styled(
                    format!("   {display}"),
                    Style::default().fg(theme.text_secondary),
                )));
            }
        }

        if body_count > max_body {
            lines.push(Line::from(Span::styled(
                format!("   ... +{} more lines", body_count - max_body),
                Style::default().fg(theme.text_muted),
            )));
        }
    }

    // Fallback: if no structured sections found.
    if lines.is_empty() {
        for body_line in msg.text.lines().take(30) {
            let display = compact_text(body_line.trim(), usable_width.saturating_sub(3));
            lines.push(Line::from(Span::styled(
                format!("   {display}"),
                Style::default().fg(theme.text_primary),
            )));
        }
    }

    // Scrolling: no wrapping — use line-level scroll only.
    let visible_lines = content_area.height.max(1) as usize;
    let max_scroll = lines.len().saturating_sub(visible_lines) as u16;
    let scroll_offset = app.chat_detail_popup_scroll_lines().min(max_scroll);

    // Render without Wrap to prevent line-breaking that causes border artifacts.
    let widget = Paragraph::new(lines).scroll((scroll_offset, 0));
    frame.render_widget(widget, content_area);

    // Scrollbar when content exceeds viewport.
    if max_scroll > 0 {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        let mut scrollbar_state = ScrollbarState::new(max_scroll as usize + visible_lines)
            .position(scroll_offset as usize);
        frame.render_stateful_widget(scrollbar, content_area, &mut scrollbar_state);
    }
}

fn parse_todo_checkbox_line(line: &str) -> Option<(bool, &str)> {
    let trimmed = line.trim_start();
    if let Some(rest) = trimmed
        .strip_prefix("- [x] ")
        .or_else(|| trimmed.strip_prefix("- [X] "))
    {
        return Some((true, rest.trim()));
    }
    if let Some(rest) = trimmed.strip_prefix("- [ ] ") {
        return Some((false, rest.trim()));
    }
    None
}

fn parse_io_label(label: &str) -> Option<&'static str> {
    let normalized = label.trim().to_ascii_uppercase();
    match normalized.as_str() {
        "IN" => Some("IN"),
        "OUT" => Some("OUT"),
        _ => None,
    }
}

fn render_apply_patch_detail_body(
    section: &DetailMeta,
    lines: &mut Vec<Line<'static>>,
    usable_width: usize,
    theme: &crate::theme::Theme,
) {
    let (input_lines, output_lines) = split_io_lines(&section.body_lines);

    if !input_lines.is_empty() {
        lines.push(Line::from(vec![
            Span::raw("   "),
            StatusPill::info("IN", theme).span_compact(),
        ]));

        if let Some(summary) = parse_apply_patch_input_summary(&input_lines) {
            lines.push(Line::from(Span::styled(
                format!(
                    "   │ ops: {} update, {} add, {} delete",
                    summary.update, summary.add, summary.delete
                ),
                Style::default().fg(theme.text_secondary),
            )));
            lines.push(Line::from(Span::styled(
                "   │ files:",
                Style::default().fg(theme.text_secondary),
            )));
            for file in summary.files.iter().take(8) {
                lines.push(Line::from(Span::styled(
                    format!(
                        "   │ - {}",
                        compact_text(file, usable_width.saturating_sub(8))
                    ),
                    Style::default().fg(theme.text_primary),
                )));
            }
            if summary.files.len() > 8 {
                lines.push(Line::from(Span::styled(
                    format!("   │ ... +{} more", summary.files.len() - 8),
                    Style::default().fg(theme.text_muted),
                )));
            }
        } else {
            for line in input_lines.iter().take(10) {
                lines.push(Line::from(Span::styled(
                    format!(
                        "   │ {}",
                        compact_text(line, usable_width.saturating_sub(5))
                    ),
                    Style::default().fg(theme.text_secondary),
                )));
            }
        }
    }

    if !output_lines.is_empty() {
        lines.push(Line::from(vec![
            Span::raw("   "),
            StatusPill::accent("OUT", theme).span_compact(),
        ]));

        let compact = summarize_apply_patch_output(&output_lines);
        for line in compact.iter().take(12) {
            lines.push(Line::from(Span::styled(
                format!(
                    "   │ {}",
                    compact_text(line, usable_width.saturating_sub(5))
                ),
                Style::default().fg(theme.text_secondary),
            )));
        }
        if compact.len() > 12 {
            lines.push(Line::from(Span::styled(
                format!("   │ ... +{} more", compact.len() - 12),
                Style::default().fg(theme.text_muted),
            )));
        }
    }
}

fn split_io_lines(body_lines: &[String]) -> (Vec<String>, Vec<String>) {
    let mut in_lines = Vec::new();
    let mut out_lines = Vec::new();
    let mut mode: Option<&str> = None;

    for line in body_lines {
        let trimmed = line.trim();
        if trimmed.eq_ignore_ascii_case("[IN]") {
            mode = Some("IN");
            continue;
        }
        if trimmed.eq_ignore_ascii_case("[OUT]") {
            mode = Some("OUT");
            continue;
        }
        if trimmed.is_empty() {
            continue;
        }
        match mode {
            Some("IN") => in_lines.push(trimmed.to_string()),
            Some("OUT") => out_lines.push(trimmed.to_string()),
            _ => {}
        }
    }

    (in_lines, out_lines)
}

#[derive(Default)]
struct ApplyPatchInputSummary {
    update: usize,
    add: usize,
    delete: usize,
    files: Vec<String>,
}

fn parse_apply_patch_input_summary(lines: &[String]) -> Option<ApplyPatchInputSummary> {
    let raw = lines.join("\n");
    let parsed = serde_json::from_str::<Value>(&raw).ok()?;
    let patch_text = parsed
        .as_object()
        .and_then(|map| map.get("patchText").or_else(|| map.get("patch")))
        .and_then(Value::as_str)?;

    let mut summary = ApplyPatchInputSummary::default();
    for line in patch_text.lines().map(str::trim) {
        if let Some(file) = line.strip_prefix("*** Update File: ") {
            summary.update += 1;
            summary.files.push(file.to_string());
            continue;
        }
        if let Some(file) = line.strip_prefix("*** Add File: ") {
            summary.add += 1;
            summary.files.push(file.to_string());
            continue;
        }
        if let Some(file) = line.strip_prefix("*** Delete File: ") {
            summary.delete += 1;
            summary.files.push(file.to_string());
        }
    }

    if summary.update + summary.add + summary.delete == 0 {
        None
    } else {
        Some(summary)
    }
}

fn summarize_apply_patch_output(lines: &[String]) -> Vec<String> {
    let mut important = Vec::new();
    for line in lines {
        let lower = line.to_ascii_lowercase();
        if lower.contains("success")
            || lower.contains("verification failed")
            || line.starts_with("M ")
            || line.starts_with("A ")
            || line.starts_with("D ")
            || line.starts_with("ERROR [")
        {
            important.push(line.clone());
        }
    }

    if important.is_empty() {
        lines
            .iter()
            .filter(|line| !line.trim().is_empty())
            .take(10)
            .cloned()
            .collect()
    } else {
        important
    }
}
