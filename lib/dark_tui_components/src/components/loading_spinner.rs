use std::time::{SystemTime, UNIX_EPOCH};

/// Stateless spinner glyph helper for status labels.
pub struct LoadingSpinner;

impl LoadingSpinner {
    const FRAMES: [&str; 4] = ["-", "\\", "|", "/"];
    const STEP_MS: u128 = 120;

    /// Returns the spinner glyph for the current wall-clock time.
    pub fn glyph() -> &'static str {
        let elapsed_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        Self::glyph_for_elapsed(elapsed_ms)
    }

    /// Returns the spinner glyph for an explicit elapsed millisecond value.
    pub fn glyph_for_elapsed(elapsed_ms: u128) -> &'static str {
        let index = ((elapsed_ms / Self::STEP_MS) % (Self::FRAMES.len() as u128)) as usize;
        Self::FRAMES[index]
    }
}

#[cfg(test)]
mod tests {
    use super::LoadingSpinner;

    #[test]
    fn glyph_cycles_in_order() {
        assert_eq!(LoadingSpinner::glyph_for_elapsed(0), "-");
        assert_eq!(LoadingSpinner::glyph_for_elapsed(120), "\\");
        assert_eq!(LoadingSpinner::glyph_for_elapsed(240), "|");
        assert_eq!(LoadingSpinner::glyph_for_elapsed(360), "/");
        assert_eq!(LoadingSpinner::glyph_for_elapsed(480), "-");
    }
}
