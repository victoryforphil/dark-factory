use ratatui::layout::Rect;

/// Returns whether a terminal cell lies inside the rectangle.
pub fn rect_contains(area: Rect, col: u16, row: u16) -> bool {
    col >= area.x && col < area.x + area.width && row >= area.y && row < area.y + area.height
}

/// Returns an inner rect inset by one cell on each edge.
pub fn inner_rect(area: Rect) -> Rect {
    Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    }
}

/// Appends a block cursor marker to the end of text.
pub fn with_cursor_tail(text: &str) -> String {
    let trimmed = text.trim_end();
    if trimmed.is_empty() {
        "\u{2588}".to_string()
    } else {
        format!("{trimmed}\u{2588}")
    }
}
