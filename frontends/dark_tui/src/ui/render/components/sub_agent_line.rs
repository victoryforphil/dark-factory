use ratatui::style::Style;
use ratatui::text::{Line, Span};

use dark_tui_components::StatusPill;

use crate::models::SubAgentRow;
use crate::theme::Theme;

/// Renders a compact inline sub-agent line for tree/catalog views.
///
/// The `base_indent` controls the leading whitespace before the tree
/// connector. Each sub-agent's `depth` adds additional indentation to
/// visualize nesting. Lines use muted styling and are purely decorative
/// — they must never be selectable or clickable.
pub(crate) fn sub_agent_tree_line<'a>(
    agent: &SubAgentRow,
    base_indent: &str,
    is_last: bool,
    theme: &Theme,
) -> Line<'a> {
    let depth_pad = "  ".repeat(agent.depth);
    let connector = if is_last { "└╴" } else { "├╴" };

    let title_text = if agent.title.len() > 24 {
        format!("{}…", &agent.title[..23])
    } else {
        agent.title.clone()
    };

    let status_pill = match agent.status.as_str() {
        "active" | "running" => StatusPill::ok(&agent.status, theme),
        "error" | "failed" | "dead" => StatusPill::error(&agent.status, theme),
        "idle" | "waiting" => StatusPill::warn(&agent.status, theme),
        "-" => StatusPill::muted("–", theme),
        _ => StatusPill::muted(&agent.status, theme),
    };

    let mut spans: Vec<Span<'static>> = vec![
        Span::styled(
            format!("{base_indent}{depth_pad}{connector}"),
            Style::default().fg(theme.catalog_connector),
        ),
        Span::styled(
            format!("⚙ {title_text}"),
            Style::default().fg(theme.text_muted),
        ),
        Span::raw("  "),
        status_pill.span(),
    ];

    // Append a compact summary when meaningful.
    if !agent.summary.is_empty() && agent.summary != "-" {
        let summary_display = if agent.summary.len() > 28 {
            format!("{}…", &agent.summary[..27])
        } else {
            agent.summary.clone()
        };
        spans.push(Span::raw(" "));
        spans.push(Span::styled(
            summary_display,
            Style::default().fg(theme.text_muted),
        ));
    }

    Line::from(spans)
}

/// Compact single-row sub-agent summary for space-constrained views
/// (e.g. inside actor cards in the 2D catalog layout).
///
/// Returns `None` when no sub-agents exist.
#[allow(dead_code)]
pub(crate) fn sub_agent_summary_line<'a>(
    agents: &[SubAgentRow],
    theme: &Theme,
) -> Option<Line<'a>> {
    if agents.is_empty() {
        return None;
    }

    let count = agents.len();
    let label = if count == 1 {
        "1 sub-agent".to_string()
    } else {
        format!("{count} sub-agents")
    };

    // Show up to 2 top-level names inline.
    let top_names: Vec<&str> = agents
        .iter()
        .filter(|a| a.depth == 0)
        .take(2)
        .map(|a| a.title.as_str())
        .collect();
    let names_hint = if top_names.is_empty() {
        String::new()
    } else {
        let joined = top_names.join(", ");
        let truncated = if joined.len() > 32 {
            format!("{}…", &joined[..31])
        } else {
            joined
        };
        format!(" ({truncated})")
    };

    Some(Line::from(vec![Span::styled(
        format!("  ⚙ {label}{names_hint}"),
        Style::default().fg(theme.text_muted),
    )]))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_agent(title: &str, depth: usize, status: &str, summary: &str) -> SubAgentRow {
        SubAgentRow {
            id: format!("sa_{title}"),
            parent_id: None,
            title: title.to_string(),
            status: status.to_string(),
            summary: summary.to_string(),
            updated_at: String::new(),
            depth,
        }
    }

    #[test]
    fn tree_line_renders_last_connector() {
        let theme = Theme::default();
        let agent = make_agent("worker", 0, "active", "-");
        let line = sub_agent_tree_line(&agent, "      ", true, &theme);
        let text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(text.contains("└╴"));
        assert!(text.contains("⚙ worker"));
    }

    #[test]
    fn tree_line_renders_mid_connector() {
        let theme = Theme::default();
        let agent = make_agent("worker", 0, "idle", "doing stuff");
        let line = sub_agent_tree_line(&agent, "      ", false, &theme);
        let text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(text.contains("├╴"));
        assert!(text.contains("doing stuff"));
    }

    #[test]
    fn tree_line_depth_indents() {
        let theme = Theme::default();
        let agent = make_agent("nested", 2, "active", "-");
        let line = sub_agent_tree_line(&agent, "  ", true, &theme);
        let prefix: String = line
            .spans
            .first()
            .map(|s| s.content.to_string())
            .unwrap_or_default();
        // base "  " + depth "    " (2*2 spaces) + connector
        assert!(prefix.starts_with("      "));
    }

    #[test]
    fn summary_line_none_for_empty() {
        let theme = Theme::default();
        assert!(sub_agent_summary_line(&[], &theme).is_none());
    }

    #[test]
    fn summary_line_shows_count_and_names() {
        let theme = Theme::default();
        let agents = vec![
            make_agent("alpha", 0, "active", "-"),
            make_agent("beta", 0, "active", "-"),
        ];
        let line = sub_agent_summary_line(&agents, &theme).unwrap();
        let text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(text.contains("2 sub-agents"));
        assert!(text.contains("alpha"));
        assert!(text.contains("beta"));
    }
}
