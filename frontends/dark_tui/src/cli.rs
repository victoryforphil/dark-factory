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

    #[arg(long, env = "DARK_TUI_REFRESH_SECONDS", default_value_t = 2)]
    pub refresh_seconds: u64,

    #[arg(long, env = "DARK_TUI_ACTOR_AUTO_POLL_SECONDS", default_value_t = 5)]
    pub actor_auto_poll_seconds: u64,

    #[arg(long, env = "DARK_TUI_CHAT_HISTORY_LIMIT", default_value_t = 80)]
    pub chat_history_limit: u32,

    #[arg(long, env = "DARK_TUI_CHAT_RENDER_LIMIT", default_value_t = 40)]
    pub chat_render_limit: usize,

    #[arg(long, env = "DARK_TUI_CHAT_MAX_BODY_LINES", default_value_t = 24)]
    pub chat_max_body_lines: usize,

    #[arg(long, env = "DARK_TUI_CHAT_MESSAGE_MAX_CHARS", default_value_t = 12000)]
    pub chat_message_max_chars: usize,

    #[arg(
        long,
        env = "DARK_TUI_POLL_VARIANTS",
        default_value_t = true,
        action = ArgAction::Set
    )]
    pub poll_variants: bool,
}
