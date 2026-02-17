mod app;
mod cli;
mod logging;
mod models;
mod service;
mod service_convert;
mod service_wire;
pub(crate) mod theme;
mod ui;

use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    ui::run(cli).await
}
