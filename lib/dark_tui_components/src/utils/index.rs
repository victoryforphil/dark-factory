/// Moves selection to the previous index with wrap-around.
pub fn previous_index(current: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }

    if current == 0 { len - 1 } else { current - 1 }
}

/// Moves selection to the next index with wrap-around.
pub fn next_index(current: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }

    if current >= len - 1 { 0 } else { current + 1 }
}
