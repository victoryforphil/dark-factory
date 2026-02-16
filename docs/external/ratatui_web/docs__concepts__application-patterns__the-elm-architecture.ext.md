----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /concepts/application-patterns/the-elm-architecture
- Keywords: ratatui, rust, tui, terminal ui, docs, concepts, application patterns, the elm architecture
- Summary: When building terminal user interfaces (TUI) with `ratatui`, it’s helpful to have a solid structure for organizing your application. One proven architecture comes from the Elm language, known simply as The Elm Architecture (TEA).
----

Source: https://ratatui.rs/concepts/application-patterns/the-elm-architecture

When building terminal user interfaces (TUI) with `ratatui`, it’s helpful to have a solid structure for organizing your application. One proven architecture comes from the Elm language, known simply as The Elm Architecture (TEA).

In this section, we’ll explore how to apply The Elm Architecture principles to `ratatui` TUI apps.

The Elm Architecture: A Quick Overview
--------------------------------------

[Section titled “The Elm Architecture: A Quick Overview”](http://ratatui.rs/concepts/application-patterns/the-elm-architecture#the-elm-architecture-a-quick-overview)

At its core, TEA is split into three main components:

*   **Model**: This is your application’s state. It contains all the data your application works with.
*   **Update**: When there’s a change (like user input), the update function takes the current model and the input, and produces a new model.
*   **View**: This function is responsible for displaying your model to the user. In Elm, it produces HTML. In our case, it’ll produce terminal UI elements.

Applying The Elm Architecture to `ratatui`
------------------------------------------

[Section titled “Applying The Elm Architecture to ratatui”](http://ratatui.rs/concepts/application-patterns/the-elm-architecture#applying-the-elm-architecture-to-ratatui)

Following TEA principles typically involves ensuring that you do the following things:

1.   Define Your Model
2.   Handling Updates
3.   Rendering the View

### 1. Define Your Model

[Section titled “1. Define Your Model”](http://ratatui.rs/concepts/application-patterns/the-elm-architecture#1-define-your-model)

In `ratatui`, you’ll typically use a `struct` to represent your model:

`struct Model {    //... your application's data goes here}`

For a counter app, our model may look like this:

`#[derive(Debug, Default)]struct Model {    counter: i32,    running_state: RunningState,}#[derive(Debug, Default, PartialEq, Eq)]enum RunningState {    #[default]    Running,    Done,}`

### 2. Handling Updates

[Section titled “2. Handling Updates”](http://ratatui.rs/concepts/application-patterns/the-elm-architecture#2-handling-updates)

Updates in TEA are actions triggered by events, such as user inputs. The core idea is to map each of these actions or events to a message. This can be achieved by creating an enum to keep track of messages. Based on the received message, the current state of the model is used to determine the next state.

**Defining a `Message` enum**

`enum Message {    //... various inputs or actions that your app cares about    // e.g., ButtonPressed, TextEntered, etc.}`

For a counter app, our `Message` enum may look like this:

`#[derive(PartialEq)]enum Message {    Increment,    Decrement,    Reset,    Quit,}`

**`update()` function**

The update function is at the heart of this process. It takes the current model and a message, and decides how the model should change in response to that message.

A key feature of TEA is immutability. Hence, the update function should avoid direct mutation of the model. Instead, it should produce a new instance of the model reflecting the desired changes.

`fn update(model: &Model, msg: Message) -> Model {    match msg {        // Match each possible message and decide how the model should change        // Return a new model reflecting those changes    }}`

In TEA, it’s crucial to maintain a clear separation between the data (model) and the logic that alters it (update). This immutability principle ensures predictability and makes the application easier to reason about.

In TEA, the `update()` function can not only modify the model based on the `Message`, but it can also return another `Message`. This design can be particularly useful if you want to chain messages or have an update lead to another update.

For example, this is what the `update()` function may look like for a counter app:

`fn update(model: &mut Model, msg: Message) -> Option<Message> {    match msg {        Message::Increment => {            model.counter += 1;            if model.counter > 50 {                return Some(Message::Reset);            }        }        Message::Decrement => {            model.counter -= 1;            if model.counter < -50 {                return Some(Message::Reset);            }        }        Message::Reset => model.counter = 0,        Message::Quit => {            // You can handle cleanup and exit here            model.running_state = RunningState::Done;        }    };    None}`

Remember that this design choice means that the `main` loop will need to handle the returned message, calling `update()` again based on that returned message.

Returning a `Message` from the `update()` function allows a developer to reason about their code as a “Finite State Machine”. Finite State Machines operate on defined states and transitions, where an initial state and an event (in our case, a `Message`) lead to a subsequent state. This cascading approach ensures that the system remains in a consistent and predictable state after handling a series of interconnected events.

Here’s a state transition diagram of the counter example from above:

While TEA doesn’t use the Finite State Machine terminology or strictly enforce that paradigm, thinking of your application’s state as a state machine can allow developers to break down intricate state transitions into smaller, more manageable steps. This can make designing the application’s logic clearer and improve code maintainability.

### 3. Rendering the View

[Section titled “3. Rendering the View”](http://ratatui.rs/concepts/application-patterns/the-elm-architecture#3-rendering-the-view)

The view function in the Elm Architecture is tasked with taking the current model and producing a visual representation for the user. In the case of ratatui, it translates the model into terminal UI elements. It’s essential that the view function remains a pure function: for a given state of the model, it should always produce the same UI representation.

`fn view(model: &Model) {    //... use `ratatui` functions to draw your UI based on the model's state}`

Every time the model is updated, the view function should be capable of reflecting those changes accurately in the terminal UI.

A view for a simple counter app might look like:

`fn view(model: &mut Model, frame: &mut Frame) {    frame.render_widget(        Paragraph::new(format!("Counter: {}", model.counter)),        frame.area(),    );}`

In TEA, you are expected to ensure that your view function is side-effect free. The `view()` function shouldn’t modify global state or perform any other actions. Its sole job is to map the model to a visual representation.

For a given state of the model, the view function should always produce the same visual output. This predictability makes your TUI application easier to reason about and debug.

In `ratatui`, there are [`StatefulWidget`s](https://docs.rs/ratatui/latest/ratatui/widgets/trait.StatefulWidget.html) which require a mutable reference to state during render.

For this reason, you may choose to forego the `view` immutability principle. For example, if you were interested in rendering a `List`, your `view` function may look like this:

`fn view(model: &mut Model, f: &mut Frame) {    let items = model.items.items.iter().map(|element| ListItem::new(element)).collect();    f.render_stateful_widget(List::new(items), f.area(), &mut model.items.state);}fn main() {    loop {        ...        terminal.draw(|f| view(&mut model, f) )?;        ...    }}`

Another advantage of having access to the `Frame` in the `view()` function is that you have access to setting the cursor position, which is useful for displaying text fields. For example, if you wanted to draw an input field using [`tui-input`](https://github.com/sayanarijit/tui-input), you might have a `view` that looks like this:

`fn view(model: &mut Model, f: &mut Frame) {    let area = f.area();    let input = Paragraph::new(app.input.value());    f.render_widget(input, area);    if model.mode == Mode::Insert {        f.set_cursor(            (area.x + 1 + self.input.cursor() as u16).min(area.x + area.width - 2),            area.y + 1        )    }}`

Putting it all together
-----------------------

[Section titled “Putting it all together”](http://ratatui.rs/concepts/application-patterns/the-elm-architecture#putting-it-all-together)

When you put it all together, your main application loop might look something like:

*   Listen for user input.
*   Map input to a `Message`
*   Pass that message to the update function.
*   Draw the UI with the view function.

This cycle repeats, ensuring your TUI is always up-to-date with user interactions.

As an illustrative example, here’s the [Counter App](https://ratatui.rs/tutorials/counter-app/) refactored using TEA.

The notable difference from before is that we have a `Model` struct that captures the app state, and a `Message` enum that captures the various actions your app can take.

`use std::time::Duration;use ratatui::{    crossterm::event::{self, Event, KeyCode},    widgets::Paragraph,    Frame,};#[derive(Debug, Default)]struct Model {    counter: i32,    running_state: RunningState,}#[derive(Debug, Default, PartialEq, Eq)]enum RunningState {    #[default]    Running,    Done,}#[derive(PartialEq)]enum Message {    Increment,    Decrement,    Reset,    Quit,}fn main() -> color_eyre::Result<()> {    tui::install_panic_hook();    let mut terminal = tui::init_terminal()?;    let mut model = Model::default();    while model.running_state != RunningState::Done {        // Render the current view        terminal.draw(|f| view(&mut model, f))?;        // Handle events and map to a Message        let mut current_msg = handle_event(&model)?;        // Process updates as long as they return a non-None message        while current_msg.is_some() {            current_msg = update(&mut model, current_msg.unwrap());        }    }    tui::restore_terminal()?;    Ok(())}fn view(model: &mut Model, frame: &mut Frame) {    frame.render_widget(        Paragraph::new(format!("Counter: {}", model.counter)),        frame.area(),    );}/// Convert Event to Message////// We don't need to pass in a `model` to this function in this example/// but you might need it as your project evolvesfn handle_event(_: &Model) -> color_eyre::Result<Option<Message>> {    if event::poll(Duration::from_millis(250))? {        if let Event::Key(key) = event::read()? {            if key.kind == event::KeyEventKind::Press {                return Ok(handle_key(key));            }        }    }    Ok(None)}fn handle_key(key: event::KeyEvent) -> Option<Message> {    match key.code {        KeyCode::Char('j') => Some(Message::Increment),        KeyCode::Char('k') => Some(Message::Decrement),        KeyCode::Char('q') => Some(Message::Quit),        _ => None,    }}fn update(model: &mut Model, msg: Message) -> Option<Message> {    match msg {        Message::Increment => {            model.counter += 1;            if model.counter > 50 {                return Some(Message::Reset);            }        }        Message::Decrement => {            model.counter -= 1;            if model.counter < -50 {                return Some(Message::Reset);            }        }        Message::Reset => model.counter = 0,        Message::Quit => {            // You can handle cleanup and exit here            model.running_state = RunningState::Done;        }    };    None}mod tui {    use ratatui::{        backend::{Backend, CrosstermBackend},        crossterm::{            terminal::{                disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,            },            ExecutableCommand,        },        Terminal,    };    use std::{io::stdout, panic};    pub fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {        enable_raw_mode()?;        stdout().execute(EnterAlternateScreen)?;        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;        Ok(terminal)    }    pub fn restore_terminal() -> color_eyre::Result<()> {        stdout().execute(LeaveAlternateScreen)?;        disable_raw_mode()?;        Ok(())    }    pub fn install_panic_hook() {        let original_hook = panic::take_hook();        panic::set_hook(Box::new(move |panic_info| {            stdout().execute(LeaveAlternateScreen).unwrap();            disable_raw_mode().unwrap();            original_hook(panic_info);        }));    }}`

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
