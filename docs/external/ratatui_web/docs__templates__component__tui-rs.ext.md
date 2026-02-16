----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /templates/component/tui-rs
- Keywords: ratatui, rust, tui, terminal ui, docs, templates, component, tui rs
- Summary: This page will explain how the `tui.rs` file works in the `components` template.
----

Source: https://ratatui.rs/templates/component/tui-rs

# Tui.rs

This page will explain how the `tui.rs` file works in the `components` template.

## Terminal

[Section titled “Terminal”](#terminal)

In this section of the tutorial, we are going to discuss the basic components of the `Tui` struct.

You’ll find most people setup and teardown of a terminal application using `crossterm` like so:

- ``` fn setup_terminal() -> Result&#x3C;Terminal&#x3C;CrosstermBackend&#x3C;Stdout>>> { let mut stdout = io::stdout(); crossterm::terminal::enable_raw_mode()?; crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture, HideCursor)?; Terminal::new(CrosstermBackend::new(stdout))} fn teardown_terminal(terminal: &#x26;mut Terminal&#x3C;CrosstermBackend&#x3C;Stdout>>) -> Result&#x3C;()> { let mut stdout = io::stdout(); crossterm::terminal::disable_raw_mode()?; crossterm::execute!(stdout, LeaveAlternateScreen, DisableMouseCapture, ShowCursor)?; Ok(())} fn main() -> Result&#x3C;()> { let mut terminal = setup_terminal()?; run_app(&#x26;mut terminal)?; teardown_terminal(&#x26;mut terminal)?; Ok(())} ``` You can use `termion` or `termwiz` instead here, and you’ll have to change the implementation of `setup_terminal` and `teardown_terminal`. See the [backends](https://ratatui.rs/concepts/backends/) page for more information. NoteTerminals have two screen buffers for each window. The default screen buffer is what you are dropped into when you start up a terminal. The second screen buffer, called the alternate screen, is used for running interactive apps such as the `vim`, `less` etc. Our implementation of the `Tui` struct has the following parts: Setup and teardown of the terminal

- The `Tui` struct itself

- Async event handling using `tokio`

# Terminal Setup and Teardown

[Section titled “Terminal Setup and Teardown”](#terminal-setup-and-teardown)

The `Tui` struct has a `terminal` field that is of type `ratatui::Terminal&#x3C;Backend&#x3C;Stdout>>`. This
template uses `crossterm` as the backend. In the constructor for the `Tui` struct, we create and
store a new `ratatui::Terminal`. The setup and teardown of the terminal is managed by the following
methods:

```
impl Tui {    pub fn start(&#x26;mut self) {        self.cancel(); // Cancel any existing task        self.cancellation_token = CancellationToken::new();        let event_loop = Self::event_loop(            self.event_tx.clone(),            self.cancellation_token.clone(),            self.tick_rate,            self.frame_rate,        );        self.task = tokio::spawn(async {            event_loop.await;        });    }    pub fn enter(&#x26;mut self) -> Result&#x3C;()> {        crossterm::terminal::enable_raw_mode()?;        crossterm::execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;        if self.mouse {            crossterm::execute!(stdout(), EnableMouseCapture)?;        }        if self.paste {            crossterm::execute!(stdout(), EnableBracketedPaste)?;        }        self.start();        Ok(())    }
    pub fn exit(&#x26;mut self) -> Result&#x3C;()> {        self.stop()?;        if crossterm::terminal::is_raw_mode_enabled()? {            self.flush()?;            if self.paste {                crossterm::execute!(stdout(), DisableBracketedPaste)?;            }            if self.mouse {                crossterm::execute!(stdout(), DisableMouseCapture)?;            }            crossterm::execute!(stdout(), LeaveAlternateScreen, cursor::Show)?;            crossterm::terminal::disable_raw_mode()?;        }        Ok(())    }
    pub fn stop(&#x26;self) -> Result&#x3C;()> {        self.cancel();        let mut counter = 0;        while !self.task.is_finished() {            std::thread::sleep(Duration::from_millis(1));            counter += 1;            if counter > 50 {                self.task.abort();            }            if counter > 100 {                error!("Failed to abort task in 100 milliseconds for unknown reason");                break;            }        }        Ok(())    }}
```

When we call the `run()` method on the `App` struct (the function that we called in our `main.rs`
file), the first function that runs is the `Tui::enter()` function. This function prepares the
terminal by enabling the terminal `raw_mode`, entering an `AlternateScreen`, and if the App has
mouse controls, it enables mouse capture. Then, it calls the `Tui::start()` method to initialize the
event loop.

```
self.task = tokio::spawn(async {    event_loop.await;});
```

## Event Loop

[Section titled “Event Loop”](#event-loop)

While we are in the “raw mode”, i.e. after we call `t.enter()`, any key presses in that terminal
window are sent to `stdin`. We have to read these key presses from `stdin` if we want to act on
them.

There are a number of different ways to do that. `crossterm` has a `event` module that implements
features to read these key presses for us.

Let’s assume we were building a simple “counter” application, that incremented a counter when we
pressed `j` and decremented a counter when we pressed `k`.

```
fn main() -> Result {  let mut app = App::new();
  let mut t = Tui::new()?;
  t.enter()?;
  loop {    if crossterm::event::poll(Duration::from_millis(250))? {      if let Event::Key(key) = crossterm::event::read()? {        match key.code {          KeyCode::Char('j') => app.increment(),          KeyCode::Char('k') => app.decrement(),          KeyCode::Char('q') => break,          _ => (),        }      }    };
    t.terminal.draw(|f| {      ui(app, f)    })?;  }
  t.exit()?;
  Ok(())}
```

This works perfectly fine, and many small to medium size programs can get away with doing just that.

However, this approach conflates the key input handling with app state updates, and does so in the
“draw” loop. The practical issue with this approach is we block the draw loop for 250 ms waiting for
a key press. This can have odd side effects, for example pressing an holding a key will result in
faster draws to the terminal.

In terms of architecture, the code could get complicated to reason about. For example, we may even
want key presses to mean different things depending on the state of the app (when you are focused
on an input field, you may want to enter the letter `"j"` into the text input field, but when
focused on a list of items, you may want to scroll down the list.)

First, instead of polling, we are going to introduce channels to get the key presses asynchronously
and send them over a channel. We will then receive on the channel in the main loop.

This block of code creates a new `tokio::task` to asynchronously run the event loop. This makes sure
that our main thread isn’t block due to things like polling for `key_events`.

The `event_loop` function is defined as follows:

```
async fn event_loop(        event_tx: UnboundedSender&#x3C;Event>,        cancellation_token: CancellationToken,        tick_rate: f64,        frame_rate: f64,    ) {        let mut event_stream = EventStream::new();        let mut tick_interval = interval(Duration::from_secs_f64(1.0 / tick_rate));        let mut render_interval = interval(Duration::from_secs_f64(1.0 / frame_rate));
        // if this fails, then it's likely a bug in the calling code        event_tx            .send(Event::Init)            .expect("failed to send init event");        loop {            let event = tokio::select! {                _ = cancellation_token.cancelled() => {                    break;                }                _ = tick_interval.tick() => Event::Tick,                _ = render_interval.tick() => Event::Render,                crossterm_event = event_stream.next().fuse() => match crossterm_event {                    Some(Ok(event)) => match event {                        CrosstermEvent::Key(key) if key.kind == KeyEventKind::Press => Event::Key(key),                        CrosstermEvent::Mouse(mouse) => Event::Mouse(mouse),                        CrosstermEvent::Resize(x, y) => Event::Resize(x, y),                        CrosstermEvent::FocusLost => Event::FocusLost,                        CrosstermEvent::FocusGained => Event::FocusGained,                        CrosstermEvent::Paste(s) => Event::Paste(s),                        _ => continue, // ignore other events                    }                    Some(Err(_)) => Event::Error,                    None => break, // the event stream has stopped and will not produce any more events                },            };            if event_tx.send(event).is_err() {                // the receiver has been dropped, so there's no point in continuing the loop                break;            }        }        cancellation_token.cancel();    }
```

Caution

A lot of examples out there in the wild might use the following code for sending key presses:

```
CrosstermEvent::Key(e) => tx.send(Event::Key(e)),
```

However, on Windows, when using `Crossterm`, this will send the same `Event::Key(e)` twice; one for
when you press the key, i.e. `KeyEventKind::Press` and one for when you release the key, i.e.
`KeyEventKind::Release`. On `MacOS` and `Linux` only `KeyEventKind::Press` kinds of `key` event is
generated.

To make the code work as expected across all platforms, you can do this instead:

```
CrosstermEvent::Key(key) => {    if key.kind == KeyEventKind::Press {      event_tx.send(Event::Key(key)).unwrap();    }  },
```

The event loop function takes an `event_tx`. It uses this to send events (like KeyPresses) to other
parts of our app. This is done using unbounded Multiple Producer Single Consumer (`mpsc`) channels.
The function creates initializes the tick rate (time delay between `ticks`), frame rate, and an
`event_stream`. A `tick` is a fundamental unit of time for our app. Think of it as a `CLOCK` for our
app, similar to ones found in microcontrollers. Every tick, the execution of our app moves forward.
The default tick rate is 4 ticks per second (also known as TPS). After this, the loop gets events
and passes them to our app. The possible events are:

```
pub enum Event {    Init,    Quit,    Error,    Closed,    Tick,    Render,    FocusGained,    FocusLost,    Paste(String),    Key(KeyEvent),    Mouse(MouseEvent),    Resize(u16, u16),}
```

## Cleanup and Teardown

[Section titled “Cleanup and Teardown”](#cleanup-and-teardown)

When it’s time to stop the app, the `Tui` struct has a `cancellation_token` field. This is a
`CancellationToken` that can be used to stop the `tokio` task on request. When the `exit` method is
called, it calls the `stop` method, which stops all pending `tokio` tasks. After this, we clean up
the terminal and make sure that we don’t leave the user’s terminal in an unusable state. In case our
app terminates unexpectedly, we don’t want to ruin our user’s terminal. So we implement the `Drop`
trait on the `Tui` struct. When it is dropped, it calls the exit function, restoring the terminal.

```
impl Drop for Tui {    fn drop(&#x26;mut self) {        self.exit().unwrap();    }}
```

Note

Read about graceful cleanup of the terminal in case of an error with
[panic hooks](https://ratatui.rs/recipes/apps/panic-hooks/).

## Finished Code

[Section titled “Finished Code”](#finished-code)

```
use std::{    io::{Stdout, stdout},    ops::{Deref, DerefMut},    time::Duration,};
use color_eyre::Result;use crossterm::{    cursor,    event::{        DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,        Event as CrosstermEvent, EventStream, KeyEvent, KeyEventKind, MouseEvent,    },    terminal::{EnterAlternateScreen, LeaveAlternateScreen},};use futures::{FutureExt, StreamExt};use ratatui::backend::CrosstermBackend as Backend;use serde::{Deserialize, Serialize};use tokio::{    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},    task::JoinHandle,    time::interval,};use tokio_util::sync::CancellationToken;use tracing::error;
#[derive(Clone, Debug, Serialize, Deserialize)]pub enum Event {    Init,    Quit,    Error,    Closed,    Tick,    Render,    FocusGained,    FocusLost,    Paste(String),    Key(KeyEvent),    Mouse(MouseEvent),    Resize(u16, u16),}
pub struct Tui {    pub terminal: ratatui::Terminal&#x3C;Backend&#x3C;Stdout>>,    pub task: JoinHandle&#x3C;()>,    pub cancellation_token: CancellationToken,    pub event_rx: UnboundedReceiver&#x3C;Event>,    pub event_tx: UnboundedSender&#x3C;Event>,    pub frame_rate: f64,    pub tick_rate: f64,    pub mouse: bool,    pub paste: bool,}
impl Tui {    pub fn new() -> Result&#x3C;Self> {        let (event_tx, event_rx) = mpsc::unbounded_channel();        Ok(Self {            terminal: ratatui::Terminal::new(Backend::new(stdout()))?,            task: tokio::spawn(async {}),            cancellation_token: CancellationToken::new(),            event_rx,            event_tx,            frame_rate: 60.0,            tick_rate: 4.0,            mouse: false,            paste: false,        })    }
    pub fn tick_rate(mut self, tick_rate: f64) -> Self {        self.tick_rate = tick_rate;        self    }
    pub fn frame_rate(mut self, frame_rate: f64) -> Self {        self.frame_rate = frame_rate;        self    }
    pub fn mouse(mut self, mouse: bool) -> Self {        self.mouse = mouse;        self    }
    pub fn paste(mut self, paste: bool) -> Self {        self.paste = paste;        self    }
    pub fn start(&#x26;mut self) {        self.cancel(); // Cancel any existing task        self.cancellation_token = CancellationToken::new();        let event_loop = Self::event_loop(            self.event_tx.clone(),            self.cancellation_token.clone(),            self.tick_rate,            self.frame_rate,        );        self.task = tokio::spawn(async {            event_loop.await;        });    }    async fn event_loop(        event_tx: UnboundedSender&#x3C;Event>,        cancellation_token: CancellationToken,        tick_rate: f64,        frame_rate: f64,    ) {        let mut event_stream = EventStream::new();        let mut tick_interval = interval(Duration::from_secs_f64(1.0 / tick_rate));        let mut render_interval = interval(Duration::from_secs_f64(1.0 / frame_rate));
        // if this fails, then it's likely a bug in the calling code        event_tx            .send(Event::Init)            .expect("failed to send init event");        loop {            let event = tokio::select! {                _ = cancellation_token.cancelled() => {                    break;                }                _ = tick_interval.tick() => Event::Tick,                _ = render_interval.tick() => Event::Render,                crossterm_event = event_stream.next().fuse() => match crossterm_event {                    Some(Ok(event)) => match event {                        CrosstermEvent::Key(key) if key.kind == KeyEventKind::Press => Event::Key(key),                        CrosstermEvent::Mouse(mouse) => Event::Mouse(mouse),                        CrosstermEvent::Resize(x, y) => Event::Resize(x, y),                        CrosstermEvent::FocusLost => Event::FocusLost,                        CrosstermEvent::FocusGained => Event::FocusGained,                        CrosstermEvent::Paste(s) => Event::Paste(s),                        _ => continue, // ignore other events                    }                    Some(Err(_)) => Event::Error,                    None => break, // the event stream has stopped and will not produce any more events                },            };            if event_tx.send(event).is_err() {                // the receiver has been dropped, so there's no point in continuing the loop                break;            }        }        cancellation_token.cancel();    }
    pub fn stop(&#x26;self) -> Result&#x3C;()> {        self.cancel();        let mut counter = 0;        while !self.task.is_finished() {            std::thread::sleep(Duration::from_millis(1));            counter += 1;            if counter > 50 {                self.task.abort();            }            if counter > 100 {                error!("Failed to abort task in 100 milliseconds for unknown reason");                break;            }        }        Ok(())    }
    pub fn enter(&#x26;mut self) -> Result&#x3C;()> {        crossterm::terminal::enable_raw_mode()?;        crossterm::execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;        if self.mouse {            crossterm::execute!(stdout(), EnableMouseCapture)?;        }        if self.paste {            crossterm::execute!(stdout(), EnableBracketedPaste)?;        }        self.start();        Ok(())    }
    pub fn exit(&#x26;mut self) -> Result&#x3C;()> {        self.stop()?;        if crossterm::terminal::is_raw_mode_enabled()? {            self.flush()?;            if self.paste {                crossterm::execute!(stdout(), DisableBracketedPaste)?;            }            if self.mouse {                crossterm::execute!(stdout(), DisableMouseCapture)?;            }            crossterm::execute!(stdout(), LeaveAlternateScreen, cursor::Show)?;            crossterm::terminal::disable_raw_mode()?;        }        Ok(())    }
    pub fn cancel(&#x26;self) {        self.cancellation_token.cancel();    }
    pub fn suspend(&#x26;mut self) -> Result&#x3C;()> {        self.exit()?;        #[cfg(not(windows))]        signal_hook::low_level::raise(signal_hook::consts::signal::SIGTSTP)?;        Ok(())    }
    pub fn resume(&#x26;mut self) -> Result&#x3C;()> {        self.enter()?;        Ok(())    }
    pub async fn next_event(&#x26;mut self) -> Option&#x3C;Event> {        self.event_rx.recv().await    }}
impl Deref for Tui {    type Target = ratatui::Terminal&#x3C;Backend&#x3C;Stdout>>;
    fn deref(&#x26;self) -> &#x26;Self::Target {        &#x26;self.terminal    }}
impl DerefMut for Tui {    fn deref_mut(&#x26;mut self) -> &#x26;mut Self::Target {        &#x26;mut self.terminal    }}
impl Drop for Tui {    fn drop(&#x26;mut self) {        self.exit().unwrap();    }}
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/templates/component/tui-rs.md)

 [Previous Main.rs](/templates/component/main-rs/) [Next Action.rs](/templates/component/action-rs/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
