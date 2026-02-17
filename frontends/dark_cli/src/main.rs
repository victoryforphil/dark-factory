mod cli;
mod logging;
mod output;
mod runner;

use anyhow::Result;
use clap::Parser;
use tracing::{error, info};

use crate::cli::Cli;
use dark_rust::DarkCoreClient;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let log_path = logging::init()?;
    info!(
        base_url = %cli.base_url,
        log_path = %log_path.display(),
        "Dark CLI // Startup // Logger initialized"
    );

    let api = DarkCoreClient::new(cli.base_url.clone());

    let result = runner::run(cli, &api).await;
    if let Err(error) = &result {
        error!(error = %error, "Dark CLI // Run // Command failed");
    }
    result
}
