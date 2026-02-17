use ratatui::text::Span;

use dark_tui_components::StatusPill;

use crate::theme::Theme;

/// Renders a compact sub-agent count badge as a `StatusPill` span.
///
/// Returns `None` when the count is zero so callers can skip rendering
/// rather than showing a meaningless `0 agents` pill.
///
/// Visual style uses the accent pill palette to distinguish sub-agent
/// metadata from status/provider badges without competing for attention.
pub(crate) fn sub_agent_badge(count: usize, theme: &Theme) -> Option<Span<'static>> {
    if count == 0 {
        return None;
    }

    let label = if count == 1 {
        "1 sub-agent".to_string()
    } else {
        format!("{count} sub-agents")
    };

    Some(StatusPill::accent(label, theme).span())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn badge_returns_none_for_zero() {
        let theme = Theme::default();
        assert!(sub_agent_badge(0, &theme).is_none());
    }

    #[test]
    fn badge_singular_label() {
        let theme = Theme::default();
        let span = sub_agent_badge(1, &theme).expect("should produce span");
        assert_eq!(span.content.as_ref(), " 1 sub-agent ");
    }

    #[test]
    fn badge_plural_label() {
        let theme = Theme::default();
        let span = sub_agent_badge(3, &theme).expect("should produce span");
        assert_eq!(span.content.as_ref(), " 3 sub-agents ");
    }
}
