use clap::{ArgAction, Parser};

#[derive(Debug, Parser)]
#[command(name = "dark_tui", about = "Dark Factory TUI frontend")]
pub struct Cli {
    #[arg(
        long,
        env = "DARK_CORE_BASE_URL",
        default_value = "http://localhost:4150"
    )]
    pub base_url: String,

    #[arg(long, env = "DARK_TUI_DIRECTORY")]
    pub directory: Option<String>,

    #[arg(long, env = "DARK_TUI_REFRESH_SECONDS", default_value_t = 8)]
    pub refresh_seconds: u64,

    #[arg(
        long,
        env = "DARK_TUI_POLL_VARIANTS",
        default_value_t = true,
        action = ArgAction::Set
    )]
    pub poll_variants: bool,
}
