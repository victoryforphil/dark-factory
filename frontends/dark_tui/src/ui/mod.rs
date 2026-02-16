mod render;

use std::env;
use std::io::{self, Stdout};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use arboard::Clipboard;
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    KeyModifiers, MouseButton, MouseEventKind,
};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;

use crate::app::{App, ResultsViewMode};
use crate::cli::Cli;
use crate::service::DashboardService;
use crate::theme::Theme;

type TuiTerminal = Terminal<CrosstermBackend<Stdout>>;

enum LoopAction {
    None,
    Quit,
    Refresh,
    PollVariant,
    InitProduct,
    OpenSpawnForm,
    SpawnSession,
    BuildAttach,
}

pub async fn run(cli: Cli) -> Result<()> {
    let directory = resolve_directory(cli.directory.as_deref())?;
    let service = DashboardService::new(cli.base_url.clone(), directory.clone(), cli.poll_variants);

    // Load theme — look for themes/default.toml relative to the executable,
    // falling back to the compiled-in defaults.
    let theme = {
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()));
        let candidates = [
            Some(PathBuf::from("themes/default.toml")),
            exe_dir.map(|d| d.join("themes/default.toml")),
        ];
        candidates
            .iter()
            .flatten()
            .find(|p| p.exists())
            .map(|p| Theme::load(p))
            .unwrap_or_default()
    };

    let mut app = App::new(directory, cli.refresh_seconds, theme);
    app.set_status(format!("Connected to {}", cli.base_url));

    let mut terminal = setup_terminal()?;

    let run_result = run_loop(&mut terminal, &service, &mut app).await;
    let restore_result = restore_terminal(&mut terminal);

    if let Err(error) = restore_result {
        if run_result.is_ok() {
            return Err(error);
        }
    }

    run_result
}

async fn run_loop(
    terminal: &mut TuiTerminal,
    service: &DashboardService,
    app: &mut App,
) -> Result<()> {
    let refresh_interval = Duration::from_secs(app.refresh_seconds().max(2));
    let mut force_refresh = true;
    let mut next_refresh_at = Instant::now();

    loop {
        if force_refresh || Instant::now() >= next_refresh_at {
            match service.fetch_snapshot().await {
                Ok(snapshot) => {
                    app.apply_snapshot(snapshot);
                    app.set_status(format!(
                        "World state refreshed (directory={})",
                        service.directory()
                    ));
                }
                Err(error) => {
                    app.set_status(format!("Refresh failed: {error}"));
                }
            }

            next_refresh_at = Instant::now() + refresh_interval;
            force_refresh = false;
        }

        terminal.draw(|frame| render::render_dashboard(frame, app))?;

        if !event::poll(Duration::from_millis(120))? {
            continue;
        }

        let ev = event::read()?;

        // --- Mouse events (viz-mode 2D drag/scroll) ---
        if let Event::Mouse(mouse) = &ev {
            if app.results_view_mode() == ResultsViewMode::Viz {
                match mouse.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        let size = terminal.size()?;
                        let root = Rect {
                            x: 0,
                            y: 0,
                            width: size.width,
                            height: size.height,
                        };
                        if !render::try_select_viz_node(root, app, mouse.column, mouse.row) {
                            app.start_drag(mouse.column, mouse.row);
                        }
                    }
                    MouseEventKind::Drag(_) => {
                        app.apply_drag(mouse.column, mouse.row);
                    }
                    MouseEventKind::Up(_) => {
                        app.end_drag();
                    }
                    MouseEventKind::ScrollUp => {
                        app.viz_scroll(-3);
                    }
                    MouseEventKind::ScrollDown => {
                        app.viz_scroll(3);
                    }
                    _ => {}
                }
            }
            continue;
        }

        let Event::Key(key) = ev else {
            continue;
        };

        if key.kind != KeyEventKind::Press {
            continue;
        }

        let action = handle_key(app, key);
        match action {
            LoopAction::None => {}
            LoopAction::Quit => break,
            LoopAction::Refresh => {
                force_refresh = true;
            }
            LoopAction::PollVariant => {
                let Some(variant_id) = app.selected_variant_id().map(ToString::to_string) else {
                    app.set_status("Poll skipped: no variant selected.");
                    continue;
                };

                match service.poll_variant(&variant_id).await {
                    Ok(message) => {
                        app.set_status(message);
                        force_refresh = true;
                    }
                    Err(error) => {
                        app.set_status(format!("Variant poll failed: {error}"));
                    }
                }
            }
            LoopAction::InitProduct => match service.init_product().await {
                Ok(message) => {
                    app.set_status(message);
                    force_refresh = true;
                }
                Err(error) => {
                    app.set_status(format!("Init failed: {error}"));
                }
            },
            LoopAction::OpenSpawnForm => match service.fetch_spawn_options().await {
                Ok(options) => {
                    app.open_spawn_form(options.providers, options.default_provider.as_deref());
                    app.set_status("Spawn form open. Choose provider and prompt.");
                }
                Err(error) => {
                    app.set_status(format!("Spawn options failed: {error}"));
                }
            },
            LoopAction::SpawnSession => {
                let Some(request) = app.take_spawn_request() else {
                    app.set_status("Spawn skipped: form is not open.");
                    continue;
                };

                match service
                    .create_session(&request.provider, request.initial_prompt.as_deref())
                    .await
                {
                    Ok(actor_id) => {
                        app.set_status(format!("Spawned in TUI: {actor_id}"));
                        force_refresh = true;
                    }
                    Err(error) => {
                        app.set_status(format!("Spawn failed: {error}"));
                    }
                }
            }
            LoopAction::BuildAttach => {
                let Some(actor_id) = app.selected_actor_id().map(ToString::to_string) else {
                    app.set_status("Attach skipped: no actor selected.");
                    continue;
                };

                match service.build_attach_command(&actor_id).await {
                    Ok(command) => {
                        let status = match copy_to_clipboard(&command) {
                            Ok(()) => "Attach command copied to clipboard.",
                            Err(error) => {
                                app.set_command_message(command.clone());
                                app.set_status(format!(
                                    "Attach command ready (clipboard failed: {error})"
                                ));
                                continue;
                            }
                        };
                        app.set_command_message(command);
                        app.set_status(status);
                    }
                    Err(error) => {
                        app.set_status(format!("Attach command failed: {error}"));
                    }
                }
            }
        }
    }

    Ok(())
}

fn setup_terminal() -> Result<TuiTerminal> {
    enable_raw_mode().context("Dark TUI // Terminal // Failed to enable raw mode")?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("Dark TUI // Terminal // Failed to enter alternate screen")?;

    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend).context("Dark TUI // Terminal // Failed to create terminal")
}

fn restore_terminal(terminal: &mut TuiTerminal) -> Result<()> {
    disable_raw_mode().context("Dark TUI // Terminal // Failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        DisableMouseCapture,
        LeaveAlternateScreen
    )
    .context("Dark TUI // Terminal // Failed to leave alternate screen")?;
    terminal
        .show_cursor()
        .context("Dark TUI // Terminal // Failed to show cursor")?;

    Ok(())
}

fn resolve_directory(path: Option<&str>) -> Result<String> {
    let base_path = match path {
        Some(value) => PathBuf::from(value),
        None => env::current_dir()
            .context("Dark TUI // Directory // Failed to get current directory")?,
    };

    let absolute = if base_path.is_absolute() {
        base_path
    } else {
        env::current_dir()
            .context("Dark TUI // Directory // Failed to get current directory")?
            .join(base_path)
    };

    let canonical = absolute.canonicalize().with_context(|| {
        format!(
            "Dark TUI // Directory // Expected existing path (path={})",
            absolute.display()
        )
    })?;

    Ok(canonical.to_string_lossy().to_string())
}

fn copy_to_clipboard(value: &str) -> Result<()> {
    let mut clipboard = Clipboard::new().context("clipboard init failed")?;
    clipboard
        .set_text(value.to_string())
        .context("clipboard write failed")?;
    Ok(())
}

fn handle_key(app: &mut App, key: KeyEvent) -> LoopAction {
    if app.is_spawn_form_open() {
        return handle_spawn_form_key(app, key);
    }

    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return LoopAction::Quit;
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => LoopAction::Quit,
        KeyCode::Tab => {
            app.focus_next();
            LoopAction::None
        }
        KeyCode::BackTab => {
            app.focus_previous();
            LoopAction::None
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.move_selection_down();
            LoopAction::None
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.move_selection_up();
            LoopAction::None
        }
        KeyCode::Char('r') => LoopAction::Refresh,
        KeyCode::Char('f') => {
            app.toggle_variant_filter();
            LoopAction::None
        }
        KeyCode::Char('v') | KeyCode::Char(' ') => {
            app.toggle_results_view_mode();
            app.set_status(format!(
                "Results view mode: {}",
                app.results_view_mode().label()
            ));
            LoopAction::None
        }
        KeyCode::Char('p') => LoopAction::PollVariant,
        KeyCode::Char('i') => LoopAction::InitProduct,
        KeyCode::Char('n') => LoopAction::OpenSpawnForm,
        KeyCode::Char('a') => LoopAction::BuildAttach,
        KeyCode::Char('0') => {
            app.reset_viz_offset();
            app.set_status("Pan reset to origin");
            LoopAction::None
        }
        _ => LoopAction::None,
    }
}

fn handle_spawn_form_key(app: &mut App, key: KeyEvent) -> LoopAction {
    match key.code {
        KeyCode::Esc => {
            app.close_spawn_form();
            app.set_status("Spawn form closed.");
            LoopAction::None
        }
        KeyCode::Enter => LoopAction::SpawnSession,
        KeyCode::Up | KeyCode::Char('k') => {
            app.spawn_form_move_provider_up();
            LoopAction::None
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.spawn_form_move_provider_down();
            LoopAction::None
        }
        KeyCode::Backspace => {
            app.spawn_form_backspace();
            LoopAction::None
        }
        KeyCode::Char(value)
            if !key.modifiers.contains(KeyModifiers::CONTROL)
                && !key.modifiers.contains(KeyModifiers::ALT) =>
        {
            app.spawn_form_insert_char(value);
            LoopAction::None
        }
        _ => LoopAction::None,
    }
}
