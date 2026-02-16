----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /tutorials/counter-async-app/full-async-actions
- Keywords: ratatui, rust, tui, terminal ui, docs, tutorials, counter async app, full async actions
- Summary: Now that we have introduced `Event`s and `Action`s, we are going introduce a new `mpsc::channel` for
----

Source: https://ratatui.rs/tutorials/counter-async-app/full-async-actions

# Full Async Actions

Now that we have introduced `Event`s and `Action`s, we are going introduce a new `mpsc::channel` for
`Action`s. The advantage of this is that we can programmatically trigger updates to the state of the
app by sending `Action`s on the channel.

Here’s the `run` function refactored from before to introduce an `Action` channel. In addition to
refactoring, we store the `action_tx` half of the channel in the `App`.

```
async fn run() -> Result&#x3C;()> {    let (action_tx, mut action_rx) = mpsc::unbounded_channel(); // new
    // ratatui terminal    let mut tui = tui::Tui::new()?.tick_rate(1.0).frame_rate(30.0);    tui.enter()?;
    // application state    let mut app = App {        counter: 0,        should_quit: false,        action_tx: action_tx.clone(),    };
    loop {        let e = tui.next().await?;        match e {            tui::Event::Quit => action_tx.send(Action::Quit)?,            tui::Event::Tick => action_tx.send(Action::Tick)?,            tui::Event::Render => action_tx.send(Action::Render)?,            tui::Event::Key(_) => {                let action = get_action(&#x26;app, e);                action_tx.send(action.clone())?;            }            _ => {}        };
        while let Ok(action) = action_rx.try_recv() {            // application update            update(&#x26;mut app, action.clone());            // render only when we receive Action::Render            if let Action::Render = action {                tui.draw(|f| {                    ui(f, &#x26;mut app);                })?;            }        }
        // application exit        if app.should_quit {            break;        }    }    tui.exit()?;
    Ok(())}
```

Running the code with this change should give the exact same behavior as before.

Now that we have stored the `action_tx` half of the channel in the `App`, we can use this to
schedule tasks. For example, let’s say we wanted to press `J` and `K` to perform some network
request and then increment the counter.

First, we have to update my `Action` enum:

```
#[derive(Clone)]pub enum Action {    Tick,    Increment,    Decrement,    NetworkRequestAndThenIncrement, // new    NetworkRequestAndThenDecrement, // new    Quit,    Render,    None,}
```

Next, we can update my event handler:

```
fn get_action(_app: &#x26;App, event: Event) -> Action {    match event {        Event::Error => Action::None,        Event::Tick => Action::Tick,        Event::Render => Action::Render,        Event::Key(key) => {            match key.code {                Char('j') => Action::Increment,                Char('k') => Action::Decrement,                Char('J') => Action::NetworkRequestAndThenIncrement, // new                Char('K') => Action::NetworkRequestAndThenDecrement, // new                Char('q') => Action::Quit,                _ => Action::None,            }        }        _ => Action::None,    }}
```

Finally, we can handle the action in my `update` function by spawning a tokio task:

```
fn update(app: &#x26;mut App, action: Action) {    match action {        Action::Increment => {            app.counter += 1;        }        Action::Decrement => {            app.counter -= 1;        }        Action::NetworkRequestAndThenIncrement => {            let tx = app.action_tx.clone();            tokio::spawn(async move {                tokio::time::sleep(Duration::from_secs(5)).await; // simulate network request                tx.send(Action::Increment).unwrap();            });        }        Action::NetworkRequestAndThenDecrement => {            let tx = app.action_tx.clone();            tokio::spawn(async move {                tokio::time::sleep(Duration::from_secs(5)).await; // simulate network request                tx.send(Action::Decrement).unwrap();            });        }        Action::Quit => app.should_quit = true,        _ => {}    };}
```

Here is the full code for reference:

```
mod tui;
use std::time::Duration;
use color_eyre::eyre::Result;use ratatui::{    crossterm::event::KeyCode::Char,    layout::Alignment,    style::{Color, Style},    widgets::{Block, BorderType, Borders, Paragraph},    Frame,};use tokio::sync::mpsc::{self, UnboundedSender};use tui::Event;
// App statestruct App {    counter: i64,    should_quit: bool,    action_tx: UnboundedSender&#x3C;Action>,}
// App actions#[derive(Clone)]pub enum Action {    Tick,    Increment,    Decrement,    NetworkRequestAndThenIncrement, // new    NetworkRequestAndThenDecrement, // new    Quit,    Render,    None,}
// App ui render functionfn ui(frame: &#x26;mut Frame, app: &#x26;mut App) {    let area = frame.area();    frame.render_widget(        Paragraph::new(format!(            "Press j or k to increment or decrement.\n\nCounter: {}",            app.counter,        ))        .block(            Block::default()                .title("ratatui async counter app")                .title_alignment(Alignment::Center)                .borders(Borders::ALL)                .border_type(BorderType::Rounded),        )        .style(Style::default().fg(Color::Cyan))        .alignment(Alignment::Center),        area,    );}
fn get_action(_app: &#x26;App, event: Event) -> Action {    match event {        Event::Error => Action::None,        Event::Tick => Action::Tick,        Event::Render => Action::Render,        Event::Key(key) => {            match key.code {                Char('j') => Action::Increment,                Char('k') => Action::Decrement,                Char('J') => Action::NetworkRequestAndThenIncrement, // new                Char('K') => Action::NetworkRequestAndThenDecrement, // new                Char('q') => Action::Quit,                _ => Action::None,            }        }        _ => Action::None,    }}
fn update(app: &#x26;mut App, action: Action) {    match action {        Action::Increment => {            app.counter += 1;        }        Action::Decrement => {            app.counter -= 1;        }        Action::NetworkRequestAndThenIncrement => {            let tx = app.action_tx.clone();            tokio::spawn(async move {                tokio::time::sleep(Duration::from_secs(5)).await; // simulate network request                tx.send(Action::Increment).unwrap();            });        }        Action::NetworkRequestAndThenDecrement => {            let tx = app.action_tx.clone();            tokio::spawn(async move {                tokio::time::sleep(Duration::from_secs(5)).await; // simulate network request                tx.send(Action::Decrement).unwrap();            });        }        Action::Quit => app.should_quit = true,        _ => {}    };}
async fn run() -> Result&#x3C;()> {    let (action_tx, mut action_rx) = mpsc::unbounded_channel(); // new
    // ratatui terminal    let mut tui = tui::Tui::new()?.tick_rate(1.0).frame_rate(30.0);    tui.enter()?;
    // application state    let mut app = App {        counter: 0,        should_quit: false,        action_tx: action_tx.clone(),    };
    loop {        let e = tui.next().await?;        match e {            tui::Event::Quit => action_tx.send(Action::Quit)?,            tui::Event::Tick => action_tx.send(Action::Tick)?,            tui::Event::Render => action_tx.send(Action::Render)?,            tui::Event::Key(_) => {                let action = get_action(&#x26;app, e);                action_tx.send(action.clone())?;            }            _ => {}        };
        while let Ok(action) = action_rx.try_recv() {            // application update            update(&#x26;mut app, action.clone());            // render only when we receive Action::Render            if let Action::Render = action {                tui.draw(|f| {                    ui(f, &#x26;mut app);                })?;            }        }
        // application exit        if app.should_quit {            break;        }    }    tui.exit()?;
    Ok(())}
#[tokio::main]async fn main() -> Result&#x3C;()> {    let result = run().await;
    result?;
    Ok(())}
```

With that, we have a fully async application that is tokio ready to spawn tasks to do work
concurrently.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/tutorials/counter-async-app/full-async-actions.md)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
