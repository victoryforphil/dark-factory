mod app;
mod cli;
mod models;
mod service;
pub(crate) mod theme;
mod ui;

use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = pretty_env_logger::try_init();

    let cli = Cli::parse();
    ui::run(cli).await
}
