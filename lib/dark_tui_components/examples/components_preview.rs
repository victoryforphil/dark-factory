use dark_tui_components::{
    ComponentTheme, KeyBind, KeyHintBar, LabeledField, SectionHeader, StatusPill,
};

fn main() {
    let theme = ComponentTheme::default();

    let hints = [
        KeyBind::new("q", "Quit"),
        KeyBind::new("r", "Refresh"),
        KeyBind::new("v", "View"),
    ];
    let lines = KeyHintBar::new(&hints).lines_wrapped(30, &theme);

    let label = LabeledField::new("Branch", "main").line_compact(&theme);
    let header = SectionHeader::new("Identity", theme.pill_info_fg).line(28, &theme);
    let pill = StatusPill::ok("clean", &theme).span();

    println!("{}", header);
    println!("{}", label);
    println!("{}", pill.content);
    println!("wrapped hint lines: {}", lines.len());
}
