----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /recipes/apps/spawn-vim
- Keywords: ratatui, rust, tui, terminal ui, docs, recipes, apps, spawn vim
- Summary: In this recipe, we will explore how to spawn an external editor (Vim) from within the TUI app. This
----

Source: https://ratatui.rs/recipes/apps/spawn-vim

# Spawn External Editor (Vim)

In this recipe, we will explore how to spawn an external editor (Vim) from within the TUI app. This
example demonstrates how to temporarily exit the TUI, run an external command, and then return back
to our TUI app.

Full code:

main.rs (click to expand)

```
use ratatui::{    backend::CrosstermBackend,    crossterm::{        event::{self, Event, KeyCode, KeyEventKind},        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},        ExecutableCommand,    },    widgets::Paragraph,    DefaultTerminal, Frame,};use std::io::{stdout, Result};use std::process::Command;
type Terminal = ratatui::Terminal&#x3C;CrosstermBackend&#x3C;std::io::Stdout>>;
enum Action {    EditFile,    Quit,    None,}
fn main() -> Result&#x3C;()> {    let terminal = ratatui::init();    let app_result = run(terminal);    ratatui::restore();    app_result}
fn run(mut terminal: DefaultTerminal) -> Result&#x3C;()> {    loop {        terminal.draw(draw)?;        match handle_events()? {            Action::EditFile => run_editor(&#x26;mut terminal)?,            Action::Quit => break,            Action::None => {}        }    }    Ok(())}
fn handle_events() -> Result&#x3C;Action> {    if !event::poll(std::time::Duration::from_millis(16))? {        return Ok(Action::None);    }    match event::read()? {        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {            KeyCode::Char('q') => Ok(Action::Quit),            KeyCode::Char('e') => Ok(Action::EditFile),            _ => Ok(Action::None),        },        _ => Ok(Action::None),    }}
fn run_editor(terminal: &#x26;mut Terminal) -> Result&#x3C;()> {    stdout().execute(LeaveAlternateScreen)?;    disable_raw_mode()?;    Command::new("vim").arg("/tmp/a.txt").status()?;    stdout().execute(EnterAlternateScreen)?;    enable_raw_mode()?;    terminal.clear()?;    Ok(())}
fn draw(frame: &#x26;mut Frame) {    frame.render_widget(        Paragraph::new("Hello ratatui! (press 'q' to quit, 'e' to edit a file)"),        frame.area(),    );}
```

## Setup

[Section titled “Setup”](#setup)

First, let’s look at the main function and the event handling logic:

main.rs

```
enum Action {    EditFile,    Quit,    None,}
fn main() -> Result&#x3C;()> {    let terminal = ratatui::init();    let app_result = run(terminal);    ratatui::restore();    app_result}
fn run(mut terminal: DefaultTerminal) -> Result&#x3C;()> {    loop {        terminal.draw(draw)?;        match handle_events()? {            Action::EditFile => run_editor(&#x26;mut terminal)?,            Action::Quit => break,            Action::None => {}        }    }    Ok(())}
fn handle_events() -> Result&#x3C;Action> {    if !event::poll(std::time::Duration::from_millis(16))? {        return Ok(Action::None);    }    match event::read()? {        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {            KeyCode::Char('q') => Ok(Action::Quit),            KeyCode::Char('e') => Ok(Action::EditFile),            _ => Ok(Action::None),        },        _ => Ok(Action::None),    }}
```

After initializing the terminal in `main` function, we enter a loop in `run` function where we draw
the UI and handle events. The `handle_events` function listens for key events and returns an
`Action` based on the key pressed. Here, we are calling `run_editor` function on `Action::EditFile`
which we will define in next section.

## Spawning Vim

[Section titled “Spawning Vim”](#spawning-vim)

Now, let’s define the function `run_editor` function attached to `Action::EditFile` action.

main.rs

```
fn run_editor(terminal: &#x26;mut Terminal) -> Result&#x3C;()> {    stdout().execute(LeaveAlternateScreen)?;    disable_raw_mode()?;    Command::new("vim").arg("/tmp/a.txt").status()?;    stdout().execute(EnterAlternateScreen)?;    enable_raw_mode()?;    terminal.clear()?;    Ok(())}
```

To spawn Vim from our TUI app, we first need to relinquish control of input and output, allowing Vim
to have full control over the terminal.

The `run_editor` function handles the logic for spawning vim. First, we leave the alternate screen
and disable raw mode to restore terminal to it’s original state. This part is similar to what
[`ratatui::restore`](https://docs.rs/ratatui/latest/ratatui/fn.restore.html) function does in the
`main` function. Next, we spawn a child process with
`Command::new("vim").arg("/tmp/a.txt").status()` which launches `vim` to edit the given file. At
this point, we have given up control of our TUI app to vim. Our TUI app will now wait for the exit
status of the child process. Once the user exits Vim, our TUI app regains control over the terminal
by re-entering alternate screen and enabling raw mode. Lastly, we clear the terminal to ensure the
TUI is displayed correctly.

Note

Before running another application from your app, you must relinquish control of input and output,
allowing the other app to function correctly.

In the example above, we use a simple event-handling setup. However, if you are using advanced
setups like [component template](https://github.com/ratatui/templates), you will need to pause input
events before spawning an external process like Vim. Otherwise, Vim won’t have full control over
keybindings and it won’t work properly.

Using the
[`tui` module](https://github.com/ratatui/templates/blob/5e823efc871107345d59e5deff9284235c1f0bbc/component/template/src/tui.rs)
of the component template, you can do something like this to pause and resume event handlers:

```
Action::EditFile => {  tui.exit()?;  let cmd = String::from("vim");  let cmd_args = vec!["/tmp/a.txt".into()];  let status = std::process::Command::new(&#x26;command).args(&#x26;args).status()?;  if !status.success() {    eprintln!("\nCommand failed with status: {}", status);  }  tui.enter()?;  tui.terminal.clear();}
```

One more thing to note is that when attempting to start an external process without using the
pattern in the component template, issues can arise such as ANSI RGB values being printed into the
TUI upon returning from the external process. This happens because Vim requests the terminal
background color, and when the terminal responds over stdin, those responses are read by Crossterm
instead. If you encounter such issues, please refer to
[orhun/rattler-build@84ea16a](https://github.com/orhun/rattler-build/commit/84ea16a4f5af33e2703b6330fcb977065263cef6)
and [kdheepak/taskwarrior-tui#46](https://github.com/kdheepak/taskwarrior-tui/issues/46). Using
`select!` + `cancellation_token` + `tokio` as in the component template avoids this problem.

## Running code

[Section titled “Running code”](#running-code)

Running this program will display “Hello ratatui! (press ‘q’ to quit, ‘e’ to edit a file)” in the
terminal. Pressing ‘e’ will spawn a child process to spawn Vim for editing a temporary file and then
return to the ratatui application after Vim is closed.

Feel free to adapt this example to use other editors like `nvim`, `nano`, etc., by changing the
command in the `Action::EditFile` arm.

Tip

If you prefer to launch the user-specified `$EDITOR` and retrieve the buffer (edited content) back
into your application, you can use the [`edit`](https://crates.io/crates/edit) crate. This can be
particularly useful if you need to capture the changes made by the user in the editor. There’s also
[`editor-command`](https://docs.rs/editor-command/latest/editor_command) crate if you want more
control over launching / overriding editors based on `VISUAL` or `EDITOR` environment variables.

Alternatively, you may use the [`edtui`](https://github.com/preiter93/edtui) crate from ratatui’s
ecosystem, which provides text editor widget inspired by vim.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/recipes/apps/spawn-vim.md)

 [Previous Migrate from tui-rs](/recipes/apps/migrate-from-tui-rs/) [Next Releasing Your App](/recipes/apps/release-your-app/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
