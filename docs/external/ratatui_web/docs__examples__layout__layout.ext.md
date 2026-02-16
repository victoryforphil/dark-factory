----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /examples/layout/layout
- Keywords: ratatui, rust, tui, terminal ui, docs, examples, layout, layout
- Summary: Demonstrates the [`Layout`](https://docs.rs/ratatui/latest/ratatui/layout/struct.Layout.html).
----

Source: https://ratatui.rs/examples/layout/layout

# Layout

Demonstrates the [`Layout`](https://docs.rs/ratatui/latest/ratatui/layout/struct.Layout.html).

run

```
git clone https://github.com/ratatui/ratatui.git --branch latestcd ratatuicargo run --example=layout --features=crossterm
```

layout.rs

```
//! # [Ratatui] Layout example//!//! The latest version of this example is available in the [examples] folder in the repository.//!//! Please note that the examples are designed to be run against the `main` branch of the Github//! repository. This means that you may not be able to compile with the latest release version on//! crates.io, or the one that you have installed locally.//!//! See the [examples readme] for more information on finding examples that match the version of the//! library you are using.//!//! [Ratatui]: https://github.com/ratatui/ratatui//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md
use itertools::Itertools;use ratatui::{    crossterm::event::{self, Event, KeyCode, KeyEventKind},    layout::{        Constraint::{self, Length, Max, Min, Percentage, Ratio},        Layout, Rect,    },    style::{Color, Style, Stylize},    text::Line,    widgets::{Block, Paragraph},    DefaultTerminal, Frame,};
fn main() -> color_eyre::Result&#x3C;()> {    color_eyre::install()?;    let terminal = ratatui::init();    let app_result = run(terminal);    ratatui::restore();    app_result}
fn run(mut terminal: DefaultTerminal) -> color_eyre::Result&#x3C;()> {    loop {        terminal.draw(draw)?;        if let Event::Key(key) = event::read()? {            if key.kind == KeyEventKind::Press &#x26;&#x26; key.code == KeyCode::Char('q') {                break Ok(());            }        }    }}
#[allow(clippy::too_many_lines)]fn draw(frame: &#x26;mut Frame) {    let vertical = Layout::vertical([        Length(4),  // text        Length(50), // examples        Min(0),     // fills remaining space    ]);    let [text_area, examples_area, _] = vertical.areas(frame.area());
    // title    frame.render_widget(        Paragraph::new(vec![            Line::from("Horizontal Layout Example. Press q to quit".dark_gray()).centered(),            Line::from("Each line has 2 constraints, plus Min(0) to fill the remaining space."),            Line::from("E.g. the second line of the Len/Min box is [Length(2), Min(2), Min(0)]"),            Line::from("Note: constraint labels that don't fit are truncated"),        ]),        text_area,    );
    let example_rows = Layout::vertical([        Length(9),        Length(9),        Length(9),        Length(9),        Length(9),        Min(0), // fills remaining space    ])    .split(examples_area);    let example_areas = example_rows.iter().flat_map(|area| {        Layout::horizontal([            Length(14),            Length(14),            Length(14),            Length(14),            Length(14),            Min(0), // fills remaining space        ])        .split(*area)        .iter()        .copied()        .take(5) // ignore Min(0)        .collect_vec()    });
    // the examples are a cartesian product of the following constraints    // e.g. Len/Len, Len/Min, Len/Max, Len/Perc, Len/Ratio, Min/Len, Min/Min, ...    let examples = [        (            "Len",            [                Length(0),                Length(2),                Length(3),                Length(6),                Length(10),                Length(15),            ],        ),        ("Min", [Min(0), Min(2), Min(3), Min(6), Min(10), Min(15)]),        ("Max", [Max(0), Max(2), Max(3), Max(6), Max(10), Max(15)]),        (            "Perc",            [                Percentage(0),                Percentage(25),                Percentage(50),                Percentage(75),                Percentage(100),                Percentage(150),            ],        ),        (            "Ratio",            [                Ratio(0, 4),                Ratio(1, 4),                Ratio(2, 4),                Ratio(3, 4),                Ratio(4, 4),                Ratio(6, 4),            ],        ),    ];
    for ((a, b), area) in examples        .iter()        .cartesian_product(examples.iter())        .zip(example_areas)    {        let (name_a, examples_a) = a;        let (name_b, examples_b) = b;        let constraints = examples_a.iter().copied().zip(examples_b.iter().copied());        render_example_combination(frame, area, &#x26;format!("{name_a}/{name_b}"), constraints);    }}
/// Renders a single example boxfn render_example_combination(    frame: &#x26;mut Frame,    area: Rect,    title: &#x26;str,    constraints: impl ExactSizeIterator&#x3C;Item = (Constraint, Constraint)>,) {    let block = Block::bordered()        .title(title.gray())        .style(Style::reset())        .border_style(Style::default().fg(Color::DarkGray));    let inner = block.inner(area);    frame.render_widget(block, area);    let layout = Layout::vertical(vec![Length(1); constraints.len() + 1]).split(inner);    for ((a, b), &#x26;area) in constraints.into_iter().zip(layout.iter()) {        render_single_example(frame, area, vec![a, b, Min(0)]);    }    // This is to make it easy to visually see the alignment of the examples    // with the constraints.    frame.render_widget(Paragraph::new("123456789012"), layout[6]);}
/// Renders a single example linefn render_single_example(frame: &#x26;mut Frame, area: Rect, constraints: Vec&#x3C;Constraint>) {    let red = Paragraph::new(constraint_label(constraints[0])).on_red();    let blue = Paragraph::new(constraint_label(constraints[1])).on_blue();    let green = Paragraph::new("·".repeat(12)).on_green();    let horizontal = Layout::horizontal(constraints);    let [r, b, g] = horizontal.areas(area);    frame.render_widget(red, r);    frame.render_widget(blue, b);    frame.render_widget(green, g);}
fn constraint_label(constraint: Constraint) -> String {    match constraint {        Constraint::Ratio(a, b) => format!("{a}:{b}"),        Constraint::Length(n)        | Constraint::Min(n)        | Constraint::Max(n)        | Constraint::Percentage(n)        | Constraint::Fill(n) => format!("{n}"),    }}
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/examples/Layout/layout.md)

 [Previous Flex](/examples/layout/flex/) [Next Style Examples](/examples/style/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
