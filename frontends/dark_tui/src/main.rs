mod app;
mod cli;
mod logging;
mod models;
mod service;
mod service_convert;
mod service_wire;
pub(crate) mod theme;
mod ui;

use std::env;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use dark_rust::{
    DarkCoreLaunchConfig, EnsureDarkCoreState, ensure_dark_core_in_tmux_if_needed,
    is_local_dark_core_url,
};

use crate::cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let manage_local_dark_core = should_manage_local_dark_core();
    let core_runtime_hint = if manage_local_dark_core && is_local_dark_core_url(&cli.base_url) {
        let launch_config = compiled_launch_config();
        let launch_state = ensure_dark_core_in_tmux_if_needed(&cli.base_url, launch_config).await?;
        match launch_state {
            EnsureDarkCoreState::AlreadyRunning => "core:running".to_string(),
            EnsureDarkCoreState::LaunchedTmux => "core:tmux-launched".to_string(),
            EnsureDarkCoreState::RestartedTmux => "core:tmux-restarted".to_string(),
            EnsureDarkCoreState::WaitingForTmuxSession => "core:tmux-existing".to_string(),
        }
    } else if !manage_local_dark_core {
        "core:auto-off".to_string()
    } else {
        "core:remote".to_string()
    };

    ui::run(cli, core_runtime_hint).await
}

fn compiled_launch_config() -> DarkCoreLaunchConfig {
    let mut config = DarkCoreLaunchConfig::default();
    config.restart_existing_session = false;

    if let Some(path) = option_env!("DARKFACTORY_DARK_CORE_EXECUTABLE") {
        config.executable_path = Some(PathBuf::from(path));
    }

    if let Some(path) = option_env!("DARKFACTORY_DARK_CORE_WORKDIR") {
        config.workdir = Some(PathBuf::from(path));
    }

    config
}

fn should_manage_local_dark_core() -> bool {
    env::var("DARK_TUI_AUTO_START_DARK_CORE")
        .ok()
        .map(|value| value.trim().eq_ignore_ascii_case("true") || value.trim() == "1")
        .unwrap_or(true)
}
