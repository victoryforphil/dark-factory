use crate::compact_text;
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::components::chat_types::{ChatMessageEntry, ChatMessageRole};
use crate::theme::ComponentThemeLike;

/// Message-list palette used to color per-role text.
#[derive(Debug, Clone, Copy)]
pub struct ChatPalette {
    pub text_primary: Color,
    pub role_user: Color,
    pub role_assistant: Color,
    pub role_system: Color,
    pub role_tool: Color,
    pub role_other: Color,
}

impl ChatPalette {
    /// Builds a default palette from a component theme.
    pub fn from_theme(theme: &impl ComponentThemeLike) -> Self {
        Self {
            text_primary: Color::White,
            role_user: theme.pill_info_fg(),
            role_assistant: theme.pill_accent_fg(),
            role_system: theme.pill_warn_fg(),
            role_tool: theme.pill_ok_fg(),
            role_other: theme.text_secondary(),
        }
    }

    fn role_color(&self, role: &ChatMessageRole) -> Color {
        match role {
            ChatMessageRole::User => self.role_user,
            ChatMessageRole::Assistant => self.role_assistant,
            ChatMessageRole::System => self.role_system,
            ChatMessageRole::Tool => self.role_tool,
            ChatMessageRole::Other(_) => self.role_other,
        }
    }
}

/// Props for rendering a scrollable conversation transcript.
#[derive(Debug, Clone)]
pub struct ChatMessageListProps<'a> {
    pub messages: &'a [ChatMessageEntry],
    pub empty_label: &'a str,
    pub max_messages: usize,
    pub max_body_lines_per_message: usize,
    pub scroll_offset_lines: u16,
    pub palette: ChatPalette,
}

impl<'a> ChatMessageListProps<'a> {
    /// Creates baseline list props with sensible defaults.
    pub fn from_messages(
        messages: &'a [ChatMessageEntry],
        theme: &impl ComponentThemeLike,
    ) -> Self {
        Self {
            messages,
            empty_label: "No chat messages yet.",
            max_messages: 60,
            max_body_lines_per_message: 14,
            scroll_offset_lines: 0,
            palette: ChatPalette::from_theme(theme),
        }
    }
}

/// Renderer for chat message history with tinyverse-style card layout.
pub struct ChatMessageListComponent;

#[derive(Clone)]
struct RenderedRow {
    line: Line<'static>,
    message_index: Option<usize>,
}

impl ChatMessageListComponent {
    /// Renders the message list into the target area.
    pub fn render(
        frame: &mut Frame,
        area: Rect,
        theme: &impl ComponentThemeLike,
        props: ChatMessageListProps<'_>,
    ) {
        let rows = Self::rendered_rows(theme, &props, area.width as usize);
        let lines = rows.into_iter().map(|row| row.line).collect::<Vec<_>>();
        let viewport_height = area.height as usize;
        let total_lines = lines.len();
        let base_scroll = total_lines.saturating_sub(viewport_height);
        let scroll = base_scroll.saturating_sub(props.scroll_offset_lines as usize) as u16;

        frame.render_widget(Paragraph::new(lines).scroll((scroll, 0)), area);
    }

    pub(crate) fn lines(
        theme: &impl ComponentThemeLike,
        props: &ChatMessageListProps<'_>,
        area_width: usize,
    ) -> Vec<Line<'static>> {
        Self::rendered_rows(theme, props, area_width)
            .into_iter()
            .map(|row| row.line)
            .collect()
    }

    pub fn message_index_at_row(
        theme: &impl ComponentThemeLike,
        props: &ChatMessageListProps<'_>,
        area_width: usize,
        area_height: usize,
        row_in_viewport: usize,
    ) -> Option<usize> {
        if area_height == 0 {
            return None;
        }

        let rows = Self::rendered_rows(theme, props, area_width);
        if rows.is_empty() {
            return None;
        }

        let total_lines = rows.len();
        let base_scroll = total_lines.saturating_sub(area_height);
        let scroll_top = base_scroll.saturating_sub(props.scroll_offset_lines as usize);
        let visible_row = row_in_viewport.min(area_height.saturating_sub(1));
        let absolute_row = scroll_top.saturating_add(visible_row);

        rows.get(absolute_row).and_then(|row| row.message_index)
    }

    fn rendered_rows(
        theme: &impl ComponentThemeLike,
        props: &ChatMessageListProps<'_>,
        area_width: usize,
    ) -> Vec<RenderedRow> {
        if props.messages.is_empty() {
            return vec![RenderedRow {
                line: Line::styled(
                    props.empty_label.to_string(),
                    Style::default().fg(theme.text_muted()),
                ),
                message_index: None,
            }];
        }

        let w = area_width;
        let content_width = w.saturating_sub(4).max(1);

        let mut lines = Vec::new();
        let cap = props.max_messages.max(1);
        let start = props.messages.len().saturating_sub(cap);

        for (msg_index, message) in props.messages[start..].iter().enumerate() {
            let absolute_message_index = start + msg_index;
            if msg_index > 0 {
                lines.push(RenderedRow {
                    line: Line::raw(""),
                    message_index: Some(absolute_message_index),
                });
            }

            let role_fg = props.palette.role_color(&message.role);
            let header_bg = role_header_bg(role_fg);
            let border_fg = separator_tint(role_fg, theme.pane_unfocused_border());

            // ── Top border ─────────────────────────────────────────
            lines.push(RenderedRow {
                line: boxed_rule('╭', '╮', w, border_fg),
                message_index: Some(absolute_message_index),
            });

            // ── Role-tinted header band ────────────────────────────
            let pill_text = format!(" {} ", role_label(&message.role));
            let ts_text = message
                .created_at
                .as_ref()
                .filter(|ts| !ts.trim().is_empty())
                .map(|ts| format!("  {ts}"))
                .unwrap_or_default();
            let used = pill_text.chars().count() + ts_text.chars().count();
            let pad = " ".repeat(content_width.saturating_sub(used));

            lines.push(RenderedRow {
                line: Line::from(vec![
                    Span::styled("│ ", Style::default().fg(border_fg)),
                    Span::styled(
                        pill_text,
                        Style::default()
                            .fg(role_fg)
                            .bg(header_bg)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        ts_text,
                        Style::default().fg(theme.text_muted()).bg(header_bg),
                    ),
                    Span::styled(pad, Style::default().bg(header_bg)),
                    Span::styled(" │", Style::default().fg(border_fg)),
                ]),
                message_index: Some(absolute_message_index),
            });

            // ── Body content ───────────────────────────────────────
            let body_lines = render_card_body(
                message,
                &props.palette,
                props.max_body_lines_per_message,
                theme,
                content_width,
            );

            if body_lines.is_empty() {
                lines.push(RenderedRow {
                    line: boxed_content_line(
                        Line::from(Span::styled(
                            "(no content)",
                            Style::default().fg(theme.text_muted()),
                        )),
                        content_width,
                        border_fg,
                    ),
                    message_index: Some(absolute_message_index),
                });
            } else {
                for body_line in body_lines {
                    lines.push(RenderedRow {
                        line: boxed_content_line(body_line, content_width, border_fg),
                        message_index: Some(absolute_message_index),
                    });
                }
            }

            // ── Bottom border ──────────────────────────────────────
            lines.push(RenderedRow {
                line: boxed_rule('╰', '╯', w, border_fg),
                message_index: Some(absolute_message_index),
            });
        }

        lines
    }
}

// ─────────────────────────────────────────────────────────────────────
// Card chrome helpers (ported from tinyverse)
// ─────────────────────────────────────────────────────────────────────

fn boxed_rule(left: char, right: char, width: usize, color: Color) -> Line<'static> {
    let style = Style::default().fg(color);
    let middle = "─".repeat(width.saturating_sub(2));
    Line::from(vec![
        Span::styled(left.to_string(), style),
        Span::styled(middle, style),
        Span::styled(right.to_string(), style),
    ])
}

fn boxed_content_line(
    mut content: Line<'static>,
    content_width: usize,
    border_fg: Color,
) -> Line<'static> {
    let border_style = Style::default().fg(border_fg);
    let mut spans = Vec::with_capacity(content.spans.len() + 3);
    spans.push(Span::styled("│ ", border_style));

    let mut used = 0usize;
    for span in content.spans.drain(..) {
        used += span.content.chars().count();
        spans.push(span);
    }

    if used < content_width {
        spans.push(Span::raw(" ".repeat(content_width - used)));
    }

    spans.push(Span::styled(" │", border_style));
    Line::from(spans)
}

/// Blends role accent towards the base separator color for a subtle tint.
fn separator_tint(role_fg: Color, base: Color) -> Color {
    let (rb, gb, bb) = match base {
        Color::Rgb(r, g, b) => (r, g, b),
        _ => (50, 50, 58),
    };
    let (rr, gr, br) = match role_fg {
        Color::Rgb(r, g, b) => (r, g, b),
        _ => (rb, gb, bb),
    };
    // 80% base, 20% role accent
    Color::Rgb(
        ((rb as u16 * 8 + rr as u16 * 2) / 10) as u8,
        ((gb as u16 * 8 + gr as u16 * 2) / 10) as u8,
        ((bb as u16 * 8 + br as u16 * 2) / 10) as u8,
    )
}

/// Produces a subtle dark background tint from the role accent color.
fn role_header_bg(role_fg: Color) -> Color {
    match role_fg {
        Color::Rgb(r, g, b) => {
            // Dark tint: 20% of accent over a very dark base
            Color::Rgb(
                ((r as u16 * 2 + 20 * 8) / 10).min(255) as u8,
                ((g as u16 * 2 + 20 * 8) / 10).min(255) as u8,
                ((b as u16 * 2 + 25 * 8) / 10).min(255) as u8,
            )
        }
        _ => Color::Rgb(30, 30, 38),
    }
}

fn role_label(role: &ChatMessageRole) -> &str {
    match role {
        ChatMessageRole::User => "YOU",
        ChatMessageRole::Assistant => "AGENT",
        ChatMessageRole::System => "SYSTEM",
        ChatMessageRole::Tool => "TOOL",
        ChatMessageRole::Other(value) => value.as_str(),
    }
}

// ─────────────────────────────────────────────────────────────────────
// Card body: section-aware rendering
// ─────────────────────────────────────────────────────────────────────

/// Renders message body with section-aware compact previews.
///
/// Detects `### Tool // name`, `### Shell Command`, `### Shell Output`,
/// `### Thinking` section markers (produced by `extract_message_text`)
/// and renders them as collapsible-style compact headers with one-line
/// previews, matching tinyverse chat style.
fn render_card_body(
    message: &ChatMessageEntry,
    palette: &ChatPalette,
    max_body_lines: usize,
    theme: &impl ComponentThemeLike,
    content_width: usize,
) -> Vec<Line<'static>> {
    let normalized = message.text.replace("\r\n", "\n");
    let sections = parse_message_sections(&normalized);

    if sections.is_empty() {
        return Vec::new();
    }

    let max_preview = content_width.saturating_sub(8).max(1);
    let mut lines = Vec::new();

    for section in &sections {
        match section {
            MessageSection::Text(text) => {
                let rendered = render_markdown_lines(text, *palette, theme);
                let mut trimmed = rendered;
                trim_blank_edges(&mut trimmed);
                let compacted = compact_blank_lines(trimmed, max_body_lines.max(1), theme);
                for line in compacted {
                    if !is_hidden_noise_line(&line_text(&line)) {
                        lines.push(line);
                    }
                }
            }
            MessageSection::ToolCall {
                name,
                summary,
                body,
            } => {
                let is_todo = name.eq_ignore_ascii_case("todowrite");
                let header_label = if is_todo {
                    "todo".to_string()
                } else {
                    format!("tool {name}")
                };
                let header_tag = if is_todo { "✓" } else { "tool" };
                let header_fg = if is_todo {
                    theme.pill_ok_fg()
                } else {
                    theme.pill_info_fg()
                };

                lines.push(collapsible_header(
                    &header_label,
                    header_tag,
                    header_fg,
                    content_width,
                    theme,
                ));

                // For todo calls, the summary IS the content — show it directly.
                // For other tools, fall back to body preview.
                let preview = if is_todo {
                    summary.as_deref().unwrap_or("todos updated").to_string()
                } else {
                    summary
                        .as_deref()
                        .or_else(|| first_meaningful_body_line(body))
                        .unwrap_or("(details)")
                        .to_string()
                };
                lines.push(Line::from(Span::styled(
                    format!("    {}", compact_text(&preview, max_preview)),
                    Style::default().fg(theme.text_muted()),
                )));
            }
            MessageSection::ShellCommand { command } => {
                let mut spans = vec![
                    Span::styled(
                        " CMD ",
                        Style::default()
                            .fg(theme.pill_info_fg())
                            .bg(theme.pill_info_bg())
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" "),
                    Span::styled("$ ", Style::default().fg(theme.pill_info_fg())),
                    Span::styled(
                        compact_text(command, max_preview),
                        Style::default().fg(theme.text_secondary()),
                    ),
                ];
                // Pad to avoid wrapping artifacts
                let used: usize = spans.iter().map(|s| s.content.chars().count()).sum();
                if used < content_width {
                    spans.push(Span::raw(" ".repeat(content_width - used)));
                }
                lines.push(Line::from(spans));
            }
            MessageSection::ShellOutput { body } => {
                lines.push(collapsible_header(
                    "shell output",
                    "shell",
                    theme.text_muted(),
                    content_width,
                    theme,
                ));

                let preview = body.lines().next().unwrap_or("(empty)");
                lines.push(Line::from(Span::styled(
                    format!("    {}", compact_text(preview, max_preview)),
                    Style::default().fg(theme.text_muted()),
                )));
            }
            MessageSection::Thinking { body } => {
                lines.push(collapsible_header(
                    "Reasoning",
                    "thinking",
                    theme.pill_muted_fg(),
                    content_width,
                    theme,
                ));

                let preview = body.lines().next().unwrap_or("(no detail)");
                lines.push(Line::from(Span::styled(
                    format!("    {}", compact_text(preview, max_preview)),
                    Style::default().fg(theme.text_muted()),
                )));
            }
        }
    }

    // Apply total body line cap
    if lines.len() > max_body_lines && max_body_lines > 1 {
        lines.truncate(max_body_lines - 1);
        lines.push(Line::from(Span::styled(
            "  ...",
            Style::default().fg(theme.text_muted()),
        )));
    }

    lines
}

/// Collapsible-style section header: icon + label + spacer + tag pill.
fn collapsible_header(
    label: &str,
    kind_tag: &str,
    fg: Color,
    width: usize,
    theme: &impl ComponentThemeLike,
) -> Line<'static> {
    let icon = "◆";
    let left = format!("  {icon} {label}");
    let tag = format!(" {kind_tag} ");
    let used = left.chars().count() + tag.chars().count();
    let spacer = " ".repeat(width.saturating_sub(used).max(1));

    let bg = theme.pill_muted_bg();
    let tag_bg = theme.pane_unfocused_border();

    Line::from(vec![
        Span::styled(
            left,
            Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD),
        ),
        Span::styled(spacer, Style::default().bg(bg)),
        Span::styled(
            tag,
            Style::default()
                .fg(fg)
                .bg(tag_bg)
                .add_modifier(Modifier::BOLD),
        ),
    ])
}

// ─────────────────────────────────────────────────────────────────────
// Section parsing: extract structured sections from pre-rendered text
// ─────────────────────────────────────────────────────────────────────

#[derive(Debug)]
enum MessageSection {
    Text(String),
    ToolCall {
        name: String,
        summary: Option<String>,
        body: String,
    },
    ShellCommand {
        command: String,
    },
    ShellOutput {
        body: String,
    },
    Thinking {
        body: String,
    },
}

fn parse_message_sections(text: &str) -> Vec<MessageSection> {
    let mut sections = Vec::new();
    let mut current_text = String::new();
    let mut in_code_block = false;

    let mut lines_iter = text.lines().peekable();

    while let Some(line) = lines_iter.next() {
        let trimmed = line.trim();

        // Track code fences (so we don't misparse ### inside code blocks)
        if trimmed.starts_with("```") && !in_code_block {
            in_code_block = true;
        } else if trimmed.starts_with("```") && in_code_block {
            in_code_block = false;
        }

        if in_code_block && !trimmed.starts_with("```") {
            current_text.push_str(line);
            current_text.push('\n');
            continue;
        }

        // Detect section markers
        if let Some(tool_name) = tool_marker_name(trimmed) {
            flush_text(&mut sections, &mut current_text);
            let (summary, body) = collect_tool_section_body(&mut lines_iter, &mut in_code_block);
            sections.push(MessageSection::ToolCall {
                name: tool_name.to_string(),
                summary,
                body,
            });
            continue;
        }

        if is_shell_command_marker(trimmed) {
            flush_text(&mut sections, &mut current_text);
            let body = collect_section_body(&mut lines_iter, &mut in_code_block);
            // Extract the command from the code block body
            let command = body
                .lines()
                .map(str::trim)
                .find(|l| !l.is_empty())
                .unwrap_or("(command)")
                .to_string();
            sections.push(MessageSection::ShellCommand { command });
            continue;
        }

        if is_shell_output_marker(trimmed) {
            flush_text(&mut sections, &mut current_text);
            let body = collect_section_body(&mut lines_iter, &mut in_code_block);
            sections.push(MessageSection::ShellOutput { body });
            continue;
        }

        if is_thinking_marker(trimmed) {
            flush_text(&mut sections, &mut current_text);
            let body = collect_thinking_body(&mut lines_iter);
            sections.push(MessageSection::Thinking { body });
            continue;
        }

        current_text.push_str(line);
        current_text.push('\n');
    }

    flush_text(&mut sections, &mut current_text);
    sections
}

fn flush_text(sections: &mut Vec<MessageSection>, text: &mut String) {
    let trimmed = text.trim();
    if !trimmed.is_empty() {
        sections.push(MessageSection::Text(trimmed.to_string()));
    }
    text.clear();
}

/// Collects body lines for a structured section until the next ### header or EOF.
fn collect_section_body(
    lines: &mut std::iter::Peekable<std::str::Lines<'_>>,
    in_code_block: &mut bool,
) -> String {
    let mut body = String::new();

    while let Some(line) = lines.peek() {
        let trimmed = line.trim();

        // Stop at the next section header (but only outside code blocks)
        if !*in_code_block && is_section_header_line(trimmed) {
            break;
        }

        let line = lines.next().unwrap();
        let trimmed = line.trim();

        // Track code fences
        if trimmed.starts_with("```") {
            *in_code_block = !*in_code_block;
            continue; // Skip fence markers from body
        }

        // Skip #### sub-headers (IN/OUT) — these are noise in compact view
        if trimmed.starts_with("####") {
            continue;
        }

        // Skip "summary:" lines (already captured elsewhere)
        if trimmed.starts_with("summary:") || trimmed.starts_with("summary: ") {
            continue;
        }

        body.push_str(trimmed);
        body.push('\n');
    }

    body.trim().to_string()
}

/// Collects thinking/reasoning body lines (strip blockquote markers).
fn collect_thinking_body(lines: &mut std::iter::Peekable<std::str::Lines<'_>>) -> String {
    let mut body = String::new();

    while let Some(line) = lines.peek() {
        let trimmed = line.trim();

        // Stop at the next section header
        if is_section_header_line(trimmed) {
            break;
        }

        let line = lines.next().unwrap();
        let clean = line.trim().strip_prefix("> ").unwrap_or(line.trim());

        body.push_str(clean);
        body.push('\n');
    }

    body.trim().to_string()
}

fn first_meaningful_body_line(body: &str) -> Option<&str> {
    body.lines().map(str::trim).find(|line| {
        !line.is_empty()
            && !line.starts_with('{')
            && !line.starts_with('}')
            && !line.starts_with('[')
            && !line.starts_with(']')
            && !line.starts_with("\"callID\"")
            && !line.starts_with("\"messageID\"")
            && !line.starts_with("\"sessionID\"")
            && !line.starts_with("\"metadata\"")
            && !line.starts_with("\"type\"")
            && !line.starts_with("\"state\"")
            && !line.starts_with("\"openai\"")
            && !line.starts_with("\"tool\"")
    })
}

fn collect_tool_section_body(
    lines: &mut std::iter::Peekable<std::str::Lines<'_>>,
    in_code_block: &mut bool,
) -> (Option<String>, String) {
    let mut summary: Option<String> = None;
    let mut body = String::new();

    while let Some(line) = lines.peek() {
        let trimmed = line.trim();

        if !*in_code_block && is_section_header_line(trimmed) {
            break;
        }

        let line = lines.next().unwrap();
        let trimmed = line.trim();

        if trimmed.starts_with("```") {
            *in_code_block = !*in_code_block;
            continue;
        }

        if trimmed.starts_with("summary:") && summary.is_none() {
            let value = trimmed.trim_start_matches("summary:").trim();
            if !value.is_empty() {
                summary = Some(value.to_string());
            }
            continue;
        }

        if trimmed.starts_with("####") {
            continue;
        }

        body.push_str(trimmed);
        body.push('\n');
    }

    (summary, body.trim().to_string())
}

fn tool_marker_name(line: &str) -> Option<&str> {
    line.strip_prefix("### Tool // ")
        .or_else(|| line.strip_prefix("Tool // "))
}

fn is_shell_command_marker(line: &str) -> bool {
    line == "### Shell Command" || line == "Shell Command"
}

fn is_shell_output_marker(line: &str) -> bool {
    line == "### Shell Output" || line == "Shell Output"
}

fn is_thinking_marker(line: &str) -> bool {
    line == "### Thinking" || line == "Thinking"
}

fn is_section_header_line(line: &str) -> bool {
    tool_marker_name(line).is_some()
        || is_shell_command_marker(line)
        || is_shell_output_marker(line)
        || is_thinking_marker(line)
}

fn is_hidden_noise_line(value: &str) -> bool {
    let trimmed = value.trim_start();
    trimmed.starts_with("step finished |")
        || trimmed.starts_with("shell metadata:")
        || trimmed.starts_with("shell call:")
        || trimmed.starts_with("shell status:")
}

fn line_text(line: &Line<'_>) -> String {
    line.spans
        .iter()
        .map(|span| span.content.as_ref())
        .collect()
}

// ─────────────────────────────────────────────────────────────────────
// Markdown rendering (preserved from existing impl)
// ─────────────────────────────────────────────────────────────────────

fn render_markdown_lines(
    text: &str,
    palette: ChatPalette,
    theme: &impl ComponentThemeLike,
) -> Vec<Line<'static>> {
    let parser = Parser::new_ext(text, markdown_options());
    let mut state = MarkdownState::default();
    let mut lines = Vec::<Line<'static>>::new();
    let mut current = Vec::<Span<'static>>::new();

    for event in parser {
        match event {
            Event::Start(tag) => {
                handle_start_tag(tag, &mut state, &mut current, &mut lines, theme);
            }
            Event::End(tag) => {
                handle_end_tag(tag, &mut state, &mut current, &mut lines, theme);
            }
            Event::Text(value) => {
                push_text(
                    value.as_ref(),
                    &mut state,
                    &mut current,
                    &mut lines,
                    palette,
                    theme,
                    None,
                );
            }
            Event::Code(value) => {
                state.inline_code_depth += 1;
                push_text(
                    value.as_ref(),
                    &mut state,
                    &mut current,
                    &mut lines,
                    palette,
                    theme,
                    None,
                );
                state.inline_code_depth = state.inline_code_depth.saturating_sub(1);
            }
            Event::Html(value) | Event::InlineHtml(value) => {
                push_text(
                    value.as_ref(),
                    &mut state,
                    &mut current,
                    &mut lines,
                    palette,
                    theme,
                    Some(Style::default().fg(theme.text_secondary())),
                );
            }
            Event::SoftBreak => {
                if state.code_block_depth > 0 {
                    finish_line(&mut current, &mut lines);
                } else {
                    push_text(
                        " ",
                        &mut state,
                        &mut current,
                        &mut lines,
                        palette,
                        theme,
                        None,
                    );
                }
            }
            Event::HardBreak => {
                finish_line(&mut current, &mut lines);
            }
            Event::Rule => {
                push_block_break(&mut current, &mut lines);
                lines.push(Line::from(Span::styled(
                    "  --------------------",
                    Style::default().fg(theme.text_muted()),
                )));
                push_block_break(&mut current, &mut lines);
            }
            Event::TaskListMarker(checked) => {
                let marker = if checked { "[x] " } else { "[ ] " };
                push_text(
                    marker,
                    &mut state,
                    &mut current,
                    &mut lines,
                    palette,
                    theme,
                    Some(Style::default().fg(theme.text_secondary())),
                );
            }
            Event::FootnoteReference(label) => {
                push_text(
                    &format!("[^{label}]"),
                    &mut state,
                    &mut current,
                    &mut lines,
                    palette,
                    theme,
                    Some(Style::default().fg(theme.text_secondary())),
                );
            }
            _ => {}
        }
    }

    if !current.is_empty() {
        lines.push(Line::from(current));
    }

    lines
}

fn handle_start_tag(
    tag: Tag<'_>,
    state: &mut MarkdownState,
    current: &mut Vec<Span<'static>>,
    lines: &mut Vec<Line<'static>>,
    theme: &impl ComponentThemeLike,
) {
    match tag {
        Tag::Paragraph => {}
        Tag::Heading { level, .. } => {
            push_block_break(current, lines);
            state.heading_level = Some(level);
        }
        Tag::BlockQuote(_) => {
            push_block_break(current, lines);
            state.blockquote_depth += 1;
        }
        Tag::CodeBlock(kind) => {
            push_block_break(current, lines);
            state.code_block_depth += 1;

            if let CodeBlockKind::Fenced(language) = kind {
                let language = language.trim();
                if !language.is_empty() {
                    lines.push(Line::from(Span::styled(
                        format!("  [code:{language}]"),
                        Style::default().fg(theme.text_secondary()),
                    )));
                }
            }
        }
        Tag::List(start_index) => {
            push_block_break(current, lines);
            state.list_stack.push(ListState::new(start_index));
        }
        Tag::Item => {
            finish_line(current, lines);

            let depth = state.list_stack.len();
            let marker = state
                .list_stack
                .last_mut()
                .map(|list| list.next_marker(depth))
                .unwrap_or_else(|| "- ".to_string());
            state.pending_item_prefix = Some(marker);
        }
        Tag::Emphasis => {
            state.emphasis_depth += 1;
        }
        Tag::Strong => {
            state.strong_depth += 1;
        }
        Tag::Strikethrough => {
            state.strikethrough_depth += 1;
        }
        Tag::Link { .. } => {
            state.link_depth += 1;
        }
        _ => {}
    }
}

fn handle_end_tag(
    tag: TagEnd,
    state: &mut MarkdownState,
    current: &mut Vec<Span<'static>>,
    lines: &mut Vec<Line<'static>>,
    _theme: &impl ComponentThemeLike,
) {
    match tag {
        TagEnd::Paragraph => {
            push_block_break(current, lines);
        }
        TagEnd::Heading(_) => {
            state.heading_level = None;
            push_block_break(current, lines);
        }
        TagEnd::BlockQuote(_) => {
            state.blockquote_depth = state.blockquote_depth.saturating_sub(1);
            push_block_break(current, lines);
        }
        TagEnd::CodeBlock => {
            state.code_block_depth = state.code_block_depth.saturating_sub(1);
            push_block_break(current, lines);
        }
        TagEnd::List(_) => {
            state.list_stack.pop();
            push_block_break(current, lines);
        }
        TagEnd::Item => {
            finish_line(current, lines);
        }
        TagEnd::Emphasis => {
            state.emphasis_depth = state.emphasis_depth.saturating_sub(1);
        }
        TagEnd::Strong => {
            state.strong_depth = state.strong_depth.saturating_sub(1);
        }
        TagEnd::Strikethrough => {
            state.strikethrough_depth = state.strikethrough_depth.saturating_sub(1);
        }
        TagEnd::Link => {
            state.link_depth = state.link_depth.saturating_sub(1);
        }
        _ => {}
    }
}

fn push_text(
    value: &str,
    state: &mut MarkdownState,
    current: &mut Vec<Span<'static>>,
    lines: &mut Vec<Line<'static>>,
    palette: ChatPalette,
    theme: &impl ComponentThemeLike,
    style_override: Option<Style>,
) {
    let style = style_override.unwrap_or_else(|| active_text_style(state, palette, theme));

    for (index, chunk) in value.split('\n').enumerate() {
        if index > 0 {
            finish_line(current, lines);
        }

        if chunk.is_empty() {
            continue;
        }

        ensure_line_prefix(state, current, theme);
        current.push(Span::styled(chunk.to_string(), style));
    }
}

fn ensure_line_prefix(
    state: &mut MarkdownState,
    current: &mut Vec<Span<'static>>,
    theme: &impl ComponentThemeLike,
) {
    if !current.is_empty() {
        return;
    }

    if state.blockquote_depth > 0 {
        current.push(Span::styled(
            "│ ".repeat(state.blockquote_depth),
            Style::default().fg(theme.text_secondary()),
        ));
    }

    if let Some(marker) = state.pending_item_prefix.take() {
        current.push(Span::styled(
            marker,
            Style::default().fg(theme.text_secondary()),
        ));
    } else if !state.list_stack.is_empty() {
        current.push(Span::styled(
            "  ".repeat(state.list_stack.len()),
            Style::default().fg(theme.text_muted()),
        ));
    } else if state.code_block_depth > 0 {
        current.push(Span::styled(
            "| ".to_string(),
            Style::default().fg(theme.text_secondary()),
        ));
    } else {
        current.push(Span::raw("  "));
    }
}

fn active_text_style(
    state: &MarkdownState,
    palette: ChatPalette,
    theme: &impl ComponentThemeLike,
) -> Style {
    let mut style = if state.code_block_depth > 0 {
        Style::default().fg(theme.text_secondary())
    } else {
        Style::default().fg(palette.text_primary)
    };

    if let Some(level) = state.heading_level {
        style = style
            .fg(heading_color(level, theme))
            .add_modifier(Modifier::BOLD);
    }

    if state.strong_depth > 0 {
        style = style.add_modifier(Modifier::BOLD);
    }

    if state.emphasis_depth > 0 {
        style = style.add_modifier(Modifier::ITALIC);
    }

    if state.strikethrough_depth > 0 {
        style = style.add_modifier(Modifier::CROSSED_OUT);
    }

    if state.inline_code_depth > 0 {
        style = style
            .fg(theme.pill_warn_fg())
            .bg(theme.pill_muted_bg())
            .add_modifier(Modifier::BOLD);
    }

    if state.link_depth > 0 {
        style = style
            .fg(theme.pill_info_fg())
            .add_modifier(Modifier::UNDERLINED);
    }

    style
}

fn heading_color(level: HeadingLevel, theme: &impl ComponentThemeLike) -> Color {
    match level {
        HeadingLevel::H1 | HeadingLevel::H2 => theme.pill_accent_fg(),
        HeadingLevel::H3 | HeadingLevel::H4 => theme.pill_info_fg(),
        HeadingLevel::H5 | HeadingLevel::H6 => theme.text_secondary(),
    }
}

fn finish_line(current: &mut Vec<Span<'static>>, lines: &mut Vec<Line<'static>>) {
    if current.is_empty() {
        lines.push(Line::raw(""));
    } else {
        lines.push(Line::from(std::mem::take(current)));
    }
}

fn push_block_break(current: &mut Vec<Span<'static>>, lines: &mut Vec<Line<'static>>) {
    if !current.is_empty() {
        lines.push(Line::from(std::mem::take(current)));
    }

    if let Some(last) = lines.last() {
        if !is_blank_line(last) {
            lines.push(Line::raw(""));
        }
    }
}

fn trim_blank_edges(lines: &mut Vec<Line<'static>>) {
    while matches!(lines.first(), Some(line) if is_blank_line(line)) {
        lines.remove(0);
    }

    while matches!(lines.last(), Some(line) if is_blank_line(line)) {
        lines.pop();
    }
}

fn compact_blank_lines(
    lines: Vec<Line<'static>>,
    max_body_lines: usize,
    theme: &impl ComponentThemeLike,
) -> Vec<Line<'static>> {
    let mut compacted = Vec::new();
    let mut previous_was_blank = false;

    for line in lines {
        let blank = is_blank_line(&line);
        if blank && previous_was_blank {
            continue;
        }
        previous_was_blank = blank;
        compacted.push(line);
    }

    if compacted.len() <= max_body_lines {
        return compacted;
    }

    if max_body_lines == 1 {
        return vec![Line::from(Span::styled(
            "  ...",
            Style::default().fg(theme.text_muted()),
        ))];
    }

    compacted.truncate(max_body_lines - 1);
    compacted.push(Line::from(Span::styled(
        "  ...",
        Style::default().fg(theme.text_muted()),
    )));
    compacted
}

fn is_blank_line(line: &Line<'_>) -> bool {
    line.spans.iter().all(|span| span.content.trim().is_empty())
}

fn markdown_options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options
}

#[derive(Debug, Default)]
struct MarkdownState {
    strong_depth: usize,
    emphasis_depth: usize,
    strikethrough_depth: usize,
    heading_level: Option<HeadingLevel>,
    inline_code_depth: usize,
    code_block_depth: usize,
    blockquote_depth: usize,
    link_depth: usize,
    list_stack: Vec<ListState>,
    pending_item_prefix: Option<String>,
}

#[derive(Debug, Clone, Copy)]
struct ListState {
    kind: ListKind,
}

impl ListState {
    fn new(start_index: Option<u64>) -> Self {
        let kind = match start_index {
            Some(index) => ListKind::Ordered { next: index },
            None => ListKind::Unordered,
        };

        Self { kind }
    }

    fn next_marker(&mut self, depth: usize) -> String {
        let indent = "  ".repeat(depth.saturating_sub(1));

        match &mut self.kind {
            ListKind::Unordered => format!("{indent}- "),
            ListKind::Ordered { next } => {
                let marker = format!("{indent}{}. ", *next);
                *next += 1;
                marker
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ListKind {
    Unordered,
    Ordered { next: u64 },
}

#[cfg(test)]
mod tests {
    use super::{ChatMessageEntry, ChatMessageListProps, ChatMessageRole, ChatPalette};
    use crate::components::chat_message_list::ChatMessageListComponent;
    use crate::theme::ComponentTheme;

    #[test]
    fn renders_boxed_card_with_role_header() {
        let theme = ComponentTheme::default();
        let message = ChatMessageEntry::new(
            ChatMessageRole::Assistant,
            "Hello world",
            Some("12:00".to_string()),
        );
        let props = ChatMessageListProps {
            messages: &[message],
            empty_label: "No messages",
            max_messages: 10,
            max_body_lines_per_message: 20,
            scroll_offset_lines: 0,
            palette: ChatPalette::from_theme(&theme),
        };

        let lines = ChatMessageListComponent::lines(&theme, &props, 60);
        let rendered = lines
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|span| span.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        assert!(rendered.contains("╭"));
        assert!(rendered.contains("╰"));
        assert!(rendered.contains("AGENT"));
        assert!(rendered.contains("12:00"));
        assert!(rendered.contains("Hello world"));
    }

    #[test]
    fn renders_tool_call_as_collapsible() {
        let theme = ComponentTheme::default();
        let text = "### Tool // bash\n\nsummary: git status\n\n#### IN\n```json\n\"git status\"\n```\n\n#### OUT\n```json\n\"clean\"\n```";
        let message = ChatMessageEntry::new(ChatMessageRole::Assistant, text, None);
        let props = ChatMessageListProps {
            messages: &[message],
            empty_label: "No messages",
            max_messages: 10,
            max_body_lines_per_message: 20,
            scroll_offset_lines: 0,
            palette: ChatPalette::from_theme(&theme),
        };

        let lines = ChatMessageListComponent::lines(&theme, &props, 60);
        let rendered = lines
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|span| span.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        assert!(rendered.contains("◆"));
        assert!(rendered.contains("tool bash"));
        assert!(rendered.contains("tool"));
    }

    #[test]
    fn renders_shell_command_with_cmd_pill() {
        let theme = ComponentTheme::default();
        let text = "### Shell Command\n```bash\ngit status --short\n```";
        let message = ChatMessageEntry::new(ChatMessageRole::Assistant, text, None);
        let props = ChatMessageListProps {
            messages: &[message],
            empty_label: "No messages",
            max_messages: 10,
            max_body_lines_per_message: 20,
            scroll_offset_lines: 0,
            palette: ChatPalette::from_theme(&theme),
        };

        let lines = ChatMessageListComponent::lines(&theme, &props, 60);
        let rendered = lines
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|span| span.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        assert!(rendered.contains(" CMD "));
        assert!(rendered.contains("$ "));
        assert!(rendered.contains("git status --short"));
    }

    #[test]
    fn renders_thinking_as_collapsible() {
        let theme = ComponentTheme::default();
        let text = "### Thinking\n> Let me analyze this carefully.\n> Checking the code.";
        let message = ChatMessageEntry::new(ChatMessageRole::Assistant, text, None);
        let props = ChatMessageListProps {
            messages: &[message],
            empty_label: "No messages",
            max_messages: 10,
            max_body_lines_per_message: 20,
            scroll_offset_lines: 0,
            palette: ChatPalette::from_theme(&theme),
        };

        let lines = ChatMessageListComponent::lines(&theme, &props, 60);
        let rendered = lines
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|span| span.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        assert!(rendered.contains("◆"));
        assert!(rendered.contains("Reasoning"));
        assert!(rendered.contains("thinking"));
        assert!(rendered.contains("Let me analyze"));
    }

    #[test]
    fn markdown_content_renders_structural_markers() {
        let theme = ComponentTheme::default();
        let message = ChatMessageEntry::new(
            ChatMessageRole::Assistant,
            "# Title\n\n- item one\n- item two\n\n```rust\nlet x = 1;\n```\n",
            None,
        );
        let props = ChatMessageListProps {
            messages: &[message],
            empty_label: "No messages",
            max_messages: 10,
            max_body_lines_per_message: 20,
            scroll_offset_lines: 0,
            palette: ChatPalette::from_theme(&theme),
        };

        let lines = ChatMessageListComponent::lines(&theme, &props, 80);
        let rendered = lines
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|span| span.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        assert!(rendered.contains("Title"));
        assert!(rendered.contains("- item one"));
        assert!(rendered.contains("[code:rust]"));
        assert!(rendered.contains("let x = 1;"));
    }

    #[test]
    fn markdown_body_obeys_line_cap() {
        let theme = ComponentTheme::default();
        let message = ChatMessageEntry::new(
            ChatMessageRole::Assistant,
            "line1\n\nline2\n\nline3\n\nline4\n\nline5",
            None,
        );
        let props = ChatMessageListProps {
            messages: &[message],
            empty_label: "No messages",
            max_messages: 10,
            max_body_lines_per_message: 3,
            scroll_offset_lines: 0,
            palette: ChatPalette::from_theme(&theme),
        };

        let lines = ChatMessageListComponent::lines(&theme, &props, 80);
        let rendered = lines
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|span| span.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        assert!(rendered.contains("line1"));
        assert!(rendered.contains("..."));
        assert!(!rendered.contains("line5"));
    }

    #[test]
    fn markdown_list_continuation_does_not_repeat_marker() {
        let theme = ComponentTheme::default();
        let message = ChatMessageEntry::new(
            ChatMessageRole::Assistant,
            "- first line  \ncontinuation line",
            None,
        );
        let props = ChatMessageListProps {
            messages: &[message],
            empty_label: "No messages",
            max_messages: 10,
            max_body_lines_per_message: 20,
            scroll_offset_lines: 0,
            palette: ChatPalette::from_theme(&theme),
        };

        let lines = ChatMessageListComponent::lines(&theme, &props, 80);
        let rendered_lines = lines
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|span| span.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<_>>();

        assert!(rendered_lines
            .iter()
            .any(|line| line.contains("- first line")));
        assert!(rendered_lines
            .iter()
            .any(|line| line.contains("  continuation line")));
        assert!(!rendered_lines
            .iter()
            .any(|line| line.contains("- continuation line")));
    }

    #[test]
    fn empty_messages_show_empty_label() {
        let theme = ComponentTheme::default();
        let props = ChatMessageListProps {
            messages: &[],
            empty_label: "Nothing here",
            max_messages: 10,
            max_body_lines_per_message: 20,
            scroll_offset_lines: 0,
            palette: ChatPalette::from_theme(&theme),
        };

        let lines = ChatMessageListComponent::lines(&theme, &props, 60);
        let rendered = lines
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|span| span.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        assert!(rendered.contains("Nothing here"));
        // No box chrome for empty state
        assert!(!rendered.contains("╭"));
    }
}
