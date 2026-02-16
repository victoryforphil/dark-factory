----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /templates/component/components-home-rs
- Keywords: ratatui, rust, tui, terminal ui, docs, templates, component, components home rs
- Summary: Here’s an example of the `Home` component with additional state:
----

Source: https://ratatui.rs/templates/component/components-home-rs

# Components/home.rs

Here’s an example of the `Home` component with additional state:

- `show_help` is a `bool` that tracks whether or not help should be rendered or not

- `ticker` is a counter that increments every `AppTick`.

This `Home` component also adds fields for `input: Input`, and stores a reference to
`action_tx: mpsc::UnboundedSender&#x3C;Action>`

```
use std::{collections::HashMap, time::Duration};
use color_eyre::eyre::Result;use log::error;use ratatui::{  crossterm::event::{KeyCode, KeyEvent},  layout::{Alignment, Constraint, Layout, Margin, Position, Rect},  style::{Color, Modifier, Style, Stylize},  text::{Line, Span},  widgets::{Block, BorderType, Borders, Clear, Paragraph, Row, Table},  Frame,};use tokio::sync::mpsc::UnboundedSender;use tracing::trace;use tui_input::{backend::crossterm::EventHandler, Input};
use super::Component;use crate::{action::Action, config::key_event_to_string};
#[derive(Default, Copy, Clone, PartialEq, Eq)]pub enum Mode {  #[default]  Normal,  Insert,  Processing,}
#[derive(Default)]pub struct Home {  pub show_help: bool,  pub counter: usize,  pub app_ticker: usize,  pub render_ticker: usize,  pub mode: Mode,  pub input: Input,  pub action_tx: Option&#x3C;UnboundedSender&#x3C;Action>>,  pub keymap: HashMap&#x3C;KeyEvent, Action>,  pub text: Vec&#x3C;String>,  pub last_events: Vec&#x3C;KeyEvent>,}
impl Home {  pub fn new() -> Self {    Self::default()  }
  pub fn keymap(mut self, keymap: HashMap&#x3C;KeyEvent, Action>) -> Self {    self.keymap = keymap;    self  }
  pub fn tick(&#x26;mut self) {    log::info!("Tick");    self.app_ticker = self.app_ticker.saturating_add(1);    self.last_events.drain(..);  }
  pub fn render_tick(&#x26;mut self) {    log::debug!("Render Tick");    self.render_ticker = self.render_ticker.saturating_add(1);  }
  pub fn add(&#x26;mut self, s: String) {    self.text.push(s)  }
  pub fn schedule_increment(&#x26;mut self, i: usize) {    let tx = self.action_tx.clone().unwrap();    tokio::spawn(async move {      tx.send(Action::EnterProcessing).unwrap();      tokio::time::sleep(Duration::from_secs(1)).await;      tx.send(Action::Increment(i)).unwrap();      tx.send(Action::ExitProcessing).unwrap();    });  }
  pub fn schedule_decrement(&#x26;mut self, i: usize) {    let tx = self.action_tx.clone().unwrap();    tokio::spawn(async move {      tx.send(Action::EnterProcessing).unwrap();      tokio::time::sleep(Duration::from_secs(1)).await;      tx.send(Action::Decrement(i)).unwrap();      tx.send(Action::ExitProcessing).unwrap();    });  }
  pub fn increment(&#x26;mut self, i: usize) {    self.counter = self.counter.saturating_add(i);  }
  pub fn decrement(&#x26;mut self, i: usize) {    self.counter = self.counter.saturating_sub(i);  }}
impl Component for Home {  fn register_action_handler(&#x26;mut self, tx: UnboundedSender&#x3C;Action>) -> Result&#x3C;()> {    self.action_tx = Some(tx);    Ok(())  }
  fn handle_key_events(&#x26;mut self, key: KeyEvent) -> Result&#x3C;Option&#x3C;Action>> {    self.last_events.push(key);    let action = match self.mode {      Mode::Normal | Mode::Processing => return Ok(None),      Mode::Insert => match key.code {        KeyCode::Esc => Action::EnterNormal,        KeyCode::Enter => {          if let Some(sender) = &#x26;self.action_tx {            if let Err(e) = sender.send(Action::CompleteInput(self.input.value().to_string())) {              error!("Failed to send action: {:?}", e);            }          }          Action::EnterNormal        },        _ => {          self.input.handle_event(&#x26;ratatui::crossterm::event::Event::Key(key));          Action::Update        },      },    };    Ok(Some(action))  }
  fn update(&#x26;mut self, action: Action) -> Result&#x3C;Option&#x3C;Action>> {    match action {      Action::Tick => self.tick(),      Action::Render => self.render_tick(),      Action::ToggleShowHelp => self.show_help = !self.show_help,      Action::ScheduleIncrement => self.schedule_increment(1),      Action::ScheduleDecrement => self.schedule_decrement(1),      Action::Increment(i) => self.increment(i),      Action::Decrement(i) => self.decrement(i),      Action::CompleteInput(s) => self.add(s),      Action::EnterNormal => {        self.mode = Mode::Normal;      },      Action::EnterInsert => {        self.mode = Mode::Insert;      },      Action::EnterProcessing => {        self.mode = Mode::Processing;      },      Action::ExitProcessing => {        // TODO: Make this go to previous mode instead        self.mode = Mode::Normal;      },      _ => (),    }    Ok(None)  }
  fn draw(&#x26;mut self, f: &#x26;mut Frame&#x3C;'_>, rect: Rect) -> Result&#x3C;()> {    let rects = Layout::default().constraints([Constraint::Percentage(100), Constraint::Min(3)].as_ref()).split(rect);
    let mut text: Vec&#x3C;Line> = self.text.clone().iter().map(|l| Line::from(l.clone())).collect();    text.insert(0, "".into());    text.insert(0, "Type into input and hit enter to display here".dim().into());    text.insert(0, "".into());    text.insert(0, format!("Render Ticker: {}", self.render_ticker).into());    text.insert(0, format!("App Ticker: {}", self.app_ticker).into());    text.insert(0, format!("Counter: {}", self.counter).into());    text.insert(0, "".into());    text.insert(      0,      Line::from(vec![        "Press ".into(),        Span::styled("j", Style::default().fg(Color::Red)),        " or ".into(),        Span::styled("k", Style::default().fg(Color::Red)),        " to ".into(),        Span::styled("increment", Style::default().fg(Color::Yellow)),        " or ".into(),        Span::styled("decrement", Style::default().fg(Color::Yellow)),        ".".into(),      ]),    );    text.insert(0, "".into());
    f.render_widget(      Paragraph::new(text)        .block(          Block::default()            .title("ratatui async template")            .title_alignment(Alignment::Center)            .borders(Borders::ALL)            .border_style(match self.mode {              Mode::Processing => Style::default().fg(Color::Yellow),              _ => Style::default(),            })            .border_type(BorderType::Rounded),        )        .style(Style::default().fg(Color::Cyan))        .alignment(Alignment::Center),      rects[0],    );    let width = rects[1].width.max(3) - 3; // keep 2 for borders and 1 for cursor    let scroll = self.input.visual_scroll(width as usize);    let input = Paragraph::new(self.input.value())      .style(match self.mode {        Mode::Insert => Style::default().fg(Color::Yellow),        _ => Style::default(),      })      .scroll((0, scroll as u16))      .block(Block::default().borders(Borders::ALL).title(Line::from(vec![        Span::raw("Enter Input Mode "),        Span::styled("(Press ", Style::default().fg(Color::DarkGray)),        Span::styled("/", Style::default().add_modifier(Modifier::BOLD).fg(Color::Gray)),        Span::styled(" to start, ", Style::default().fg(Color::DarkGray)),        Span::styled("ESC", Style::default().add_modifier(Modifier::BOLD).fg(Color::Gray)),        Span::styled(" to finish)", Style::default().fg(Color::DarkGray)),      ])));    f.render_widget(input, rects[1]);    if self.mode == Mode::Insert {      let position = Position {        x: (rects[1].x + 1 + self.input.cursor() as u16).min(rects[1].x + rects[1].width - 2),        y: rects[1].y + 1,      };      f.set_cursor_position(position)    }
    if self.show_help {      let rect = rect.inner(Margin { horizontal: 4, vertical: 2 });      f.render_widget(Clear, rect);      let block = Block::default()        .title(Line::from(vec![Span::styled("Key Bindings", Style::default().add_modifier(Modifier::BOLD))]))        .borders(Borders::ALL)        .border_style(Style::default().fg(Color::Yellow));      f.render_widget(block, rect);      let rows = vec![        Row::new(vec!["j", "Increment"]),        Row::new(vec!["k", "Decrement"]),        Row::new(vec!["/", "Enter Input"]),        Row::new(vec!["ESC", "Exit Input"]),        Row::new(vec!["Enter", "Submit Input"]),        Row::new(vec!["q", "Quit"]),        Row::new(vec!["?", "Open Help"]),      ];      let table = Table::new(rows, [Constraint::Percentage(10), Constraint::Percentage(90)])        .header(Row::new(vec!["Key", "Action"]).bottom_margin(1).style(Style::default().add_modifier(Modifier::BOLD)))        .column_spacing(1);      f.render_widget(table, rect.inner(Margin { vertical: 4, horizontal: 2 }));    };
    f.render_widget(      Block::default()        .title(          Line::from(format!(            "{:?}",            &#x26;self.last_events.iter().map(key_event_to_string).collect::&#x3C;Vec&#x3C;_>>()          ))          .alignment(Alignment::Right),        )        .title_style(Style::default().add_modifier(Modifier::BOLD)),      Rect { x: rect.x + 1, y: rect.height.saturating_sub(1), width: rect.width.saturating_sub(2), height: 1 },    );
    Ok(())  }}
```

The `render` function takes a `Frame` and draws a paragraph to display a counter as well as a text
box input:

The `Home` component has a couple of methods `increment` and `decrement` that we saw earlier, but
this time additional `Action`s are sent on the `action_tx` channel to track the start and end of the
increment.

```
pub fn schedule_increment(&#x26;mut self, i: usize) {    let tx = self.action_tx.clone().unwrap();    tokio::task::spawn(async move {      tx.send(Action::EnterProcessing).unwrap();      tokio::time::sleep(Duration::from_secs(5)).await;      tx.send(Action::Increment(i)).unwrap();      tx.send(Action::ExitProcessing).unwrap();    });  }
  pub fn schedule_decrement(&#x26;mut self, i: usize) {    let tx = self.action_tx.clone().unwrap();    tokio::task::spawn(async move {      tx.send(Action::EnterProcessing).unwrap();      tokio::time::sleep(Duration::from_secs(5)).await;      tx.send(Action::Decrement(i)).unwrap();      tx.send(Action::ExitProcessing).unwrap();    });  }
```

When a `Action` is sent on the action channel, it is received in the `main` thread in the
`app.run()` loop which then calls the `dispatch` method with the appropriate action:

```
fn dispatch(&#x26;mut self, action: Action) -> Option&#x3C;Action> {    match action {      Action::Tick => self.tick(),      Action::ToggleShowHelp => self.show_help = !self.show_help,      Action::ScheduleIncrement=> self.schedule_increment(1),      Action::ScheduleDecrement=> self.schedule_decrement(1),      Action::Increment(i) => self.increment(i),      Action::Decrement(i) => self.decrement(i),      Action::EnterNormal => {        self.mode = Mode::Normal;      },      Action::EnterInsert => {        self.mode = Mode::Insert;      },      Action::EnterProcessing => {        self.mode = Mode::Processing;      },      Action::ExitProcessing => {        // TODO: Make this go to previous mode instead        self.mode = Mode::Normal;      },      _ => (),    }    None  }
```

This way, you can have `Action` affect multiple components by propagating the actions down all of
them.

When the `Mode` is switched to `Insert`, all events are handled off the `Input` widget from the
excellent [`tui-input` crate](https://github.com/sayanarijit/tui-input).

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/templates/component/components-home-rs.md)

 [Previous Components.rs](/templates/component/components-rs/) [Next Config.rs](/templates/component/config-rs/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
