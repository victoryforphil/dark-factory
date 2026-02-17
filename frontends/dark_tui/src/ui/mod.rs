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
use crate::service::{CloneVariantOptions, DashboardService, SpawnOptions};
use crate::theme::Theme;

type TuiTerminal = Terminal<CrosstermBackend<Stdout>>;
const API_TIMEOUT_SECONDS: u64 = 20;

enum LoopAction {
    None,
    Quit,
    Refresh,
    OpenCloneForm,
    CloneVariant,
    OpenDeleteVariantForm,
    DeleteVariant,
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
    CloneVariant(Result<String>),
    DeleteVariant(Result<String>),
    PollVariant(Result<String>),
    ImportVariantActors(Result<String>),
    InitProduct(Result<String>),
    SpawnOptions(Result<SpawnOptions>),
    SpawnSession(Result<String>),
    BuildAttach(Result<String>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BackgroundActionKind {
    CloneVariant,
    DeleteVariant,
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
    let refresh_interval = Duration::from_secs(app.refresh_seconds().max(1));
    let mut force_refresh = true;
    let mut next_refresh_at = Instant::now();
    let mut snapshot_task: Option<tokio::task::JoinHandle<Result<DashboardSnapshot>>> = None;
    let mut chat_refresh_task: Option<
        tokio::task::JoinHandle<(String, Result<Vec<ActorChatMessageRow>>)>,
    > = None;
    let mut chat_send_task: Option<tokio::task::JoinHandle<(String, Result<()>)>> = None;
    let mut chat_options_task: Option<
        tokio::task::JoinHandle<(String, Result<(Vec<String>, Vec<String>)>)>,
    > = None;
    let mut action_tasks: Vec<ActionTask> = Vec::new();

    loop {
        let route_mutation_events = service.consume_route_mutation_events().await;
        if route_mutation_events > 0 {
            force_refresh = true;
            if snapshot_task.is_none() {
                app.set_status(format!(
                    "Realtime update received ({route_mutation_events})"
                ));
            }
        }

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
                        app.set_status(format!("Delete failed: {error}"));
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

        // --- Mouse events ---
        if let Event::Mouse(mouse) = &ev {
            let size = terminal.size()?;
            let root = Rect {
                x: 0,
                y: 0,
                width: size.width,
                height: size.height,
            };

            if let Some(target) = app.resizing_target() {
                match mouse.kind {
                    MouseEventKind::Drag(MouseButton::Left) => {
                        if render::resize_divider(root, app, target, mouse.column) {
                            app.set_status("Resizing panels...");
                        }
                    }
                    MouseEventKind::Up(MouseButton::Left) | MouseEventKind::Up(MouseButton::Right) => {
                        app.stop_resize();
                        app.set_status("Panel resize complete.");
                    }
                    _ => {}
                }
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

            if app.results_view_mode() == ResultsViewMode::Viz {
                match mouse.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
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
            LoopAction::CloneVariant => {
                let Some(product_id) = app.selected_product().map(|product| product.id.to_string())
                else {
                    app.set_status("Clone skipped: no product selected.");
                    continue;
                };

                let Some(request) = app.take_clone_request() else {
                    app.set_status("Clone skipped: clone form is not open.");
                    continue;
                };

                if has_action_in_flight(&action_tasks, BackgroundActionKind::CloneVariant) {
                    app.set_status("Variant clone already in progress.");
                    continue;
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
                    continue;
                };

                app.open_delete_variant_form(&variant_id);
                app.set_status("Delete confirmation open. Toggle clone removal with Space.");
            }
            LoopAction::DeleteVariant => {
                let Some(request) = app.take_delete_variant_request() else {
                    app.set_status("Delete skipped: confirmation not open.");
                    continue;
                };

                if has_action_in_flight(&action_tasks, BackgroundActionKind::DeleteVariant) {
                    app.set_status("Variant delete already in progress.");
                    continue;
                }

                let dry = request.dry;
                let variant_id = request.variant_id;
                app.set_status(format!(
                    "Deleting variant {variant_id} (dry={dry})..."
                ));
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
                    continue;
                }

                app.open_clone_form();
                app.set_status("Clone form open. Enter options or leave blank for defaults.");
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
                            run_with_api_timeout(service.create_session(
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
                    if chat_options_task.is_none() {
                        if let Some(actor) = app.chat_actor().cloned() {
                            let service = service.clone();
                            chat_options_task = Some(tokio::spawn(async move {
                                let result =
                                    run_with_api_timeout(service.fetch_actor_chat_options(&actor))
                                        .await;
                                (actor.id.clone(), result)
                            }));
                        }
                    }
                    "Chat panel shown."
                } else {
                    "Chat panel hidden."
                };
                app.set_status(status);
            }
            LoopAction::OpenChatCompose => {
                if app.open_chat_composer() {
                    if chat_options_task.is_none() {
                        if let Some(actor) = app.chat_actor().cloned() {
                            let service = service.clone();
                            chat_options_task = Some(tokio::spawn(async move {
                                let result =
                                    run_with_api_timeout(service.fetch_actor_chat_options(&actor))
                                        .await;
                                (actor.id.clone(), result)
                            }));
                        }
                    }
                    app.set_status("Chat compose mode enabled.");
                } else {
                    app.set_status("Chat compose skipped: select an actor first.");
                }
            }
            LoopAction::SendChatMessage => {
                let Some(actor) = app.chat_actor().cloned() else {
                    app.set_status("Chat send skipped: no actor selected.");
                    continue;
                };
                let actor_id = actor.id.clone();
                let Some(prompt) = app.current_chat_prompt() else {
                    app.set_status("Chat send skipped: prompt is empty.");
                    continue;
                };

                if chat_send_task.is_some() {
                    app.set_status("Chat send already in progress.");
                    continue;
                }

                app.set_status("Queueing prompt to OpenCode session...");
                let service = service.clone();
                let selected_model = app.chat_active_model().map(ToString::to_string);
                let selected_agent = app.chat_active_agent().map(ToString::to_string);
                app.set_chat_send_in_flight(true);
                chat_send_task = Some(tokio::spawn(async move {
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

    Ok(())
}

async fn run_with_api_timeout<T>(future: impl Future<Output = Result<T>>) -> Result<T> {
    match tokio::time::timeout(Duration::from_secs(API_TIMEOUT_SECONDS), future).await {
        Ok(result) => result,
        Err(_) => Err(anyhow!("request timed out after {}s", API_TIMEOUT_SECONDS)),
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
    if app.is_delete_variant_form_open() {
        return handle_delete_variant_form_key(app, key);
    }

    if app.is_clone_form_open() {
        return handle_clone_form_key(app, key);
    }

    if app.is_spawn_form_open() {
        return handle_spawn_form_key(app, key);
    }

    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return LoopAction::Quit;
    }

    if app.chat_picker_open().is_some() {
        return handle_chat_picker_key(app, key);
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
        KeyCode::Char('x') => LoopAction::OpenCloneForm,
        KeyCode::Char('d') => LoopAction::OpenDeleteVariantForm,
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
