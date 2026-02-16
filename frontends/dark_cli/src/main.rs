mod cli;
mod output;
mod runner;

use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;
use dark_rust::DarkCoreClient;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  let cli = Cli::parse();
  let api = DarkCoreClient::new(cli.base_url.clone());

  runner::run(cli, &api).await
}
