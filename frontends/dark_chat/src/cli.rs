use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ProviderKind {
    #[value(name = "opencode/server", alias = "opencode")]
    OpencodeServer,
}

#[derive(Debug, Parser)]
#[command(name = "dark_chat", about = "Dark Factory OpenCode chat TUI")]
pub struct Cli {
    #[arg(
        long,
        env = "DARK_CHAT_BASE_URL",
        default_value = "http://127.0.0.1:4096"
    )]
    pub base_url: String,

    #[arg(long, env = "DARK_CHAT_DIRECTORY")]
    pub directory: Option<String>,

    #[arg(long, env = "DARK_CHAT_REFRESH_SECONDS", default_value_t = 3)]
    pub refresh_seconds: u64,

    #[arg(long, env = "DARK_CHAT_SESSION")]
    pub session: Option<String>,

    #[arg(long, env = "DARK_CHAT_SESSION_TITLE")]
    pub session_title: Option<String>,

    #[arg(
        long,
        env = "DARK_CHAT_PROVIDER",
        value_enum,
        default_value_t = ProviderKind::OpencodeServer
    )]
    pub provider: ProviderKind,
}
