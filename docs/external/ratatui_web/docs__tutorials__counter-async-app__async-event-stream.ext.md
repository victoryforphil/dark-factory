----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /tutorials/counter-async-app/async-event-stream
- Keywords: ratatui, rust, tui, terminal ui, docs, tutorials, counter async app, async event stream
- Summary: In this section, we are going to create an `EventHandler` with “green” threads or tasks, i.e. rust’s
----

Source: https://ratatui.rs/tutorials/counter-async-app/async-event-stream

# Async Event Stream

In this section, we are going to create an `EventHandler` with “green” threads or tasks, i.e. rust’s
`async`-`await` features + a future executor. We will be using `tokio` for this.

Here’s example code of reading key presses asynchronously comparing `std::thread` and `tokio::task`.
Notably, we are using `tokio::sync::mpsc` channels instead of `std::sync::mpsc` channels. And
because of this, receiving on a channel needs to be `.await`’d and hence needs to be in a `async fn`
method.

```
enum Event {  Key(crossterm::event::KeyEvent)}
struct EventHandler {  rx: std::sync::mpsc::Receiver&#x3C;Event>,  rx: tokio::sync::mpsc::UnboundedReceiver&#x3C;Event>,}
impl EventHandler {  fn new() -> Self {    let tick_rate = std::time::Duration::from_millis(250);    let (tx, rx) =  std::sync::mpsc::channel();    let (tx, mut rx) =  tokio::sync::mpsc::unbounded_channel();    std::thread::spawn(move || {    tokio::spawn(async move {      loop {        if crossterm::event::poll(tick_rate).unwrap() {          match crossterm::event::read().unwrap() {            CrosstermEvent::Key(key) => {              if key.kind == event::KeyEventKind::Press {                tx.send(Event::Key(key)).unwrap()              }            },            _ => unimplemented!(),          }        }      }    });
    EventHandler { rx }  }
  fn next(&#x26;self) -> Result&#x3C;Event> {  async fn next(&#x26;mut self) -> Result&#x3C;Event> {    Ok(self.rx.recv()?)    self.rx.recv().await.ok_or(color_eyre::eyre::eyre!("Unable to get event"))  }}
```

Even with this change, our `EventHandler` behaves the same way as before. In order to take advantage
of using `tokio` we have to use `tokio::select!`.

We can use [`tokio`’s `select!` macro](https://tokio.rs/tokio/tutorial/select) to wait on multiple
`async` computations and return when a any single computation completes.

Note

Using `crossterm::event::EventStream::new()` requires the `event-stream` feature to be enabled. This
also requires the `futures` crate. Naturally you’ll also need `tokio`.

If you haven’t already, add the following to your `Cargo.toml`:

```
crossterm = { version = "0.28.0", features = ["event-stream"] }futures = "0.3.28"tokio = { version = "1.32.0", features = ["full"] }tokio-util = "0.7.9" # required for `CancellationToken` introduced in the next section
```

Here’s what the `EventHandler` looks like with the `select!` macro:

```
use color_eyre::eyre::Result;use ratatui::crossterm::event::KeyEvent;use futures::{FutureExt, StreamExt};use tokio::{sync::mpsc, task::JoinHandle};
#[derive(Clone, Copy, Debug)]pub enum Event {  Error,  Tick,  Key(KeyEvent),}
#[derive(Debug)]pub struct EventHandler {  _tx: mpsc::UnboundedSender&#x3C;Event>,  rx: mpsc::UnboundedReceiver&#x3C;Event>,  task: Option&#x3C;JoinHandle&#x3C;()>>,}
impl EventHandler {  pub fn new() -> Self {    let tick_rate = std::time::Duration::from_millis(250);
    let (tx, rx) = mpsc::unbounded_channel();    let _tx = tx.clone();
    let task = tokio::spawn(async move {      let mut reader = crossterm::event::EventStream::new();      let mut interval = tokio::time::interval(tick_rate);      loop {        let delay = interval.tick();        let crossterm_event = reader.next().fuse();        tokio::select! {          maybe_event = crossterm_event => {            match maybe_event {              Some(Ok(evt)) => {                match evt {                  crossterm::event::Event::Key(key) => {                    if key.kind == crossterm::event::KeyEventKind::Press {                      tx.send(Event::Key(key)).unwrap();                    }                  },                  _ => {},                }              }              Some(Err(_)) => {                tx.send(Event::Error).unwrap();              }              None => {},            }          },          _ = delay => {              tx.send(Event::Tick).unwrap();          },        }      }    });
    Self { _tx, rx, task: Some(task) }  }
  pub async fn next(&#x26;mut self) -> Result&#x3C;Event> {    self.rx.recv().await.ok_or(color_eyre::eyre::eyre!("Unable to get event"))  }}
```

As mentioned before, since `EventHandler::next()` is a `async` function, when we use it we have to
call `.await` on it. And the function that is the call site of `event_handler.next().await` also
needs to be an `async` function. In our tutorial, we are going to use the event handler in the
`run()` function which will now be `async`.

Also, now that we are getting events asynchronously, we don’t need to call
`crossterm::event::poll()` in the `update` function. Let’s make the `update` function take an
`Event` instead.

If you place the above `EventHandler` in a `src/tui.rs` file, then here’s what our application now
looks like:

```
mod tui;
fn update(app: &#x26;mut App, event: Event) -> Result&#x3C;()> {  if let Event::Key(key) = event {    match key.code {      Char('j') => app.counter += 1,      Char('k') => app.counter -= 1,      Char('q') => app.should_quit = true,      _ => {},    }  }  Ok(())}
async fn run() -> Result&#x3C;()> {
  let mut events = tui::EventHandler::new(); // new
  // ratatui terminal  let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
  // application state  let mut app = App { counter: 0, should_quit: false };
  loop {    let event = events.next().await?; // new
    // application update    update(&#x26;mut app, event)?;
    // application render    t.draw(|f| {      ui(f, &#x26;app);    })?;
    // application exit    if app.should_quit {      break;    }  }
  Ok(())}
#[tokio::main]async fn main() -> Result&#x3C;()> {  // setup terminal  startup()?;
  let result = run().await;
  // teardown terminal before unwrapping Result of app run  shutdown()?;
  result?;
  Ok(())}
```

Using `tokio` in this manner however only makes the key events asynchronous but doesn’t make the
rest of our application asynchronous yet. We will discuss that in the next section.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/tutorials/counter-async-app/async-event-stream.md)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
