use std::time::{SystemTime, UNIX_EPOCH};

/// Minimal animated spinner with ASCII-safe frames.
pub struct LoadingSpinner;

impl LoadingSpinner {
    const FRAMES: [&str; 4] = ["-", "\\", "|", "/"];
    const STEP_MS: u128 = 120;

    pub fn glyph() -> &'static str {
        let elapsed_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let index = ((elapsed_ms / Self::STEP_MS) % (Self::FRAMES.len() as u128)) as usize;
        Self::FRAMES[index]
    }
}
