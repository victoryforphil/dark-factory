use ratatui::layout::Rect;

#[derive(Debug, Clone)]
pub struct HorizontalSplit {
    percents: Vec<u16>,
    min_percents: Vec<u16>,
}

impl HorizontalSplit {
    pub fn new(percents: Vec<u16>, min_percents: Vec<u16>) -> Self {
        let mut split = Self {
            percents,
            min_percents,
        };
        split.normalize();
        split
    }

    pub fn two(left_percent: u16, right_percent: u16, min_left: u16, min_right: u16) -> Self {
        Self::new(vec![left_percent, right_percent], vec![min_left, min_right])
    }

    pub fn three(
        left_percent: u16,
        middle_percent: u16,
        right_percent: u16,
        min_left: u16,
        min_middle: u16,
        min_right: u16,
    ) -> Self {
        Self::new(
            vec![left_percent, middle_percent, right_percent],
            vec![min_left, min_middle, min_right],
        )
    }

    pub fn percents(&self) -> &[u16] {
        &self.percents
    }

    pub fn resolve(&self, area: Rect) -> Vec<Rect> {
        if self.percents.is_empty() || area.width == 0 {
            return Vec::new();
        }

        let mut output = Vec::with_capacity(self.percents.len());
        let mut current_x = area.x;

        for (index, percent) in self.percents.iter().copied().enumerate() {
            let width = if index == self.percents.len() - 1 {
                area.x.saturating_add(area.width).saturating_sub(current_x)
            } else {
                ((area.width as u32 * percent as u32) / 100) as u16
            };

            output.push(Rect {
                x: current_x,
                y: area.y,
                width,
                height: area.height,
            });
            current_x = current_x.saturating_add(width);
        }

        output
    }

    pub fn divider_col(&self, area: Rect, divider_index: usize) -> Option<u16> {
        if divider_index + 1 >= self.percents.len() {
            return None;
        }

        let segments = self.resolve(area);
        segments
            .get(divider_index + 1)
            .map(|segment| segment.x.saturating_sub(1))
    }

    pub fn divider_hit(&self, area: Rect, col: u16, tolerance: u16) -> Option<usize> {
        (0..self.percents.len().saturating_sub(1)).find(|divider_index| {
            self.divider_col(area, *divider_index)
                .is_some_and(|divider_col| divider_col.abs_diff(col) <= tolerance)
        })
    }

    pub fn resize_from_pointer(&mut self, area: Rect, divider_index: usize, col: u16) -> bool {
        if area.width == 0 || divider_index + 1 >= self.percents.len() {
            return false;
        }

        self.normalize();

        let left_total_before: u16 = self.percents.iter().take(divider_index).copied().sum();
        let pair_total =
            self.percents[divider_index].saturating_add(self.percents[divider_index + 1]);
        let min_left = *self.min_percents.get(divider_index).unwrap_or(&0);
        let min_right = *self.min_percents.get(divider_index + 1).unwrap_or(&0);

        let pointer_percent = (((col.saturating_sub(area.x) as f32) / area.width as f32) * 100.0)
            .round()
            .clamp(0.0, 100.0) as i32;
        let desired_left = pointer_percent - left_total_before as i32;

        let min_allowed_left = min_left as i32;
        let max_allowed_left = pair_total.saturating_sub(min_right) as i32;
        let clamped_left = desired_left.clamp(min_allowed_left, max_allowed_left) as u16;
        let clamped_right = pair_total.saturating_sub(clamped_left);

        let changed = self.percents[divider_index] != clamped_left
            || self.percents[divider_index + 1] != clamped_right;

        self.percents[divider_index] = clamped_left;
        self.percents[divider_index + 1] = clamped_right;
        self.normalize();

        changed
    }

    fn normalize(&mut self) {
        if self.percents.len() < 2 || self.percents.len() != self.min_percents.len() {
            return;
        }

        for (percent, min_percent) in self.percents.iter_mut().zip(self.min_percents.iter()) {
            if *percent < *min_percent {
                *percent = *min_percent;
            }
        }

        let min_sum: u16 = self.min_percents.iter().copied().sum();
        if min_sum > 100 {
            let equal = 100 / self.percents.len() as u16;
            self.percents.fill(equal);
            let mut remainder = 100u16.saturating_sub(equal * self.percents.len() as u16);
            let mut index = 0usize;
            while remainder > 0 {
                self.percents[index] = self.percents[index].saturating_add(1);
                remainder = remainder.saturating_sub(1);
                index = (index + 1) % self.percents.len();
            }
            return;
        }

        let mut sum: u16 = self.percents.iter().copied().sum();
        while sum > 100 {
            let mut reduced = false;
            for index in (0..self.percents.len()).rev() {
                if self.percents[index] > self.min_percents[index] {
                    self.percents[index] = self.percents[index].saturating_sub(1);
                    sum = sum.saturating_sub(1);
                    reduced = true;
                    if sum == 100 {
                        break;
                    }
                }
            }

            if !reduced {
                break;
            }
        }

        while sum < 100 {
            self.percents[0] = self.percents[0].saturating_add(1);
            sum = sum.saturating_add(1);
        }
    }
}
