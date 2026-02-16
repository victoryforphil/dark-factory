----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /examples/widgets/canvas
- Keywords: ratatui, rust, tui, terminal ui, docs, examples, widgets, canvas
- Summary: Demonstrates the [`Canvas`](https://docs.rs/ratatui/latest/ratatui/widgets/canvas/index.html) widget
----

Source: https://ratatui.rs/examples/widgets/canvas

# Canvas

Demonstrates the [`Canvas`](https://docs.rs/ratatui/latest/ratatui/widgets/canvas/index.html) widget
and related shapes in the
[`canvas`](https://docs.rs/ratatui/latest/ratatui/widgets/canvas/index.html) module.

run

```
git clone https://github.com/ratatui/ratatui.git --branch latestcd ratatuicargo run --example=canvas --features=crossterm
```

canvas.rs

```
//! # [Ratatui] Canvas example//!//! The latest version of this example is available in the [examples] folder in the repository.//!//! Please note that the examples are designed to be run against the `main` branch of the Github//! repository. This means that you may not be able to compile with the latest release version on//! crates.io, or the one that you have installed locally.//!//! See the [examples readme] for more information on finding examples that match the version of the//! library you are using.//!//! [Ratatui]: https://github.com/ratatui/ratatui//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md
use std::{    io::stdout,    time::{Duration, Instant},};
use color_eyre::Result;use crossterm::{    event::{DisableMouseCapture, EnableMouseCapture, KeyEventKind},    ExecutableCommand,};use itertools::Itertools;use ratatui::{    crossterm::event::{self, Event, KeyCode, MouseEventKind},    layout::{Constraint, Layout, Position, Rect},    style::{Color, Stylize},    symbols::Marker,    widgets::{        canvas::{Canvas, Circle, Map, MapResolution, Points, Rectangle},        Block, Widget,    },    DefaultTerminal, Frame,};
fn main() -> Result&#x3C;()> {    color_eyre::install()?;    stdout().execute(EnableMouseCapture)?;    let terminal = ratatui::init();    let app_result = App::new().run(terminal);    ratatui::restore();    stdout().execute(DisableMouseCapture)?;    app_result}
struct App {    exit: bool,    x: f64,    y: f64,    ball: Circle,    playground: Rect,    vx: f64,    vy: f64,    tick_count: u64,    marker: Marker,    points: Vec&#x3C;Position>,    is_drawing: bool,}
impl App {    const fn new() -> Self {        Self {            exit: false,            x: 0.0,            y: 0.0,            ball: Circle {                x: 20.0,                y: 40.0,                radius: 10.0,                color: Color::Yellow,            },            playground: Rect::new(10, 10, 200, 100),            vx: 1.0,            vy: 1.0,            tick_count: 0,            marker: Marker::Dot,            points: vec![],            is_drawing: false,        }    }
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result&#x3C;()> {        let tick_rate = Duration::from_millis(16);        let mut last_tick = Instant::now();        while !self.exit {            terminal.draw(|frame| self.draw(frame))?;            let timeout = tick_rate.saturating_sub(last_tick.elapsed());            if event::poll(timeout)? {                match event::read()? {                    Event::Key(key) => self.handle_key_press(key),                    Event::Mouse(event) => self.handle_mouse_event(event),                    _ => (),                }            }
            if last_tick.elapsed() >= tick_rate {                self.on_tick();                last_tick = Instant::now();            }        }        Ok(())    }
    fn handle_key_press(&#x26;mut self, key: event::KeyEvent) {        if key.kind != KeyEventKind::Press {            return;        }        match key.code {            KeyCode::Char('q') => self.exit = true,            KeyCode::Down | KeyCode::Char('j') => self.y += 1.0,            KeyCode::Up | KeyCode::Char('k') => self.y -= 1.0,            KeyCode::Right | KeyCode::Char('l') => self.x += 1.0,            KeyCode::Left | KeyCode::Char('h') => self.x -= 1.0,            _ => {}        }    }
    fn handle_mouse_event(&#x26;mut self, event: event::MouseEvent) {        match event.kind {            MouseEventKind::Down(_) => self.is_drawing = true,            MouseEventKind::Up(_) => self.is_drawing = false,            MouseEventKind::Drag(_) => {                self.points.push(Position::new(event.column, event.row));            }            _ => {}        }    }
    fn on_tick(&#x26;mut self) {        self.tick_count += 1;        // only change marker every 180 ticks (3s) to avoid stroboscopic effect        if (self.tick_count % 180) == 0 {            self.marker = match self.marker {                Marker::Dot => Marker::Braille,                Marker::Braille => Marker::Block,                Marker::Block => Marker::HalfBlock,                Marker::HalfBlock => Marker::Bar,                Marker::Bar => Marker::Dot,            };        }        // bounce the ball by flipping the velocity vector        let ball = &#x26;self.ball;        let playground = self.playground;        if ball.x - ball.radius &#x3C; f64::from(playground.left())            || ball.x + ball.radius > f64::from(playground.right())        {            self.vx = -self.vx;        }        if ball.y - ball.radius &#x3C; f64::from(playground.top())            || ball.y + ball.radius > f64::from(playground.bottom())        {            self.vy = -self.vy;        }
        self.ball.x += self.vx;        self.ball.y += self.vy;    }
    fn draw(&#x26;self, frame: &#x26;mut Frame) {        let horizontal =            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);        let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);        let [left, right] = horizontal.areas(frame.area());        let [draw, map] = vertical.areas(left);        let [pong, boxes] = vertical.areas(right);
        frame.render_widget(self.map_canvas(), map);        frame.render_widget(self.draw_canvas(draw), draw);        frame.render_widget(self.pong_canvas(), pong);        frame.render_widget(self.boxes_canvas(boxes), boxes);    }
    fn map_canvas(&#x26;self) -> impl Widget + '_ {        Canvas::default()            .block(Block::bordered().title("World"))            .marker(self.marker)            .paint(|ctx| {                ctx.draw(&#x26;Map {                    color: Color::Green,                    resolution: MapResolution::High,                });                ctx.print(self.x, -self.y, "You are here".yellow());            })            .x_bounds([-180.0, 180.0])            .y_bounds([-90.0, 90.0])    }
    fn draw_canvas(&#x26;self, area: Rect) -> impl Widget + '_ {        Canvas::default()            .block(Block::bordered().title("Draw here"))            .marker(self.marker)            .x_bounds([0.0, f64::from(area.width)])            .y_bounds([0.0, f64::from(area.height)])            .paint(move |ctx| {                let points = self                    .points                    .iter()                    .map(|p| {                        (                            f64::from(p.x) - f64::from(area.left()),                            f64::from(area.bottom()) - f64::from(p.y),                        )                    })                    .collect_vec();                ctx.draw(&#x26;Points {                    coords: &#x26;points,                    color: Color::White,                });            })    }
    fn pong_canvas(&#x26;self) -> impl Widget + '_ {        Canvas::default()            .block(Block::bordered().title("Pong"))            .marker(self.marker)            .paint(|ctx| {                ctx.draw(&#x26;self.ball);            })            .x_bounds([10.0, 210.0])            .y_bounds([10.0, 110.0])    }
    fn boxes_canvas(&#x26;self, area: Rect) -> impl Widget {        let left = 0.0;        let right = f64::from(area.width);        let bottom = 0.0;        let top = f64::from(area.height).mul_add(2.0, -4.0);        Canvas::default()            .block(Block::bordered().title("Rects"))            .marker(self.marker)            .x_bounds([left, right])            .y_bounds([bottom, top])            .paint(|ctx| {                for i in 0..=11 {                    ctx.draw(&#x26;Rectangle {                        x: f64::from(i * i + 3 * i) / 2.0 + 2.0,                        y: 2.0,                        width: f64::from(i),                        height: f64::from(i),                        color: Color::Red,                    });                    ctx.draw(&#x26;Rectangle {                        x: f64::from(i * i + 3 * i) / 2.0 + 2.0,                        y: 21.0,                        width: f64::from(i),                        height: f64::from(i),                        color: Color::Blue,                    });                }                for i in 0..100 {                    if i % 10 != 0 {                        ctx.print(f64::from(i) + 1.0, 0.0, format!("{i}", i = i % 10));                    }                    if i % 2 == 0 &#x26;&#x26; i % 10 != 0 {                        ctx.print(0.0, f64::from(i), format!("{i}", i = i % 10));                    }                }            })    }}
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/examples/Widgets/canvas.md)

 [Previous Calendar](/examples/widgets/calendar/) [Next Chart](/examples/widgets/chart/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
