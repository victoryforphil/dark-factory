use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};

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

/// Renderer for chat message history with markdown-aware formatting.
pub struct ChatMessageListComponent;

impl ChatMessageListComponent {
    /// Renders the message list into the target area.
    pub fn render(
        frame: &mut Frame,
        area: Rect,
        theme: &impl ComponentThemeLike,
        props: ChatMessageListProps<'_>,
    ) {
        let lines = Self::lines(theme, &props);
        let viewport_height = area.height as usize;
        let total_lines = lines.len();
        let base_scroll = total_lines.saturating_sub(viewport_height);
        let scroll = base_scroll.saturating_sub(props.scroll_offset_lines as usize) as u16;

        frame.render_widget(
            Paragraph::new(lines)
                .wrap(Wrap { trim: false })
                .scroll((scroll, 0)),
            area,
        );
    }

    fn lines(
        theme: &impl ComponentThemeLike,
        props: &ChatMessageListProps<'_>,
    ) -> Vec<Line<'static>> {
        if props.messages.is_empty() {
            return vec![Line::styled(
                props.empty_label.to_string(),
                Style::default().fg(theme.text_muted()),
            )];
        }

        let mut lines = Vec::new();
        let cap = props.max_messages.max(1);
        let start = props.messages.len().saturating_sub(cap);

        for message in &props.messages[start..] {
            let role_style = Style::default()
                .fg(props.palette.role_color(&message.role))
                .add_modifier(Modifier::BOLD);

            let mut header_spans = vec![Span::styled(
                format!("[{}]", role_label(&message.role)),
                role_style,
            )];

            if let Some(created_at) = message.created_at.as_ref() {
                header_spans.push(Span::raw(" "));
                header_spans.push(Span::styled(
                    created_at.clone(),
                    Style::default().fg(theme.text_muted()),
                ));
            }

            lines.push(Line::from(header_spans));

            let mut rendered_body = render_message_body(
                message,
                props.palette,
                props.max_body_lines_per_message,
                theme,
            );

            if rendered_body.is_empty() {
                rendered_body.push(Line::from(Span::styled(
                    "  (no content)",
                    Style::default().fg(theme.text_muted()),
                )));
            }

            lines.extend(rendered_body);
            lines.push(Line::from(Span::styled(
                "------------------------",
                Style::default().fg(theme.text_muted()),
            )));
            lines.push(Line::raw(""));
        }

        lines
    }
}

fn render_message_body(
    message: &ChatMessageEntry,
    palette: ChatPalette,
    max_body_lines: usize,
    theme: &impl ComponentThemeLike,
) -> Vec<Line<'static>> {
    let normalized = message.text.replace("\r\n", "\n");
    let mut lines = render_markdown_lines(&normalized, palette, theme);

    if lines.is_empty() {
        return lines;
    }

    trim_blank_edges(&mut lines);
    compact_blank_lines(lines, max_body_lines.max(1), theme)
}

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

        let lines = ChatMessageListComponent::lines(&theme, &props);
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

        let lines = ChatMessageListComponent::lines(&theme, &props);
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

        let lines = ChatMessageListComponent::lines(&theme, &props);
        let rendered_lines = lines
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|span| span.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<_>>();

        assert!(
            rendered_lines
                .iter()
                .any(|line| line.contains("- first line"))
        );
        assert!(
            rendered_lines
                .iter()
                .any(|line| line.contains("  continuation line"))
        );
        assert!(
            !rendered_lines
                .iter()
                .any(|line| line.contains("- continuation line"))
        );
    }
}

fn role_label(role: &ChatMessageRole) -> &str {
    match role {
        ChatMessageRole::User => "YOU",
        ChatMessageRole::Assistant => "AI",
        ChatMessageRole::System => "SYS",
        ChatMessageRole::Tool => "TOOL",
        ChatMessageRole::Other(value) => value.as_str(),
    }
}
