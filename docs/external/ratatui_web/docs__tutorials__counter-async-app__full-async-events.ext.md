----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /tutorials/counter-async-app/full-async-events
- Keywords: ratatui, rust, tui, terminal ui, docs, tutorials, counter async app, full async events
- Summary: There are a number of ways to make our application work more in an `async` manner. The easiest way
----

Source: https://ratatui.rs/tutorials/counter-async-app/full-async-events

# Full Async Events

There are a number of ways to make our application work more in an `async` manner. The easiest way
to do this is to add more `Event` variants to our existing `EventHandler`. Specifically, we would
like to only render in the main run loop when we receive a `Event::Render` variant:

- ``` #[derive(Clone, Debug, Serialize, Deserialize)]pub enum Event { Quit, Error, Tick, Render, // new Key(KeyEvent),} ``` Another thing I personally like to do is combine the `EventHandler` struct and the `Terminal` functionality. To do this, we are going to rename our `EventHandler` struct to a `Tui` struct. We are also going to include a few more `Event` variants for making our application more capable. Below is the relevant snippet of an updated `Tui` struct. You can click on the “Show hidden lines” button at the top right of the code block or check out [this section of the book](/recipes/apps/terminal-and-event-handler/) for the full version this struct. The key things to note are that we create a `tick_interval`, `render_interval` and `reader` stream that can be polled using `tokio::select!`. This means that even while waiting for a key press, we will still send a `Event::Tick` and `Event::Render` at regular intervals. ``` #[derive(Clone, Debug)]pub enum Event { Init, Quit, Error, Closed, Tick, Render, FocusGained, FocusLost, Paste(String), Key(KeyEvent), Mouse(MouseEvent), Resize(u16, u16),} pub struct Tui { pub terminal: ratatui::Terminal&#x3C;Backend&#x3C;std::io::Stderr>>, pub task: JoinHandle&#x3C;()>, pub event_rx: UnboundedReceiver&#x3C;Event>, pub event_tx: UnboundedSender&#x3C;Event>, pub frame_rate: f64, pub tick_rate: f64,} impl Tui { pub fn start(&#x26;mut self) { let tick_delay = std::time::Duration::from_secs_f64(1.0 / self.tick_rate); let render_delay = std::time::Duration::from_secs_f64(1.0 / self.frame_rate); let _event_tx = self.event_tx.clone(); self.task = tokio::spawn(async move { let mut reader = crossterm::event::EventStream::new(); let mut tick_interval = tokio::time::interval(tick_delay); let mut render_interval = tokio::time::interval(render_delay); loop { let tick_delay = tick_interval.tick(); let render_delay = render_interval.tick(); let crossterm_event = reader.next().fuse(); tokio::select! { maybe_event = crossterm_event => { match maybe_event { Some(Ok(evt)) => { match evt { CrosstermEvent::Key(key) => { if key.kind == KeyEventKind::Press { _event_tx.send(Event::Key(key)).unwrap(); } }, } } Some(Err(_)) => { _event_tx.send(Event::Error).unwrap(); } None => {}, } }, _ = tick_delay => { _event_tx.send(Event::Tick).unwrap(); }, _ = render_delay => { _event_tx.send(Event::Render).unwrap(); }, } } }); } ``` We made a number of changes to the `Tui` struct. We added a `Deref` and `DerefMut` so we can call `tui.draw(|f| ...)` to have it call `tui.terminal.draw(|f| ...)`.

- We moved the `startup()` and `shutdown()` functionality into the `Tui` struct.

- We also added a `CancellationToken` so that we can start and stop the tokio task more easily.

- We added `Event` variants for `Resize`, `Focus`, and `Paste`.

- We added methods to set the `tick_rate`, `frame_rate`, and whether we want to enable `mouse` or `paste` events.

Here’s the code for the fully async application:

```
mod tui;
use color_eyre::eyre::Result;use ratatui::crossterm::event::KeyCode::Char;use ratatui::{prelude::CrosstermBackend, widgets::Paragraph};use tui::Event;
// App statestruct App {  counter: i64,  should_quit: bool,}
// App ui render functionfn ui(f: &#x26;mut Frame, app: &#x26;App) {  f.render_widget(Paragraph::new(format!("Counter: {}", app.counter)), f.area());}
fn update(app: &#x26;mut App, event: Event) {  match event {    Event::Key(key) => {      match key.code {        Char('j') => app.counter += 1,        Char('k') => app.counter -= 1,        Char('q') => app.should_quit = true,        _ => Action::None,      }    },    _ => {},  };}
async fn run() -> Result&#x3C;()> {  // ratatui terminal  let mut tui = tui::Tui::new()?.tick_rate(1.0).frame_rate(30.0);  tui.enter()?;
  // application state  let mut app = App { counter: 0, should_quit: false };
  loop {    let event = tui.next().await?; // blocks until next event
    if let Event::Render = event.clone() {      // application render      tui.draw(|f| {        ui(f, &#x26;app);      })?;    }
    // application update    update(&#x26;mut app, event);
    // application exit    if app.should_quit {      break;    }  }  tui.exit()?;
  Ok(())}
#[tokio::main]async fn main() -> Result&#x3C;()> {  let result = run().await;
  result?;
  Ok(())}
```

The above code ensures that we render at a consistent frame rate. As an exercise, play around with
this frame rate and tick rate to see how the CPU utilization changes as you change those numbers.

Even though our application renders in an “async” manner, we also want to perform “actions” in an
asynchronous manner. We will improve this in the next section to make our application truly async
capable.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/tutorials/counter-async-app/full-async-events.md)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
