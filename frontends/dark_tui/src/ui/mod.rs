mod command_palette;
mod render;

use std::env;
use std::future::Future;
use std::io::{self, Stdout};
use std::path::PathBuf;
use std::process::Command;
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
use tracing::{error, info, warn};

use crate::app::{App, ResultsViewMode, VizSelection};
use crate::cli::Cli;
use crate::logging;
use crate::models::{ActorChatMessageRow, DashboardSnapshot};
use crate::service::{CloneVariantOptions, DashboardService, SpawnOptions};
use crate::theme::Theme;

use self::command_palette::{CommandId, ContextMenuState, resolve_key_command};

type TuiTerminal = Terminal<CrosstermBackend<Stdout>>;
type ChatOptionsTask =
    Option<tokio::task::JoinHandle<(String, Result<(Vec<String>, Vec<String>)>)>>;
type ChatSendTask = Option<tokio::task::JoinHandle<(String, Result<()>)>>;
const API_TIMEOUT_SECONDS: u64 = 20;

enum LoopAction {
    None,
    Quit,
    Refresh,
    OpenCloneForm,
    CloneVariant,
    OpenDeleteVariantForm,
    DeleteVariant,
    OpenMoveActorForm,
    MoveActor,
    PollVariant,
    PollActor,
    ImportVariantActors,
    InitProduct,
    OpenSpawnForm,
    SpawnSession,
    BuildAttach,
    RunAttach,
    ToggleInspector,
    ToggleChat,
    OpenChatCompose,
    SendChatMessage,
}

enum BackgroundActionResult {
    CloneVariant(Result<String>),
    DeleteVariant(Result<String>),
    PollVariant(Result<String>),
    PollActor(Result<String>),
    MoveActor(Result<String>),
    ImportVariantActors(Result<String>),
    InitProduct(Result<String>),
    SpawnOptions(Result<(SpawnOptions, String)>),
    SpawnSession(Result<String>),
    BuildAttach(Result<String>),
    RunAttach(Result<String>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BackgroundActionKind {
    CloneVariant,
    DeleteVariant,
    PollVariant,
    PollActor,
    MoveActor,
    ImportVariantActors,
    InitProduct,
    SpawnOptions,
    SpawnSession,
    BuildAttach,
    RunAttach,
}

struct ActionTask {
    kind: BackgroundActionKind,
    handle: tokio::task::JoinHandle<BackgroundActionResult>,
}

#[derive(Debug, Clone)]
struct ActorDragState {
    actor_id: String,
    actor_label: String,
    source_variant_id: String,
    origin_col: u16,
    origin_row: u16,
    current_col: u16,
    current_row: u16,
    hovered_variant_id: Option<String>,
    moved: bool,
}

pub async fn run(cli: Cli) -> Result<()> {
    let directory = resolve_directory(cli.directory.as_deref())?;
    let log_path = logging::init(&directory)?;
    info!(
        base_url = %cli.base_url,
        directory = %directory,
        log_path = %log_path.display(),
        "Dark TUI // Startup // Logger initialized"
    );

    let service =
        DashboardService::new(cli.base_url.clone(), directory.clone(), cli.poll_variants).await;

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
    let transport = if service.uses_realtime_transport() {
        "websocket"
    } else {
        "rest"
    };
    let mut status = format!("Connected to {} via {}", cli.base_url, transport);
    match app.restore_chat_selection_from_disk() {
        Ok(true) => {
            status.push_str(" (restored chat model/agent)");
        }
        Ok(false) => {}
        Err(error) => {
            status.push_str(&format!(" (chat selection restore failed: {error})"));
        }
    }
    app.set_status(status);

    let mut terminal = setup_terminal()?;

    let actor_auto_poll_interval = if cli.actor_auto_poll_seconds == 0 {
        None
    } else {
        Some(Duration::from_secs(cli.actor_auto_poll_seconds.max(1)))
    };

    let run_result = run_loop(&mut terminal, &service, &mut app, actor_auto_poll_interval).await;
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
    actor_auto_poll_interval: Option<Duration>,
) -> Result<()> {
    let refresh_interval = Duration::from_secs(app.refresh_seconds().max(1));
    let mut force_refresh = true;
    let mut next_refresh_at = Instant::now();
    let mut next_actor_auto_poll_at = Instant::now();
    let mut snapshot_task: Option<tokio::task::JoinHandle<Result<DashboardSnapshot>>> = None;
    let mut chat_refresh_task: Option<
        tokio::task::JoinHandle<(String, Result<Vec<ActorChatMessageRow>>)>,
    > = None;
    let mut chat_send_task: ChatSendTask = None;
    let mut chat_options_task: ChatOptionsTask = None;
    let mut action_tasks: Vec<ActionTask> = Vec::new();
    let mut context_menu: Option<ContextMenuState> = None;
    let mut actor_drag_state: Option<ActorDragState> = None;
    let mut key_hint_hover_token: Option<render::KeyHoverToken> = None;
    let mut key_hint_hover: Option<String> = None;

    loop {
        if snapshot_task
            .as_ref()
            .is_some_and(|task| task.is_finished())
        {
            let Some(task) = snapshot_task.take() else {
                unreachable!("snapshot task should exist when marked finished");
            };
            app.set_snapshot_refresh_in_flight(false);
            match task.await {
                Ok(Ok(snapshot)) => {
                    app.apply_snapshot(snapshot);
                    context_menu = None;
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
            let should_auto_poll_actors =
                actor_auto_poll_interval.is_some_and(|_| Instant::now() >= next_actor_auto_poll_at);
            let auto_poll_actor_ids: Vec<String> = if should_auto_poll_actors {
                app.actors().iter().map(|actor| actor.id.clone()).collect()
            } else {
                Vec::new()
            };
            let service = service.clone();
            app.set_snapshot_refresh_in_flight(true);
            snapshot_task = Some(tokio::spawn(async move {
                if !auto_poll_actor_ids.is_empty() {
                    let mut poll_tasks = tokio::task::JoinSet::new();
                    for actor_id in auto_poll_actor_ids {
                        let service = service.clone();
                        poll_tasks.spawn(async move {
                            let _ = run_with_api_timeout(service.poll_actor(&actor_id)).await;
                        });
                    }

                    while poll_tasks.join_next().await.is_some() {}
                }
                run_with_api_timeout(service.fetch_snapshot()).await
            }));

            if should_auto_poll_actors {
                next_actor_auto_poll_at =
                    Instant::now() + actor_auto_poll_interval.expect("auto poll interval exists");
            }
            force_refresh = false;
        }

        if chat_refresh_task
            .as_ref()
            .is_some_and(|task| task.is_finished())
        {
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
                let Some(actor) = app.chat_actor().cloned() else {
                    app.set_status("Chat refresh skipped: actor missing.");
                    continue;
                };
                let service = service.clone();
                app.set_chat_refresh_in_flight(true);
                chat_refresh_task = Some(tokio::spawn(async move {
                    let result =
                        run_with_api_timeout(service.fetch_actor_messages(&actor, Some(80))).await;
                    (actor_id, result)
                }));
            }
        }

        if chat_send_task
            .as_ref()
            .is_some_and(|task| task.is_finished())
        {
            let Some(task) = chat_send_task.take() else {
                unreachable!("chat send task should exist when marked finished");
            };
            app.set_chat_send_in_flight(false);
            match task.await {
                Ok((actor_id, Ok(()))) => {
                    app.commit_sent_chat_prompt();
                    app.request_chat_refresh();
                    app.set_status(format!(
                        "OpenCode response completed for {actor_id}; syncing chat..."
                    ));
                }
                Ok((_actor_id, Err(error))) => {
                    app.set_status(format!("Chat send failed: {error}"));
                }
                Err(error) => {
                    app.set_status(format!("Chat send task failed: {error}"));
                }
            }
        }

        if chat_options_task
            .as_ref()
            .is_some_and(|task| task.is_finished())
        {
            let Some(task) = chat_options_task.take() else {
                unreachable!("chat options task should exist when marked finished");
            };

            match task.await {
                Ok((actor_id, Ok((models, agents)))) => {
                    if app.chat_actor().is_some_and(|actor| actor.id == actor_id) {
                        app.set_chat_options(models, agents);
                    }
                }
                Ok((_actor_id, Err(error))) => {
                    app.set_status(format!("Chat options failed: {error}"));
                }
                Err(error) => {
                    app.set_status(format!("Chat options task failed: {error}"));
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
                Ok(BackgroundActionResult::PollActor(result)) => match result {
                    Ok(message) => {
                        app.set_status(message);
                        force_refresh = true;
                    }
                    Err(error) => {
                        app.set_status(format!("Actor poll failed: {error}"));
                    }
                },
                Ok(BackgroundActionResult::MoveActor(result)) => match result {
                    Ok(message) => {
                        app.set_status(message);
                        force_refresh = true;
                    }
                    Err(error) => {
                        app.set_status(format!("Move actor failed: {error}"));
                    }
                },
                Ok(BackgroundActionResult::CloneVariant(result)) => match result {
                    Ok(message) => {
                        app.set_status(message);
                        force_refresh = true;
                    }
                    Err(error) => {
                        app.set_status(format!("Clone failed: {error}"));
                    }
                },
                Ok(BackgroundActionResult::DeleteVariant(result)) => match result {
                    Ok(message) => {
                        app.set_status(message);
                        force_refresh = true;
                    }
                    Err(error) => {
                        app.set_status(format_delete_variant_error(&error));
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
                    Ok((options, variant_id)) => {
                        app.open_spawn_form(
                            &variant_id,
                            options.providers,
                            options.default_provider.as_deref(),
                        );
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
                        info!(command = %command, "Dark TUI // Attach // Attach command built");
                        let status = match copy_to_clipboard(&command) {
                            Ok(()) => "Attach command copied to clipboard.",
                            Err(error) => {
                                warn!(
                                    error = %error,
                                    "Dark TUI // Attach // Clipboard copy failed"
                                );
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
                Ok(BackgroundActionResult::RunAttach(result)) => match result {
                    Ok(command) => {
                        info!(command = %command, "Dark TUI // Attach // Running attach handoff");
                        app.set_command_message(command.clone());
                        app.set_status("Running attach command...");
                        match run_attach_handoff(terminal, &command) {
                            Ok(exit_status) => {
                                let code = exit_status
                                    .code()
                                    .map(|value| value.to_string())
                                    .unwrap_or_else(|| "signal".to_string());
                                info!(exit = %code, "Dark TUI // Attach // Attach command finished");
                                app.set_status(format!("Attach command finished (exit={code})."));
                            }
                            Err(error) => {
                                error!(error = %error, "Dark TUI // Attach // Attach handoff failed");
                                app.set_status(format!("Attach run failed: {error}"));
                            }
                        }
                        force_refresh = true;
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

        let drag_preview = actor_drag_state.as_ref().map(|state| render::DragPreview {
            col: state.current_col,
            row: state.current_row,
            actor_label: state.actor_label.clone(),
            can_drop: state
                .hovered_variant_id
                .as_ref()
                .is_some_and(|variant_id| variant_id != &state.source_variant_id),
        });

        terminal.draw(|frame| {
            render::render_dashboard(
                frame,
                app,
                context_menu.as_ref(),
                drag_preview.as_ref(),
                key_hint_hover_token.as_ref(),
                key_hint_hover.as_deref(),
            )
        })?;

        if !event::poll(Duration::from_millis(120))? {
            continue;
        }

        let ev = event::read()?;
        let mut injected_key: Option<KeyEvent> = None;

        // --- Mouse events ---
        if let Event::Mouse(mouse) = &ev {
            let size = terminal.size()?;
            let root = Rect {
                x: 0,
                y: 0,
                width: size.width,
                height: size.height,
            };
            key_hint_hover_token = render::key_bar_hover_token(root, app, mouse.row, mouse.column);
            key_hint_hover = render::key_bar_hover_hint(root, app, mouse.row, mouse.column);

            if let Some(target) = app.resizing_target() {
                match mouse.kind {
                    MouseEventKind::Drag(MouseButton::Left) => {
                        if render::resize_divider(root, app, target, mouse.column) {
                            app.set_status("Resizing panels...");
                        }
                    }
                    MouseEventKind::Up(MouseButton::Left)
                    | MouseEventKind::Up(MouseButton::Right) => {
                        app.stop_resize();
                        app.set_status("Panel resize complete.");
                    }
                    _ => {}
                }
                continue;
            }

            if let Some(menu) = context_menu.as_mut() {
                match mouse.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        match render::context_menu_hit_test(root, menu, mouse.column, mouse.row) {
                            render::ContextMenuHit::Item(index) => {
                                menu.set_selected(index);
                                if let Some(command) = menu.selected_command() {
                                    injected_key = command_key_event(command);
                                }
                                context_menu = None;
                            }
                            render::ContextMenuHit::Menu => {}
                            render::ContextMenuHit::Outside => {
                                context_menu = None;
                            }
                        }
                    }
                    MouseEventKind::ScrollUp => {
                        if !matches!(
                            render::context_menu_hit_test(root, menu, mouse.column, mouse.row),
                            render::ContextMenuHit::Outside
                        ) {
                            menu.move_up();
                        }
                    }
                    MouseEventKind::ScrollDown => {
                        if !matches!(
                            render::context_menu_hit_test(root, menu, mouse.column, mouse.row),
                            render::ContextMenuHit::Outside
                        ) {
                            menu.move_down();
                        }
                    }
                    MouseEventKind::Down(MouseButton::Right) => {
                        context_menu = None;
                    }
                    _ => {}
                }

                if injected_key.is_none() {
                    continue;
                }
            }

            if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                if let Some(key_hint_action) =
                    render::key_bar_hit_test(root, app, mouse.row, mouse.column)
                {
                    injected_key = key_event_from_key_hint(key_hint_action);

                    if injected_key.is_none() {
                        continue;
                    }
                }
            }

            if injected_key.is_some() {
                // Key-bar clicks should behave like button presses only.
                // Skip other mouse handlers (chat/viz resize, pan/drag, selection).
                let key = injected_key.expect("injected key should be set");
                let action = handle_key(app, key);
                if matches!(action, LoopAction::Quit) {
                    break;
                }
                process_loop_action(
                    action,
                    app,
                    service,
                    &mut action_tasks,
                    &mut chat_options_task,
                    &mut chat_send_task,
                    &mut force_refresh,
                );
                continue;
            }

            if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                if let Some(target) = render::divider_hit(root, app, mouse.column) {
                    app.start_resize(target);
                    app.set_status("Resize mode: drag divider left/right.");
                    continue;
                }
            }

            match render::chat_hit_test(root, app, mouse.column, mouse.row) {
                render::ChatPanelHit::ModelLabel => {
                    if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                        app.open_chat_model_picker();
                        app.set_status("Model picker opened. Type to filter.");
                    }
                    continue;
                }
                render::ChatPanelHit::AgentLabel => {
                    if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                        app.open_chat_agent_picker();
                        app.set_status("Agent picker opened. Type to filter.");
                    }
                    continue;
                }
                render::ChatPanelHit::ComposerBody => {
                    if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                        if app.open_chat_composer() {
                            app.set_status("Chat compose mode enabled.");
                        } else {
                            app.set_status("Chat compose unavailable: select an actor first.");
                        }
                    }
                    continue;
                }
                render::ChatPanelHit::PickerItem(index) => {
                    if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                        app.chat_picker_set_selected(index);
                        if let Some(value) = app.apply_chat_picker_selection() {
                            app.set_status(format!("Chat option selected: {value}"));
                        }
                    }
                    continue;
                }
                render::ChatPanelHit::PickerPopup => {
                    match mouse.kind {
                        MouseEventKind::ScrollUp => app.chat_picker_move_up(),
                        MouseEventKind::ScrollDown => app.chat_picker_move_down(),
                        MouseEventKind::Down(MouseButton::Left) => {}
                        _ => {}
                    }
                    continue;
                }
                render::ChatPanelHit::AutocompleteItem(index) => {
                    if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                        app.chat_autocomplete_set_selected(index);
                        let _ = app.apply_chat_autocomplete_selection();
                    }
                    continue;
                }
                render::ChatPanelHit::AutocompletePopup => {
                    match mouse.kind {
                        MouseEventKind::ScrollUp => app.chat_autocomplete_move_up(),
                        MouseEventKind::ScrollDown => app.chat_autocomplete_move_down(),
                        MouseEventKind::Down(MouseButton::Left) => {}
                        _ => {}
                    }
                    continue;
                }
                render::ChatPanelHit::Outside => {
                    if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                        app.close_chat_picker();
                    }
                }
            }

            if app.results_view_mode().is_spatial() {
                match mouse.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        if let Some(selection) =
                            render::viz_hit_test(root, app, mouse.column, mouse.row)
                        {
                            app.set_viz_selection(selection.clone());
                            actor_drag_state = match selection {
                                VizSelection::Actor {
                                    actor_id,
                                    variant_id,
                                    ..
                                } => Some(ActorDragState {
                                    actor_label: app
                                        .actors()
                                        .iter()
                                        .find(|actor| actor.id == actor_id)
                                        .map(|actor| actor.title.clone())
                                        .unwrap_or_else(|| actor_id.clone()),
                                    actor_id,
                                    source_variant_id: variant_id,
                                    origin_col: mouse.column,
                                    origin_row: mouse.row,
                                    current_col: mouse.column,
                                    current_row: mouse.row,
                                    hovered_variant_id: None,
                                    moved: false,
                                }),
                                _ => None,
                            };
                        } else {
                            actor_drag_state = None;
                            app.start_drag(mouse.column, mouse.row);
                        }
                    }
                    MouseEventKind::Down(MouseButton::Right) => {
                        actor_drag_state = None;
                        app.end_drag();
                        if render::try_select_viz_node(root, app, mouse.column, mouse.row) {
                            if let Some(target) = app.viz_selection().cloned() {
                                context_menu =
                                    ContextMenuState::open(app, target, mouse.column, mouse.row);
                            }
                        } else {
                            context_menu = None;
                        }
                    }
                    MouseEventKind::Drag(_) => {
                        if let Some(state) = actor_drag_state.as_mut() {
                            state.current_col = mouse.column;
                            state.current_row = mouse.row;
                            if mouse.column != state.origin_col || mouse.row != state.origin_row {
                                state.moved = true;
                            }

                            state.hovered_variant_id =
                                resolve_drop_target_variant(root, app, mouse.column, mouse.row);
                        } else {
                            app.apply_drag(mouse.column, mouse.row);
                        }
                    }
                    MouseEventKind::Up(_) => {
                        app.end_drag();
                        if let Some(state) = actor_drag_state.take() {
                            if state.moved {
                                let target_variant =
                                    resolve_drop_target_variant(root, app, mouse.column, mouse.row)
                                        .or(state.hovered_variant_id);

                                if let Some(target_variant_id) = target_variant {
                                    if target_variant_id == state.source_variant_id {
                                        app.set_status("Actor already on that variant.");
                                    } else if has_action_in_flight(
                                        &action_tasks,
                                        BackgroundActionKind::MoveActor,
                                    ) {
                                        app.set_status("Actor move already in progress.");
                                    } else {
                                        let target_variant_name = app
                                            .variants()
                                            .iter()
                                            .find(|variant| variant.id == target_variant_id)
                                            .map(|variant| variant.name.clone())
                                            .unwrap_or_else(|| "target".to_string());

                                        app.set_status(format!(
                                            "Moving actor {} to {}...",
                                            state.actor_id, target_variant_id
                                        ));
                                        let service = service.clone();
                                        action_tasks.push(ActionTask {
                                            kind: BackgroundActionKind::MoveActor,
                                            handle: tokio::spawn(async move {
                                                BackgroundActionResult::MoveActor(
                                                    run_with_api_timeout(service.move_actor(
                                                        &state.actor_id,
                                                        &state.source_variant_id,
                                                        &target_variant_id,
                                                        &target_variant_name,
                                                    ))
                                                    .await,
                                                )
                                            }),
                                        });
                                        app.set_action_requests_in_flight(action_tasks.len());
                                    }
                                }
                            }
                        }
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

            if app.results_view_mode() == ResultsViewMode::Table {
                match mouse.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        if let Some(selection) =
                            render::tree_hit_test(root, app, mouse.column, mouse.row)
                        {
                            app.set_viz_selection(selection);
                        }
                    }
                    MouseEventKind::Down(MouseButton::Right) => {
                        if let Some(selection) =
                            render::tree_hit_test(root, app, mouse.column, mouse.row)
                        {
                            app.set_viz_selection(selection.clone());
                            context_menu =
                                ContextMenuState::open(app, selection, mouse.column, mouse.row);
                        } else {
                            context_menu = None;
                        }
                    }
                    _ => {}
                }
            }

            if injected_key.is_none() {
                continue;
            }
        }

        let key = match injected_key {
            Some(key) => key,
            None => {
                let Event::Key(key) = ev else {
                    continue;
                };
                key
            }
        };

        if key.kind != KeyEventKind::Press {
            continue;
        }

        if let Some(menu) = context_menu.as_mut() {
            match handle_context_menu_key(menu, key) {
                ContextMenuKeyOutcome::Consumed => {
                    continue;
                }
                ContextMenuKeyOutcome::Close => {
                    context_menu = None;
                    continue;
                }
                ContextMenuKeyOutcome::Dispatch(command) => {
                    context_menu = None;
                    if let Some(injected) = command_key_event(command) {
                        let action = handle_key(app, injected);
                        if matches!(
                            action,
                            LoopAction::OpenCloneForm
                                | LoopAction::OpenDeleteVariantForm
                                | LoopAction::OpenMoveActorForm
                                | LoopAction::OpenSpawnForm
                        ) {
                            context_menu = None;
                        }
                        process_loop_action(
                            action,
                            app,
                            service,
                            &mut action_tasks,
                            &mut chat_options_task,
                            &mut chat_send_task,
                            &mut force_refresh,
                        );
                    }
                    continue;
                }
            }
        }

        let action = handle_key(app, key);
        if matches!(
            action,
            LoopAction::OpenCloneForm
                | LoopAction::OpenDeleteVariantForm
                | LoopAction::OpenMoveActorForm
                | LoopAction::OpenSpawnForm
        ) {
            context_menu = None;
        }
        if matches!(action, LoopAction::Quit) {
            break;
        }

        process_loop_action(
            action,
            app,
            service,
            &mut action_tasks,
            &mut chat_options_task,
            &mut chat_send_task,
            &mut force_refresh,
        );
    }

    Ok(())
}

async fn run_with_api_timeout<T>(future: impl Future<Output = Result<T>>) -> Result<T> {
    match tokio::time::timeout(Duration::from_secs(API_TIMEOUT_SECONDS), future).await {
        Ok(result) => result,
        Err(_) => Err(anyhow!("request timed out after {}s", API_TIMEOUT_SECONDS)),
    }
}

fn resolve_drop_target_variant(root: Rect, app: &App, col: u16, row: u16) -> Option<String> {
    match render::viz_hit_test(root, app, col, row) {
        Some(VizSelection::Variant { variant_id, .. }) => Some(variant_id),
        Some(VizSelection::Actor { variant_id, .. }) => Some(variant_id),
        _ => None,
    }
}

fn format_delete_variant_error(error: &anyhow::Error) -> String {
    let message = error.to_string();
    if message.contains("VARIANTS_DELETE_UNDO_BLOCKED") || message.contains("Undo blocked") {
        return "Delete blocked by safe-undo checks. Retry in keep clone directory mode (Space in delete confirmation)."
            .to_string();
    }

    format!("Delete failed: {message}")
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

fn run_attach_handoff(terminal: &mut TuiTerminal, command: &str) -> Result<std::process::ExitStatus> {
    suspend_terminal_for_handoff(terminal)?;

    let run_result = run_attach_command(command);

    let resume_result = resume_terminal_after_handoff(terminal);

    match (run_result, resume_result) {
        (Ok(status), Ok(())) => Ok(status),
        (Err(run_error), Ok(())) => Err(run_error),
        (Ok(_), Err(resume_error)) => Err(resume_error),
        (Err(run_error), Err(resume_error)) => {
            Err(anyhow!("{run_error}; additionally failed to restore terminal: {resume_error}"))
        }
    }
}

fn run_attach_command(command: &str) -> Result<std::process::ExitStatus> {
    info!(command = %command, "Dark TUI // Attach // Executing shell command");
    Command::new("/bin/sh")
        .arg("-lc")
        .arg(command)
        .status()
        .with_context(|| format!("failed to run attach command: {command}"))
}

fn suspend_terminal_for_handoff(terminal: &mut TuiTerminal) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode for attach handoff")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)
        .context("failed to leave alternate screen for attach handoff")?;
    terminal
        .show_cursor()
        .context("failed to show cursor for attach handoff")?;
    Ok(())
}

fn resume_terminal_after_handoff(terminal: &mut TuiTerminal) -> Result<()> {
    enable_raw_mode().context("failed to re-enable raw mode after attach handoff")?;
    execute!(terminal.backend_mut(), EnterAlternateScreen, EnableMouseCapture)
        .context("failed to re-enter alternate screen after attach handoff")?;
    terminal
        .autoresize()
        .context("failed to autoresize terminal after attach handoff")?;
    terminal
        .clear()
        .context("failed to clear terminal after attach handoff")?;
    terminal
        .hide_cursor()
        .context("failed to hide cursor after attach handoff")?;
    Ok(())
}

enum ContextMenuKeyOutcome {
    Consumed,
    Close,
    Dispatch(CommandId),
}

fn handle_context_menu_key(menu: &mut ContextMenuState, key: KeyEvent) -> ContextMenuKeyOutcome {
    match key.code {
        KeyCode::Esc => ContextMenuKeyOutcome::Close,
        KeyCode::Up | KeyCode::Char('k') => {
            menu.move_up();
            ContextMenuKeyOutcome::Consumed
        }
        KeyCode::Down | KeyCode::Char('j') => {
            menu.move_down();
            ContextMenuKeyOutcome::Consumed
        }
        KeyCode::Enter => menu
            .selected_command()
            .map(ContextMenuKeyOutcome::Dispatch)
            .unwrap_or(ContextMenuKeyOutcome::Close),
        KeyCode::Char(_) => menu
            .shortcut_command(key)
            .map(ContextMenuKeyOutcome::Dispatch)
            .unwrap_or(ContextMenuKeyOutcome::Consumed),
        _ => ContextMenuKeyOutcome::Close,
    }
}

fn key_event_from_key_hint(action: render::KeyHintAction) -> Option<KeyEvent> {
    let key = match action {
        render::KeyHintAction::Quit => KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
        render::KeyHintAction::Focus => KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE),
        render::KeyHintAction::Select => KeyEvent::new(KeyCode::Down, KeyModifiers::NONE),
        render::KeyHintAction::Refresh => KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE),
        render::KeyHintAction::View => KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE),
        render::KeyHintAction::Filter => KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE),
        render::KeyHintAction::ToggleInspector => {
            KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE)
        }
        render::KeyHintAction::Poll => KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE),
        render::KeyHintAction::PollActor => KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE),
        render::KeyHintAction::Move => KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE),
        render::KeyHintAction::Clone => KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE),
        render::KeyHintAction::Delete => KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE),
        render::KeyHintAction::Import => KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE),
        render::KeyHintAction::Init => KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
        render::KeyHintAction::Spawn => KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE),
        render::KeyHintAction::Attach => KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
        render::KeyHintAction::Chat => KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE),
        render::KeyHintAction::Compose => KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE),
        render::KeyHintAction::ResetPan => KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE),
        render::KeyHintAction::Send => KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
        render::KeyHintAction::Cancel => KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
        render::KeyHintAction::ToggleRemove => {
            KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE)
        }
        render::KeyHintAction::FieldNav => KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE),
    };

    Some(key)
}

fn command_key_event(command: CommandId) -> Option<KeyEvent> {
    let key = match command {
        CommandId::Quit => KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
        CommandId::ToggleFocus => KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE),
        CommandId::MoveDown => KeyEvent::new(KeyCode::Down, KeyModifiers::NONE),
        CommandId::MoveUp => KeyEvent::new(KeyCode::Up, KeyModifiers::NONE),
        CommandId::Refresh => KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE),
        CommandId::ToggleFilter => KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE),
        CommandId::ToggleInspector => KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE),
        CommandId::ToggleView => KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE),
        CommandId::PollVariant => KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE),
        CommandId::PollActor => KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE),
        CommandId::OpenMoveActorForm => KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE),
        CommandId::OpenCloneForm => KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE),
        CommandId::OpenDeleteVariantForm => KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE),
        CommandId::ImportVariantActors => KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE),
        CommandId::InitProduct => KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
        CommandId::OpenSpawnForm => KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE),
        CommandId::BuildAttach => KeyEvent::new(KeyCode::Char('A'), KeyModifiers::SHIFT),
        CommandId::RunAttach => KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
        CommandId::ToggleChat => KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE),
        CommandId::OpenChatCompose => KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE),
        CommandId::ResetPan => KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE),
    };

    Some(key)
}

fn apply_command(app: &mut App, command: CommandId) -> LoopAction {
    match command {
        CommandId::Quit => LoopAction::Quit,
        CommandId::ToggleFocus => {
            app.focus_next();
            LoopAction::None
        }
        CommandId::MoveDown => {
            app.move_selection_down();
            LoopAction::None
        }
        CommandId::MoveUp => {
            app.move_selection_up();
            LoopAction::None
        }
        CommandId::Refresh => LoopAction::Refresh,
        CommandId::ToggleFilter => {
            app.toggle_variant_filter();
            LoopAction::None
        }
        CommandId::ToggleInspector => LoopAction::ToggleInspector,
        CommandId::ToggleView => {
            app.toggle_results_view_mode();
            app.set_status(format!(
                "View mode: {}",
                app.results_view_mode().display_label()
            ));
            LoopAction::None
        }
        CommandId::PollVariant => LoopAction::PollVariant,
        CommandId::PollActor => LoopAction::PollActor,
        CommandId::OpenMoveActorForm => LoopAction::OpenMoveActorForm,
        CommandId::OpenCloneForm => LoopAction::OpenCloneForm,
        CommandId::OpenDeleteVariantForm => LoopAction::OpenDeleteVariantForm,
        CommandId::ImportVariantActors => LoopAction::ImportVariantActors,
        CommandId::InitProduct => LoopAction::InitProduct,
        CommandId::OpenSpawnForm => LoopAction::OpenSpawnForm,
        CommandId::BuildAttach => LoopAction::BuildAttach,
        CommandId::RunAttach => LoopAction::RunAttach,
        CommandId::ToggleChat => LoopAction::ToggleChat,
        CommandId::OpenChatCompose => LoopAction::OpenChatCompose,
        CommandId::ResetPan => {
            app.reset_viz_offset();
            app.set_status("Reset pan to origin.");
            LoopAction::None
        }
    }
}

fn process_loop_action(
    action: LoopAction,
    app: &mut App,
    service: &DashboardService,
    action_tasks: &mut Vec<ActionTask>,
    chat_options_task: &mut ChatOptionsTask,
    chat_send_task: &mut ChatSendTask,
    force_refresh: &mut bool,
) {
    match action {
        LoopAction::None | LoopAction::Quit => {}
        LoopAction::Refresh => {
            *force_refresh = true;
        }
        LoopAction::PollVariant => {
            let Some(variant_id) = app.selected_variant_id().map(ToString::to_string) else {
                app.set_status("Poll skipped: no variant selected.");
                return;
            };

            if has_action_in_flight(action_tasks, BackgroundActionKind::PollVariant) {
                app.set_status("Variant poll already in progress.");
                return;
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
        LoopAction::PollActor => {
            let Some(actor_id) = app.selected_actor_id().map(ToString::to_string) else {
                app.set_status("Actor poll skipped: no actor selected.");
                return;
            };

            if has_action_in_flight(action_tasks, BackgroundActionKind::PollActor) {
                app.set_status("Actor poll already in progress.");
                return;
            }

            app.set_status(format!("Polling actor {actor_id}..."));
            let service = service.clone();
            action_tasks.push(ActionTask {
                kind: BackgroundActionKind::PollActor,
                handle: tokio::spawn(async move {
                    BackgroundActionResult::PollActor(
                        run_with_api_timeout(service.poll_actor(&actor_id)).await,
                    )
                }),
            });
            app.set_action_requests_in_flight(action_tasks.len());
        }
        LoopAction::OpenMoveActorForm => {
            if app.open_move_actor_form() {
                app.set_status("Move actor dialog open. Choose destination variant.");
            } else {
                app.set_status("Move actor unavailable: select an actor with alternate variants.");
            }
        }
        LoopAction::MoveActor => {
            let Some(request) = app.take_move_actor_request() else {
                app.set_status("Move actor skipped: dialog is not open.");
                return;
            };

            if has_action_in_flight(action_tasks, BackgroundActionKind::MoveActor) {
                app.set_status("Actor move already in progress.");
                return;
            }

            app.set_status(format!(
                "Moving actor {} to {}...",
                request.actor_id, request.target_variant_id
            ));
            let service = service.clone();
            action_tasks.push(ActionTask {
                kind: BackgroundActionKind::MoveActor,
                handle: tokio::spawn(async move {
                    BackgroundActionResult::MoveActor(
                        run_with_api_timeout(service.move_actor(
                            &request.actor_id,
                            &request.source_variant_id,
                            &request.target_variant_id,
                            &request.target_variant_name,
                        ))
                        .await,
                    )
                }),
            });
            app.set_action_requests_in_flight(action_tasks.len());
        }
        LoopAction::CloneVariant => {
            let Some(product_id) = app.selected_product().map(|product| product.id.to_string())
            else {
                app.set_status("Clone skipped: no product selected.");
                return;
            };

            let Some(request) = app.take_clone_request() else {
                app.set_status("Clone skipped: clone form is not open.");
                return;
            };

            if has_action_in_flight(action_tasks, BackgroundActionKind::CloneVariant) {
                app.set_status("Variant clone already in progress.");
                return;
            }

            app.set_status(format!("Cloning variant for product {product_id}..."));
            let service = service.clone();
            action_tasks.push(ActionTask {
                kind: BackgroundActionKind::CloneVariant,
                handle: tokio::spawn(async move {
                    BackgroundActionResult::CloneVariant(
                        run_with_api_timeout(service.clone_product_variant(
                            &product_id,
                            &CloneVariantOptions {
                                name: request.name,
                                target_path: request.target_path,
                                branch_name: request.branch_name,
                                clone_type: request.clone_type,
                                source_variant_id: request.source_variant_id,
                            },
                        ))
                        .await,
                    )
                }),
            });
            app.set_action_requests_in_flight(action_tasks.len());
        }
        LoopAction::OpenDeleteVariantForm => {
            let Some(variant_id) = app.selected_variant_id().map(ToString::to_string) else {
                app.set_status("Delete unavailable: select a variant first.");
                return;
            };

            app.open_delete_variant_form(&variant_id);
            app.set_status("Delete confirmation open. Toggle clone removal with Space.");
        }
        LoopAction::DeleteVariant => {
            let Some(request) = app.take_delete_variant_request() else {
                app.set_status("Delete skipped: confirmation not open.");
                return;
            };

            if has_action_in_flight(action_tasks, BackgroundActionKind::DeleteVariant) {
                app.set_status("Variant delete already in progress.");
                return;
            }

            let dry = request.dry;
            let variant_id = request.variant_id;
            app.set_status(format!("Deleting variant {variant_id} (dry={dry})..."));
            let service = service.clone();
            action_tasks.push(ActionTask {
                kind: BackgroundActionKind::DeleteVariant,
                handle: tokio::spawn(async move {
                    BackgroundActionResult::DeleteVariant(
                        run_with_api_timeout(service.delete_variant(&variant_id, dry)).await,
                    )
                }),
            });
            app.set_action_requests_in_flight(action_tasks.len());
        }
        LoopAction::OpenCloneForm => {
            if app.selected_product().is_none() {
                app.set_status("Clone form unavailable: select a product first.");
                return;
            }

            app.open_clone_form();
            app.set_status("Clone form open. Enter options or leave blank for defaults.");
        }
        LoopAction::ImportVariantActors => {
            let Some(variant_id) = app.selected_variant_id().map(ToString::to_string) else {
                app.set_status("Import skipped: no variant selected.");
                return;
            };

            if has_action_in_flight(action_tasks, BackgroundActionKind::ImportVariantActors) {
                app.set_status("Actor import already in progress.");
                return;
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
            if has_action_in_flight(action_tasks, BackgroundActionKind::InitProduct) {
                app.set_status("Product init already in progress.");
                return;
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
            let Some(variant_id) = app.selected_variant_id().map(ToString::to_string) else {
                app.set_status("Spawn unavailable: select a variant first.");
                return;
            };

            if has_action_in_flight(action_tasks, BackgroundActionKind::SpawnOptions) {
                app.set_status("Spawn options request already in progress.");
                return;
            }

            app.set_status("Loading spawn provider options...");
            let service = service.clone();
            action_tasks.push(ActionTask {
                kind: BackgroundActionKind::SpawnOptions,
                handle: tokio::spawn(async move {
                    BackgroundActionResult::SpawnOptions(
                        run_with_api_timeout(service.fetch_spawn_options())
                            .await
                            .map(|options| (options, variant_id)),
                    )
                }),
            });
            app.set_action_requests_in_flight(action_tasks.len());
        }
        LoopAction::SpawnSession => {
            if has_action_in_flight(action_tasks, BackgroundActionKind::SpawnSession) {
                app.set_status("Spawn session already in progress.");
                return;
            }

            let Some(request) = app.take_spawn_request() else {
                app.set_status("Spawn skipped: form is not open.");
                return;
            };

            app.set_status("Spawning actor session...");
            let service = service.clone();
            action_tasks.push(ActionTask {
                kind: BackgroundActionKind::SpawnSession,
                handle: tokio::spawn(async move {
                    BackgroundActionResult::SpawnSession(
                        run_with_api_timeout(service.create_session(
                            &request.variant_id,
                            &request.provider,
                            request.initial_prompt.as_deref(),
                        ))
                        .await,
                    )
                }),
            });
            app.set_action_requests_in_flight(action_tasks.len());
        }
        LoopAction::BuildAttach => {
            let Some(actor_id) = app.selected_actor_id().map(ToString::to_string) else {
                app.set_status("Attach skipped: no actor selected.");
                return;
            };

            if has_action_in_flight(action_tasks, BackgroundActionKind::BuildAttach) {
                app.set_status("Attach build already in progress.");
                return;
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
        LoopAction::RunAttach => {
            let Some(actor_id) = app.selected_actor_id().map(ToString::to_string) else {
                app.set_status("Attach run skipped: no actor selected.");
                return;
            };

            if has_action_in_flight(action_tasks, BackgroundActionKind::RunAttach) {
                app.set_status("Attach run already in progress.");
                return;
            }

            app.set_status(format!("Preparing attach handoff for {actor_id}..."));
            let service = service.clone();
            action_tasks.push(ActionTask {
                kind: BackgroundActionKind::RunAttach,
                handle: tokio::spawn(async move {
                    BackgroundActionResult::RunAttach(
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
                if chat_options_task.is_none() {
                    if let Some(actor) = app.chat_actor().cloned() {
                        let service = service.clone();
                        *chat_options_task = Some(tokio::spawn(async move {
                            let result =
                                run_with_api_timeout(service.fetch_actor_chat_options(&actor))
                                    .await;
                            (actor.id.clone(), result)
                        }));
                    }
                }
                "Chat shown."
            } else {
                "Chat hidden."
            };
            app.set_status(status);
        }
        LoopAction::ToggleInspector => {
            app.toggle_inspector_visibility();
            let status = if app.is_inspector_visible() {
                "Sidebar shown."
            } else {
                "Sidebar hidden."
            };
            app.set_status(status);
        }
        LoopAction::OpenChatCompose => {
            if app.open_chat_composer() {
                if chat_options_task.is_none() {
                    if let Some(actor) = app.chat_actor().cloned() {
                        let service = service.clone();
                        *chat_options_task = Some(tokio::spawn(async move {
                            let result =
                                run_with_api_timeout(service.fetch_actor_chat_options(&actor))
                                    .await;
                            (actor.id.clone(), result)
                        }));
                    }
                }
                app.set_status("Compose chat enabled.");
            } else {
                app.set_status("Compose chat skipped: select an actor first.");
            }
        }
        LoopAction::SendChatMessage => {
            let Some(actor) = app.chat_actor().cloned() else {
                app.set_status("Chat send skipped: no actor selected.");
                return;
            };
            let actor_id = actor.id.clone();
            let Some(prompt) = app.current_chat_prompt() else {
                app.set_status("Chat send skipped: prompt is empty.");
                return;
            };

            if chat_send_task.is_some() {
                app.set_status("Chat send already in progress.");
                return;
            }

            app.set_status("Queueing prompt to OpenCode session...");
            let service = service.clone();
            let selected_model = app.chat_active_model().map(ToString::to_string);
            let selected_agent = app.chat_active_agent().map(ToString::to_string);
            app.set_chat_send_in_flight(true);
            *chat_send_task = Some(tokio::spawn(async move {
                let result = run_with_api_timeout(service.send_actor_prompt(
                    &actor,
                    &prompt,
                    selected_model.as_deref(),
                    selected_agent.as_deref(),
                ))
                .await;
                (actor_id, result)
            }));
        }
    }
}

fn handle_key(app: &mut App, key: KeyEvent) -> LoopAction {
    if app.is_delete_variant_form_open() {
        return handle_delete_variant_form_key(app, key);
    }

    if app.is_clone_form_open() {
        return handle_clone_form_key(app, key);
    }

    if app.is_spawn_form_open() {
        return handle_spawn_form_key(app, key);
    }

    if app.is_move_actor_form_open() {
        return handle_move_actor_form_key(app, key);
    }

    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return LoopAction::Quit;
    }

    if key.code == KeyCode::BackTab {
        app.focus_previous();
        return LoopAction::None;
    }

    if app.chat_picker_open().is_some() {
        return handle_chat_picker_key(app, key);
    }

    if app.is_chat_composing() {
        return handle_chat_compose_key(app, key);
    }

    resolve_key_command(app, key)
        .map(|command| apply_command(app, command))
        .unwrap_or(LoopAction::None)
}

fn handle_delete_variant_form_key(app: &mut App, key: KeyEvent) -> LoopAction {
    match key.code {
        KeyCode::Esc => {
            app.close_delete_variant_form();
            app.set_status("Delete confirmation closed.");
            LoopAction::None
        }
        KeyCode::Enter => LoopAction::DeleteVariant,
        KeyCode::Char(' ') => {
            app.toggle_delete_variant_remove_clone_directory();
            let remove = app.delete_variant_form_remove_clone_directory();
            app.set_status(format!(
                "Delete mode: {}",
                if remove {
                    "remove clone directory"
                } else {
                    "keep clone directory"
                }
            ));
            LoopAction::None
        }
        _ => LoopAction::None,
    }
}

fn handle_clone_form_key(app: &mut App, key: KeyEvent) -> LoopAction {
    match key.code {
        KeyCode::Esc => {
            app.close_clone_form();
            app.set_status("Clone form closed.");
            LoopAction::None
        }
        KeyCode::Enter => LoopAction::CloneVariant,
        KeyCode::Up | KeyCode::Char('k') | KeyCode::BackTab => {
            app.clone_form_move_up();
            LoopAction::None
        }
        KeyCode::Down | KeyCode::Char('j') | KeyCode::Tab => {
            app.clone_form_move_down();
            LoopAction::None
        }
        KeyCode::Backspace => {
            app.clone_form_backspace();
            LoopAction::None
        }
        KeyCode::Char(value)
            if !key.modifiers.contains(KeyModifiers::CONTROL)
                && !key.modifiers.contains(KeyModifiers::ALT) =>
        {
            app.clone_form_insert_char(value);
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

fn handle_move_actor_form_key(app: &mut App, key: KeyEvent) -> LoopAction {
    match key.code {
        KeyCode::Esc => {
            app.close_move_actor_form();
            app.set_status("Move actor dialog closed.");
            LoopAction::None
        }
        KeyCode::Enter => LoopAction::MoveActor,
        KeyCode::Up | KeyCode::Char('k') | KeyCode::BackTab => {
            app.move_actor_form_move_up();
            LoopAction::None
        }
        KeyCode::Down | KeyCode::Char('j') | KeyCode::Tab => {
            app.move_actor_form_move_down();
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

fn handle_chat_picker_key(app: &mut App, key: KeyEvent) -> LoopAction {
    match key.code {
        KeyCode::Esc => {
            app.close_chat_picker();
            app.set_status("Chat picker closed.");
            LoopAction::None
        }
        KeyCode::Enter => {
            if let Some(value) = app.apply_chat_picker_selection() {
                app.set_status(format!("Chat option selected: {value}"));
            } else {
                app.set_status("No matching option selected.");
            }
            LoopAction::None
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.chat_picker_move_up();
            LoopAction::None
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.chat_picker_move_down();
            LoopAction::None
        }
        KeyCode::Backspace => {
            app.chat_picker_backspace();
            LoopAction::None
        }
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.clear_chat_picker_query();
            LoopAction::None
        }
        KeyCode::Char(value)
            if !key.modifiers.contains(KeyModifiers::CONTROL)
                && !key.modifiers.contains(KeyModifiers::ALT) =>
        {
            app.chat_picker_insert_char(value);
            LoopAction::None
        }
        _ => LoopAction::None,
    }
}

#[cfg(test)]
mod tests {
    use super::run_attach_command;

    #[test]
    fn run_attach_command_reports_success() {
        let status = run_attach_command("exit 0").expect("command should run");
        assert!(status.success());
    }

    #[test]
    fn run_attach_command_reports_nonzero_exit() {
        let status = run_attach_command("exit 7").expect("command should run");
        assert_eq!(status.code(), Some(7));
    }
}
