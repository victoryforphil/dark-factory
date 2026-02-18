mod cli;
mod logging;
mod output;
mod runner;

use std::env;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use tracing::{error, info};

use crate::cli::Cli;
use dark_rust::{
    DarkCoreClient, DarkCoreLaunchConfig, EnsureDarkCoreState, ensure_dark_core_in_tmux_if_needed,
    is_local_dark_core_url,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let log_path = logging::init()?;
    info!(
        base_url = %cli.base_url,
        log_path = %log_path.display(),
        "Dark CLI // Startup // Logger initialized"
    );

    if should_manage_local_dark_core() && is_local_dark_core_url(&cli.base_url) {
        let launch_config = compiled_launch_config();
        let launch_state = ensure_dark_core_in_tmux_if_needed(&cli.base_url, launch_config).await?;

        match launch_state {
            EnsureDarkCoreState::AlreadyRunning => {}
            EnsureDarkCoreState::LaunchedTmux => {
                info!("Dark CLI // Runtime // Started dark_core in tmux session");
            }
            EnsureDarkCoreState::RestartedTmux => {
                info!("Dark CLI // Runtime // Restarted dark_core tmux session");
            }
            EnsureDarkCoreState::WaitingForTmuxSession => {
                info!("Dark CLI // Runtime // Reused existing tmux dark_core session");
            }
        }
    }

    let api = DarkCoreClient::new(cli.base_url.clone());

    let result = runner::run(cli, &api).await;
    if let Err(error) = &result {
        error!(error = %error, "Dark CLI // Run // Command failed");
    }
    result
}

fn compiled_launch_config() -> DarkCoreLaunchConfig {
    let mut config = DarkCoreLaunchConfig::default();

    if let Some(path) = option_env!("DARKFACTORY_DARK_CORE_EXECUTABLE") {
        config.executable_path = Some(PathBuf::from(path));
    }

    if let Some(path) = option_env!("DARKFACTORY_DARK_CORE_WORKDIR") {
        config.workdir = Some(PathBuf::from(path));
    }

    config
}

fn should_manage_local_dark_core() -> bool {
    env::var("DARK_CLI_AUTO_START_DARK_CORE")
        .ok()
        .map(|value| value.trim().eq_ignore_ascii_case("true") || value.trim() == "1")
        .unwrap_or(true)
}
