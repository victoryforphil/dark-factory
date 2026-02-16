use anyhow::Result;
use clap::Parser;

use dark_chat::cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = pretty_env_logger::try_init();

    let cli = Cli::parse();
    dark_chat::tui::run(cli).await
}
