use dark_tui_components::{ComponentTheme, KeyBind, KeyHintBar, LoadingSpinner, StatusPill};

#[test]
fn key_hint_bar_produces_non_empty_lines() {
    let theme = ComponentTheme::default();
    let binds = [KeyBind::new("q", "Quit"), KeyBind::new("r", "Refresh")];
    let lines = KeyHintBar::new(&binds).lines_wrapped(64, &theme);
    assert!(!lines.is_empty());
}

#[test]
fn status_pill_wraps_label_with_padding() {
    let theme = ComponentTheme::default();
    let span = StatusPill::info("runtime:ok", &theme).span();
    assert_eq!(span.content, " runtime:ok ");
}

#[test]
fn spinner_frames_remain_ascii_safe() {
    let valid = ["-", "\\", "|", "/"];
    let glyph = LoadingSpinner::glyph_for_elapsed(240);
    assert!(valid.contains(&glyph));
}
