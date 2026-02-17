use dark_tui_components::{
    compact_id, compact_id_len, compact_label, compact_locator, compact_session_id, compact_tail,
    compact_text, compact_text_normalized, compact_timestamp, inner_rect, next_index,
    previous_index, rect_contains, with_cursor_tail, HorizontalSplit,
};
use ratatui::layout::Rect;

#[test]
fn compact_text_truncates_with_ellipsis() {
    assert_eq!(compact_text("hello", 10), "hello");
    assert_eq!(compact_text("abcdefghijklmnopqrstuvwxyz", 8), "abcde...");
}

#[test]
fn compact_text_handles_unicode_boundaries() {
    assert_eq!(compact_text("alpha beta", 5), "al...");
    assert_eq!(compact_text("zeta \u{03A9}", 5), "ze...");
}

#[test]
fn compact_text_normalized_replaces_newlines() {
    assert_eq!(compact_text_normalized("  one\ntwo  ", 20), "one two");
    assert_eq!(compact_text_normalized("long value here", 7), "long...");
}

#[test]
fn compact_tail_preserves_tail() {
    assert_eq!(compact_tail("abc", 8), "abc");
    assert_eq!(compact_tail("abcdefghijklmnopqrstuvwxyz", 8), "...vwxyz");
    assert_eq!(compact_tail("abcdef", 2), "..");
}

#[test]
fn compact_label_handles_none_and_truncation() {
    assert_eq!(compact_label(None, 10), "-");
    assert_eq!(compact_label(Some("model-123456"), 8), "model...");
    assert_eq!(compact_label(Some("a"), 1), "a");
}

#[test]
fn compact_id_and_locator_helpers() {
    assert_eq!(compact_id("short-id"), "short-id");
    assert_eq!(compact_id("abcdefghijklmno"), "abcdefghijkl...");
    assert_eq!(compact_id_len("abcdefghij", 6), "abcdef...");
    assert_eq!(
        compact_locator("@local://workspace/feature", 12),
        "...e/feature"
    );
}

#[test]
fn compact_timestamp_formats_iso() {
    assert_eq!(compact_timestamp(""), "-");
    assert_eq!(
        compact_timestamp("2026-02-16T12:34:56.000Z"),
        "2026-02-16 12:34:56"
    );
    assert_eq!(compact_timestamp("unix:123"), "unix:123");
}

#[test]
fn compact_session_id_limits_length() {
    assert_eq!(compact_session_id("short"), "short");
    assert_eq!(compact_session_id("12345678901234567890"), "12345678901234");
}

#[test]
fn rect_helpers_cover_bounds() {
    let area = Rect::new(10, 5, 4, 3);
    assert!(rect_contains(area, 10, 5));
    assert!(rect_contains(area, 13, 7));
    assert!(!rect_contains(area, 14, 7));
    assert!(!rect_contains(area, 13, 8));

    let inner = inner_rect(area);
    assert_eq!(inner, Rect::new(11, 6, 2, 1));

    let tiny = inner_rect(Rect::new(0, 0, 1, 1));
    assert_eq!(tiny.width, 0);
    assert_eq!(tiny.height, 0);
}

#[test]
fn with_cursor_tail_appends_block_cursor() {
    assert_eq!(with_cursor_tail(""), "\u{2588}");
    assert_eq!(with_cursor_tail("draft"), "draft\u{2588}");
    assert_eq!(with_cursor_tail("draft   "), "draft\u{2588}");
}

#[test]
fn index_helpers_wrap_in_both_directions() {
    assert_eq!(previous_index(0, 0), 0);
    assert_eq!(next_index(0, 0), 0);
    assert_eq!(previous_index(0, 5), 4);
    assert_eq!(next_index(4, 5), 0);
    assert_eq!(next_index(2, 5), 3);
}

#[test]
fn split_layout_resolves_expected_widths() {
    let split = HorizontalSplit::three(24, 54, 22, 16, 20, 16);
    let area = Rect::new(0, 0, 100, 10);
    let columns = split.resolve(area);

    assert_eq!(columns.len(), 3);
    assert_eq!(columns[0].width, 24);
    assert_eq!(columns[1].width, 54);
    assert_eq!(columns[2].width, 22);
}

#[test]
fn split_layout_divider_hit_detects_boundary() {
    let split = HorizontalSplit::three(24, 54, 22, 16, 20, 16);
    let area = Rect::new(0, 0, 100, 10);
    assert_eq!(split.divider_hit(area, 23, 1), Some(0));
    assert_eq!(split.divider_hit(area, 77, 1), Some(1));
    assert_eq!(split.divider_hit(area, 50, 1), None);
}

#[test]
fn split_layout_resize_respects_minimums() {
    let mut split = HorizontalSplit::three(24, 54, 22, 16, 20, 16);
    let area = Rect::new(0, 0, 100, 10);

    let changed = split.resize_from_pointer(area, 0, 2);
    assert!(changed);

    let percents = split.percents();
    assert_eq!(percents.len(), 3);
    assert!(percents[0] >= 16);
    assert!(percents[1] >= 20);
    assert_eq!(percents.iter().copied().sum::<u16>(), 100);
}
