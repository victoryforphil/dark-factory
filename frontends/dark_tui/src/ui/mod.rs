mod render;

use std::env;
use std::future::Future;
use std::io::{self, Stdout};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, anyhow};
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
use crate::models::{ActorChatMessageRow, DashboardSnapshot};
use crate::service::{DashboardService, SpawnOptions};
use crate::theme::Theme;

type TuiTerminal = Terminal<CrosstermBackend<Stdout>>;
const API_TIMEOUT_SECONDS: u64 = 20;

enum LoopAction {
    None,
    Quit,
    Refresh,
    PollVariant,
    ImportVariantActors,
    InitProduct,
    OpenSpawnForm,
    SpawnSession,
    BuildAttach,
    ToggleChat,
    OpenChatCompose,
    SendChatMessage,
}

enum BackgroundActionResult {
    PollVariant(Result<String>),
    ImportVariantActors(Result<String>),
    InitProduct(Result<String>),
    SpawnOptions(Result<SpawnOptions>),
    SpawnSession(Result<String>),
    BuildAttach(Result<String>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BackgroundActionKind {
    PollVariant,
    ImportVariantActors,
    InitProduct,
    SpawnOptions,
    SpawnSession,
    BuildAttach,
}

struct ActionTask {
    kind: BackgroundActionKind,
    handle: tokio::task::JoinHandle<BackgroundActionResult>,
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
    let mut snapshot_task: Option<tokio::task::JoinHandle<Result<DashboardSnapshot>>> = None;
    let mut chat_refresh_task: Option<
        tokio::task::JoinHandle<(String, Result<Vec<ActorChatMessageRow>>)>,
    > = None;
    let mut chat_send_task: Option<tokio::task::JoinHandle<(String, Result<()>)>> = None;
    let mut action_tasks: Vec<ActionTask> = Vec::new();

    loop {
        if snapshot_task.as_ref().is_some_and(|task| task.is_finished()) {
            let Some(task) = snapshot_task.take() else {
                unreachable!("snapshot task should exist when marked finished");
            };
            app.set_snapshot_refresh_in_flight(false);
            match task.await {
                Ok(Ok(snapshot)) => {
                    app.apply_snapshot(snapshot);
                    app.set_status(format!(
                        "World state refreshed (directory={})",
                        service.directory()
                    ));
                }
                Ok(Err(error)) => {
                    app.set_status(format!("Refresh failed: {error}"));
                }
                Err(error) => {
                    app.set_status(format!("Refresh task failed: {error}"));
                }
            }

            next_refresh_at = Instant::now() + refresh_interval;
        }

        if snapshot_task.is_none() && (force_refresh || Instant::now() >= next_refresh_at) {
            let service = service.clone();
            app.set_snapshot_refresh_in_flight(true);
            snapshot_task = Some(tokio::spawn(async move {
                run_with_api_timeout(service.fetch_snapshot()).await
            }));
            force_refresh = false;
        }

        if chat_refresh_task.as_ref().is_some_and(|task| task.is_finished()) {
            let Some(task) = chat_refresh_task.take() else {
                unreachable!("chat refresh task should exist when marked finished");
            };
            app.set_chat_refresh_in_flight(false);
            match task.await {
                Ok((actor_id, Ok(messages))) => {
                    app.apply_chat_messages(&actor_id, messages);
                }
                Ok((_actor_id, Err(error))) => {
                    app.set_status(format!("Chat refresh failed: {error}"));
                }
                Err(error) => {
                    app.set_status(format!("Chat refresh task failed: {error}"));
                }
            }
        }

        if chat_refresh_task.is_none() {
            if let Some(actor_id) = app.take_chat_refresh_request() {
                let service = service.clone();
                app.set_chat_refresh_in_flight(true);
                chat_refresh_task = Some(tokio::spawn(async move {
                    let result =
                        run_with_api_timeout(service.fetch_actor_messages(&actor_id, Some(80)))
                            .await;
                    (actor_id, result)
                }));
            }
        }

        if chat_send_task.as_ref().is_some_and(|task| task.is_finished()) {
            let Some(task) = chat_send_task.take() else {
                unreachable!("chat send task should exist when marked finished");
            };
            app.set_chat_send_in_flight(false);
            match task.await {
                Ok((actor_id, Ok(()))) => {
                    app.commit_sent_chat_prompt();
                    app.request_chat_refresh();
                    app.set_status(format!("Message sent to {actor_id}."));
                }
                Ok((_actor_id, Err(error))) => {
                    app.set_status(format!("Chat send failed: {error}"));
                }
                Err(error) => {
                    app.set_status(format!("Chat send task failed: {error}"));
                }
            }
        }

        let mut action_index = 0;
        while action_index < action_tasks.len() {
            if !action_tasks[action_index].handle.is_finished() {
                action_index += 1;
                continue;
            }

            let task = action_tasks.swap_remove(action_index);
            match task.handle.await {
                Ok(BackgroundActionResult::PollVariant(result)) => match result {
                    Ok(message) => {
                        app.set_status(message);
                        force_refresh = true;
                    }
                    Err(error) => {
                        app.set_status(format!("Variant poll failed: {error}"));
                    }
                },
                Ok(BackgroundActionResult::ImportVariantActors(result)) => match result {
                    Ok(message) => {
                        app.set_status(message);
                        force_refresh = true;
                    }
                    Err(error) => {
                        app.set_status(format!("Import failed: {error}"));
                    }
                },
                Ok(BackgroundActionResult::InitProduct(result)) => match result {
                    Ok(message) => {
                        app.set_status(message);
                        force_refresh = true;
                    }
                    Err(error) => {
                        app.set_status(format!("Init failed: {error}"));
                    }
                },
                Ok(BackgroundActionResult::SpawnOptions(result)) => match result {
                    Ok(options) => {
                        app.open_spawn_form(options.providers, options.default_provider.as_deref());
                        app.set_status("Spawn form open. Choose provider and prompt.");
                    }
                    Err(error) => {
                        app.set_status(format!("Spawn options failed: {error}"));
                    }
                },
                Ok(BackgroundActionResult::SpawnSession(result)) => match result {
                    Ok(actor_id) => {
                        app.set_status(format!("Spawned in TUI: {actor_id}"));
                        force_refresh = true;
                    }
                    Err(error) => {
                        app.set_status(format!("Spawn failed: {error}"));
                    }
                },
                Ok(BackgroundActionResult::BuildAttach(result)) => match result {
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
                },
                Err(error) => {
                    app.set_status(format!("Action task failed: {error}"));
                }
            }
        }
        app.set_action_requests_in_flight(action_tasks.len());

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

                if has_action_in_flight(&action_tasks, BackgroundActionKind::PollVariant) {
                    app.set_status("Variant poll already in progress.");
                    continue;
                }

                app.set_status(format!("Polling variant {variant_id}..."));
                let service = service.clone();
                action_tasks.push(ActionTask {
                    kind: BackgroundActionKind::PollVariant,
                    handle: tokio::spawn(async move {
                        BackgroundActionResult::PollVariant(
                            run_with_api_timeout(service.poll_variant(&variant_id)).await,
                        )
                    }),
                });
                app.set_action_requests_in_flight(action_tasks.len());
            }
            LoopAction::ImportVariantActors => {
                let Some(variant_id) = app.selected_variant_id().map(ToString::to_string) else {
                    app.set_status("Import skipped: no variant selected.");
                    continue;
                };

                if has_action_in_flight(&action_tasks, BackgroundActionKind::ImportVariantActors) {
                    app.set_status("Actor import already in progress.");
                    continue;
                }

                app.set_status(format!("Importing provider actors for {variant_id}..."));
                let service = service.clone();
                action_tasks.push(ActionTask {
                    kind: BackgroundActionKind::ImportVariantActors,
                    handle: tokio::spawn(async move {
                        BackgroundActionResult::ImportVariantActors(
                            run_with_api_timeout(service.import_variant_actors(&variant_id, None))
                                .await,
                        )
                    }),
                });
                app.set_action_requests_in_flight(action_tasks.len());
            }
            LoopAction::InitProduct => {
                if has_action_in_flight(&action_tasks, BackgroundActionKind::InitProduct) {
                    app.set_status("Product init already in progress.");
                    continue;
                }

                app.set_status("Initializing product from current directory...");
                let service = service.clone();
                action_tasks.push(ActionTask {
                    kind: BackgroundActionKind::InitProduct,
                    handle: tokio::spawn(async move {
                        BackgroundActionResult::InitProduct(
                            run_with_api_timeout(service.init_product()).await,
                        )
                    }),
                });
                app.set_action_requests_in_flight(action_tasks.len());
            }
            LoopAction::OpenSpawnForm => {
                if has_action_in_flight(&action_tasks, BackgroundActionKind::SpawnOptions) {
                    app.set_status("Spawn options request already in progress.");
                    continue;
                }

                app.set_status("Loading spawn provider options...");
                let service = service.clone();
                action_tasks.push(ActionTask {
                    kind: BackgroundActionKind::SpawnOptions,
                    handle: tokio::spawn(async move {
                        BackgroundActionResult::SpawnOptions(
                            run_with_api_timeout(service.fetch_spawn_options()).await,
                        )
                    }),
                });
                app.set_action_requests_in_flight(action_tasks.len());
            }
            LoopAction::SpawnSession => {
                if has_action_in_flight(&action_tasks, BackgroundActionKind::SpawnSession) {
                    app.set_status("Spawn session already in progress.");
                    continue;
                }

                let Some(request) = app.take_spawn_request() else {
                    app.set_status("Spawn skipped: form is not open.");
                    continue;
                };

                app.set_status("Spawning actor session...");
                let service = service.clone();
                action_tasks.push(ActionTask {
                    kind: BackgroundActionKind::SpawnSession,
                    handle: tokio::spawn(async move {
                        BackgroundActionResult::SpawnSession(
                            run_with_api_timeout(
                                service.create_session(
                                    &request.provider,
                                    request.initial_prompt.as_deref(),
                                ),
                            )
                            .await,
                        )
                    }),
                });
                app.set_action_requests_in_flight(action_tasks.len());
            }
            LoopAction::BuildAttach => {
                let Some(actor_id) = app.selected_actor_id().map(ToString::to_string) else {
                    app.set_status("Attach skipped: no actor selected.");
                    continue;
                };

                if has_action_in_flight(&action_tasks, BackgroundActionKind::BuildAttach) {
                    app.set_status("Attach build already in progress.");
                    continue;
                }

                app.set_status(format!("Building attach command for {actor_id}..."));
                let service = service.clone();
                action_tasks.push(ActionTask {
                    kind: BackgroundActionKind::BuildAttach,
                    handle: tokio::spawn(async move {
                        BackgroundActionResult::BuildAttach(
                            run_with_api_timeout(service.build_attach_command(&actor_id)).await,
                        )
                    }),
                });
                app.set_action_requests_in_flight(action_tasks.len());
            }
            LoopAction::ToggleChat => {
                app.toggle_chat_visibility();
                let status = if app.is_chat_visible() {
                    app.request_chat_refresh();
                    "Chat panel shown."
                } else {
                    "Chat panel hidden."
                };
                app.set_status(status);
            }
            LoopAction::OpenChatCompose => {
                if app.open_chat_composer() {
                    app.set_status("Chat compose mode enabled.");
                } else {
                    app.set_status("Chat compose skipped: select an actor first.");
                }
            }
            LoopAction::SendChatMessage => {
                let Some(actor_id) = app.chat_actor_id().map(ToString::to_string) else {
                    app.set_status("Chat send skipped: no actor selected.");
                    continue;
                };
                let Some(prompt) = app.current_chat_prompt() else {
                    app.set_status("Chat send skipped: prompt is empty.");
                    continue;
                };

                if chat_send_task.is_some() {
                    app.set_status("Chat send already in progress.");
                    continue;
                }

                app.set_status(format!("Sending message to {actor_id}..."));
                let service = service.clone();
                app.set_chat_send_in_flight(true);
                chat_send_task = Some(tokio::spawn(async move {
                    let result = run_with_api_timeout(service.send_actor_prompt(&actor_id, &prompt))
                        .await;
                    (actor_id, result)
                }));
            }
        }
    }

    Ok(())
}

async fn run_with_api_timeout<T>(future: impl Future<Output = Result<T>>) -> Result<T> {
    match tokio::time::timeout(Duration::from_secs(API_TIMEOUT_SECONDS), future).await {
        Ok(result) => result,
        Err(_) => Err(anyhow!(
            "request timed out after {}s",
            API_TIMEOUT_SECONDS
        )),
    }
}

fn has_action_in_flight(tasks: &[ActionTask], kind: BackgroundActionKind) -> bool {
    tasks.iter().any(|task| task.kind == kind)
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

    if app.is_chat_composing() {
        return handle_chat_compose_key(app, key);
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
        KeyCode::Char('m') => LoopAction::ImportVariantActors,
        KeyCode::Char('i') => LoopAction::InitProduct,
        KeyCode::Char('n') => LoopAction::OpenSpawnForm,
        KeyCode::Char('a') => LoopAction::BuildAttach,
        KeyCode::Char('t') => LoopAction::ToggleChat,
        KeyCode::Char('c') => LoopAction::OpenChatCompose,
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

fn handle_chat_compose_key(app: &mut App, key: KeyEvent) -> LoopAction {
    match key.code {
        KeyCode::Esc => {
            app.cancel_chat_composer();
            app.set_status("Chat compose cancelled.");
            LoopAction::None
        }
        KeyCode::Enter => LoopAction::SendChatMessage,
        KeyCode::Backspace => {
            app.chat_backspace();
            LoopAction::None
        }
        KeyCode::Char(value)
            if !key.modifiers.contains(KeyModifiers::CONTROL)
                && !key.modifiers.contains(KeyModifiers::ALT) =>
        {
            app.chat_insert_char(value);
            LoopAction::None
        }
        _ => LoopAction::None,
    }
}
