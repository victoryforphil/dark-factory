use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Pretty,
    Json,
    Toml,
}

#[derive(Debug, Parser)]
#[command(name = "dark_cli", about = "Dark Factory CLI frontend")]
pub struct Cli {
    #[arg(
        long,
        env = "DARK_CORE_BASE_URL",
        default_value = "http://localhost:4150"
    )]
    pub base_url: String,

    #[arg(long, value_enum, env = "DARK_CLI_FORMAT", default_value_t = OutputFormat::Pretty)]
    pub format: OutputFormat,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Init { path: Option<String> },
    Service(ServiceCommand),
    System(SystemCommand),
    Products(ProductsCommand),
    Opencode(OpencodeCommand),
}

#[derive(Debug, Args)]
pub struct ServiceCommand {
    #[command(subcommand)]
    pub action: ServiceAction,
}

#[derive(Debug, Subcommand)]
pub enum ServiceAction {
    Status,
}

#[derive(Debug, Args)]
pub struct SystemCommand {
    #[command(subcommand)]
    pub action: SystemAction,
}

#[derive(Debug, Subcommand)]
pub enum SystemAction {
    Health,
    Info,
    Metrics,
}

#[derive(Debug, Args)]
pub struct ProductsCommand {
    #[command(subcommand)]
    pub action: ProductsAction,
}

#[derive(Debug, Subcommand)]
pub enum ProductsAction {
    List {
        #[arg(long)]
        cursor: Option<String>,
        #[arg(long)]
        limit: Option<u32>,
    },
    Create {
        #[arg(long)]
        locator: String,
        #[arg(long)]
        display_name: Option<String>,
    },
}

#[derive(Debug, Args)]
pub struct OpencodeCommand {
    #[command(subcommand)]
    pub action: OpencodeAction,
}

#[derive(Debug, Subcommand)]
pub enum OpencodeAction {
    State {
        #[arg(long)]
        directory: String,
    },
    Sessions(OpencodeSessionsCommand),
}

#[derive(Debug, Args)]
pub struct OpencodeSessionsCommand {
    #[command(subcommand)]
    pub action: OpencodeSessionsAction,
}

#[derive(Debug, Subcommand)]
pub enum OpencodeSessionsAction {
    List {
        #[arg(long)]
        directory: String,
    },
    Create {
        #[arg(long)]
        directory: String,
        #[arg(long)]
        title: Option<String>,
    },
    Get {
        #[arg(long)]
        id: String,
        #[arg(long)]
        directory: String,
        #[arg(long)]
        include_messages: bool,
    },
    Attach {
        #[arg(long)]
        id: String,
        #[arg(long)]
        directory: String,
        #[arg(long)]
        model: Option<String>,
        #[arg(long)]
        agent: Option<String>,
    },
    Command {
        #[arg(long)]
        id: String,
        #[arg(long)]
        directory: String,
        #[arg(long)]
        command: String,
    },
    Prompt {
        #[arg(long)]
        id: String,
        #[arg(long)]
        directory: String,
        #[arg(long)]
        prompt: String,
        #[arg(long)]
        no_reply: bool,
    },
    Abort {
        #[arg(long)]
        id: String,
        #[arg(long)]
        directory: String,
    },
    Delete {
        #[arg(long)]
        id: String,
        #[arg(long)]
        directory: String,
    },
}
