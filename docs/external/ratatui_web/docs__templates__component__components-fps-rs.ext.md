----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /templates/component/components-fps-rs
- Keywords: ratatui, rust, tui, terminal ui, docs, templates, component, components fps rs
- Summary: Here’s an example of the `FpsCounter` component implemented in the template.
----

Source: https://ratatui.rs/templates/component/components-fps-rs

# Components/fps.rs

Here’s an example of the `FpsCounter` component implemented in the template.

## State

[Section titled “State”](#state)

The component has the following state:

- `last_tick_update` is a `Instant` that tracks the last time the `tick` method was called.

- `tick_count` is a `u32` that tracks the number of ticks in the last second.

- `ticks_per_second` is a `f64` that tracks the number of ticks per second.

- `last_frame_update` is a `Instant` that tracks the last time the `render` method was called.

- `frames_per_second` is a `f64` that tracks the number of frames rendered per second.

```
#[derive(Debug, Clone, PartialEq)]pub struct FpsCounter {    last_tick_update: Instant,    tick_count: u32,    ticks_per_second: f64,
    last_frame_update: Instant,    frame_count: u32,    frames_per_second: f64,}
```

## Methods

[Section titled “Methods”](#methods)

### `impl FpsCounter`

[Section titled “impl FpsCounter”](#impl-fpscounter)

In this `impl` block, we define the `new` method that creates a new `FpsCounter` component. We also
define some methods to calculate the `tick count`, `ticks_per_second` and more.

```
impl FpsCounter {    pub fn new() -> Self {        Self {            last_tick_update: Instant::now(),            tick_count: 0,            ticks_per_second: 0.0,            last_frame_update: Instant::now(),            frame_count: 0,            frames_per_second: 0.0,        }    }
    fn app_tick(&#x26;mut self) -> Result&#x3C;()> {        self.tick_count += 1;        let now = Instant::now();        let elapsed = (now - self.last_tick_update).as_secs_f64();        if elapsed >= 1.0 {            self.ticks_per_second = self.tick_count as f64 / elapsed;            self.last_tick_update = now;            self.tick_count = 0;        }        Ok(())    }
    fn render_tick(&#x26;mut self) -> Result&#x3C;()> {        self.frame_count += 1;        let now = Instant::now();        let elapsed = (now - self.last_frame_update).as_secs_f64();        if elapsed >= 1.0 {            self.frames_per_second = self.frame_count as f64 / elapsed;            self.last_frame_update = now;            self.frame_count = 0;        }        Ok(())    }}
```

### `impl Component`

[Section titled “impl Component”](#impl-component)

Lastly, we implement the `Component` trait for the `FpsCounter` component. This allows us to use the
component.

```
impl Component for FpsCounter {    fn update(&#x26;mut self, action: Action) -> Result&#x3C;Option&#x3C;Action>> {        match action {            Action::Tick => self.app_tick()?,            Action::Render => self.render_tick()?,            _ => {}        };        Ok(None)    }
    fn draw(&#x26;mut self, frame: &#x26;mut Frame, area: Rect) -> Result&#x3C;()> {        let [top, _] = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(area);        let message = format!(            "{:.2} ticks/sec, {:.2} FPS",            self.ticks_per_second, self.frames_per_second        );        let span = Span::styled(message, Style::new().dim());        let paragraph = Paragraph::new(span).right_aligned();        frame.render_widget(paragraph, top);        Ok(())    }}
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/templates/component/components-fps-rs.md)

 [Previous Cli.rs](/templates/component/cli-rs/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
