mod app;
mod commands;
mod components;
mod input;
mod panels;
mod realtime;
mod views;

use std::env;
use std::future::Future;
use std::io::{self, Stdout};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, anyhow};
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind, MouseButton, MouseEventKind,
};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use tokio::sync::mpsc::error::TryRecvError;

use crate::cli::{Cli, ProviderKind};
use crate::core::{ChatBackend, ChatSnapshot};
use crate::providers::OpenCodeProvider;
use crate::tui::app::{App, FocusPane};
use crate::tui::commands::{
    LocalSlashCommand, build_prompt_with_file_context, parse_local_slash_command,
    parse_remote_slash_command, run_local_grep_summary,
};
use crate::tui::input::{LoopAction, handle_key};
use crate::tui::panels::{
    AgentSelectorHit, ComposerAutocompleteHit, ComposerMetaHit, ModelSelectorHit, SessionsPanel,
};
use crate::tui::realtime::event_requires_refresh;
use crate::tui::views::{MainView, PanelHit};

type TuiTerminal = Terminal<CrosstermBackend<Stdout>>;
const API_TIMEOUT_SECONDS: u64 = 20;

pub async fn run(cli: Cli) -> Result<()> {
    let directory = resolve_directory(cli.directory.as_deref())?;
    let provider: Arc<dyn crate::providers::ChatProvider> = match cli.provider {
        ProviderKind::OpencodeServer => Arc::new(OpenCodeProvider::new(cli.base_url.clone())),
    };
    let backend = ChatBackend::new(provider, directory.clone());

    let bootstrap_snapshot = run_with_api_timeout(
        backend.bootstrap(cli.session.as_deref(), cli.session_title.as_deref()),
    )
    .await?;

    let mut app = App::new(
        cli.base_url.clone(),
        directory,
        backend.provider_name().to_string(),
        cli.refresh_seconds,
    );
    app.apply_snapshot(bootstrap_snapshot);
    match app.restore_selection_from_disk() {
        Ok(true) => {
            app.set_status_message("Connected to OpenCode and restored saved chat selection.");
        }
        Ok(false) => {
            app.set_status_message("Connected to OpenCode and loaded session state.");
        }
        Err(error) => {
            app.set_status_message(format!(
                "Connected to OpenCode; chat selection restore failed: {error}"
            ));
        }
    }
    app.set_realtime_supported(backend.supports_realtime());

    let mut terminal = setup_terminal()?;
    let run_result = run_loop(&mut terminal, &backend, &mut app).await;
    let restore_result = restore_terminal(&mut terminal);

    if let Err(error) = restore_result {
        if run_result.is_ok() {
            return Err(error);
        }
    }

    run_result
}

async fn run_loop(terminal: &mut TuiTerminal, backend: &ChatBackend, app: &mut App) -> Result<()> {
    let refresh_interval = Duration::from_secs(app.refresh_seconds().max(1));
    let mut force_refresh = false;
    let mut next_refresh_at = Instant::now() + refresh_interval;
    let mut next_realtime_retry_at = Instant::now();

    let mut refresh_task: Option<tokio::task::JoinHandle<Result<ChatSnapshot>>> = None;
    let mut send_task: Option<tokio::task::JoinHandle<Result<String>>> = None;
    let mut create_task: Option<tokio::task::JoinHandle<Result<String>>> = None;
    let mut realtime_events = backend.start_realtime_stream();

    loop {
        if backend.supports_realtime()
            && realtime_events.is_none()
            && Instant::now() >= next_realtime_retry_at
        {
            realtime_events = backend.start_realtime_stream();
            next_realtime_retry_at = Instant::now() + Duration::from_secs(5);
        }

        if let Some(receiver) = realtime_events.as_mut() {
            loop {
                match receiver.try_recv() {
                    Ok(event) => {
                        app.record_realtime_event(&event.event_type);

                        if event.event_type == "stream.connected" {
                            app.set_realtime_connected(true);
                            app.set_status_message("Realtime event stream connected.");
                            continue;
                        }

                        if event.event_type == "stream.disconnected" {
                            app.set_realtime_connected(false);
                            app.set_status_message("Realtime event stream disconnected.");
                            continue;
                        }

                        if event.event_type.starts_with("stream.error:") {
                            app.set_realtime_connected(false);
                            app.set_status_message(format!("Realtime error: {}", event.event_type));
                            continue;
                        }

                        if event_requires_refresh(&event, app.active_session_id()) {
                            force_refresh = true;
                        }
                    }
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        app.set_realtime_connected(false);
                        app.set_status_message("Realtime stream dropped. Reconnecting...");
                        realtime_events = None;
                        next_realtime_retry_at = Instant::now() + Duration::from_secs(5);
                        break;
                    }
                }
            }
        }

        if refresh_task.as_ref().is_some_and(|task| task.is_finished()) {
            let Some(task) = refresh_task.take() else {
                unreachable!("refresh task should exist when finished");
            };
            app.set_refresh_in_flight(false);
            match task.await {
                Ok(Ok(snapshot)) => {
                    app.apply_snapshot(snapshot);
                    app.set_status_message("Session state refreshed.");
                }
                Ok(Err(error)) => {
                    app.set_status_message(format!("Refresh failed: {error}"));
                }
                Err(error) => {
                    app.set_status_message(format!("Refresh task failed: {error}"));
                }
            }

            next_refresh_at = Instant::now() + refresh_interval;
        }

        if send_task.as_ref().is_some_and(|task| task.is_finished()) {
            let Some(task) = send_task.take() else {
                unreachable!("send task should exist when finished");
            };
            app.set_send_in_flight(false);

            match task.await {
                Ok(Ok(message)) => {
                    app.clear_draft_after_send();
                    app.set_status_message(message);
                    force_refresh = true;
                }
                Ok(Err(error)) => {
                    app.set_status_message(format!("Prompt send failed: {error}"));
                }
                Err(error) => {
                    app.set_status_message(format!("Prompt task failed: {error}"));
                }
            }
        }

        if create_task.as_ref().is_some_and(|task| task.is_finished()) {
            let Some(task) = create_task.take() else {
                unreachable!("create task should exist when finished");
            };
            app.set_create_in_flight(false);

            match task.await {
                Ok(Ok(session_id)) => {
                    app.set_status_message(format!("Session created: {session_id}"));
                    app.set_active_session_id(&session_id);
                    force_refresh = true;
                }
                Ok(Err(error)) => {
                    app.set_status_message(format!("Session create failed: {error}"));
                }
                Err(error) => {
                    app.set_status_message(format!("Session create task failed: {error}"));
                }
            }
        }

        if refresh_task.is_none() && (force_refresh || Instant::now() >= next_refresh_at) {
            let backend = backend.clone();
            let active_session_id = app.active_session_id().map(ToString::to_string);
            app.set_refresh_in_flight(true);
            refresh_task = Some(tokio::spawn(async move {
                run_with_api_timeout(backend.refresh(active_session_id.as_deref())).await
            }));
            force_refresh = false;
        }

        terminal.draw(|frame| MainView::render(frame, app))?;

        if !event::poll(Duration::from_millis(120))? {
            continue;
        }

        let ev = event::read()?;

        if let Event::Mouse(mouse) = ev {
            let size = terminal.size()?;
            let layout = MainView::layout(ratatui::layout::Rect {
                x: 0,
                y: 0,
                width: size.width,
                height: size.height,
            });

            if app.is_model_selector_open() {
                let hit = crate::tui::panels::ChatPanel::model_selector_hit(
                    layout.chat,
                    layout.chat_composer,
                    app,
                    mouse.column,
                    mouse.row,
                );

                match mouse.kind {
                    MouseEventKind::Down(MouseButton::Left) => match hit {
                        ModelSelectorHit::ListItem(index) => {
                            app.model_selector_set_selected(index);
                            if let Some(model) = app.confirm_model_selector() {
                                app.set_status_message(format!("Model selected: {model}"));
                            }
                        }
                        ModelSelectorHit::Query => {
                            if !app.model_selector_raw_mode() {
                                app.model_selector_toggle_mode();
                                app.set_status_message("Model selector: raw input mode.");
                            }
                        }
                        ModelSelectorHit::Popup => {}
                        ModelSelectorHit::Outside => {
                            app.close_model_selector();
                            app.set_status_message("Model selector closed.");
                        }
                    },
                    MouseEventKind::ScrollUp => {
                        if hit != ModelSelectorHit::Outside {
                            app.model_selector_move_up();
                        }
                    }
                    MouseEventKind::ScrollDown => {
                        if hit != ModelSelectorHit::Outside {
                            app.model_selector_move_down();
                        }
                    }
                    _ => {}
                }

                continue;
            }

            if app.is_agent_selector_open() {
                let hit = crate::tui::panels::ChatPanel::agent_selector_hit(
                    layout.chat,
                    layout.chat_composer,
                    app,
                    mouse.column,
                    mouse.row,
                );

                match mouse.kind {
                    MouseEventKind::Down(MouseButton::Left) => match hit {
                        AgentSelectorHit::ListItem(index) => {
                            app.agent_selector_set_selected(index);
                            if let Some(agent) = app.confirm_agent_selector() {
                                app.set_status_message(format!("Agent selected: {agent}"));
                            }
                        }
                        AgentSelectorHit::Query => {}
                        AgentSelectorHit::Popup => {}
                        AgentSelectorHit::Outside => {
                            app.close_agent_selector();
                            app.set_status_message("Agent selector closed.");
                        }
                    },
                    MouseEventKind::ScrollUp => {
                        if hit != AgentSelectorHit::Outside {
                            app.agent_selector_move_up();
                        }
                    }
                    MouseEventKind::ScrollDown => {
                        if hit != AgentSelectorHit::Outside {
                            app.agent_selector_move_down();
                        }
                    }
                    _ => {}
                }

                continue;
            }

            if app.composer_autocomplete_open() {
                let hit = crate::tui::panels::ChatPanel::composer_autocomplete_hit(
                    layout.chat,
                    layout.chat_composer,
                    app,
                    mouse.column,
                    mouse.row,
                );

                match mouse.kind {
                    MouseEventKind::Down(MouseButton::Left) => match hit {
                        ComposerAutocompleteHit::ListItem(index) => {
                            app.composer_autocomplete_set_selected(index);
                            let _ = app.apply_composer_autocomplete_selection();
                        }
                        ComposerAutocompleteHit::Outside => {
                            app.close_composer_autocomplete();
                        }
                        ComposerAutocompleteHit::Popup => {}
                    },
                    MouseEventKind::ScrollUp => {
                        if hit != ComposerAutocompleteHit::Outside {
                            app.composer_autocomplete_move_up();
                        }
                    }
                    MouseEventKind::ScrollDown => {
                        if hit != ComposerAutocompleteHit::Outside {
                            app.composer_autocomplete_move_down();
                        }
                    }
                    _ => {}
                }

                continue;
            }

            match mouse.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    match MainView::hit_test(layout, mouse.column, mouse.row) {
                        PanelHit::Sessions => {
                            app.set_focus(FocusPane::Sessions);

                            if let Some(index) =
                                SessionsPanel::session_index_at(layout.sessions, app, mouse.row)
                            {
                                let changed = app.set_selected_session_index(index);
                                if changed {
                                    app.clear_messages();
                                    force_refresh = true;
                                }

                                if let Some(session) = app.active_session() {
                                    app.set_status_message(format!(
                                        "Selected session: {}",
                                        session.title
                                    ));
                                }
                            } else {
                                app.set_status_message("Focused sessions panel.");
                            }
                        }
                        PanelHit::Chat => {
                            app.set_focus(FocusPane::Chat);
                            app.set_status_message("Focused conversation panel.");
                        }
                        PanelHit::ChatComposer => {
                            match crate::tui::panels::ChatPanel::composer_meta_hit(
                                layout.chat,
                                layout.chat_composer,
                                app,
                                mouse.column,
                                mouse.row,
                            ) {
                                ComposerMetaHit::Model => {
                                    app.open_model_selector_at(mouse.column);
                                    app.set_status_message(
                                        "Model selector opened. Type to filter, Tab for raw.",
                                    );
                                }
                                ComposerMetaHit::Agent => {
                                    app.open_agent_selector_at(mouse.column);
                                    app.set_status_message(
                                        "Agent selector opened. Type to filter.",
                                    );
                                }
                                ComposerMetaHit::None => {
                                    app.open_composer();
                                    app.set_status_message("Focused composer.");
                                }
                            }
                        }
                        PanelHit::Runtime => {
                            app.set_focus(FocusPane::Runtime);
                            app.set_status_message("Focused runtime panel.");
                        }
                        PanelHit::Other => {}
                    }
                }
                MouseEventKind::ScrollUp => {
                    match MainView::hit_test(layout, mouse.column, mouse.row) {
                        PanelHit::Sessions => {
                            app.set_focus(FocusPane::Sessions);
                            app.scroll_sessions_up(1);
                        }
                        PanelHit::Chat | PanelHit::ChatComposer => {
                            app.set_focus(FocusPane::Chat);
                            app.scroll_chat_up(2);
                        }
                        PanelHit::Runtime => {
                            app.set_focus(FocusPane::Runtime);
                            app.scroll_runtime_up(2);
                        }
                        _ => {}
                    }
                }
                MouseEventKind::ScrollDown => {
                    match MainView::hit_test(layout, mouse.column, mouse.row) {
                        PanelHit::Sessions => {
                            app.set_focus(FocusPane::Sessions);
                            app.scroll_sessions_down(1);
                        }
                        PanelHit::Chat | PanelHit::ChatComposer => {
                            app.set_focus(FocusPane::Chat);
                            app.scroll_chat_down(2);
                        }
                        PanelHit::Runtime => {
                            app.set_focus(FocusPane::Runtime);
                            app.scroll_runtime_down(2);
                        }
                        _ => {}
                    }
                }
                _ => {}
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
            LoopAction::SelectNextSession => {
                app.select_next_session();
                app.clear_messages();
                force_refresh = true;
            }
            LoopAction::SelectPreviousSession => {
                app.select_previous_session();
                app.clear_messages();
                force_refresh = true;
            }
            LoopAction::CreateSession => {
                if create_task.is_some() {
                    app.set_status_message("Session create already in progress.");
                    continue;
                }

                app.set_status_message("Creating session...");
                app.set_create_in_flight(true);
                let backend = backend.clone();
                create_task = Some(tokio::spawn(async move {
                    let created = run_with_api_timeout(backend.create_session(None)).await?;
                    Ok(created.id)
                }));
            }
            LoopAction::SelectNextAgent => {
                app.select_next_agent();
                app.set_status_message(format!(
                    "Agent selected: {}",
                    app.active_agent().unwrap_or("-")
                ));
            }
            LoopAction::OpenModelSelector => {
                app.open_model_selector();
                app.set_status_message("Model selector opened. Type to filter, Tab for raw.");
            }
            LoopAction::OpenCompose => {
                if app.active_session().is_none() {
                    app.set_status_message("Compose unavailable: no active session.");
                } else {
                    app.open_composer();
                    app.set_status_message("Compose mode enabled.");
                }
            }
            LoopAction::ScrollChatUp => {
                app.scroll_chat_up(2);
            }
            LoopAction::ScrollChatDown => {
                app.scroll_chat_down(2);
            }
            LoopAction::ScrollRuntimeUp => {
                app.scroll_runtime_up(2);
            }
            LoopAction::ScrollRuntimeDown => {
                app.scroll_runtime_down(2);
            }
            LoopAction::SendPrompt => {
                let Some(session_id) = app.active_session_id().map(ToString::to_string) else {
                    app.set_status_message("Send skipped: no active session.");
                    continue;
                };
                let Some(prompt) = app.take_prompt() else {
                    app.set_status_message("Send skipped: prompt is empty.");
                    continue;
                };

                if let Some(local_command) = parse_local_slash_command(&prompt) {
                    app.clear_draft_after_send();

                    match local_command {
                        LocalSlashCommand::ToggleHelp => {
                            app.toggle_help();
                            app.set_status_message("Help panel toggled.");
                        }
                        LocalSlashCommand::Refresh => {
                            force_refresh = true;
                            app.set_status_message("Refresh requested.");
                        }
                        LocalSlashCommand::CreateSession => {
                            if create_task.is_some() {
                                app.set_status_message("Session create already in progress.");
                            } else {
                                app.set_status_message("Creating session...");
                                app.set_create_in_flight(true);
                                let backend = backend.clone();
                                create_task = Some(tokio::spawn(async move {
                                    let created =
                                        run_with_api_timeout(backend.create_session(None)).await?;
                                    Ok(created.id)
                                }));
                            }
                        }
                        LocalSlashCommand::ClearMessages => {
                            app.clear_messages();
                            app.set_status_message("Cleared visible messages.");
                        }
                        LocalSlashCommand::Sessions => {
                            app.set_status_message(format!(
                                "Loaded sessions: {}",
                                app.sessions().len()
                            ));
                        }
                        LocalSlashCommand::SetAgent(agent) => {
                            if app.set_active_agent_by_name(&agent) {
                                app.set_status_message(format!("Agent selected: {agent}"));
                            } else {
                                app.set_status_message(format!("Unknown agent: {agent}"));
                            }
                        }
                        LocalSlashCommand::SetModel(model) => {
                            if app.set_active_model_by_name(&model) {
                                app.set_status_message(format!("Model selected: {model}"));
                            } else {
                                app.set_status_message(format!("Unknown model: {model}"));
                            }
                        }
                        LocalSlashCommand::Grep(pattern) => {
                            match run_local_grep_summary(app.directory(), &pattern) {
                                Ok(summary) => app.set_status_message(summary),
                                Err(error) => {
                                    app.set_status_message(format!("Grep failed: {error}"));
                                }
                            }
                        }
                    }
                    continue;
                }

                let remote_command = parse_remote_slash_command(&prompt);

                if send_task.is_some() {
                    app.set_status_message("Send already in progress.");
                    continue;
                }

                let model = app.active_model().map(ToString::to_string);
                let agent = app.active_agent().map(ToString::to_string);
                let workspace_directory = backend.directory().to_string();
                app.set_send_in_flight(true);
                app.set_status_message(format!("Sending prompt to {session_id}..."));
                let backend = backend.clone();
                send_task = Some(tokio::spawn(async move {
                    if let Some(command) = remote_command {
                        run_with_api_timeout(backend.run_command(&session_id, &command))
                            .await
                            .map(|_| format!("Command sent to {session_id}: /{command}"))
                    } else {
                        let (enriched_prompt, referenced_files) =
                            build_prompt_with_file_context(&workspace_directory, &prompt);
                        run_with_api_timeout(backend.send_prompt(
                            &session_id,
                            &enriched_prompt,
                            model.as_deref(),
                            agent.as_deref(),
                        ))
                        .await
                        .map(|_| {
                            if referenced_files == 0 {
                                format!("Prompt sent to {session_id}.")
                            } else {
                                format!(
                                    "Prompt sent to {session_id} with {referenced_files} @file context refs."
                                )
                            }
                        })
                    }
                }));
            }
            LoopAction::ToggleHelp => {
                app.toggle_help();
            }
        }
    }

    Ok(())
}

async fn run_with_api_timeout<T>(future: impl Future<Output = Result<T>>) -> Result<T> {
    match tokio::time::timeout(Duration::from_secs(API_TIMEOUT_SECONDS), future).await {
        Ok(result) => result,
        Err(_) => Err(anyhow!("request timed out after {}s", API_TIMEOUT_SECONDS)),
    }
}

fn setup_terminal() -> Result<TuiTerminal> {
    enable_raw_mode().context("Dark Chat // Terminal // failed to enable raw mode")?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("Dark Chat // Terminal // failed to enter alternate screen")?;

    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend).context("Dark Chat // Terminal // failed to create terminal")
}

fn restore_terminal(terminal: &mut TuiTerminal) -> Result<()> {
    disable_raw_mode().context("Dark Chat // Terminal // failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        DisableMouseCapture,
        LeaveAlternateScreen
    )
    .context("Dark Chat // Terminal // failed to leave alternate screen")?;
    terminal
        .show_cursor()
        .context("Dark Chat // Terminal // failed to show cursor")?;

    Ok(())
}

fn resolve_directory(path: Option<&str>) -> Result<String> {
    let base_path = match path {
        Some(value) => PathBuf::from(value),
        None => env::current_dir()
            .context("Dark Chat // Directory // failed to read current directory")?,
    };

    let absolute = if base_path.is_absolute() {
        base_path
    } else {
        env::current_dir()
            .context("Dark Chat // Directory // failed to read current directory")?
            .join(base_path)
    };

    let canonical = absolute.canonicalize().with_context(|| {
        format!(
            "Dark Chat // Directory // expected existing path (path={})",
            absolute.display()
        )
    })?;

    Ok(canonical.to_string_lossy().to_string())
}
