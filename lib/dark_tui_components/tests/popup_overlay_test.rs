use dark_tui_components::{
    ComponentTheme, FooterBar, FooterBarProps, PopupAnchor, PopupHit, PopupItem, PopupOverlay,
    PopupOverlayProps,
};
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui::text::Span;

#[test]
fn popup_overlay_area_is_clamped_to_parent() {
    let parent = Rect::new(0, 0, 32, 10);
    let props = PopupOverlayProps {
        title: "Picker".to_string(),
        items: vec![PopupItem {
            label: "first".to_string(),
            tag: None,
            active: false,
        }],
        selected: 0,
        query: Some("abc".to_string()),
        query_label: Some("FILTER".to_string()),
        hint: Some("hint".to_string()),
        anchor: PopupAnchor::At { x: 200, y: 200 },
        max_visible: 6,
        min_width: 24,
        max_width: 48,
    };

    let area = PopupOverlay::area(parent, &props).expect("area should exist");
    assert!(area.x + area.width <= parent.x + parent.width);
    assert!(area.y + area.height <= parent.y + parent.height);
}

#[test]
fn popup_overlay_hit_test_distinguishes_regions() {
    let parent = Rect::new(0, 0, 40, 12);
    let props = PopupOverlayProps {
        title: "Model Picker".to_string(),
        items: vec![
            PopupItem {
                label: "a".to_string(),
                tag: None,
                active: false,
            },
            PopupItem {
                label: "b".to_string(),
                tag: None,
                active: false,
            },
        ],
        selected: 0,
        query: Some("mo".to_string()),
        query_label: Some("FILTER".to_string()),
        hint: None,
        anchor: PopupAnchor::Center,
        max_visible: 6,
        min_width: 24,
        max_width: 40,
    };

    let area = PopupOverlay::area(parent, &props).expect("area should exist");
    assert_eq!(
        PopupOverlay::hit_test(parent, &props, 0, 0),
        PopupHit::Outside
    );

    let list_hit = PopupOverlay::hit_test(
        parent,
        &props,
        area.x.saturating_add(2),
        area.y.saturating_add(2),
    );
    assert!(matches!(list_hit, PopupHit::ListItem(_)));

    let query_hit = PopupOverlay::hit_test(
        parent,
        &props,
        area.x.saturating_add(2),
        area.y.saturating_add(area.height.saturating_sub(2)),
    );
    assert_eq!(query_hit, PopupHit::Query);
}

#[test]
fn footer_bar_render_writes_segments() {
    let theme = ComponentTheme::default();
    let backend = TestBackend::new(40, 2);
    let mut terminal = Terminal::new(backend).expect("terminal should initialize");

    terminal
        .draw(|frame| {
            FooterBar::render(
                frame,
                Rect::new(0, 0, 40, 1),
                FooterBarProps {
                    segments: vec![Span::raw("left"), Span::raw("right")],
                    separator: " | ",
                },
                &theme,
            );
        })
        .expect("draw should succeed");

    let mut row = String::new();
    for x in 0..12 {
        row.push_str(
            terminal
                .backend()
                .buffer()
                .cell((x, 0))
                .expect("cell")
                .symbol(),
        );
    }

    assert!(row.contains("left"));
    assert!(row.contains("|"));
}
