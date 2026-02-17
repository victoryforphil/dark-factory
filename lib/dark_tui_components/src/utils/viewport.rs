/// Viewport window for rendering a subrange of list items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListViewport {
    pub start: usize,
    pub end: usize,
    pub visible: usize,
}

impl ListViewport {
    /// Computes a viewport range anchored around `selected`.
    pub fn new(total: usize, visible: usize, selected: usize) -> Self {
        let safe_visible = visible.max(1);
        let start = if selected >= safe_visible {
            selected - safe_visible + 1
        } else {
            0
        }
        .min(total.saturating_sub(safe_visible));
        let end = (start + safe_visible).min(total);

        Self {
            start,
            end,
            visible: safe_visible,
        }
    }

    /// Returns whether `abs_index` is the currently selected index.
    pub fn is_selected(&self, abs_index: usize, selected: usize) -> bool {
        abs_index == selected
    }

    /// Converts an absolute index into viewport-relative row index.
    pub fn relative_index(&self, abs_index: usize) -> Option<usize> {
        if abs_index >= self.start && abs_index < self.end {
            Some(abs_index - self.start)
        } else {
            None
        }
    }
}
