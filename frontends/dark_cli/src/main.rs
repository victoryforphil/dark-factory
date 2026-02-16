mod api;
mod cli;
mod errors;
mod output;
mod runner;

use anyhow::Result;
use clap::Parser;

use crate::api::ApiClient;
use crate::cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  let cli = Cli::parse();
  let api = ApiClient::new(cli.base_url.clone());

  runner::run(cli, &api).await
}
