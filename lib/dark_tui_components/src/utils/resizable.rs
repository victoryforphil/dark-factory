use ratatui::layout::Rect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThreePanePercents {
    pub left: u16,
    pub middle: u16,
    pub right: u16,
}

impl ThreePanePercents {
    pub fn from_middle_right(
        middle: u16,
        right: u16,
        min_left: u16,
        min_middle: u16,
        min_right: u16,
    ) -> Self {
        let mut middle = middle.clamp(min_middle, 100);
        let mut right = right.clamp(min_right, 100);
        let min_total = min_left
            .saturating_add(min_middle)
            .saturating_add(min_right);

        if min_total > 100 {
            return Self {
                left: 0,
                middle,
                right,
            };
        }

        if middle.saturating_add(right).saturating_add(min_left) > 100 {
            let allowed = 100u16.saturating_sub(min_left);
            if right > allowed.saturating_sub(min_middle) {
                right = allowed.saturating_sub(min_middle);
            }
            middle = allowed.saturating_sub(right).max(min_middle);
        }

        let left = 100u16.saturating_sub(middle).saturating_sub(right);
        Self {
            left,
            middle,
            right,
        }
    }
}

pub fn percent_from_right_edge(area: Rect, col: u16) -> Option<u16> {
    if area.width == 0 {
        return None;
    }

    let width = area.width as f32;
    let right = area.x.saturating_add(area.width);
    let cols_from_right = right.saturating_sub(col).min(area.width);
    Some(((cols_from_right as f32 / width) * 100.0) as u16)
}

pub fn percent_from_left_edge(area: Rect, col: u16) -> Option<u16> {
    if area.width == 0 {
        return None;
    }

    let width = area.width as f32;
    let cols_from_left = col.saturating_sub(area.x).min(area.width);
    Some(((cols_from_left as f32 / width) * 100.0) as u16)
}
